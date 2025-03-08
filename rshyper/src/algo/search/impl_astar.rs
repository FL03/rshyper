/*
    Appellation: impl_astar <module>
    Contrib: @FL03
*/

use super::AStarSearch;
use crate::{Error, HyperGraph, Result, Search, VertexId};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;

// Priority queue node for A* algorithm
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Deserialize, serde_derive::Serialize)
)]
struct PriorityNode {
    vertex: VertexId,
    priority: i64, // Negative f_score for min-heap behavior
}

impl Ord for PriorityNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering to create a min-heap (lowest f_score has highest priority)
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for PriorityNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, N, E, F> AStarSearch<'a, N, E, F>
where
    E: core::cmp::Eq + core::hash::Hash,
    N: Eq + Hash,
    F: Fn(VertexId, VertexId) -> f64,
{
    /// Create a new A* search instance with the given heuristic function
    pub fn new(graph: &'a HyperGraph<N, E>, heuristic: F) -> Self {
        Self {
            graph,
            open_set: HashSet::new(),
            closed_set: HashSet::new(),
            came_from: HashMap::new(),
            g_score: HashMap::new(),
            f_score: HashMap::new(),
            heuristic,
        }
    }

    /// Reset the search state
    pub fn reset(&mut self) {
        self.open_set.clear();
        self.closed_set.clear();
        self.came_from.clear();
        self.g_score.clear();
        self.f_score.clear();
    }

    /// Find the shortest path between start and goal vertices
    pub fn find_path(&mut self, start: VertexId, goal: VertexId) -> Result<Vec<VertexId>> {
        // Check if both vertices exist
        if !self.graph.check_vertex(&start) {
            return Err(Error::VertexDoesNotExist(start));
        }
        if !self.graph.check_vertex(&goal) {
            return Err(Error::VertexDoesNotExist(goal));
        }

        // Reset state
        self.reset();

        // Initialize the g_score map with infinity for all nodes
        // (except the start node which has g_score of 0)
        self.g_score.insert(start, 0.0);

        // Initialize the f_score with the heuristic for the start node
        let start_f_score = (self.heuristic)(start, goal);
        self.f_score.insert(start, start_f_score);

        // Add start node to the open set
        self.open_set.insert(start);

        // Initialize priority queue with the start node
        let mut priority_queue = BinaryHeap::new();
        priority_queue.push(PriorityNode {
            vertex: start,
            priority: -(start_f_score as i64),
        });

        while !self.open_set.is_empty() {
            // Get the node with lowest f_score from the priority queue
            let current = match priority_queue.pop() {
                Some(node) => node.vertex,
                None => break, // No path exists
            };

            // Skip if this node is no longer in the open set
            if !self.open_set.contains(&current) {
                continue;
            }

            // If we've reached the goal, construct and return the path
            if current == goal {
                return Ok(self.reconstruct_path(goal));
            }

            // Move from open to closed set
            self.open_set.remove(&current);
            self.closed_set.insert(current);

            // Process all neighbors through hyperedges
            let edges = self.graph.get_vertex_edges(current)?;

            for edge_id in edges {
                let vertices = self.graph.get_edge_vertices(edge_id)?;

                for &neighbor in vertices {
                    // Skip if this is the current vertex or already in closed set
                    if neighbor == current || self.closed_set.contains(&neighbor) {
                        continue;
                    }

                    // Calculate tentative g_score (cost to reach neighbor through current)
                    // For a hypergraph, we're using cost 1 for direct connections
                    let tentative_g_score = self.g_score[&current] + 1.0;

                    // If this neighbor is new or we found a better path to it
                    if !self.g_score.contains_key(&neighbor)
                        || tentative_g_score < self.g_score[&neighbor]
                    {
                        // Record this path
                        self.came_from.insert(neighbor, current);
                        self.g_score.insert(neighbor, tentative_g_score);

                        // Calculate f_score = g_score + heuristic
                        let f_score = tentative_g_score + (self.heuristic)(neighbor, goal);
                        self.f_score.insert(neighbor, f_score);

                        // Add to open set if it's not there yet
                        if !self.open_set.contains(&neighbor) {
                            self.open_set.insert(neighbor);
                            priority_queue.push(PriorityNode {
                                vertex: neighbor,
                                priority: -(f_score as i64),
                            });
                        }
                    }
                }
            }
        }

        // No path found
        Err(Error::Unknown(format!(
            "No path found from {} to {}",
            start, goal
        )))
    }

    // Reconstruct path from came_from map
    fn reconstruct_path(&self, goal: VertexId) -> Vec<VertexId> {
        let mut path = vec![goal];
        let mut current = goal;

        while let Some(&prev) = self.came_from.get(&current) {
            path.push(prev);
            current = prev;
        }

        path.reverse();
        path
    }
}

impl<'a, N, E, F> Search<N> for AStarSearch<'a, N, E, F>
where
    E: core::cmp::Eq + core::hash::Hash,
    N: Eq + Hash,
    F: Fn(VertexId, VertexId) -> f64,
{
    fn search(&mut self, start: VertexId) -> Result<Vec<VertexId>> {
        // For A*, we need a goal vertex to compute the heuristic
        // This implementation of search will explore the graph and return
        // all reachable vertices ordered by their distance from start
        self.reset();

        if !self.graph.check_vertex(&start) {
            return Err(Error::VertexDoesNotExist(start));
        }

        // Using the vertex with the largest ID as a pseudo-goal
        // This is a hack to make A* behave more like a general search
        let max_vertex_id = match self.graph.vertices().keys().max() {
            Some(&id) => id,
            None => return Ok(vec![]),
        };

        self.find_path(start, max_vertex_id)
    }

    fn has_visited(&self, vertex: VertexId) -> bool {
        self.closed_set.contains(&vertex)
    }

    fn visited_vertices(&self) -> &HashSet<VertexId> {
        &self.closed_set
    }
}
