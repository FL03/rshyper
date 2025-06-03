/*
    appellation: hyper_edge <module>
    authors: @FL03
*/
use super::RawStore;
use crate::index::{EdgeId, RawIndex};
use crate::{GraphKind, Weight};

/// [`HyperEdge`] is the base type for hyperedges in a graph. These edges are generic over the
/// edge store type `S`, the graph kind `K`, and the index type `Idx`. This allows for
/// flexibility in how edges store their vertices and how they are identified within the graph.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[repr(C)]
pub struct HyperEdge<S, K, Idx = usize>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    pub(crate) id: EdgeId<Idx>,
    pub(crate) points: S,
    pub(crate) _kind: core::marker::PhantomData<K>,
}

/// The [`HyperFacet`] implementation associates some weight with a hyperedge.
/// Typically, the term **facet** is used to denote the surface of a particular polytope,
/// however, here it is used to aptly define a _**weighted**_ hyperedge.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[repr(C)]
pub struct HyperFacet<T, S, K, Idx = usize>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(crate) edge: HyperEdge<S, K, Idx>,
    pub(crate) weight: Weight<T>,
}
