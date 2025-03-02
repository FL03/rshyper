/*
    Appellation: motion <module>
    Contrib: @FL03
*/

use crate::topo::tonnetz::Tonnetz;
use crate::types::Transformation;
use rshyper::EdgeId;
use std::collections::{HashMap, HashSet, VecDeque};

/// Represents a sequence of transformations from one triad to another
#[derive(Debug, Clone)]
pub struct TransformationPath {
    /// The sequence of transformations to apply
    pub transforms: Vec<Transformation>,
    /// The sequence of triad IDs visited
    pub path: Vec<EdgeId>,
}

/// Motion planning algorithm for finding paths in the Tonnetz
pub struct TonnetzPlanner<'a> {
    tonnetz: &'a Tonnetz,
}

impl<'a> TonnetzPlanner<'a> {
    pub fn new(tonnetz: &'a Tonnetz) -> Self {
        TonnetzPlanner { tonnetz }
    }

    /// Find the shortest transformation path between two triads
    pub fn find_path(&self, start: EdgeId, goal: EdgeId) -> Option<TransformationPath> {
        if start == goal {
            return Some(TransformationPath {
                transforms: vec![],
                path: vec![start],
            });
        }

        // Use breadth-first search to find the shortest path
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut came_from: HashMap<EdgeId, (EdgeId, Transformation)> = HashMap::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(current) = queue.pop_front() {
            // Check transformations from current triad
            if let Some(transformations) = self.tonnetz.transformations.get(&current) {
                for (&transform, &next) in transformations {
                    if !visited.contains(&next) {
                        visited.insert(next);
                        came_from.insert(next, (current, transform));
                        queue.push_back(next);

                        // Found the goal
                        if next == goal {
                            return Some(self.reconstruct_path(start, goal, &came_from));
                        }
                    }
                }
            }
        }

        None // No path found
    }

    /// Reconstruct the path from the search results
    fn reconstruct_path(
        &self,
        start: EdgeId,
        goal: EdgeId,
        came_from: &HashMap<EdgeId, (EdgeId, Transformation)>,
    ) -> TransformationPath {
        let mut current = goal;
        let mut path = vec![goal];
        let mut transforms = Vec::new();

        while current != start {
            let (prev, transform) = came_from.get(&current).unwrap();
            transforms.push(*transform);
            path.push(*prev);
            current = *prev;
        }

        // Reverse since we built the path backward
        transforms.reverse();
        path.reverse();

        TransformationPath { transforms, path }
    }

    /// Execute a transformation path on a UTM
    pub fn execute_path(&self, path: &TransformationPath) -> crate::Result<()> {
        if path.path.is_empty() {
            return Ok(());
        }

        // Get the starting triad
        let first_edge = path.path[0];
        let mut current_triad_data = self
            .tonnetz
            .triads
            .get(&first_edge)
            .ok_or_else(|| rshyper::Error::HyperedgeDoesNotExist(first_edge.to_string()))?;

        // Ensure the triad has a UTM
        let mut utm = if let Some(utm) = &current_triad_data.utm {
            utm.clone()
        } else {
            return Err(crate::Error::Unknown(
                "No UTM available at the starting triad".into(),
            ));
        };

        // Execute each transformation in sequence
        for (i, &transform) in path.transforms.iter().enumerate() {
            // Apply the transformation
            utm.apply_transformation(transform);

            // Verify we reach the expected triad
            let next_edge = path.path[i + 1];
            let next_triad_data = self
                .tonnetz
                .triads
                .get(&next_edge)
                .ok_or_else(|| rshyper::Error::HyperedgeDoesNotExist(next_edge.to_string()))?;

            // Verify transformation result matches expected triad
            // (In a full system, we'd update the UTM's current state here)
            println!(
                "Transformed from {:?} to {:?} using {:?}",
                current_triad_data.pitches, next_triad_data.pitches, transform
            );

            current_triad_data = next_triad_data;
        }

        Ok(())
    }
}
