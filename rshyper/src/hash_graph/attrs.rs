/*
    appellation: attrs <module>
    authors: @FL03
*/
use crate::GraphKind;
use crate::index::RawIndex;

pub trait HyperGraphAttributes {
    type Idx: RawIndex;
    type Kind: GraphKind;
}

pub struct Attr<I, K>
where
    I: RawIndex,
    K: GraphKind,
{
    pub idx: I,
    pub kind: core::marker::PhantomData<K>,
}

impl<I, K> Attr<I, K>
where
    I: RawIndex,
    K: GraphKind,
{
    pub fn new(idx: I) -> Self {
        Attr {
            idx,
            kind: core::marker::PhantomData::<K>,
        }
    }
}

impl<I, K> HyperGraphAttributes for Attr<I, K>
where
    I: RawIndex,
    K: GraphKind,
{
    type Idx = I;
    type Kind = K;
}
