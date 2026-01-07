/*
    appellation: search <module>
    authors: @FL03
*/
use crate::error::Result;
use crate::traits::Traversal;

/// The [`Search`] establishes a common interface for operators on hypergraphs capable of
/// performing a search.
pub trait Search<N> {
    type Output;

    /// begins a search of the graph, starting with the given index.
    fn search(&mut self, start: N) -> Result<Self::Output>;
}
/// The [`GraphSearch`] trait is an automatically implemented trait for types that implement
/// both the [`Search`] and [`Traversal`] traits indicating it can successfully perform a
/// search on some graph structure while also allowing traversal of the graph.
pub trait GraphSearch<Idx>: Search<Idx> + Traversal<Idx> {
    private!();
}

/*
 ************* Implementations *************
*/
impl<T, Idx> GraphSearch<Idx> for T
where
    T: Search<Idx> + Traversal<Idx>,
{
    seal!();
}
