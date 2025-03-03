/*
    Appellation: motion <module>
    Contrib: @FL03
*/

use crate::nrt::{LPR, Tonnetz};
use rshyper::EdgeId;
use std::collections::{HashMap, HashSet, VecDeque};

/// Represents a sequence of transformations from one triad to another
#[derive(Debug, Clone)]
pub struct TransformationChain {
    /// The sequence of transformations to apply
    pub transforms: Vec<LPR>,
    /// The sequence of triad IDs visited
    pub path: Vec<EdgeId>,
}

/// Motion planning algorithm for finding paths in the Tonnetz
pub struct MotionPlanner<'a> {
    tonnetz: &'a Tonnetz,
}

impl<'a> MotionPlanner<'a> {
    pub fn new(tonnetz: &'a Tonnetz) -> Self {
        MotionPlanner { tonnetz }
    }

    /// Find the shortest transformation path between two triads
    pub fn find_path(&self, start: EdgeId, goal: EdgeId) -> Option<TransformationChain> {
        if start == goal {
            return Some(TransformationChain {
                transforms: vec![],
                path: vec![start],
            });
        }

        // Use breadth-first search to find the shortest path
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut came_from: HashMap<EdgeId, (EdgeId, LPR)> = HashMap::new();

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
        came_from: &HashMap<EdgeId, (EdgeId, LPR)>,
    ) -> TransformationChain {
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

        TransformationChain { transforms, path }
    }

    /// Execute a transformation path on a UTM
    pub fn execute_path(&self, path: &TransformationChain) -> crate::Result<()> {
        if path.path.is_empty() {
            return Ok(());
        }

        // Get the starting triad
        let first_edge = path.path[0];
        let mut cplant = self
            .tonnetz
            .plants
            .get(&first_edge)
            .ok_or_else(|| rshyper::Error::HyperedgeDoesNotExist(first_edge.to_string()))?;

        // Ensure the triad has a UTM
        let mut utm = cplant.utm.clone();

        // Execute each transformation in sequence
        for (i, &transform) in path.transforms.iter().enumerate() {
            // Apply the transformation
            let nplant = cplant.transform(transform);
            utm.set_alphabet(*nplant.utm().alphabet());

            // Verify we reach the expected triad
            let next_edge = path.path[i + 1];
            let next_triad_data = self
                .tonnetz
                .plants
                .get(&next_edge)
                .ok_or_else(|| rshyper::Error::HyperedgeDoesNotExist(next_edge.to_string()))?;

            // Verify transformation result matches expected triad
            // (In a full system, we'd update the UTM's current state here)
            println!(
                "Transformed from {:?} to {:?} using {:?}",
                cplant.utm().alphabet(),
                next_triad_data.utm().alphabet(),
                transform
            );

            cplant = next_triad_data;
        }

        Ok(())
    }

    /// Find a transformation path to include a target symbol in the alphabet
    pub fn find_path_to_symbol(
        &self,
        start_triad: EdgeId,
        target_symbol: usize,
    ) -> Option<TransformationChain> {
        // Get the starting triad data
        let start_data = self.tonnetz.plants.get(&start_triad)?;

        // Check if the target symbol is already in the current triad
        if start_data.contains(&target_symbol) {
            return Some(TransformationChain {
                transforms: vec![],
                path: vec![start_triad],
            });
        }

        // Use breadth-first search to find a triad containing the target symbol
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut came_from: HashMap<EdgeId, (EdgeId, LPR)> = HashMap::new();

        queue.push_back(start_triad);
        visited.insert(start_triad);

        while let Some(current) = queue.pop_front() {
            // Check transformations from current triad
            if let Some(transformations) = self.tonnetz.transformations.get(&current) {
                for (&transform, &next) in transformations {
                    if !visited.contains(&next) {
                        visited.insert(next);
                        came_from.insert(next, (current, transform));
                        queue.push_back(next);

                        // Check if this triad contains our target symbol
                        if let Some(next_data) = self.tonnetz.plants.get(&next) {
                            if next_data.contains(&target_symbol) {
                                return Some(self.reconstruct_path(start_triad, next, &came_from));
                            }
                        }
                    }
                }
            }
        }

        // If we get here, we didn't find a path to a triad with the target symbol
        None
    }

    /// Find paths that allow alphabet expansion to include more symbols
    pub fn find_alphabet_expansion(
        &self,
        start_triad: EdgeId,
    ) -> HashMap<usize, TransformationChain> {
        let mut symbol_paths = HashMap::new();
        let start_data = match self.tonnetz.plants.get(&start_triad) {
            Some(data) => data,
            None => return symbol_paths,
        };

        // For each possible pitch class (0-11 in standard music theory)
        for symbol in 0..12 {
            // Skip symbols already in our alphabet
            if start_data.contains(&symbol) {
                continue;
            }

            // Find a path to include this symbol
            if let Some(path) = self.find_path_to_symbol(start_triad, symbol) {
                symbol_paths.insert(symbol, path);
            }
        }

        symbol_paths
    }

    /// Analyze available transformations from current triad
    pub fn analyze_transformations(&self, triad_id: EdgeId) -> Vec<(LPR, EdgeId, [usize; 3])> {
        let mut results = Vec::new();

        if let Some(transformations) = self.tonnetz.transformations.get(&triad_id) {
            for (&transform, &next_id) in transformations {
                if let Some(next_triad) = self.tonnetz.plants.get(&next_id) {
                    results.push((transform, next_id, *next_triad.alphabet()));
                }
            }
        }

        results
    }

    // Keep existing methods like reconstruct_path and execute_path

    /// Given a starting triad and target symbol, suggest transformations to add it
    pub fn suggest_transformations_for_symbol(
        &self,
        start: EdgeId,
        target_symbol: usize,
    ) -> Vec<(LPR, EdgeId)> {
        let mut suggestions = Vec::new();

        // Get starting triad data
        let start_data = match self.tonnetz.plants.get(&start) {
            Some(data) => data,
            None => return suggestions,
        };

        // For each transformation type
        for transform in [LPR::Leading, LPR::Parallel, LPR::Relative] {
            // Calculate what the resulting triad would be
            let next = start_data.transform(transform);

            // Check if the result contains our target symbol
            if next.contains(&target_symbol) {
                // Find the edge ID of this triad if it exists
                for (&edge_id, plant) in &self.tonnetz.plants {
                    if plant.utm().alphabet() == next.utm().alphabet() {
                        suggestions.push((transform, edge_id));
                        break;
                    }
                }

                // If the triad doesn't exist yet, suggest creating it
                if !suggestions.iter().any(|(t, _)| t == &transform) {
                    println!(
                        "Suggestion: Create new triad {:?} using {transform:?} transformation",
                        next.utm().alphabet()
                    );
                }
            }
        }

        suggestions
    }
}
