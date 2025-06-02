/*
    appellation: impl_repr <module>
    authors: @FL03
*/
use crate::GraphKind;
use crate::hash_graph::{DirectedHashGraph, HashGraph, UndirectedHashGraph};
use crate::index::{NumIndex, RawIndex, VertexId};

impl<N, E, Idx> DirectedHashGraph<N, E, Idx>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    Idx: Eq + RawIndex + core::hash::Hash,
{
    /// initialize a new, empty hypergraph
    pub fn directed() -> Self
    where
        Idx: Default,
    {
        HashGraph::new()
    }
}
impl<N, E, Idx> UndirectedHashGraph<N, E, Idx>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    Idx: Eq + RawIndex + core::hash::Hash,
{
    /// initialize a new, empty hypergraph
    pub fn undirected() -> Self
    where
        Idx: Default,
    {
        HashGraph::new()
    }
}

impl<E, K, Idx> HashGraph<(), E, K, Idx>
where
    E: Eq + core::hash::Hash,
    Idx: NumIndex,
    K: GraphKind,
{
    pub fn insert_empty_node(&mut self) -> VertexId<Idx>
    where
        Idx: core::ops::Add<Output = Idx> + num_traits::One,
    {
        self.add_node(())
    }
}

impl<N, E, K, Idx> HashGraph<Option<N>, E, K, Idx>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    Idx: NumIndex,
    K: GraphKind,
{
    /// insert [`Some`] vertex with weight `T` and return its ID
    pub fn insert_some_node(&mut self, weight: N) -> VertexId<Idx> {
        self.add_node(Some(weight))
    }

    pub fn insert_empty_node(&mut self) -> VertexId<Idx> {
        self.add_node(None)
    }
}
