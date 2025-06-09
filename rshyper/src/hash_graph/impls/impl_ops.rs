/*
    appellation: impl_ops <module>
    authors: @FL03
*/
use crate::algo::search;
use crate::hash_graph::{HashFacet, HashGraph};
use core::hash::Hash;
use rshyper_core::index::{EdgeId, NumIndex, RawIndex, VertexId};
use rshyper_core::node::HyperNode;
use rshyper_core::{Combine, GraphAttributes, GraphKind};

/// implementations for various algorithms and operators on the hypergraph
impl<N, E, A, K, Idx> HashGraph<N, E, A>
where
    A: GraphAttributes<Idx = Idx, Kind = K>,
    E: Eq + Hash,
    N: Eq + Hash,
    K: GraphKind,
    Idx: Eq + RawIndex + Hash,
{
    /// search the hypergraph using the A* algorithm with the given heuristic function
    pub fn astar<F>(&self, heuristic: F) -> search::AStarSearch<'_, N, E, A, F>
    where
        F: search::Heuristic<Idx, Output = f64>,
    {
        search::AStarSearch::new(self, heuristic)
    }
    /// search the hypergraph using the depth-first traversal algorithm
    pub fn dft(&self) -> search::DepthFirstTraversal<'_, N, E, A, Self>
    where
        N: Default,
        E: Default,
        Idx: NumIndex,
    {
        search::DepthFirstTraversal::new(self)
    }
    /// search the hypergraph using the breadth-first traversal algorithm
    pub fn bft(&self) -> search::BreadthFirstTraversal<'_, N, E, A, Self>
    where
        N: Default,
        E: Default,
        Idx: NumIndex,
    {
        search::BreadthFirstTraversal::new(self)
    }
}

impl<N, E, A, K, Idx> Combine<EdgeId<Idx>, EdgeId<Idx>> for HashGraph<N, E, A>
where
    A: GraphAttributes<Idx = Idx, Kind = K>,
    E: Clone + Eq + Hash + core::ops::Add<Output = E>,
    N: Eq + Hash,
    K: GraphKind,
    Idx: NumIndex,
    for<'a> &'a E: core::ops::Add<Output = E>,
{
    type Output = EdgeId<Idx>;

    fn combine(&mut self, src: EdgeId<Idx>, tgt: EdgeId<Idx>) -> crate::Result<Self::Output> {
        self.merge_edges(&src, &tgt)
    }
}

impl<'a, N, E, A, K, Idx> Combine<&'a EdgeId<Idx>, &'a EdgeId<Idx>> for HashGraph<N, E, A>
where
    A: GraphAttributes<Idx = Idx, Kind = K>,
    E: Eq + Hash,
    N: Eq + Hash,
    K: GraphKind,
    Idx: NumIndex,
    for<'b> &'b E: core::ops::Add<Output = E>,
{
    type Output = EdgeId<Idx>;

    fn combine(
        &mut self,
        src: &'a EdgeId<Idx>,
        tgt: &'a EdgeId<Idx>,
    ) -> crate::Result<Self::Output> {
        self.merge_edges(src, tgt)
    }
}

impl<N, E, A, K, Idx> core::ops::Index<&EdgeId<Idx>> for HashGraph<N, E, A>
where
    A: GraphAttributes<Idx = Idx, Kind = K>,
    E: Eq + Hash,
    N: Eq + Hash,
    K: GraphKind,
    Idx: Eq + RawIndex + Hash,
{
    type Output = HashFacet<E, K, Idx>;

    fn index(&self, index: &EdgeId<Idx>) -> &Self::Output {
        self.get_surface(index).expect("Edge not found")
    }
}

impl<N, E, A, K, Idx> core::ops::IndexMut<&EdgeId<Idx>> for HashGraph<N, E, A>
where
    A: GraphAttributes<Idx = Idx, Kind = K>,
    E: Eq + Hash,
    N: Eq + Hash,
    K: GraphKind,
    Idx: Eq + RawIndex + Hash,
{
    fn index_mut(&mut self, index: &EdgeId<Idx>) -> &mut Self::Output {
        self.get_surface_mut(index).expect("Edge not found")
    }
}

impl<N, E, A, K, Idx> core::ops::Index<&VertexId<Idx>> for HashGraph<N, E, A>
where
    A: GraphAttributes<Idx = Idx, Kind = K>,
    E: Eq + Hash,
    N: Eq + Hash,
    K: GraphKind,
    Idx: Eq + RawIndex + Hash,
{
    type Output = HyperNode<N, Idx>;

    fn index(&self, index: &VertexId<Idx>) -> &Self::Output {
        self.get_node(index).expect("Node not found")
    }
}

impl<N, E, A, K, Idx> core::ops::IndexMut<&VertexId<Idx>> for HashGraph<N, E, A>
where
    A: GraphAttributes<Idx = Idx, Kind = K>,
    E: Eq + Hash,
    N: Eq + Hash,
    K: GraphKind,
    Idx: Eq + RawIndex + Hash,
{
    fn index_mut(&mut self, index: &VertexId<Idx>) -> &mut Self::Output {
        self.get_node_mut(index).expect("Node not found")
    }
}
