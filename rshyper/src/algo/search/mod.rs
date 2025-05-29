/*
    appellation: search <module>
    authors: @FL03
*/
#[doc(inline)]
pub use self::{
    astar::AStarSearch, breadth_first::BreadthFirstTraversal, depth_first::DepthFirstTraversal,
};

pub mod astar;
pub mod breadth_first;
pub mod depth_first;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::astar::*;
    #[doc(inline)]
    pub use super::breadth_first::*;
    #[doc(inline)]
    pub use super::depth_first::*;

    #[doc(inline)]
    pub use super::Search;
}

use crate::VertexId;
use std::collections::HashSet;

/// A trait defining a search algorithm for a hypergraph
pub trait Search<N> {
    /// Execute the search algorithm starting from the given vertex
    fn search(&mut self, start: VertexId) -> crate::Result<Vec<VertexId>>;

    /// Check if the search has visited a specific vertex
    fn has_visited(&self, vertex: VertexId) -> bool;

    /// Get all vertices that have been visited during the search
    fn visited_vertices(&self) -> &HashSet<VertexId>;
}
