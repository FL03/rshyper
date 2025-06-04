/*
    Appellation: node <module>
    Contrib: @FL03
*/
use super::{HyperEdge, RawEdge, RawFacet, RawStore};
use crate::index::{EdgeId, RawIndex};
use crate::{GraphKind, Weight};

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

impl<T, S, Idx, K> RawEdge for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    type Kind = K;
    type Idx = Idx;
    type Store = S;

    seal!();

    fn index(&self) -> &EdgeId<Idx> {
        self.edge().id()
    }

    fn vertices(&self) -> &S {
        self.edge().points()
    }
}

impl<T, S, Idx, K> RawFacet<T> for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    seal!();

    fn weight(&self) -> &Weight<T> {
        self.as_ref()
    }

    fn weight_mut(&mut self) -> &mut Weight<T> {
        self.as_mut()
    }
}
