/*
    Appellation: bft <module>
    Contrib: @FL03
*/
use crate::hash_graph::HashGraph;
use rshyper_core::{Error, VertexId};
use std::collections::{HashSet, VecDeque};

use super::{Search, Traversal};

/// Breadth-First Traversal algorithm for hypergraphs
pub struct BreadthFirstTraversal<'a, N, E> {
    pub(crate) graph: &'a HashGraph<N, E>,
    pub(crate) queue: VecDeque<VertexId>,
    pub(crate) visited: HashSet<VertexId>,
}

impl<'a, N, E> BreadthFirstTraversal<'a, N, E>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
{
    /// create a new instance from a hypergraph
    pub(crate) fn from_hypergraph(graph: &'a HashGraph<N, E>) -> Self {
        Self {
            graph,
            queue: VecDeque::new(),
            visited: HashSet::new(),
        }
    }
    /// returns an immutable reference to the queue
    pub const fn queue(&self) -> &VecDeque<VertexId> {
        &self.queue
    }
    /// returns a mutable reference to the queue
    pub const fn queue_mut(&mut self) -> &mut VecDeque<VertexId> {
        &mut self.queue
    }
    /// returns an immutable reference to the visited vertices
    pub const fn visited(&self) -> &HashSet<VertexId> {
        &self.visited
    }
    /// returns a mutable reference to the visited vertices
    pub const fn visited_mut(&mut self) -> &mut HashSet<VertexId> {
        &mut self.visited
    }
    /// Reset the traversal state to allow reusing the instance
    pub fn reset(&mut self) -> &mut Self {
        self.queue_mut().clear();
        self.visited_mut().clear();
        self
    }
    /// a convience method to perform a search
    pub fn search(&mut self, start: VertexId) -> crate::Result<Vec<VertexId>> {
        Search::search(self, start)
    }
}

impl<'a, N, E> Search<VertexId> for BreadthFirstTraversal<'a, N, E>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
{
    type Output = Vec<VertexId>;

    fn search(&mut self, start: VertexId) -> crate::Result<Self::Output> {
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
            if let Ok(edges) = self.graph.get_edges_with_vertex(&current) {
                // visit all vertices within each edge that haven't been visited yet
                for edge_id in edges {
                    for &vertex in self.graph.get_vertices_for_edge(&edge_id)? {
                        if !self.has_visited(&vertex) {
                            self.queue.push_back(vertex);
                            self.visited.insert(vertex);
                        }
                    }
                }
            }
        }

        Ok(path)
    }
}

impl<'a, N, E> Traversal<VertexId> for BreadthFirstTraversal<'a, N, E>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
{
    type Store<I2> = HashSet<I2>;

    fn has_visited(&self, vertex: &VertexId) -> bool {
        self.visited().contains(vertex)
    }

    fn visited(&self) -> &Self::Store<VertexId> {
        &self.visited
    }
}
