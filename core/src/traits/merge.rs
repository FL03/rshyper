/*
    appellation: merge <module>
    authors: @FL03
*/

/// [`Combine`] defines a common interface for merging two edges in a hypergraph.
pub trait Combine<A, B> {
    type Output;

    fn combine(&mut self, src: A, tgt: B) -> crate::HyperResult<Self::Output>;
}
/// [`Concat`] defines an interface for _concatenating_ two entities into another
pub trait Concat<Rhs = Self> {
    type Output;

    /// Concatenates `self` with `rhs` and returns the result.
    fn concat(&self, rhs: Rhs) -> Self::Output;
}
/// [`Merge`] defines a common interface for _merging_ two entities into another
pub trait Merge<Rhs = Self> {
    type Output;

    fn merge(&self, rhs: Rhs) -> Self::Output;
}
