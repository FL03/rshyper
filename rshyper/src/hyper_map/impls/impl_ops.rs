/*
    appellation: impl_ops <module>
    authors: @FL03
*/
use crate::algo;
use crate::hyper_map::{HashFacet, HyperMap};
use core::hash::{BuildHasher, Hash};
use rshyper_core::idx::{EdgeId, NumIndex, RawIndex, VertexId};
use rshyper_core::node::Node;
use rshyper_core::{Combine, GraphAttributes, GraphType};

/// implementations for various algorithms and operators on the hypergraph
impl<N, E, A, S> HyperMap<N, E, A, S>
where
    S: BuildHasher + Default,
    A: GraphAttributes,
    A::Ix: NumIndex + Eq + Hash,
{
    /// search the hypergraph using the A* algorithm with the given heuristic function
    pub fn astar<F>(&self, heuristic: F) -> algo::AStarSearch<'_, N, E, A, F, Self>
    where
        F: algo::Heuristic<A::Ix, Output = f64>,
    {
        algo::AStarSearch::new(self, heuristic)
    }
    /// search the hypergraph using the breadth-first traversal algorithm
    pub fn bft(&self) -> algo::BreadthFirstTraversal<'_, N, E, A, Self> {
        algo::BreadthFirstTraversal::new(self)
    }
    /// search the hypergraph using the depth-first traversal algorithm
    pub fn dft(&self) -> algo::DepthFirstTraversal<'_, N, E, A, Self> {
        algo::DepthFirstTraversal::new(self)
    }
    /// returns a new instance of the Dijkstra's algorithm for the hypergraph
    pub fn dijkstra(&self) -> algo::Dijkstra<'_, N, E, A, Self> {
        algo::Dijkstra::new(self)
    }
}

impl<N, E, A, S, K, Idx> Combine<EdgeId<Idx>, EdgeId<Idx>> for HyperMap<N, E, A, S>
where
    A: GraphAttributes<Ix = Idx, Kind = K>,
    E: Clone + core::ops::Add<Output = E>,
    K: GraphType,
    Idx: NumIndex,
    S: BuildHasher + Default,
    for<'a> &'a E: core::ops::Add<Output = E>,
{
    type Output = EdgeId<Idx>;

    fn combine(
        &mut self,
        src: EdgeId<Idx>,
        tgt: EdgeId<Idx>,
    ) -> rshyper_core::Result<Self::Output> {
        self.merge_edges(&src, &tgt)
            .map_err(|_e| format!("Failed to combine the hyperedges").into())
    }
}

impl<'a, N, E, A, S, K, Idx> Combine<&'a EdgeId<Idx>, &'a EdgeId<Idx>> for HyperMap<N, E, A, S>
where
    A: GraphAttributes<Ix = Idx, Kind = K>,
    K: GraphType,
    Idx: NumIndex,
    S: BuildHasher + Default,
    for<'b> &'b E: core::ops::Add<Output = E>,
{
    type Output = EdgeId<Idx>;

    fn combine(
        &mut self,
        src: &'a EdgeId<Idx>,
        tgt: &'a EdgeId<Idx>,
    ) -> Result<Self::Output, crate::Error> {
        self.merge_edges(src, tgt)
            .map_err(|_e| format!("Failed to combine the hyperedges").into())
    }
}

impl<N, E, A, S, K, Idx> core::ops::Index<&EdgeId<Idx>> for HyperMap<N, E, A, S>
where
    A: GraphAttributes<Ix = Idx, Kind = K>,
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    type Output = HashFacet<E, K, Idx, S>;

    fn index(&self, index: &EdgeId<Idx>) -> &Self::Output {
        self.get_surface(index).expect("Edge not found")
    }
}

impl<N, E, A, S, K, Idx> core::ops::IndexMut<&EdgeId<Idx>> for HyperMap<N, E, A, S>
where
    A: GraphAttributes<Ix = Idx, Kind = K>,
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    fn index_mut(&mut self, index: &EdgeId<Idx>) -> &mut Self::Output {
        self.get_surface_mut(index).expect("Edge not found")
    }
}

impl<N, E, A, S, K, Idx> core::ops::Index<&VertexId<Idx>> for HyperMap<N, E, A, S>
where
    A: GraphAttributes<Ix = Idx, Kind = K>,
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    type Output = Node<N, Idx>;

    fn index(&self, index: &VertexId<Idx>) -> &Self::Output {
        self.get_node(index).expect("Node not found")
    }
}

impl<N, E, A, S, K, Idx> core::ops::IndexMut<&VertexId<Idx>> for HyperMap<N, E, A, S>
where
    A: GraphAttributes<Ix = Idx, Kind = K>,
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    fn index_mut(&mut self, index: &VertexId<Idx>) -> &mut Self::Output {
        self.get_node_mut(index).expect("Node not found")
    }
}
