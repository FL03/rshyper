/*
    appellation: surface <module>
    authors: @FL03
*/
use crate::types::HashFacet;
use core::hash::{BuildHasher, Hash};
use rshyper_core::GraphType;
use rshyper_core::idx::{EdgeId, RawIndex};
use std::collections::hash_map;
use std::hash::RandomState;

/// an iterator over the keys of the surfaces within a hypergraph, yielding the
/// [`EdgeId`]s of the entries.
#[repr(transparent)]
pub struct Edges<'a, E, K, Idx, S = RandomState>
where
    Idx: RawIndex + Eq + Hash,
    K: GraphType,
    S: BuildHasher,
{
    pub(crate) iter: hash_map::Keys<'a, EdgeId<Idx>, HashFacet<E, K, Idx, S>>,
}

/// [`SurfaceIter`] is an iterator over the edges of a hypergraph, yielding pairs of
/// [`EdgeId`] and the corresponding [`HashFacet`].
#[repr(transparent)]
pub struct SurfaceIter<'a, E, K, Idx, S = RandomState>
where
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    pub(crate) iter: hash_map::Iter<'a, EdgeId<Idx>, HashFacet<E, K, Idx, S>>,
}
/// [`SurfaceIterMut`] is a mutable iterator over the edges of a hypergraph, yielding pairs of
/// [`EdgeId`] and a mutable reference to the corresponding [`HashFacet`].
#[repr(transparent)]
pub struct SurfaceIterMut<'a, E, K, Idx, S = RandomState>
where
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    pub(crate) iter: hash_map::IterMut<'a, EdgeId<Idx>, HashFacet<E, K, Idx, S>>,
}

/*
 ************* Implementations *************
*/
impl<'a, E, K, Idx, S> Iterator for Edges<'a, E, K, Idx, S>
where
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    type Item = &'a EdgeId<Idx>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, E, K, Idx, S> Iterator for SurfaceIter<'a, E, K, Idx, S>
where
    K: GraphType,
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
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    type Item = (&'a EdgeId<Idx>, &'a mut HashFacet<E, K, Idx, S>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
