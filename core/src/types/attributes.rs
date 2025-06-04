/*
    appellation: attrs <module>
    authors: @FL03
*/
use crate::index::RawIndex;
use crate::{Directed, GraphKind, Undirected};
use core::marker::PhantomData;

/// a type alias for graph [`Attributes`] configured with a [`Directed`] graph type.
pub type DirectedAttributes<Idx> = Attributes<Idx, Directed>;
/// a type alias for graph [`Attributes`] configured with an [`Undirected`] graph type.
pub type UndirectedAttributes<Idx> = Attributes<Idx, Undirected>;

/// The [`GraphAttributes`] trait abstracts several generic types used to define a hyper graph
/// into a single entity.
pub trait GraphAttributes: 'static + Copy + Send + Sync {
    type Idx: RawIndex;
    type Kind: GraphKind;

    private!();

    fn new() -> Self
    where
        Self: Sized;
}

/// [`Attributes`] is a generic implementation of the [`GraphAttributes`] trait enabling the
/// definition of hypergraphs with different index types and graph kinds (directed or
/// undirected).
#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, rename_all = "snake_case")
)]
pub struct Attributes<Idx, K>
where
    Idx: RawIndex,
    K: GraphKind,
{
    /// the inner type of index used by the graph
    pub(crate) idx: PhantomData<Idx>,
    /// the kind of graph, either directed or undirected
    pub(crate) kind: PhantomData<K>,
}

impl<I, K> Attributes<I, K>
where
    I: RawIndex,
    K: GraphKind,
{
    pub fn new() -> Self {
        Attributes {
            idx: PhantomData::<I>,
            kind: PhantomData::<K>,
        }
    }

    pub fn with_kind<K2>(self) -> Attributes<I, K2>
    where
        K2: GraphKind,
    {
        Attributes {
            idx: self.idx,
            kind: PhantomData::<K2>,
        }
    }

    pub fn with_idx<I2>(self) -> Attributes<I2, K>
    where
        I2: RawIndex,
    {
        Attributes {
            idx: PhantomData::<I2>,
            kind: self.kind,
        }
    }
}

impl<I> Attributes<I, Directed>
where
    I: RawIndex,
{
    pub fn directed() -> Self {
        Attributes {
            idx: PhantomData::<I>,
            kind: PhantomData::<Directed>,
        }
    }
}

impl<I> Attributes<I, Undirected>
where
    I: RawIndex,
{
    pub fn undirected() -> Self {
        Attributes {
            idx: PhantomData::<I>,
            kind: PhantomData::<Undirected>,
        }
    }
}

impl<I, K> GraphAttributes for (PhantomData<I>, PhantomData<K>)
where
    I: RawIndex,
    K: GraphKind,
{
    type Idx = I;
    type Kind = K;

    seal!();

    fn new() -> Self {
        (PhantomData::<I>, PhantomData::<K>)
    }
}

impl<I, K> GraphAttributes for Attributes<I, K>
where
    I: RawIndex,
    K: GraphKind,
{
    type Idx = I;
    type Kind = K;

    seal!();

    fn new() -> Self {
        Attributes::new()
    }
}

impl<I, K> Clone for Attributes<I, K>
where
    I: RawIndex,
    K: GraphKind,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<I, K> Copy for Attributes<I, K>
where
    I: RawIndex,
    K: GraphKind,
{
}

impl<I, K> Default for Attributes<I, K>
where
    I: RawIndex,
    K: GraphKind,
{
    fn default() -> Self {
        Attributes::new()
    }
}

unsafe impl<I, K> Send for Attributes<I, K>
where
    I: RawIndex,
    K: GraphKind,
{
}

unsafe impl<I, K> Sync for Attributes<I, K>
where
    I: RawIndex,
    K: GraphKind,
{
}

impl<I, K> core::fmt::Debug for Attributes<I, K>
where
    I: RawIndex,
    K: GraphKind,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Attributes<{}, {}>",
            core::any::type_name::<I>(),
            core::any::type_name::<K>()
        )
    }
}

impl<I, K> core::fmt::Display for Attributes<I, K>
where
    I: RawIndex,
    K: GraphKind,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Attributes<{}, {}>",
            core::any::type_name::<I>(),
            core::any::type_name::<K>()
        )
    }
}
