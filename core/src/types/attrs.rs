/*
    appellation: attrs <module>
    authors: @FL03
*/
use crate::GraphKind;
use crate::index::RawIndex;
use core::marker::PhantomData;

/// a type alias for a [directed](crate::Directed) [`Attributes`]
pub type DirectedAttributes<Idx> = Attributes<Idx, crate::Directed>;
/// a type alias for an [undirected](crate::Undirected) [`Attributes`]
pub type UndirectedAttributes<Idx> = Attributes<Idx, crate::Undirected>;

pub trait HyperGraphAttributes: 'static + Send + Sync + core::fmt::Debug {
    type Idx: RawIndex;
    type Kind: GraphKind;
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
    pub idx: PhantomData<Idx>,
    pub kind: PhantomData<K>,
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

impl<I> Attributes<I, crate::Directed>
where
    I: RawIndex,
{
    pub fn directed() -> Self {
        Attributes {
            idx: PhantomData::<I>,
            kind: PhantomData::<crate::Directed>,
        }
    }
}

impl<I> Attributes<I, crate::Undirected>
where
    I: RawIndex,
{
    pub fn undirected() -> Self {
        Attributes {
            idx: PhantomData::<I>,
            kind: PhantomData::<crate::Undirected>,
        }
    }
}

impl<I, K> HyperGraphAttributes for Attributes<I, K>
where
    I: RawIndex,
    K: GraphKind,
{
    type Idx = I;
    type Kind = K;
}
