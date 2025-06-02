/*
    Appellation: dft <module>
    Contrib: @FL03
*/
use super::{Search, Traversal};
use crate::hash_graph::HashGraph;
use rshyper_core::GraphKind;
use rshyper_core::index::{HashIndex, NumIndex, VertexId};
use std::collections::HashSet;

/// Depth-First Traversal algorithm for hypergraphs
pub struct DepthFirstTraversal<'a, N, E, K, Idx = crate::Udx>
where
    K: GraphKind,
    Idx: HashIndex,
{
    pub(crate) graph: &'a HashGraph<N, E, K, Idx>,
    pub(crate) stack: Vec<VertexId<Idx>>,
    pub(crate) visited: HashSet<VertexId<Idx>>,
}

impl<'a, N, E, K, Idx> DepthFirstTraversal<'a, N, E, K, Idx>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    K: GraphKind,
    Idx: HashIndex,
{
    /// Create a new DepthFirstTraversal instance
    pub(crate) fn new(graph: &'a HashGraph<N, E, K, Idx>) -> Self {
        Self {
            graph,
            stack: Vec::new(),
            visited: HashSet::new(),
        }
    }
    /// returns an immutable reference to the stack
    pub const fn stack(&self) -> &Vec<VertexId<Idx>> {
        &self.stack
    }
    /// returns a mutable reference to the stack
    pub const fn stack_mut(&mut self) -> &mut Vec<VertexId<Idx>> {
        &mut self.stack
    }
    /// returns an immutable reference to the indices of the visited nodes
    pub const fn visited(&self) -> &HashSet<VertexId<Idx>> {
        &self.visited
    }
    /// returns a mutable reference to the indices of the visited nodes
    pub const fn visited_mut(&mut self) -> &mut HashSet<VertexId<Idx>> {
        &mut self.visited
    }
    /// reset the traversal state
    pub fn reset(&mut self) -> &mut Self {
        self.stack_mut().clear();
        self.visited_mut().clear();
        self
    }
    /// a convience method to perform a search
    pub fn search(
        &mut self,
        start: VertexId<Idx>,
    ) -> crate::Result<<Self as Search<VertexId<Idx>>>::Output>
    where
        Self: Search<VertexId<Idx>>,
        Idx: NumIndex,
    {
        Search::search(self, start)
    }
    /// include the given index in both the stack and visited stores
    pub fn register_vertex(&mut self, index: VertexId<Idx>) -> &mut Self
    where
        Idx: Copy,
    {
        self.stack_mut().push(index);
        self.visited_mut().insert(index);
        self
    }
}

impl<'a, N, E, K, Idx> Traversal<VertexId<Idx>> for DepthFirstTraversal<'a, N, E, K, Idx>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    K: GraphKind,
    Idx: HashIndex,
{
    type Store<I2> = HashSet<I2>;

    fn has_visited(&self, vertex: &VertexId<Idx>) -> bool {
        self.visited().contains(vertex)
    }

    fn visited(&self) -> &Self::Store<VertexId<Idx>> {
        &self.visited
    }
}

impl<'a, N, E, K, Idx> Search<VertexId<Idx>> for DepthFirstTraversal<'a, N, E, K, Idx>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    K: GraphKind,
    Idx: NumIndex,
{
    type Output = Vec<VertexId<Idx>>;

    fn search(&mut self, start: VertexId<Idx>) -> crate::Result<Self::Output> {
        // Reset state
        self.reset();

        // Check if starting vertex exists
        if !self.graph.contains_node(&start) {
            return Err(crate::Error::NodeNotFound);
        }

        // Add start vertex to stack and mark as visited
        self.register_vertex(start);

        // Path to return (traversal order)
        let mut path = Vec::new();

        // DFT algorithm
        while let Some(current) = self.stack.pop() {
            path.push(current);

            // Get all hyperedges containing the current vertex
            if let Ok(edges) = self.graph.get_edges_with_vertex(&current) {
                // For each hyperedge, visit all vertices that haven't been visited yet
                for edge_id in edges {
                    let vertices = self.graph.get_edge_vertices(&edge_id)?;

                    // Add vertices in reverse order to maintain expected DFS behavior
                    let mut new_vertices = vertices
                        .iter()
                        .filter(|&v| !self.visited.contains(v))
                        .copied()
                        .collect::<Vec<_>>();

                    // Sort in reverse order (arbitrary but consistent)
                    new_vertices.sort_by(|&a, &b| b.cmp(a));

                    for v in new_vertices {
                        // add the index to the stack and indicate it has been viewed
                        self.register_vertex(v);
                    }
                }
            }
        }

        Ok(path)
    }
}
