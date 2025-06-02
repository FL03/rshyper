/*
    appellation: iter <module>
    authors: @FL03
*/
use super::VertexSet;
use rshyper_core::cmp::HyperNode;
use rshyper_core::index::{EdgeId, RawIndex, VertexId};
use std::collections::hash_map;

/// [`EdgeIter`] is an iterator over the edges of a hypergraph, yielding pairs of
/// [`EdgeId`] and the corresponding [`VertexSet`].
pub struct EdgeIter<'a, Idx>
where
    Idx: RawIndex + Eq + core::hash::Hash,
{
    pub(crate) iter: hash_map::Iter<'a, EdgeId<Idx>, VertexSet<Idx>>,
}

/// [`FacetIter`] is an iterator over the facets of a hypergraph, yielding pairs of
/// [`EdgeId`] and the corresponding weight `E`.
pub struct FacetIter<'a, E, Idx>
where
    Idx: RawIndex + Eq + core::hash::Hash,
{
    pub(crate) iter: hash_map::Iter<'a, EdgeId<Idx>, crate::Weight<E>>,
}

/// [`NodeIter`] is an iterator over the nodes of a hypergraph, yielding pairs of
/// [`VertexId`] and the corresponding [`HyperNode`].
pub struct NodeIter<'a, N, Idx>
where
    Idx: RawIndex + Eq + core::hash::Hash,
{
    pub(crate) iter: hash_map::Iter<'a, VertexId<Idx>, HyperNode<N, Idx>>,
}

impl<'a, Idx> Iterator for EdgeIter<'a, Idx>
where
    Idx: RawIndex + Eq + core::hash::Hash,
{
    type Item = (&'a EdgeId<Idx>, &'a VertexSet<Idx>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, E, Idx> Iterator for FacetIter<'a, E, Idx>
where
    Idx: RawIndex + Eq + core::hash::Hash,
{
    type Item = (&'a EdgeId<Idx>, &'a crate::Weight<E>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, N, Idx> Iterator for NodeIter<'a, N, Idx>
where
    Idx: RawIndex + Eq + core::hash::Hash,
{
    type Item = (&'a VertexId<Idx>, &'a HyperNode<N, Idx>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
