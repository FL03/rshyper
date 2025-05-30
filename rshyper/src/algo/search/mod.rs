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
use crate::{RawIndex, VertexId};

use std::collections::HashSet;
/// [`Traversal`] trait defines an interface for operators capable of _traversing_ some type,
/// which in this case is a hypergraph.
pub trait Traversal<N> {
    type Store<I2: RawIndex>;
    /// Check if the search has visited a specific vertex
    fn has_visited(&self, vertex: &VertexId<N>) -> bool {
        self.visited().contains(&vertex)
    }
    /// Get all vertices that have been visited during the search
    fn visited(&self) -> &Self::Store<VertexId<N>>;
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

impl<T, Idx> GraphSearch<Idx> for T
where
    T: Search<Idx> + Traversal<Idx>,
{
    seal!();
}
