/*
    Appellation: search <module>
    Contrib: @FL03
*/
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
