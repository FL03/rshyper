/*
    appellation: attrs <module>
    authors: @FL03
*/
use super::GraphAttributes;
use crate::index::RawIndex;
use crate::{Directed, GraphKind, Undirected};
use core::marker::PhantomData;

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
    pub(crate) index: PhantomData<Idx>,
    /// the kind of graph, either directed or undirected
    pub(crate) kind: PhantomData<K>,
}

impl<I, K> Attributes<I, K>
where
    I: RawIndex,
    K: GraphKind,
{
    /// returns a new instance of [`Attributes`] initialized with the given index and kind.
    pub fn new() -> Self {
        Attributes {
            index: PhantomData::<I>,
            kind: PhantomData::<K>,
        }
    }
    /// consumes the current instance to create another with the given kind
    pub fn with_kind<K2>(self) -> Attributes<I, K2>
    where
        K2: GraphKind,
    {
        Attributes {
            index: self.index,
            kind: PhantomData::<K2>,
        }
    }
    /// consumes the current instance to create another with the given index type
    pub fn with_index<I2>(self) -> Attributes<I2, K>
    where
        I2: RawIndex,
    {
        Attributes {
            index: PhantomData::<I2>,
            kind: self.kind,
        }
    }
    /// returns true if the current kind `K` is the same as the given kind `K2`
    pub fn is_kind<K2>(&self) -> bool
    where
        K2: GraphKind,
    {
        core::any::TypeId::of::<K2>() == core::any::TypeId::of::<K>()
    }

    /// returns true if the current index type `I` is the same as the given index type `I2`
    pub fn is_index<I2>(&self) -> bool
    where
        I2: RawIndex,
    {
        core::any::TypeId::of::<I2>() == core::any::TypeId::of::<I>()
    }
}

impl<I> Attributes<I, Directed>
where
    I: RawIndex,
{
    pub fn directed() -> Self {
        Attributes {
            index: PhantomData::<I>,
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
            index: PhantomData::<I>,
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
