/*
    appellation: sequential <edges>
    authors: @FL03
*/
//! this module implements sequential iterator over the edges of a [`HyperMap`](crate::HyperMap)
use crate::iter;

use core::hash::BuildHasher;
use core::slice;
use rshyper_core::GraphType;
use rshyper_core::edge::HashEdge;
use rshyper_core::idx::{EdgeId, HashIndex, RawIndex};

/// [`SeqEdgeIter`] is an iterator producing references to the edge entries w.r.t. the order
/// in-which they were inserted.
pub struct SeqEdgeIter<'a, E, K, Ix, S>
where
    S: BuildHasher + 'a,
    E: 'a,
    K: GraphType,
    Ix: RawIndex,
{
    pub(crate) keys: slice::Iter<'a, EdgeId<Ix>>,
    pub(crate) iter: iter::EdgeIter<'a, E, K, Ix, S>,
}
/// [`SeqEdgeIterMut`] is a mutable iterator producing mutable references to the edge entries
/// w.r.t. the order in-which they were inserted.
pub struct SeqEdgeIterMut<'a, E, K, Ix, S>
where
    S: BuildHasher + 'a,
    E: 'a,
    K: GraphType,
    Ix: RawIndex,
{
    pub(crate) keys: slice::Iter<'a, EdgeId<Ix>>,
    pub(crate) iter: iter::EdgeIterMut<'a, E, K, Ix, S>,
}
/// [`SeqEdgeKeys`] is a sequential iterator over the _keys_ of the edges within the
/// [`HyperMap`](crate::HyperMap) yielding the [`EdgeId`] of each edge
/// w.r.t. the order in-which they were inserted.
pub struct SeqEdgeKeys<'a, Ix>
where
    Ix: RawIndex,
{
    pub(crate) keys: slice::Iter<'a, EdgeId<Ix>>,
}
/// [`SeqEdgeValues`] is an iterator producing references to the edges of a hypergraph w.r.t. the
/// order in-which they were inserted.
pub struct SeqEdgeValues<'a, E, K, Ix, S>
where
    S: BuildHasher + 'a,
    E: 'a,
    K: GraphType,
    Ix: RawIndex,
{
    pub(crate) keys: slice::Iter<'a, EdgeId<Ix>>,
    pub(crate) values: iter::EdgeValues<'a, E, K, Ix, S>,
}
/// [`SeqEdgeValuesMut`] is a mutable iterator producing mutable references to the edges of a
/// hypergraph in a manner that respects the order in-which they were inserted.
pub struct SeqEdgeValuesMut<'a, E, K, Ix, S>
where
    S: BuildHasher + 'a,
    E: 'a,
    K: GraphType,
    Ix: RawIndex,
{
    pub(crate) keys: slice::Iter<'a, EdgeId<Ix>>,
    pub(crate) values: iter::EdgeValuesMut<'a, E, K, Ix, S>,
}

/*
 ************* Implementations *************
*/
impl<'a, E, K, Ix, S> Iterator for SeqEdgeIter<'a, E, K, Ix, S>
where
    S: BuildHasher + 'a,
    E: 'a,
    K: GraphType,
    Ix: HashIndex,
{
    type Item = (&'a EdgeId<Ix>, &'a HashEdge<E, K, Ix, S>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.keys.next() {
            // Find the edge with the matching id in the iterator
            let value = self.iter.find(|(id, _)| id == next)?;
            // Return the found edge
            return Some(value);
        }
        None
    }
}

impl<'a, E, K, Ix, S> Iterator for SeqEdgeIterMut<'a, E, K, Ix, S>
where
    S: BuildHasher + 'a,
    E: 'a,
    K: GraphType,
    Ix: HashIndex,
{
    type Item = (&'a EdgeId<Ix>, &'a mut HashEdge<E, K, Ix, S>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.keys.next() {
            // Find the edge with the matching id in the iterator
            let value = self.iter.find(|(id, _)| id == next)?;
            // Return the found edge
            return Some(value);
        }
        None
    }
}

impl<'a, Ix> Iterator for SeqEdgeKeys<'a, Ix>
where
    Ix: RawIndex,
{
    type Item = &'a EdgeId<Ix>;

    fn next(&mut self) -> Option<Self::Item> {
        self.keys.next()
    }
}

impl<'a, E, K, Ix, S> Iterator for SeqEdgeValues<'a, E, K, Ix, S>
where
    S: BuildHasher + 'a,
    E: 'a,
    K: GraphType,
    Ix: HashIndex,
{
    type Item = &'a HashEdge<E, K, Ix, S>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.keys.next() {
            // Find the node with the matching id in the nodes iterator
            let value = self.values.find(|v| v.id() == next)?;
            // Return the found node
            return Some(value);
        }
        None
    }
}

impl<'a, E, K, Ix, S> Iterator for SeqEdgeValuesMut<'a, E, K, Ix, S>
where
    S: BuildHasher + 'a,
    E: 'a,
    K: GraphType,
    Ix: HashIndex,
{
    type Item = &'a mut HashEdge<E, K, Ix, S>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.keys.next() {
            // Find the node with the matching id in the nodes iterator
            let value = self.values.find(|v| v.id() == next)?;
            // Return the found node
            return Some(value);
        }
        None
    }
}
