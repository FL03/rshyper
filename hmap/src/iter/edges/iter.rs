/*
    appellation: surface <module>
    authors: @FL03
*/
use crate::types::HashEdge;
use core::hash::{BuildHasher, Hash};
use hashbrown::hash_map;
use rshyper_core::GraphType;
use rshyper_core::idx::{EdgeId, RawIndex};

/// [`EdgeIter`] is an iterator over the edge entries within the `HyperMap`, yielding
/// a 2-tuple consisting of references to the [`EdgeId`] and [`HashSurface`] of the entry.
#[repr(transparent)]
pub struct EdgeIter<'a, E, K, Idx, S>
where
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    pub(crate) iter: hash_map::Iter<'a, EdgeId<Idx>, HashEdge<E, K, Idx, S>>,
}
/// [`EdgeIterMut`] is a mutable iterator over the edge entries within the `HyperMap`,
/// yielding a 2-tuple consisting of references to the entry [`EdgeId`] and a mutable
/// reference to the [`HashSurface`].
#[repr(transparent)]
pub struct EdgeIterMut<'a, E, K, Idx, S>
where
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    pub(crate) iter: hash_map::IterMut<'a, EdgeId<Idx>, HashEdge<E, K, Idx, S>>,
}
/// an iterator over the keys of the surfaces within a hypergraph, yielding the
/// [`EdgeId`]s of the entries.
#[repr(transparent)]
pub struct EdgeKeys<'a, E, K, Idx, S>
where
    Idx: RawIndex + Eq + Hash,
    K: GraphType,
    S: BuildHasher,
{
    pub(crate) iter: hash_map::Keys<'a, EdgeId<Idx>, HashEdge<E, K, Idx, S>>,
}
/// [`Edges`] is an iterator over the actual surfaces of a hypergraph, yielding
pub struct Edges<'a, E, K, Idx, S>
where
    E: 'a,
    Idx: RawIndex,
    K: GraphType,
    S: BuildHasher,
{
    pub(crate) iter: hash_map::Values<'a, EdgeId<Idx>, HashEdge<E, K, Idx, S>>,
}
/// [`EdgesMut`] is a mutable iterator over the surfaces of a hypergraph, yielding
pub struct EdgesMut<'a, E, K, Idx, S>
where
    E: 'a,
    Idx: RawIndex,
    K: GraphType,
    S: BuildHasher,
{
    pub(crate) iter: hash_map::ValuesMut<'a, EdgeId<Idx>, HashEdge<E, K, Idx, S>>,
}

/*
 ************* Implementations *************
*/
impl<'a, E, K, Idx, S> Iterator for EdgeKeys<'a, E, K, Idx, S>
where
    E: 'a,
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher + 'a,
{
    type Item = &'a EdgeId<Idx>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, E, K, Idx, S> Iterator for Edges<'a, E, K, Idx, S>
where
    E: 'a,
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher + 'a,
{
    type Item = &'a HashEdge<E, K, Idx, S>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, E, K, Idx, S> Iterator for EdgesMut<'a, E, K, Idx, S>
where
    E: 'a,
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher + 'a,
{
    type Item = &'a mut HashEdge<E, K, Idx, S>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, E, K, Idx, S> Iterator for EdgeIter<'a, E, K, Idx, S>
where
    E: 'a,
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher + 'a,
{
    type Item = (&'a EdgeId<Idx>, &'a HashEdge<E, K, Idx, S>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, E, K, Idx, S> Iterator for EdgeIterMut<'a, E, K, Idx, S>
where
    E: 'a,
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher + 'a,
{
    type Item = (&'a EdgeId<Idx>, &'a mut HashEdge<E, K, Idx, S>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
