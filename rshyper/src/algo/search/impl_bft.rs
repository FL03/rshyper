/*
    Appellation: bft <module>
    Contrib: @FL03
*/

use super::BreadthFirstTraversal;
use crate::{Error, HashGraph, Result, Search, VertexId};
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

impl<'a, N, E> BreadthFirstTraversal<'a, N, E>
where
    E: Eq + Hash,
    N: Eq + Hash,
{
    /// Create a new BreadthFirstTraversal instance
    pub(crate) fn new(graph: &'a HashGraph<N, E>) -> Self {
        Self {
            graph,
            queue: VecDeque::new(),
            visited: HashSet::new(),
        }
    }

    /// Reset the traversal state to allow reusing the instance
    pub fn reset(&mut self) {
        self.queue.clear();
        self.visited.clear();
    }
}

impl<'a, N, E> Search<N> for BreadthFirstTraversal<'a, N, E>
where
    E: Eq + Hash,
    N: Eq + Hash,
{
    fn search(&mut self, start: VertexId) -> Result<Vec<VertexId>> {
        // Reset state
        self.reset();

        // Check if starting vertex exists
        if !self.graph.contains_node(&start) {
            return Err(Error::VertexDoesNotExist(start));
        }

        // Add start vertex to queue and mark as visited
        self.queue.push_back(start);
        self.visited.insert(start);

        // Path to return (traversal order)
        let mut path = Vec::new();

        // BFT algorithm
        while let Some(current) = self.queue.pop_front() {
            path.push(current);

            // Get all hyperedges containing the current vertex
            let edges = self.graph.get_edges_with_vertex(current)?;

            // For each hyperedge, visit all vertices that haven't been visited yet
            for edge_id in edges {
                let vertices = self.graph.get_edge_vertices(edge_id)?;

                for &vertex in vertices {
                    if !self.visited.contains(&vertex) {
                        self.queue.push_back(vertex);
                        self.visited.insert(vertex);
                    }
                }
            }
        }

        Ok(path)
    }

    fn has_visited(&self, vertex: VertexId) -> bool {
        self.visited.contains(&vertex)
    }

    fn visited_vertices(&self) -> &HashSet<VertexId> {
        &self.visited
    }
}
