/*
    appellation: attributes <module>
    authors: @FL03
*/
//! this module implements the [`GraphAttributes`] trait and provides a concrete implementation
//! with the [`Attributes`] struct. These objects are used to define the _attributes_ of a
//! hypergraph such as the type of index used to identify vertices and edges as well as the
//! type of graph (directed or undirected).
#[doc(inline)]
pub use self::attributes::Attributes;

mod attributes;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::attributes::Attributes;
    #[doc(inline)]
    pub use super::{DirAttributes, GraphAttributes, UnAttributes};
}
use crate::{Directed, GraphKind, RawIndex, Undirected};

/// a type alias for graph [`Attributes`] configured with a [`Directed`] graph type.
pub type DirAttributes<Idx> = Attributes<Idx, Directed>;
/// a type alias for graph [`Attributes`] configured with an [`Undirected`] graph type.
pub type UnAttributes<Idx> = Attributes<Idx, Undirected>;

/// The [`GraphAttributes`] trait abstracts several generic types used to define a hyper graph
/// into a single entity.
pub trait GraphAttributes: 'static + Copy + Send + Sync {
    type Idx: RawIndex;
    type Kind: GraphKind;

    private!();

    /// returns a new instance of the graph attributes.
    fn new() -> Self;
    /// returns a [`PhantomData`] instance of the graph attributes.
    fn phantom() -> PhantomData<(Self::Kind, Self::Idx)> {
        PhantomData::<(Self::Kind, Self::Idx)>
    }
}

/*
 ************* Implementations *************
*/
use core::marker::PhantomData;

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

impl<I, K> GraphAttributes for PhantomData<(K, I)>
where
    I: RawIndex,
    K: GraphKind,
{
    type Idx = I;
    type Kind = K;

    seal!();

    fn new() -> Self {
        PhantomData::<(K, I)>
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
