/*
    appellation: impl_repr <module>
    authors: @FL03
*/
use crate::hash_graph::{DirectedHashGraph, HashGraph, UndirectedHashGraph};
use crate::index::{RawIndex, VertexId};
use crate::{GraphAttributes, GraphKind};
use core::hash::Hash;

impl<N, E, Idx> DirectedHashGraph<N, E, Idx>
where
    E: Eq + Hash,
    N: Eq + Hash,
    Idx: Eq + RawIndex + Hash,
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
    E: Eq + Hash,
    N: Eq + Hash,
    Idx: Eq + RawIndex + Hash,
{
    /// initialize a new, empty hypergraph
    pub fn undirected() -> Self
    where
        Idx: Default,
    {
        HashGraph::new()
    }
}

impl<E, A, K, Idx> HashGraph<(), E, A>
where
    A: GraphAttributes<Kind = K, Idx = Idx>,
    E: Eq + Hash,
    Idx: RawIndex + Eq + Hash,
    K: GraphKind,
{
    pub fn insert_empty_node(&mut self) -> crate::Result<VertexId<Idx>>
    where
        Idx: Copy + core::ops::Add<Output = A::Idx> + num_traits::One,
    {
        self.add_node(())
    }
}

impl<N, E, A, K, Idx> HashGraph<Option<N>, E, A>
where
    A: GraphAttributes<Kind = K, Idx = Idx>,
    E: Eq + Hash,
    N: Eq + Hash,
    K: GraphKind,
    Idx: RawIndex + Eq + Hash,
{
    /// insert [`Some`] vertex with weight `T` and return its ID
    pub fn insert_some_node(&mut self, weight: N) -> crate::Result<VertexId<Idx>>
    where
        A::Idx: Copy + core::ops::Add<Output = Idx> + num_traits::One,
    {
        self.add_node(Some(weight))
    }
    /// insert [`None`] vertex with weight `T` and return its ID
    pub fn add_none_node(&mut self) -> crate::Result<VertexId<Idx>>
    where
        A::Idx: Copy + core::ops::Add<Output = Idx> + num_traits::One,
    {
        self.add_node(None)
    }
}
