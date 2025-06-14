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
}
