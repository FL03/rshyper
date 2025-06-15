/*
    appellation: sequential <edges>
    authors: @FL03
*/
//! this module implements sequential iterator over the edges of a [`HyperMap`]
use crate::HashSurface;

use super::{Facets, FacetsMut};
use core::hash::{BuildHasher, Hash};
use rshyper_core::idx::{EdgeId, RawIndex};
use rshyper_core::prelude::GraphType;
use std::hash::RandomState;

/// [`SeqEdgeIter`] is an iterator producing references to the edges of a hypergraph w.r.t. the
/// order in-which they were inserted.
pub struct SeqFacetIter<'a, E, K, Idx, S = RandomState>
where
    S: BuildHasher,
    E: 'a,
    Idx: RawIndex,
    K: GraphType,
{
    pub(crate) keys: core::slice::Iter<'a, EdgeId<Idx>>,
    pub(crate) values: Facets<'a, E, K, Idx, S>,
}
/// [`SeqFacetIterMut`] is a mutable iterator producing mutable references to the edges of a
/// hypergraph in a manner that respects the order in-which they were inserted.
pub struct SeqFacetIterMut<'a, E, K, Idx, S = RandomState>
where
    S: BuildHasher,
    E: 'a,
    Idx: RawIndex,
    K: GraphType,
{
    pub(crate) keys: core::slice::Iter<'a, EdgeId<Idx>>,
    pub(crate) values: FacetsMut<'a, E, K, Idx, S>,
}

/*
 ************* Implementations *************
*/
impl<'a, E, K, Idx, S> Iterator for SeqFacetIter<'a, E, K, Idx, S>
where
    S: BuildHasher,
    E: 'a,
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
{
    type Item = &'a HashSurface<E, K, Idx, S>;

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

impl<'a, E, K, Idx, S> Iterator for SeqFacetIterMut<'a, E, K, Idx, S>
where
    S: BuildHasher,
    E: 'a,
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
{
    type Item = &'a mut HashSurface<E, K, Idx, S>;

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
