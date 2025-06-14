/*
    Appellation: dft <module>
    Contrib: @FL03
*/
use crate::algo::{Search, Traversal};
use crate::hash_graph::HashGraph;
use core::hash::Hash;
use rshyper_core::idx::{NumIndex, RawIndex, VertexId};
use rshyper_core::{GraphAttributes, GraphType, HyperGraph};
use std::collections::HashSet;

/// Depth-First Traversal algorithm for hypergraphs
pub struct DepthFirstTraversal<'a, N, E, A, H>
where
    A: GraphAttributes,
    H: HyperGraph<N, E, A>,
{
    pub(crate) graph: &'a H,
    pub(crate) stack: Vec<VertexId<A::Ix>>,
    pub(crate) visited: HashSet<VertexId<A::Ix>>,
    _marker: core::marker::PhantomData<(N, E)>,
}

impl<'a, N, E, H, A, K, Idx> DepthFirstTraversal<'a, N, E, A, H>
where
    A: GraphAttributes<Ix = Idx, Kind = K>,
    H: HyperGraph<N, E, A>,
    K: GraphType,
    Idx: RawIndex,
{
    /// Create a new DepthFirstTraversal instance
    pub(crate) fn new(graph: &'a H) -> Self {
        Self {
            graph,
            stack: Vec::new(),
            visited: HashSet::new(),
            _marker: core::marker::PhantomData::<(N, E)>,
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
    pub(crate) fn register_vertex(&mut self, index: VertexId<Idx>) -> &mut Self
    where
        Idx: Copy + Eq + Hash,
    {
        self.stack_mut().push(index);
        self.visited_mut().insert(index);
        self
    }
}

impl<'a, N, E, A, H> Traversal<VertexId<A::Ix>> for DepthFirstTraversal<'a, N, E, A, H>
where
    A: GraphAttributes,
    H: HyperGraph<N, E, A>,
    A::Ix: Eq + Hash,
{
    type Store<I2> = HashSet<I2>;

    fn has_visited(&self, vertex: &VertexId<A::Ix>) -> bool {
        self.visited().contains(vertex)
    }

    fn visited(&self) -> &Self::Store<VertexId<A::Ix>> {
        &self.visited
    }
}

impl<'a, N, E, A> Search<VertexId<A::Ix>> for DepthFirstTraversal<'a, N, E, A, HashGraph<N, E, A>>
where
    A: GraphAttributes,
    E: Eq + Hash,
    N: Eq + Hash,
    A::Ix: crate::NumIndex,
{
    type Output = Vec<VertexId<A::Ix>>;

    fn search(&mut self, start: VertexId<A::Ix>) -> crate::Result<Self::Output> {
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
            if let Ok(edges) = self.graph.find_edges_with_node(&current) {
                // For each hyperedge, visit all vertices that haven't been visited yet
                for edge_id in edges {
                    let vertices = self.graph.get_edge_vertices(&edge_id)?;

                    // Add vertices in reverse order to maintain expected DFS behavior
                    let mut new_vertices = vertices
                        .iter()
                        .filter(|&v| !self.has_visited(v))
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
