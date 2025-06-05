/*
    appellation: merge <module>
    authors: @FL03
*/

/// [`Merge`] defines a common interface for _merging_ two entities into another
pub trait Merge<Rhs = Self> {
    type Output;

    fn merge(self, rhs: Rhs) -> Self::Output;
}

/// [`Combine`] defines a common interface for merging two edges in a hypergraph.
pub trait Combine<A, B> {
    type Output;

    fn combine(&mut self, src: A, tgt: B) -> crate::Result<Self::Output>;
}
