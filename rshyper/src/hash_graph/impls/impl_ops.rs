/*
    appellation: impl_ops <module>
    authors: @FL03
*/
use crate::algo::search;
use crate::hash_graph::{HashFacet, HashGraph};
use rshyper_core::GraphKind;
use rshyper_core::cmp::HyperNode;
use rshyper_core::index::{EdgeId, HashIndex, NumIndex, VertexId};

/// implementations for various algorithms and operators on the hypergraph
impl<N, E, K, Idx> HashGraph<N, E, K, Idx>
where
    N: Eq + core::hash::Hash,
    E: Eq + core::hash::Hash,
    K: GraphKind,
    Idx: HashIndex,
{
    /// search the hypergraph using the A* algorithm with the given heuristic function
    pub fn astar<F>(&self, heuristic: F) -> search::AStarSearch<'_, N, E, F, K, Idx>
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
    pub fn bft(&self) -> search::BreadthFirstTraversal<'_, N, E, K, Idx> {
        search::BreadthFirstTraversal::from_hypergraph(self)
    }
}

impl<N, E, K, Idx> core::ops::Index<&EdgeId<Idx>> for HashGraph<N, E, K, Idx>
where
    N: Eq + core::hash::Hash,
    E: Eq + core::hash::Hash,
    K: GraphKind,
    Idx: HashIndex,
{
    type Output = HashFacet<E, K, Idx>;

    fn index(&self, index: &EdgeId<Idx>) -> &Self::Output {
        self.get_surface(index).expect("Edge not found")
    }
}

impl<N, E, K, Idx> core::ops::IndexMut<&EdgeId<Idx>> for HashGraph<N, E, K, Idx>
where
    N: Eq + core::hash::Hash,
    E: Eq + core::hash::Hash,
    K: GraphKind,
    Idx: HashIndex,
{
    fn index_mut(&mut self, index: &EdgeId<Idx>) -> &mut Self::Output {
        self.get_surface_mut(index).expect("Edge not found")
    }
}

impl<N, E, K, Idx> core::ops::Index<&VertexId<Idx>> for HashGraph<N, E, K, Idx>
where
    N: Eq + core::hash::Hash,
    E: Eq + core::hash::Hash,
    K: GraphKind,
    Idx: HashIndex,
{
    type Output = HyperNode<N, Idx>;

    fn index(&self, index: &VertexId<Idx>) -> &Self::Output {
        self.get_node(index).expect("Node not found")
    }
}

impl<N, E, K, Idx> core::ops::IndexMut<&VertexId<Idx>> for HashGraph<N, E, K, Idx>
where
    N: Eq + core::hash::Hash,
    E: Eq + core::hash::Hash,
    K: GraphKind,
    Idx: HashIndex,
{
    fn index_mut(&mut self, index: &VertexId<Idx>) -> &mut Self::Output {
        self.get_node_mut(index).expect("Node not found")
    }
}
