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

pub mod astar;
pub mod breadth_first;
pub mod depth_first;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::astar::AStarSearch;
    #[doc(inline)]
    pub use super::breadth_first::BreadthFirstTraversal;
    #[doc(inline)]
    pub use super::depth_first::DepthFirstTraversal;

    #[doc(inline)]
    pub use super::{Search, Traversal};
}

use std::collections::HashSet;

pub trait Traversal<N> {
    /// Check if the search has visited a specific vertex
    fn has_visited(&self, vertex: &N) -> bool {
        self.visited().contains(&vertex)
    }
    /// Get all vertices that have been visited during the search
    fn visited(&self) -> &HashSet<N>;
}

/// A trait defining a search algorithm for a hypergraph
pub trait Search<N> {
    type Output;
    /// Execute the search algorithm starting from the given vertex
    fn search(&mut self, start: N) -> crate::Result<Self::Output>;
}

pub trait GraphSearch<Idx>: Search<Idx> + Traversal<Idx> {}
