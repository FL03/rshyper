/*
    appellation: iter <module>
    authors: @FL03
*/
use super::HashFacet;
use rshyper_core::GraphKind;
use rshyper_core::cmp::HyperNode;
use rshyper_core::index::{EdgeId, RawIndex, VertexId};
use std::collections::hash_map;

/// [`SurfaceIter`] is an iterator over the edges of a hypergraph, yielding pairs of
/// [`EdgeId`] and the corresponding [`HashFacet`].
pub struct SurfaceIter<'a, E, K, Idx>
where
    K: GraphKind,
    Idx: RawIndex + Eq + core::hash::Hash,
{
    pub(crate) iter: hash_map::Iter<'a, EdgeId<Idx>, HashFacet<E, K, Idx>>,
}

/// [`NodeIter`] is an iterator over the nodes of a hypergraph, yielding pairs of
/// [`VertexId`] and the corresponding [`HyperNode`].
pub struct NodeIter<'a, N, Idx>
where
    Idx: RawIndex + Eq + core::hash::Hash,
{
    pub(crate) iter: hash_map::Iter<'a, VertexId<Idx>, HyperNode<N, Idx>>,
}

/*
 ************* Implementations *************
*/

impl<'a, N, Idx> Iterator for NodeIter<'a, N, Idx>
where
    Idx: RawIndex + Eq + core::hash::Hash,
{
    type Item = (&'a VertexId<Idx>, &'a HyperNode<N, Idx>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, E, K, Idx> Iterator for SurfaceIter<'a, E, K, Idx>
where
    K: GraphKind,
    Idx: RawIndex + Eq + core::hash::Hash,
{
    type Item = (&'a EdgeId<Idx>, &'a HashFacet<E, K, Idx>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
