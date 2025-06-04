/*
    appellation: impl_hyper_facet <module>
    authors: @FL03
*/
use crate::edge::{HyperEdge, HyperFacet, RawStore};
use crate::index::{EdgeId, RawIndex};
use crate::{GraphKind, Weight};

impl<T, S, K, Idx> Default for HyperFacet<T, S, K, Idx>
where
    Idx: Default + RawIndex,
    K: GraphKind,
    T: Default,
    S: RawStore<Idx> + Default,
{
    fn default() -> Self {
        Self {
            edge: HyperEdge::default(),
            weight: Weight::default(),
        }
    }
}

impl<T, S, K, Idx> core::fmt::Display for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    T: core::fmt::Display,
    S: RawStore<Idx> + core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{{ edge: {}, weight: {} }}", self.edge, self.weight)
    }
}

impl<T, S, K, Idx> From<HyperEdge<S, K, Idx>> for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
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
    S: RawStore<Idx>,
{
    fn from(facet: HyperFacet<T, S, K, Idx>) -> Self {
        facet.edge
    }
}

impl<T, S, K, Idx> From<EdgeId<Idx>> for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: Default + RawStore<Idx>,
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
    S: RawStore<Idx>,
{
    fn as_ref(&self) -> &Weight<T> {
        &self.weight
    }
}

impl<T, S, K, Idx> AsMut<Weight<T>> for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    fn as_mut(&mut self) -> &mut Weight<T> {
        &mut self.weight
    }
}

impl<T, S, K, Idx> core::borrow::Borrow<EdgeId<Idx>> for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    fn borrow(&self) -> &EdgeId<Idx> {
        self.id()
    }
}

impl<T, S, K, Idx> core::borrow::BorrowMut<EdgeId<Idx>> for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    fn borrow_mut(&mut self) -> &mut EdgeId<Idx> {
        self.id_mut()
    }
}

impl<T, S, K, Idx> core::ops::Deref for HyperFacet<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
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
    S: RawStore<Idx>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.edge_mut()
    }
}
