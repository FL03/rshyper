/*
    appellation: impl_ops <module>
    authors: @FL03
*/
use crate::algo::search;
use crate::hash_graph::{HashFacet, HashGraph};
use rshyper_core::cmp::HyperNode;
use rshyper_core::index::{EdgeId, NumIndex, RawIndex, VertexId};
use rshyper_core::{GraphKind, HyperGraphAttributes};

/// implementations for various algorithms and operators on the hypergraph
impl<N, E, K, Idx, A> HashGraph<N, E, A>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    A: HyperGraphAttributes<Idx = Idx, Kind = K>,
    K: GraphKind,
    Idx: Eq + RawIndex + core::hash::Hash,
{
    /// search the hypergraph using the A* algorithm with the given heuristic function
    pub fn astar<F>(&self, heuristic: F) -> search::AStarSearch<'_, N, E, A, F>
    where
        F: search::astar::HeuristicFunc<Idx, Output = f64>,
    {
        search::AStarSearch::new(self, heuristic)
    }
    /// search the hypergraph using the depth-first traversal algorithm
    pub fn dft(&self) -> search::DepthFirstTraversal<'_, N, E, Self>
    where
        N: Default,
        E: Default,
        Idx: NumIndex,
    {
        search::DepthFirstTraversal::new(self)
    }
    /// search the hypergraph using the breadth-first traversal algorithm
    pub fn bft(&self) -> search::BreadthFirstTraversal<'_, N, E, A> {
        search::BreadthFirstTraversal::from_hypergraph(self)
    }
}

impl<N, E, K, Idx, A> core::ops::Index<&EdgeId<Idx>> for HashGraph<N, E, A>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    A: HyperGraphAttributes<Idx = Idx, Kind = K>,
    K: GraphKind,
    Idx: Eq + RawIndex + core::hash::Hash,
{
    type Output = HashFacet<E, K, Idx>;

    fn index(&self, index: &EdgeId<Idx>) -> &Self::Output {
        self.get_surface(index).expect("Edge not found")
    }
}

impl<N, E, K, Idx, A> core::ops::IndexMut<&EdgeId<Idx>> for HashGraph<N, E, A>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    A: HyperGraphAttributes<Idx = Idx, Kind = K>,
    K: GraphKind,
    Idx: Eq + RawIndex + core::hash::Hash,
{
    fn index_mut(&mut self, index: &EdgeId<Idx>) -> &mut Self::Output {
        self.get_surface_mut(index).expect("Edge not found")
    }
}

impl<N, E, K, Idx, A> core::ops::Index<&VertexId<Idx>> for HashGraph<N, E, A>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    A: HyperGraphAttributes<Idx = Idx, Kind = K>,
    K: GraphKind,
    Idx: Eq + RawIndex + core::hash::Hash,
{
    type Output = HyperNode<N, Idx>;

    fn index(&self, index: &VertexId<Idx>) -> &Self::Output {
        self.get_node(index).expect("Node not found")
    }
}

impl<N, E, K, Idx, A> core::ops::IndexMut<&VertexId<Idx>> for HashGraph<N, E, A>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    A: HyperGraphAttributes<Idx = Idx, Kind = K>,
    K: GraphKind,
    Idx: Eq + RawIndex + core::hash::Hash,
{
    fn index_mut(&mut self, index: &VertexId<Idx>) -> &mut Self::Output {
        self.get_node_mut(index).expect("Node not found")
    }
}
