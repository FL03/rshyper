/*
    Appellation: tonnetz <module>
    Contrib: @FL03
*/
use super::Triad;
use crate::{Transformation, TriadClass, WolframUTM};
use rshyper::prelude::{EdgeId, HyperGraph, VertexId};
use std::collections::HashMap;

/// Represents the global Tonnetz structure with UTMs as "plants"
#[derive(Clone, Debug)]
pub struct Tonnetz {
    /// The underlying hypergraph structure
    pub graph: HyperGraph<usize>,
    /// Maps EdgeIds to TriadData for efficient access
    pub triads: HashMap<EdgeId, Triad>,
    /// Tracks adjacency between triads via transformations
    pub transformations: HashMap<EdgeId, HashMap<Transformation, EdgeId>>,
}

impl Tonnetz {
    /// Create a new Tonnetz structure
    pub fn new() -> Self {
        Tonnetz {
            graph: HyperGraph::new(),
            triads: HashMap::new(),
            transformations: HashMap::new(),
        }
    }

    /// Add a new pitch class vertex to the Tonnetz
    pub fn add_pitch_class(&mut self, pitch: usize) -> VertexId {
        self.graph.add_vertex(pitch)
    }

    /// Add a new triad to the Tonnetz
    pub fn add_triad(&mut self, pitches: [usize; 3], class: TriadClass) -> crate::Result<EdgeId> {
        // Ensure we have vertices for all pitch classes
        let vertices: Vec<VertexId> = pitches
            .iter()
            .map(|&p| {
                // Try to find existing vertex with this pitch class
                if let Some(v) = self.find_vertex_by_pitch(p) {
                    v
                } else {
                    // Add new vertex if not found
                    self.add_pitch_class(p)
                }
            })
            .collect();

        // Add the hyperedge representing this triad
        let edge_id = self.graph.add_hyperedge(vertices)?;

        // Store the triad data
        self.triads.insert(
            edge_id,
            Triad {
                pitches,
                class,
                utm: None,
            },
        );

        Ok(edge_id)
    }

    /// Compute and store all possible transformations between triads
    pub fn compute_transformations(&mut self) {
        // Clear existing transformations
        self.transformations.clear();

        // For each pair of triads, check if there's a transformation between them
        let edges: Vec<EdgeId> = self.triads.keys().cloned().collect();

        for &edge1 in &edges {
            self.transformations.insert(edge1, HashMap::new());

            if let Some(triad1) = self.triads.get(&edge1) {
                for &edge2 in &edges {
                    if edge1 == edge2 {
                        continue;
                    }

                    if let Some(triad2) = self.triads.get(&edge2) {
                        // Check if there's a transformation from triad1 to triad2
                        if let Some(transform) = Self::get_transformation(triad1, triad2) {
                            self.transformations
                                .get_mut(&edge1)
                                .unwrap()
                                .insert(transform, edge2);
                        }
                    }
                }
            }
        }
    }

    /// Deploy a UTM to a specific triad
    pub fn deploy_utm(&mut self, edge_id: EdgeId, utm: WolframUTM) -> crate::Result<()> {
        if let Some(triad) = self.triads.get_mut(&edge_id) {
            triad.utm = Some(utm);
            Ok(())
        } else {
            Err(rshyper::Error::HyperedgeDoesNotExist(edge_id.to_string()).into())
        }
    }

    /// Apply a transformation to a triad
    fn apply_transform(
        pitches: [usize; 3],
        class: TriadClass,
        transform: Transformation,
    ) -> ([usize; 3], TriadClass) {
        use crate::utils::pymod;

        match (class, transform) {
            (TriadClass::Major, Transformation::Leading) => {
                let [x, y, z] = pitches;
                (
                    [y, z, pymod(x as isize - 1, 12) as usize],
                    TriadClass::Minor,
                )
            }
            (TriadClass::Minor, Transformation::Leading) => {
                let [x, y, z] = pitches;
                (
                    [pymod(z as isize + 1, 12) as usize, x, y],
                    TriadClass::Major,
                )
            }
            (TriadClass::Major, Transformation::Parallel) => {
                let [x, y, z] = pitches;
                (
                    [x, pymod(y as isize - 1, 12) as usize, z],
                    TriadClass::Minor,
                )
            }
            (TriadClass::Minor, Transformation::Parallel) => {
                let [x, y, z] = pitches;
                (
                    [x, pymod(y as isize + 1, 12) as usize, z],
                    TriadClass::Major,
                )
            }
            (TriadClass::Major, Transformation::Relative) => {
                let [x, y, ..] = pitches;
                (
                    [pymod(x as isize - 3, 12) as usize, x, y],
                    TriadClass::Minor,
                )
            }
            (TriadClass::Minor, Transformation::Relative) => {
                let [y, z, ..] = pitches;
                (
                    [y, z, pymod(z as isize + 3, 12) as usize],
                    TriadClass::Major,
                )
            }
            // Simplified handling for augmented and diminished
            (_, _) => (pitches, class),
        }
    }

    /// Find a vertex by its pitch class value
    fn find_vertex_by_pitch(&self, pitch: usize) -> Option<VertexId> {
        self.graph
            .vertices()
            .iter()
            .find(|(_, node)| *node.weight() == pitch)
            .map(|(id, _)| *id)
    }

    /// Determine if there's a single transformation between two triads
    fn get_transformation(triad1: &Triad, triad2: &Triad) -> Option<Transformation> {
        // Leading transformation
        let leading_result =
            Self::apply_transform(triad1.pitches, triad1.class, Transformation::Leading);
        if leading_result.0 == triad2.pitches && leading_result.1 == triad2.class {
            return Some(Transformation::Leading);
        }

        // Parallel transformation
        let parallel_result =
            Self::apply_transform(triad1.pitches, triad1.class, Transformation::Parallel);
        if parallel_result.0 == triad2.pitches && parallel_result.1 == triad2.class {
            return Some(Transformation::Parallel);
        }

        // Relative transformation
        let relative_result =
            Self::apply_transform(triad1.pitches, triad1.class, Transformation::Relative);
        if relative_result.0 == triad2.pitches && relative_result.1 == triad2.class {
            return Some(Transformation::Relative);
        }

        None
    }
}
