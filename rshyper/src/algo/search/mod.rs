/*
    appellation: search <module>
    authors: @FL03
*/
//! the `search` module implements various search algorithms for hypergraphs
//!
//! ## Overview
//!
//! - `astar`: implements the A* search algorithm
//! - `breadth_first`: implements the breadth-first search algorithm
//! - `depth_first`: implements the depth-first search algorithm
//!

#[doc(inline)]
pub use self::prelude::*;

/// this module implements the A* search algorithm
pub mod astar;
/// this module implements the breadth-first search algorithm
pub mod breadth_first;
/// this module implements the depth-first search algorithm
pub mod depth_first;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::astar::AStarSearch;
    #[doc(inline)]
    pub use super::breadth_first::BreadthFirstTraversal;
    #[doc(inline)]
    pub use super::depth_first::DepthFirstTraversal;

    #[doc(inline)]
    pub use super::{GraphSearch, Heuristic, Search, Traversal};
}

use crate::index::{RawIndex, VertexId};
/// [`Heuristic`] defines a common interface for heuristic functions compatible with the [`A*`](AStarSearch)
/// search implementation
pub trait Heuristic<T = crate::Udx> {
    type Output;

    fn compute(&self, start: VertexId<T>, goal: VertexId<T>) -> Self::Output;
}

/// [`Traversal`] trait defines an interface for operators capable of _traversing_ some type,
/// which in this case is a hypergraph.
pub trait Traversal<N> {
    type Store<I2>;
    /// Check if the search has visited a specific vertex
    fn has_visited(&self, dest: &N) -> bool;
    /// Get all vertices that have been visited during the search
    fn visited(&self) -> &Self::Store<N>;
}

/// A trait defining a search algorithm for a hypergraph
pub trait Search<N> {
    type Output;

    /// Execute the search algorithm starting from the given vertex
    fn search(&mut self, start: N) -> crate::Result<Self::Output>;
}
/// The [`GraphSearch`] trait is an automatically implemented trait for types that implement
/// both the [`Search`] and [`Traversal`] traits indicating it can successfully perform a
/// search on some graph structure while also allowing traversal of the graph.
pub trait GraphSearch<Idx>: Search<Idx> + Traversal<Idx> {
    private!();
}

/*
 ************* Implementations *************
*/

impl<F, Idx> Heuristic<Idx> for F
where
    Idx: RawIndex,
    F: Fn(VertexId<Idx>, VertexId<Idx>) -> f64,
{
    type Output = f64;

    fn compute(&self, start: VertexId<Idx>, goal: VertexId<Idx>) -> Self::Output {
        self(start, goal)
    }
}

impl<T, Idx> GraphSearch<Idx> for T
where
    T: Search<Idx> + Traversal<Idx>,
{
    seal!();
}
