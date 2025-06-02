/*
    appellation: attrs <module>
    authors: @FL03
*/
use crate::GraphKind;
use crate::index::RawIndex;
use core::marker::PhantomData;

pub trait HyperGraphAttributes {
    type Idx: RawIndex;
    type Kind: GraphKind;
}
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, rename_all = "snake_case")
)]
pub struct Attr<A>
where
    A: HyperGraphAttributes,
{
    pub idx: PhantomData<A::Idx>,
    pub kind: PhantomData<A::Kind>,
}

impl<A, I, K> Attr<A>
where
    A: HyperGraphAttributes<Idx = I, Kind = K>,
    I: RawIndex,
    K: GraphKind,
{
    pub fn new() -> Self {
        Attr {
            idx: PhantomData::<I>,
            kind: PhantomData::<K>,
        }
    }
}

impl<A, I> Attr<A>
where
    A: HyperGraphAttributes<Idx = I, Kind = crate::Directed>,
    I: RawIndex,
{
    pub fn directed() -> Self {
        Attr {
            idx: PhantomData::<I>,
            kind: PhantomData::<crate::Directed>,
        }
    }
}

impl<A, I> Attr<A>
where
    A: HyperGraphAttributes<Idx = I, Kind = crate::Undirected>,
    I: RawIndex,
{
    pub fn undirected() -> Self {
        Attr {
            idx: PhantomData::<I>,
            kind: PhantomData::<crate::Undirected>,
        }
    }
}

impl<A> HyperGraphAttributes for Attr<A>
where
    A: HyperGraphAttributes,
{
    type Idx = A::Idx;
    type Kind = A::Kind;
}
