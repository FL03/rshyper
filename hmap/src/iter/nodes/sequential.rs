/*
    appellation: seq <module>
    authors: @FL03
*/
//! this module implements sequential iterator for the [`HyperMap`](crate::HyperMap)
//! which iterates over the nodes in the hypergraph in an ordered, sequential manner.
use crate::iter;
use core::slice;
use rshyper_core::idx::{HashIndex, VertexId};
use rshyper_core::node::Node;

/// [`SeqNodeIter`] is an iterator producing references to the node entries w.r.t. the order
/// in-which they were inserted.
pub struct SeqNodeIter<'a, N, Ix>
where
    N: 'a,
{
    pub(crate) keys: slice::Iter<'a, VertexId<Ix>>,
    pub(crate) iter: iter::NodeIter<'a, N, Ix>,
}
/// [`SeqNodeIterMut`] is a mutable iterator producing mutable references to the node entries
/// w.r.t. the order in-which they were inserted.
pub struct SeqNodeIterMut<'a, N, Ix>
where
    N: 'a,
{
    pub(crate) keys: slice::Iter<'a, VertexId<Ix>>,
    pub(crate) iter: iter::NodeIterMut<'a, N, Ix>,
}
/// [`SeqNodeKeys`] is a sequential iterator over the _keys_ of the nodes within the [`HyperMap`](crate::HyperMap)
/// yielding the [`VertexId`] of each node w.r.t. the order in-which they were inserted.
pub struct SeqNodeKeys<'a, Ix> {
    pub(crate) keys: slice::Iter<'a, VertexId<Ix>>,
}
/// [`SeqNodeValues`] is an iterator producing references to the nodes of a hypergraph w.r.t.
/// the order in-which they were inserted.
pub struct SeqNodeValues<'a, N, Ix>
where
    N: 'a,
{
    pub(crate) keys: slice::Iter<'a, VertexId<Ix>>,
    pub(crate) values: iter::NodeValues<'a, N, Ix>,
}
/// [`SeqNodeValuesMut`] is a mutable iterator producing mutable references to the nodes of the
/// graph in a manner that respects the order in-which they were inserted.
pub struct SeqNodeValuesMut<'a, N, Ix>
where
    N: 'a,
{
    pub(crate) keys: slice::Iter<'a, VertexId<Ix>>,
    pub(crate) values: iter::NodeValuesMut<'a, N, Ix>,
}

/*
 ************* Implementations *************
*/
impl<'a, N, Ix> Iterator for SeqNodeIter<'a, N, Ix>
where
    N: 'a,
    Ix: HashIndex,
{
    type Item = (&'a VertexId<Ix>, &'a Node<N, Ix>);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.keys.next()?;
        // Find the node with the matching id in the nodes iterator
        self.iter.find(|(id, _)| id == next)
    }
}

impl<'a, N, Ix> Iterator for SeqNodeIterMut<'a, N, Ix>
where
    N: 'a,
    Ix: HashIndex,
{
    type Item = (&'a VertexId<Ix>, &'a mut Node<N, Ix>);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.keys.next()?;
        // Find the node with the matching id in the nodes iterator
        self.iter.find(|(id, _)| id == next)
    }
}

impl<'a, Idx> Iterator for SeqNodeKeys<'a, Idx>
where
    Idx: HashIndex,
{
    type Item = &'a VertexId<Idx>;

    fn next(&mut self) -> Option<Self::Item> {
        self.keys.next()
    }
}

impl<'a, N, Idx> Iterator for SeqNodeValues<'a, N, Idx>
where
    N: 'a,
    Idx: HashIndex,
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

impl<'a, N, Idx> Iterator for SeqNodeValuesMut<'a, N, Idx>
where
    N: 'a,
    Idx: HashIndex,
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

#[allow(dead_code)]
mod _impls {
    use super::*;
    use crate::iter;

    impl<'a, Idx> SeqNodeKeys<'a, Idx>
    where
        Idx: HashIndex,
    {
        /// Creates a new sequential node keys iterator from the given slice of vertex ids.
        pub(crate) fn new(keys: &'a [VertexId<Idx>]) -> Self {
            Self { keys: keys.iter() }
        }
    }

    impl<'a, N, Idx> SeqNodeValues<'a, N, Idx>
    where
        N: 'a,
        Idx: HashIndex,
    {
        /// Creates a new sequential node values iterator from the given slice of vertex ids and the
        /// node values.
        pub(crate) fn new(keys: &'a [VertexId<Idx>], values: iter::NodeValues<'a, N, Idx>) -> Self {
            Self {
                keys: keys.iter(),
                values,
            }
        }
    }

    impl<'a, N, Idx> SeqNodeValuesMut<'a, N, Idx>
    where
        N: 'a,
        Idx: HashIndex,
    {
        /// Creates a new sequential node values mutable iterator from the given slice of vertex ids
        /// and the node values.
        pub(crate) fn new(
            keys: &'a [VertexId<Idx>],
            values: iter::NodeValuesMut<'a, N, Idx>,
        ) -> Self {
            Self {
                keys: keys.iter(),
                values,
            }
        }
    }
}
