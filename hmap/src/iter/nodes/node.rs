/*
    appellation: node <nodes>
    authors: @FL03
*/
use core::hash::Hash;
use rshyper_core::Node;
use rshyper_core::idx::{RawIndex, VertexId};
use std::collections::hash_map;

/// returns an interators over the vertices of a hypergraph, yielding the _keys_ of the nodes.
pub struct Vertices<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) iter: hash_map::Keys<'a, VertexId<Idx>, Node<N, Idx>>,
}

/// [`NodeIter`] is an iterator over the nodes of a hypergraph, yielding pairs of
/// [`VertexId`] and the corresponding [`Node`].
pub struct NodeIter<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) iter: hash_map::Iter<'a, VertexId<Idx>, Node<N, Idx>>,
}

/// [`NodeIterMut`] is a mutable iterator over the nodes of a hypergraph, yielding pairs of
/// [`VertexId`] and a mutable reference to the corresponding [`Node`].
pub struct NodeIterMut<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) iter: hash_map::IterMut<'a, VertexId<Idx>, Node<N, Idx>>,
}
/// [`NodeIterValues`] is an iterator over the values of the nodes in a hypergraph, yielding
/// the corresponding [`Node`]s.
pub struct NodeIterValues<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) iter: hash_map::Values<'a, VertexId<Idx>, Node<N, Idx>>,
}

/*
 ************* Implementations *************
*/

impl<'a, N, Idx> Iterator for Vertices<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    type Item = &'a VertexId<Idx>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

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

impl<'a, N, Idx> Iterator for NodeIterValues<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    type Item = &'a Node<N, Idx>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
