/*
    appellation: iter <module>
    authors: @FL03
*/
use super::HashFacet;
use core::hash::BuildHasher;
use core::hash::Hash;
use rshyper_core::index::{EdgeId, RawIndex, VertexId};
use rshyper_core::{GraphKind, HyperNode};
use std::collections::hash_map;
use std::hash::RandomState;

/// [`SurfaceIter`] is an iterator over the edges of a hypergraph, yielding pairs of
/// [`EdgeId`] and the corresponding [`HashFacet`].
pub struct SurfaceIter<'a, E, K, Idx, S = RandomState>
where
    K: GraphKind,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    pub(crate) iter: hash_map::Iter<'a, EdgeId<Idx>, HashFacet<E, K, Idx, S>>,
}

/// [`NodeIter`] is an iterator over the nodes of a hypergraph, yielding pairs of
/// [`VertexId`] and the corresponding [`HyperNode`].
pub struct NodeIter<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) iter: hash_map::Iter<'a, VertexId<Idx>, HyperNode<N, Idx>>,
}

/*
 ************* Implementations *************
*/

impl<'a, N, Idx> Iterator for NodeIter<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    type Item = (&'a VertexId<Idx>, &'a HyperNode<N, Idx>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, E, K, Idx, S> Iterator for SurfaceIter<'a, E, K, Idx, S>
where
    K: GraphKind,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    type Item = (&'a EdgeId<Idx>, &'a HashFacet<E, K, Idx, S>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
