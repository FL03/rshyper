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
pub use self::{astar::*, breadth_first::*, depth_first::*};

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
    pub use super::Heuristic;
}

use crate::idx::{RawIndex, VertexId};
/// [`Heuristic`] defines a common interface for heuristic functions compatible with the [`A*`](AStarSearch)
/// search implementation
pub trait Heuristic<T = crate::Udx> {
    type Output;

    fn compute(&self, start: VertexId<T>, goal: VertexId<T>) -> Self::Output;
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
