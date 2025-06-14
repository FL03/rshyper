/*
    appellation: path <module>
    authors: @FL03
*/

/// The [`PathFinder`] establishes an interface for path-finding operators on hypergraphs. Each
/// implementor will provide a particular algorithm for finding paths between any two vertices
/// in a hypergraph.
pub trait PathFinder<Idx> {
    type Path;
    /// returns a
    fn find_path(&mut self, from: Idx, to: Idx) -> crate::Result<Self::Path>;

    fn reconstruct_path(&self, tgt: Idx) -> Self::Path;
}
