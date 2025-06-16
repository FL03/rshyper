/*
    appellation: node <nodes>
    authors: @FL03
*/
use core::hash::Hash;
use rshyper::prelude::{Node, RawIndex, VertexId};
use std::collections::hash_map;

/// [`NodeIter`] is an iterator over the node entries within the `HyperMap`, yielding a 2-tuple
/// consisting of references to both each component of the entry, namely:
///
/// - `0`: &'a [`VertexId`]
/// - `1`: $'a [`Node`]
pub struct NodeIter<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) iter: hash_map::Iter<'a, VertexId<Idx>, Node<N, Idx>>,
}
/// [`NodeIterMut`] is a mutable iterator over the node entries within the `HyperMap`, yielding
/// a 2-tuple consisting of references to both each component of the entry, namely:
///
/// - `0`: &'a [`VertexId`]
/// - `1`: &'a mut [`Node`]
pub struct NodeIterMut<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) iter: hash_map::IterMut<'a, VertexId<Idx>, Node<N, Idx>>,
}
/// The [`Vertices`] iterator directly yields the _nodes_ of a hypergraph, which are strctural
/// representations of hypernodes that have knowledge of their own id.
pub struct Vertices<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) iter: hash_map::Values<'a, VertexId<Idx>, Node<N, Idx>>,
}
/// The [`VerticesMut`] iterator yields mutable references to each of the nodes within the
/// `HyperMap`
pub struct VerticesMut<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) iter: hash_map::ValuesMut<'a, VertexId<Idx>, Node<N, Idx>>,
}
/// The [`Points`] iterator yields the keys, or indices, associated with each node in the graph
pub struct Points<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) iter: hash_map::Keys<'a, VertexId<Idx>, Node<N, Idx>>,
}

/*
 ************* Implementations *************
*/

impl<'a, N, Idx> Iterator for NodeIter<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    type Item = (&'a VertexId<Idx>, &'a Node<N, Idx>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, N, Idx> Iterator for NodeIterMut<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    type Item = (&'a VertexId<Idx>, &'a mut Node<N, Idx>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, N, Idx> Iterator for Vertices<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    type Item = &'a Node<N, Idx>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, N, Idx> Iterator for VerticesMut<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    type Item = &'a mut Node<N, Idx>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, N, Idx> Iterator for Points<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    type Item = &'a VertexId<Idx>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
