/*
    appellation: impl_repr <module>
    authors: @FL03
*/
use crate::hash_graph::{DiHashGraph, HashGraph, UnHashGraph};
use crate::index::{RawIndex, VertexId};
use crate::{GraphAttributes, GraphKind, Weight};
use core::hash::{BuildHasher, Hash};

impl<N, E, S, Idx> DiHashGraph<N, E, Idx, S>
where
    E: Eq + Hash,
    N: Eq + Hash,
    Idx: Eq + RawIndex + Hash,
    S: BuildHasher,
{
    /// initialize a new, empty hypergraph
    pub fn directed() -> Self
    where
        Idx: Default,
        S: Clone + Default,
    {
        HashGraph::new()
    }
}
impl<N, E, S, Idx> UnHashGraph<N, E, Idx, S>
where
    E: Eq + Hash,
    N: Eq + Hash,
    Idx: Eq + RawIndex + Hash,
    S: BuildHasher,
{
    /// initialize a new, empty hypergraph
    pub fn undirected() -> Self
    where
        Idx: Default,
        S: Clone + Default,
    {
        HashGraph::new()
    }
}

impl<E, A, S, K, Idx> HashGraph<(), E, A, S>
where
    A: GraphAttributes<Kind = K, Idx = Idx>,
    E: Eq + Hash,
    Idx: RawIndex + Eq + Hash,
    K: GraphKind,
    S: BuildHasher,
{
    pub fn add_empty_node(&mut self) -> crate::Result<VertexId<Idx>>
    where
        Idx: Copy + core::ops::Add<Output = A::Idx> + num_traits::One,
    {
        let weight = Weight::new(());
        self.add_node(weight)
    }
    #[deprecated(since = "0.9.0", note = "use `add_empty_node` instead")]
    pub fn insert_empty_node(&mut self) -> crate::Result<VertexId<Idx>>
    where
        Idx: Copy + core::ops::Add<Output = A::Idx> + num_traits::One,
    {
        self.add_empty_node()
    }
}

impl<N, E, A, S, K, Idx> HashGraph<Option<N>, E, A, S>
where
    A: GraphAttributes<Kind = K, Idx = Idx>,
    E: Eq + Hash,
    N: Eq + Hash,
    S: BuildHasher,
    K: GraphKind,
    Idx: RawIndex + Eq + Hash,
{
    /// insert [`Some`] vertex with weight `T` and return its ID
    pub fn add_some_node(&mut self, weight: N) -> crate::Result<VertexId<Idx>>
    where
        A::Idx: Copy + core::ops::Add<Output = Idx> + num_traits::One,
    {
        self.add_node(Weight::some(weight))
    }
    /// insert [`None`] vertex with weight `T` and return its ID
    pub fn add_none_node(&mut self) -> crate::Result<VertexId<Idx>>
    where
        A::Idx: Copy + core::ops::Add<Output = Idx> + num_traits::One,
    {
        self.add_node(Weight::none())
    }
    #[deprecated(since = "0.9.0", note = "use `add_some_node` instead")]
    pub fn insert_some_node(&mut self, weight: N) -> crate::Result<VertexId<Idx>>
    where
        A::Idx: Copy + core::ops::Add<Output = Idx> + num_traits::One,
    {
        self.add_some_node(weight)
    }
}
