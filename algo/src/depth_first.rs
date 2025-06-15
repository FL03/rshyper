/*
    Appellation: dft <module>
    Contrib: @FL03
*/
use crate::{Search, Traversal};
use core::hash::Hash;
use rshyper_core::edge::RawEdge;
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
    pub fn new(graph: &'a H) -> Self {
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
    /// returns an immutable reference to the visited vertices
    pub const fn visited(&self) -> &HashSet<VertexId<Idx>> {
        &self.visited
    }
    /// returns a mutable reference to the visited vertices
    pub const fn visited_mut(&mut self) -> &mut HashSet<VertexId<Idx>> {
        &mut self.visited
    }
    /// returns true if the vertex has been visited
    pub fn has_visited<Q>(&self, vertex: &Q) -> bool
    where
        Idx: Eq + Hash,
        Q: ?Sized + Eq + Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.visited().contains(vertex)
    }
    /// reset the traversal state
    pub fn reset(&mut self) -> &mut Self {
        #[cfg(feature = "tracing")]
        tracing::debug!("resetting the depth-first traversal operator state...");
        // clear the stack
        self.stack_mut().clear();
        // clear the visited set
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

impl<'a, N, E, A, H> Search<VertexId<A::Ix>> for DepthFirstTraversal<'a, N, E, A, H>
where
    A: GraphAttributes,
    H: HyperGraph<N, E, A>,
    A::Ix: NumIndex,
    for<'b> &'b <H::Edge<E> as RawEdge>::Store: IntoIterator<Item = &'b VertexId<A::Ix>>,
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

            // For each hyperedge, visit all vertices that haven't been visited yet
            for edge_id in self.graph.find_edges_with_node(&current) {
                let vertices = self.graph.get_edge_domain(&edge_id).expect("Edge is empty");

                // Add vertices in reverse order to maintain expected DFS behavior
                let mut new_vertices = vertices
                    .into_iter()
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

        Ok(path)
    }
}
