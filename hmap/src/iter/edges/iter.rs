/*
    appellation: surface <module>
    authors: @FL03
*/
use core::hash::BuildHasher;
use hashbrown::hash_map;
use rshyper_core::idx::{EdgeId, HashIndex, RawIndex};
use rshyper_core::edge::HashEdge;
use rshyper_core::GraphType;

/// [`EdgeIter`] is an iterator over the edge entries within the `HyperMap`, yielding a 2-tuple
/// consisting of:
///
/// - `0`: a reference to the [`EdgeId`] of the entry
/// - `1`: a reference to the [`HashEdge`] associated with the entry.
#[repr(transparent)]
pub struct EdgeIter<'a, E, K, Ix, S>
where
    K: GraphType,
    Ix: RawIndex,
    S: BuildHasher,
{
    pub(crate) iter: hash_map::Iter<'a, EdgeId<Ix>, HashEdge<E, K, Ix, S>>,
}
/// [`EdgeIterMut`] is a mutable iterator over the edge entries within the `HyperMap`, yielding
/// a 2-tuple consisting of:
///
/// - `0`: a reference to the [`EdgeId`] of the entry
/// - `1`: a mutable reference to the [`HashEdge`] associated with the entry.
#[repr(transparent)]
pub struct EdgeIterMut<'a, E, K, Ix, S>
where
    K: GraphType,
    Ix: RawIndex,
    S: BuildHasher,
{
    pub(crate) iter: hash_map::IterMut<'a, EdgeId<Ix>, HashEdge<E, K, Ix, S>>,
}
/// an iterator over the keys of the surfaces within a hypergraph, yielding the
/// [`EdgeId`]s of the entries.
#[repr(transparent)]
pub struct EdgeKeys<'a, E, K, Ix, S>
where
    K: GraphType,
    Ix: RawIndex,
    S: BuildHasher,
{
    pub(crate) iter: hash_map::Keys<'a, EdgeId<Ix>, HashEdge<E, K, Ix, S>>,
}
/// [`EdgeValues`] is an iterator over the actual surfaces of a hypergraph, yielding
pub struct EdgeValues<'a, E, K, Ix, S>
where
    E: 'a,
    Ix: RawIndex,
    K: GraphType,
    S: BuildHasher,
{
    pub(crate) iter: hash_map::Values<'a, EdgeId<Ix>, HashEdge<E, K, Ix, S>>,
}
/// [`EdgeValuesMut`] is a mutable iterator over the surfaces of a hypergraph, yielding
pub struct EdgeValuesMut<'a, E, K, Ix, S>
where
    E: 'a,
    Ix: RawIndex,
    K: GraphType,
    S: BuildHasher,
{
    pub(crate) iter: hash_map::ValuesMut<'a, EdgeId<Ix>, HashEdge<E, K, Ix, S>>,
}

/*
 ************* Implementations *************
*/
impl<'a, E, K, Ix, S> Iterator for EdgeKeys<'a, E, K, Ix, S>
where
    E: 'a,
    K: GraphType,
    Ix: HashIndex,
    S: BuildHasher + 'a,
{
    type Item = &'a EdgeId<Ix>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, E, K, Ix, S> Iterator for EdgeValues<'a, E, K, Ix, S>
where
    E: 'a,
    K: GraphType,
    Ix: HashIndex,
    S: BuildHasher + 'a,
{
    type Item = &'a HashEdge<E, K, Ix, S>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, E, K, Ix, S> Iterator for EdgeValuesMut<'a, E, K, Ix, S>
where
    E: 'a,
    K: GraphType,
    Ix: HashIndex,
    S: BuildHasher + 'a,
{
    type Item = &'a mut HashEdge<E, K, Ix, S>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, E, K, Ix, S> Iterator for EdgeIter<'a, E, K, Ix, S>
where
    E: 'a,
    K: GraphType,
    Ix: HashIndex,
    S: BuildHasher + 'a,
{
    type Item = (&'a EdgeId<Ix>, &'a HashEdge<E, K, Ix, S>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, E, K, Ix, S> Iterator for EdgeIterMut<'a, E, K, Ix, S>
where
    E: 'a,
    K: GraphType,
    Ix: HashIndex,
    S: BuildHasher + 'a,
{
    type Item = (&'a EdgeId<Ix>, &'a mut HashEdge<E, K, Ix, S>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
