/*
    appellation: surface <module>
    authors: @FL03
*/
use crate::hash_graph::HashFacet;
use core::hash::{BuildHasher, Hash};
use rshyper_core::GraphKind;
use rshyper_core::index::{EdgeId, RawIndex};
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
/// [`SurfaceIterMut`] is a mutable iterator over the edges of a hypergraph, yielding pairs of
/// [`EdgeId`] and a mutable reference to the corresponding [`HashFacet`].
pub struct SurfaceIterMut<'a, E, K, Idx, S = RandomState>
where
    K: GraphKind,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    pub(crate) iter: hash_map::IterMut<'a, EdgeId<Idx>, HashFacet<E, K, Idx, S>>,
}

/*
 ************* Implementations *************
*/

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

impl<'a, E, K, Idx, S> Iterator for SurfaceIterMut<'a, E, K, Idx, S>
where
    K: GraphKind,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    type Item = (&'a EdgeId<Idx>, &'a mut HashFacet<E, K, Idx, S>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
