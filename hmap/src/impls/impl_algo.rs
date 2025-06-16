/*
    appellation: impl_ops <module>
    authors: @FL03
*/
use crate::HyperMap;
use core::hash::BuildHasher;
use rshyper::prelude::{GraphProps, NumIndex};
use rshyper_algo::{AStarSearch, BreadthFirstTraversal, DepthFirstTraversal, Dijkstra, Heuristic};

/// implementations for various algorithms and operators on the hypergraph
impl<N, E, A, S> HyperMap<N, E, A, S>
where
    S: BuildHasher + Default,
    A: GraphProps,
    A::Ix: NumIndex,
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
