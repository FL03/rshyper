/*
    Appellation: dft <module>
    Contrib: @FL03
*/
use super::{Search, Traversal};
use crate::hash_graph::HashGraph;
use rshyper_core::{Error, VertexId};
use std::collections::HashSet;

/// Depth-First Traversal algorithm for hypergraphs
pub struct DepthFirstTraversal<'a, N, E> {
    pub(crate) graph: &'a HashGraph<N, E>,
    pub(crate) stack: Vec<VertexId>,
    pub(crate) visited: HashSet<VertexId>,
}

impl<'a, N, E> DepthFirstTraversal<'a, N, E>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
{
    /// Create a new DepthFirstTraversal instance
    pub(crate) fn new(graph: &'a HashGraph<N, E>) -> Self {
        Self {
            graph,
            stack: Vec::new(),
            visited: HashSet::new(),
        }
    }
    /// returns an immutable reference to the stack
    pub const fn stack(&self) -> &Vec<VertexId> {
        &self.stack
    }
    /// returns a mutable reference to the stack
    pub const fn stack_mut(&mut self) -> &mut Vec<VertexId> {
        &mut self.stack
    }
    /// returns an immutable reference to the indices of the visited nodes
    pub const fn visited(&self) -> &HashSet<VertexId> {
        &self.visited
    }
    /// returns a mutable reference to the indices of the visited nodes
    pub const fn visited_mut(&mut self) -> &mut HashSet<VertexId> {
        &mut self.visited
    }
    /// reset the traversal state
    pub fn reset(&mut self) {
        self.stack_mut().clear();
        self.visited_mut().clear();
    }
    /// a convience method to perform a search
    pub fn search(&mut self, start: VertexId) -> crate::Result<Vec<VertexId>> {
        Search::search(self, start)
    }
    /// include the given index in both the stack and visited stores
    pub fn register_vertex(&mut self, index: VertexId) -> &mut Self {
        self.stack_mut().push(index);
        self.visited_mut().insert(index);
        self
    }
}

impl<'a, N, E> Traversal<VertexId> for DepthFirstTraversal<'a, N, E>
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

impl<'a, N, E> Search<VertexId> for DepthFirstTraversal<'a, N, E>
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

        // Add start vertex to stack and mark as visited
        self.stack.push(start);
        self.visited.insert(start);

        // Path to return (traversal order)
        let mut path = Vec::new();

        // DFT algorithm
        while let Some(current) = self.stack.pop() {
            path.push(current);

            // Get all hyperedges containing the current vertex
            let edges = self.graph.get_edges_with_vertex(&current)?;

            // For each hyperedge, visit all vertices that haven't been visited yet
            for edge_id in edges {
                let vertices = self.graph.get_vertices_for_edge(&edge_id)?;

                // Add vertices in reverse order to maintain expected DFS behavior
                let mut new_vertices = vertices
                    .iter()
                    .filter(|&v| !self.has_visited(v))
                    .cloned()
                    .collect::<Vec<_>>();

                // Sort in reverse order (arbitrary but consistent) tbefore registering the indices
                new_vertices
                    .sort_by(|&a, &b| b.cmp(a));
                

                for v in new_vertices {
                    self.register_vertex(v);
                }
            }
        }

        Ok(path)
    }
}
