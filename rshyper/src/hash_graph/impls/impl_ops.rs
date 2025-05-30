/*
    appellation: impl_ops <module>
    authors: @FL03
*/
use crate::algo::search;
use crate::hash_graph::HashGraph;
use rshyper_core::{EdgeId, Udx, HyperNode, HashIndex, VertexId};

/// implementations for various algorithms and operators on the hypergraph
impl<N, E> HashGraph<N, E, Udx>
where
    N: Eq + core::hash::Hash,
    E: Eq + core::hash::Hash,
{
    /// search the hypergraph using the A* algorithm with the given heuristic function
    pub fn astar<F>(&self, heuristic: F) -> search::AStarSearch<'_, N, E, F>
    where
        F: search::astar::HeuristicFunc<VertexId<Udx>, Output = f64>,
    {
        search::AStarSearch::new(self, heuristic)
    }
    /// search the hypergraph using the breadth-first traversal algorithm
    pub fn bft(&self) -> search::BreadthFirstTraversal<'_, N, E> {
        search::BreadthFirstTraversal::from_hypergraph(self)
    }
    /// search the hypergraph using the depth-first traversal algorithm
    pub fn dft(&self) -> search::DepthFirstTraversal<'_, N, E> {
        search::DepthFirstTraversal::new(self)
    }
}

impl<N, E, Idx> core::ops::Index<&EdgeId<Idx>> for HashGraph<N, E, Idx>
where
    N: Eq + core::hash::Hash,
    E: Eq + core::hash::Hash,
    Idx: HashIndex,
{
    type Output = E;

    fn index(&self, index: &EdgeId<Idx>) -> &Self::Output {
        self.get_facet(index).expect("Edge not found")
    }
}

impl<N, E, Idx> core::ops::IndexMut<&EdgeId<Idx>> for HashGraph<N, E, Idx>
where
    N: Eq + core::hash::Hash,
    E: Eq + core::hash::Hash,
    Idx: HashIndex,
{
    fn index_mut(&mut self, index: &EdgeId<Idx>) -> &mut Self::Output {
        self.get_facet_mut(index).expect("Edge not found")
    }
}

impl<N, E, Idx> core::ops::Index<&VertexId<Idx>> for HashGraph<N, E, Idx>
where
    N: Eq + core::hash::Hash,
    E: Eq + core::hash::Hash,
    Idx: HashIndex,
{
    type Output = HyperNode<N, Idx>;

    fn index(&self, index: &VertexId<Idx>) -> &Self::Output {
        self.get_vertex_weight(index).expect("Node not found")
    }
}

impl<N, E, Idx> core::ops::IndexMut<&VertexId<Idx>> for HashGraph<N, E, Idx>
where
    N: Eq + core::hash::Hash,
    E: Eq + core::hash::Hash,
    Idx: HashIndex,
{
    fn index_mut(&mut self, index: &VertexId<Idx>) -> &mut Self::Output {
        self.get_vertex_weight_mut(index).expect("Node not found")
    }
}
