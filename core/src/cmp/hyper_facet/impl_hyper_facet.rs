/*
    appellation: impl_hyper_facet <module>
    authors: @FL03
*/
use crate::cmp::{HyperEdge, HyperFacet, RawEdgeStore};
use crate::index::{EdgeId, RawIndex};
use crate::{GraphKind, Weight};

impl<T, S, K, Idx> From<HyperEdge<S, K, Idx>> for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawEdgeStore<Idx>,
    T: Default,
{
    fn from(edge: HyperEdge<S, K, Idx>) -> Self {
        Self::from_edge(edge)
    }
}

impl<T, S, K, Idx> From<HyperFacet<T, S, K, Idx>> for HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawEdgeStore<Idx>,
{
    fn from(facet: HyperFacet<T, S, K, Idx>) -> Self {
        facet.edge
    }
}

impl<T, S, K, Idx> From<EdgeId<Idx>> for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: Default + RawEdgeStore<Idx>,
    T: Default,
{
    fn from(id: EdgeId<Idx>) -> Self {
        Self::from_id(id)
    }
}

impl<T, S, K, Idx> AsRef<Weight<T>> for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawEdgeStore<Idx>,
{
    fn as_ref(&self) -> &Weight<T> {
        &self.weight
    }
}

impl<T, S, K, Idx> AsMut<Weight<T>> for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawEdgeStore<Idx>,
{
    fn as_mut(&mut self) -> &mut Weight<T> {
        &mut self.weight
    }
}

impl<T, S, K, Idx> core::borrow::Borrow<EdgeId<Idx>> for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawEdgeStore<Idx>,
{
    fn borrow(&self) -> &EdgeId<Idx> {
        self.id()
    }
}

impl<T, S, K, Idx> core::borrow::BorrowMut<EdgeId<Idx>> for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawEdgeStore<Idx>,
{
    fn borrow_mut(&mut self) -> &mut EdgeId<Idx> {
        self.id_mut()
    }
}

impl<T, S, K, Idx> core::ops::Deref for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawEdgeStore<Idx>,
{
    type Target = HyperEdge<S, K, Idx>;

    fn deref(&self) -> &Self::Target {
        self.edge()
    }
}

impl<T, S, K, Idx> core::ops::DerefMut for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawEdgeStore<Idx>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.edge_mut()
    }
}
