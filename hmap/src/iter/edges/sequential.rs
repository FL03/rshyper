/*
    appellation: sequential <edges>
    authors: @FL03
*/
//! this module implements sequential iterator over the edges of a [`HyperMap`]
use crate::HashSurface;

use super::Facets;
use core::hash::{BuildHasher, Hash};
use rshyper_core::idx::{EdgeId, RawIndex};
use rshyper_core::prelude::GraphType;
use std::hash::RandomState;

/// [`SeqEdgeIter`] is an iterator over the nodes of a hypergraph, yielding elements according
/// to the order in-which they were inserted.
pub struct SeqEdgeIter<'a, E, K, Idx, S = RandomState>
where
    S: BuildHasher,
    E: 'a,
    Idx: RawIndex,
    K: GraphType,
{
    pub(crate) keys: core::slice::Iter<'a, EdgeId<Idx>>,
    pub(crate) values: Facets<'a, E, K, Idx, S>,
}

/*
 ************* Implementations *************
*/
impl<'a, E, K, Idx, S> Iterator for SeqEdgeIter<'a, E, K, Idx, S>
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
