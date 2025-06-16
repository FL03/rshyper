/*
    appellation: seq <module>
    authors: @FL03
*/
//! this module implements sequential iterator for the [`HyperMap`](crate::HyperMap)
//! which iterates over the nodes in the hypergraph in an ordered, sequential manner.
use super::{Vertices, VerticesMut};
use core::hash::Hash;
use rshyper::idx::{RawIndex, VertexId};
use rshyper::node::Node;

/// [`SeqVertexIter`] is an iterator over the nodes of a hypergraph, yielding elements
/// according to the order in-which they were inserted.
pub struct SeqVertexIter<'a, N, Idx>
where
    N: 'a,
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) keys: core::slice::Iter<'a, VertexId<Idx>>,
    pub(crate) values: Vertices<'a, N, Idx>,
}

/// [`SeqVertexIterMut`] is a mutable iterator producing mutable references to the nodes of the
/// graph in a manner that respects the order in-which they were inserted.
pub struct SeqVertexIterMut<'a, N, Idx>
where
    N: 'a,
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) keys: core::slice::Iter<'a, VertexId<Idx>>,
    pub(crate) values: VerticesMut<'a, N, Idx>,
}

/*
 ************* Implementations *************
*/

impl<'a, N, Idx> Iterator for SeqVertexIter<'a, N, Idx>
where
    N: 'a,
    Idx: RawIndex + Eq + Hash,
{
    type Item = &'a Node<N, Idx>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.keys.next() {
            // Find the node with the matching id in the nodes iterator
            let node = self.values.find(|node| node.id() == next)?;
            // Return the found node
            return Some(node);
        }
        None
    }
}

impl<'a, N, Idx> Iterator for SeqVertexIterMut<'a, N, Idx>
where
    N: 'a,
    Idx: RawIndex + Eq + Hash,
{
    type Item = &'a mut Node<N, Idx>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.keys.next() {
            // Find the node with the matching id in the nodes iterator
            let node = self.values.find(|node| node.id() == next)?;
            // Return the found node
            return Some(node);
        }
        None
    }
}
