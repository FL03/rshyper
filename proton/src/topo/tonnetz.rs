/*
    Appellation: tonnetz <module>
    Contrib: @FL03
*/
use super::{Plant, Triad};
use crate::{Transformation, TriadClass, WolframUTM};
use rshyper::prelude::{EdgeId, HyperGraph, VertexId};
use std::collections::HashMap;

/// Represents the global Tonnetz structure with UTMs as "plants"
#[derive(Clone, Debug)]
pub struct Tonnetz {
    /// The underlying hypergraph structure
    pub graph: HyperGraph<usize>,
    /// Maps EdgeIds to TriadData for efficient access
    pub plants: HashMap<EdgeId, Plant>,
    /// Tracks adjacency between triads via transformations
    pub transformations: HashMap<EdgeId, HashMap<Transformation, EdgeId>>,
}

impl Tonnetz {
    /// Create a new Tonnetz structure
    pub fn new() -> Self {
        Tonnetz {
            graph: HyperGraph::new(),
            plants: HashMap::new(),
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
        let triad = Triad::new(pitches, class);
        // Store the triad data
        self.plants.insert(edge_id, Plant::new(triad));

        Ok(edge_id)
    }

    /// Compute and store all possible transformations between triads
    pub fn compute_transformations(&mut self) {
        // Clear existing transformations
        self.transformations.clear();

        // For each pair of triads, check if there's a transformation between them
        let edges: Vec<EdgeId> = self.plants.keys().cloned().collect();

        for &edge1 in &edges {
            self.transformations.insert(edge1, HashMap::new());

            if let Some(plant1) = self.plants.get(&edge1) {
                for &edge2 in &edges {
                    if edge1 == edge2 {
                        continue;
                    }

                    if let Some(plant2) = self.plants.get(&edge2) {
                        // Check if there's a transformation from triad1 to triad2
                        if let Some(transform) =
                            Self::get_transformation(&plant1.triad, &plant2.triad)
                        {
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
        if let Some(plant) = self.plants.get_mut(&edge_id) {
            plant.utm = utm;
            Ok(())
        } else {
            Err(rshyper::Error::HyperedgeDoesNotExist(edge_id.to_string()).into())
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
        let leading_result = triad1.apply_transform(Transformation::Leading);
        if leading_result == *triad2 {
            return Some(Transformation::Leading);
        }

        // Parallel transformation
        let parallel_result = triad1.apply_transform(Transformation::Parallel);
        if parallel_result == *triad2 {
            return Some(Transformation::Parallel);
        }

        // Relative transformation
        let relative_result = triad1.apply_transform(Transformation::Relative);
        if relative_result == *triad2 {
            return Some(Transformation::Relative);
        }

        None
    }
}
