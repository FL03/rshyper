/*
    appellation: impl_repr <module>
    authors: @FL03
*/
use crate::{DiHyperMap, HyperMap, UnHyperMap};
use core::hash::{BuildHasher, Hash};
use rshyper::error::Result;
use rshyper::idx::{RawIndex, VertexId};
use rshyper::{AddStep, GraphProps, Mode, Weight};

impl<N, E, A, S> HyperMap<N, E, A, S>
where
    A: GraphProps<Kind = Mode>,
    S: BuildHasher,
{
    /// initialize a new, empty hypergraph with a dynamic [`Mode`] kind and the logical default
    /// for the indices.
    pub fn dynamic() -> Self
    where
        A::Ix: Default,
        S: Clone + Default,
    {
        HyperMap::new()
    }
}

impl<N, E, S, Ix> DiHyperMap<N, E, Ix, S>
where
    Ix: RawIndex,
    S: BuildHasher,
{
    /// initialize a new, empty hypergraph
    pub fn directed() -> Self
    where
        Ix: Default,
        S: Clone + Default,
    {
        HyperMap::new()
    }
}
impl<N, E, S, Ix> UnHyperMap<N, E, Ix, S>
where
    Ix: RawIndex,
    S: BuildHasher,
{
    /// initialize a new, empty hypergraph
    pub fn undirected() -> Self
    where
        Ix: Default,
        S: Clone + Default,
    {
        HyperMap::new()
    }
}

impl<E, A, S> HyperMap<(), E, A, S>
where
    A: GraphProps,
    S: BuildHasher,
    A::Ix: Eq + Hash,
{
    pub fn add_empty_node(&mut self) -> Result<VertexId<A::Ix>>
    where
        A::Ix: AddStep<Output = A::Ix> + Copy,
    {
        let weight = Weight::new(());
        self.add_node(weight)
    }
    #[deprecated(since = "0.9.0", note = "use `add_empty_node` instead")]
    pub fn insert_empty_node(&mut self) -> Result<VertexId<A::Ix>>
    where
        A::Ix: AddStep<Output = A::Ix> + Copy,
    {
        self.add_empty_node()
    }
}

impl<N, E, A, S> HyperMap<Option<N>, E, A, S>
where
    A: GraphProps,
    S: BuildHasher,
    A::Ix: Eq + Hash,
{
    /// insert [`Some`] vertex with weight `T` and return its ID
    pub fn add_some_node(&mut self, weight: N) -> Result<VertexId<A::Ix>>
    where
        A::Ix: AddStep<Output = A::Ix> + Copy,
    {
        self.add_node(Weight::some(weight))
    }
    /// insert [`None`] vertex with weight `T` and return its ID
    pub fn add_none_node(&mut self) -> Result<VertexId<A::Ix>>
    where
        A::Ix: AddStep<Output = A::Ix> + Copy,
    {
        self.add_node(Weight::none())
    }
    #[deprecated(since = "0.9.0", note = "use `add_some_node` instead")]
    pub fn insert_some_node(&mut self, weight: N) -> Result<VertexId<A::Ix>>
    where
        A::Ix: AddStep<Output = A::Ix> + Copy,
    {
        self.add_some_node(weight)
    }
}
