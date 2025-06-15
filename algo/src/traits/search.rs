/*
    appellation: search <module>
    authors: @FL03
*/
use super::Traversal;

/// A trait defining a search algorithm for a hypergraph
pub trait Search<N> {
    type Output;

    /// Execute the search algorithm starting from the given vertex
    fn search(&mut self, start: N) -> crate::AlgoResult<Self::Output>;
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
