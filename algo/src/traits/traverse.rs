/*
    appellation: traverse <module>
    authors: @FL03
*/

/// [`Traversal`] trait defines an interface for operators capable of _traversing_ some type,
/// which in this case is a hypergraph.
pub trait Traversal<N> {
    /// defines the associated container used to store visited vertices
    type Store<I2>;
    /// Check if the search has visited a specific vertex
    fn has_visited(&self, dest: &N) -> bool;
    /// Get all vertices that have been visited during the search
    fn visited(&self) -> &Self::Store<N>;
}
