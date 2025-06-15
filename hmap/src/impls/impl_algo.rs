/*
    appellation: impl_ops <module>
    authors: @FL03
*/
use crate::HyperMap;
use algo::{AStarSearch, BreadthFirstTraversal, DepthFirstTraversal, Dijkstra, Heuristic};
use core::hash::{BuildHasher, Hash};
use rshyper_core::prelude::{GraphAttributes, NumIndex};

/// implementations for various algorithms and operators on the hypergraph
impl<N, E, A, S> HyperMap<N, E, A, S>
where
    S: BuildHasher + Default,
    A: GraphAttributes,
    A::Ix: NumIndex + Eq + Hash,
{
    /// returns a new [`A*`](AStarSearch) search operator configured with the current
    /// graph and the provided heuristic function.
    pub fn astar<F>(&self, heuristic: F) -> AStarSearch<'_, N, E, A, F, Self>
    where
        F: Heuristic<A::Ix, Output = f64>,
    {
        AStarSearch::new(self, heuristic)
    }
    /// returns the [`BreadthFirstTraversal`] operator configured with the current hypergraph.
    pub fn bft(&self) -> BreadthFirstTraversal<'_, N, E, A, Self> {
        BreadthFirstTraversal::new(self)
    }
    /// returns the [`DepthFirstTraversal`] operator configured with the current hypergraph.
    pub fn dft(&self) -> DepthFirstTraversal<'_, N, E, A, Self> {
        DepthFirstTraversal::new(self)
    }
    ///  returns the [`Dijkstra`] operator on the current hypergraph.
    pub fn dijkstra(&self) -> Dijkstra<'_, N, E, A, Self> {
        Dijkstra::new(self)
    }
}
