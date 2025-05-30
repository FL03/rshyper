/*
    appellation: impl_repr <module>
    authors: @FL03
*/
use crate::hash_graph::HashGraph;
use crate::index::{NumIndex, VertexId};

impl<E, Idx> HashGraph<(), E, Idx>
where
    E: Eq + core::hash::Hash,
    Idx: NumIndex,
{
    pub fn insert_empty_node(&mut self) -> VertexId<Idx>
    where
        Idx: core::ops::Add<Output = Idx> + num_traits::One,
    {
        self.insert_node(())
    }
}

impl<N, E, Idx> HashGraph<Option<N>, E, Idx>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    Idx: NumIndex,
{
    /// insert [`Some`] vertex with weight `T` and return its ID
    pub fn insert_some_node(&mut self, weight: N) -> VertexId<Idx> {
        self.insert_node(Some(weight))
    }

    pub fn insert_empty_node(&mut self) -> VertexId<Idx> {
        self.insert_node(None)
    }
}