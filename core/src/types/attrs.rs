/*
    appellation: attrs <module>
    authors: @FL03
*/
use crate::GraphKind;
use crate::index::RawIndex;
use core::marker::PhantomData;

/// a type alias for a [directed](crate::Directed) [`GraphAttributes`]
pub type DirectedAttributes<Idx> = GraphAttributes<Idx, crate::Directed>;
/// a type alias for an [undirected](crate::Undirected) [`GraphAttributes`]
pub type UndirectedAttributes<Idx> = GraphAttributes<Idx, crate::Undirected>;

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
pub struct GraphAttributes<Idx, K>
where
    Idx: RawIndex,
    K: GraphKind,
{
    pub idx: PhantomData<Idx>,
    pub kind: PhantomData<K>,
}

impl<I, K> GraphAttributes<I, K>
where
    I: RawIndex,
    K: GraphKind,
{
    pub fn new() -> Self {
        GraphAttributes {
            idx: PhantomData::<I>,
            kind: PhantomData::<K>,
        }
    }

    pub fn with_kind<K2>(self) -> GraphAttributes<I, K2>
    where
        K2: GraphKind,
    {
        GraphAttributes {
            idx: self.idx,
            kind: PhantomData::<K2>,
        }
    }

    pub fn with_idx<I2>(self) -> GraphAttributes<I2, K>
    where
        I2: RawIndex,
    {
        GraphAttributes {
            idx: PhantomData::<I2>,
            kind: self.kind,
        }
    }
}

impl<I> GraphAttributes<I, crate::Directed>
where
    I: RawIndex,
{
    pub fn directed() -> Self {
        GraphAttributes {
            idx: PhantomData::<I>,
            kind: PhantomData::<crate::Directed>,
        }
    }
}

impl<I> GraphAttributes<I, crate::Undirected>
where
    I: RawIndex,
{
    pub fn undirected() -> Self {
        GraphAttributes {
            idx: PhantomData::<I>,
            kind: PhantomData::<crate::Undirected>,
        }
    }
}

impl<I, K> HyperGraphAttributes for GraphAttributes<I, K>
where
    I: RawIndex,
    K: GraphKind,
{
    type Idx = I;
    type Kind = K;
}
