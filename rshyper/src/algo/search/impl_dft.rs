/*
    Appellation: dft <module>
    Contrib: @FL03
*/

use super::DepthFirstTraversal;
use crate::{Error, HashGraph, Result, Search, VertexId};
use std::collections::HashSet;
use std::hash::Hash;

impl<'a, N, E> DepthFirstTraversal<'a, N, E>
where
    E: Eq + Hash,
    N: Eq + Hash,
{
    /// Create a new DepthFirstTraversal instance
    pub(crate) fn new(graph: &'a HashGraph<N, E>) -> Self {
        Self {
            graph,
            stack: Vec::new(),
            visited: HashSet::new(),
        }
    }

    /// Reset the traversal state
    pub fn reset(&mut self) {
        self.stack.clear();
        self.visited.clear();
    }
}

impl<'a, N, E> Search<N> for DepthFirstTraversal<'a, N, E>
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

        // Add start vertex to stack and mark as visited
        self.stack.push(start);
        self.visited.insert(start);

        // Path to return (traversal order)
        let mut path = Vec::new();

        // DFT algorithm
        while let Some(current) = self.stack.pop() {
            path.push(current);

            // Get all hyperedges containing the current vertex
            let edges = self.graph.get_edges_with_vertex(current)?;

            // For each hyperedge, visit all vertices that haven't been visited yet
            for edge_id in edges {
                let vertices = self.graph.get_edge_vertices(edge_id)?;

                // Add vertices in reverse order to maintain expected DFS behavior
                let mut new_vertices: Vec<_> = vertices
                    .iter()
                    .filter(|&&v| !self.visited.contains(&v))
                    .cloned()
                    .collect();

                // Sort in reverse order (arbitrary but consistent)
                new_vertices.sort_by(|a, b| b.cmp(a));

                for vertex in new_vertices {
                    self.stack.push(vertex);
                    self.visited.insert(vertex);
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
