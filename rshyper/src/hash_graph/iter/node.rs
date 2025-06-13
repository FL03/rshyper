/*
    appellation: node <module>
    authors: @FL03
*/
use core::hash::Hash;
use rshyper_core::index::{RawIndex, VertexId};
use rshyper_core::HyperNode;
use std::collections::hash_map;


/// [`NodeIter`] is an iterator over the nodes of a hypergraph, yielding pairs of
/// [`VertexId`] and the corresponding [`HyperNode`].
pub struct NodeIter<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) iter: hash_map::Iter<'a, VertexId<Idx>, HyperNode<N, Idx>>,
}

/// [`NodeIterMut`] is a mutable iterator over the nodes of a hypergraph, yielding pairs of
/// [`VertexId`] and a mutable reference to the corresponding [`HyperNode`].
pub struct NodeIterMut<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) iter: hash_map::IterMut<'a, VertexId<Idx>, HyperNode<N, Idx>>,
}

/*
 ************* Implementations *************
*/

impl<'a, N, Idx> Iterator for NodeIter<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    type Item = (&'a VertexId<Idx>, &'a HyperNode<N, Idx>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, N, Idx> Iterator for NodeIterMut<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    type Item = (&'a VertexId<Idx>, &'a mut HyperNode<N, Idx>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
