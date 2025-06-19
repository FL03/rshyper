/*
    appellation: iter <nodes>
    authors: @FL03
*/
use core::hash::Hash;
use hashbrown::hash_map;
use rshyper::prelude::{Node, RawIndex, VertexId};

/// [`NodeIter`] is an iterator over the node entries within the `HyperMap`, yielding a 2-tuple
/// consisting of references to both each component of the entry, namely:
///
/// - `0`: &'a [`VertexId`]
/// - `1`: &'a [`Node`]
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
/// The [`NodeKeys`] iterator yields references to the keys, or "vertices", of the hypergraph.
pub struct NodeKeys<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) iter: hash_map::Keys<'a, VertexId<Idx>, Node<N, Idx>>,
}
/// [`NodeValues`] is an iterator over the actual hypernodes of the graph, yielding references
/// to each [`Node`].
pub struct NodeValues<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) iter: hash_map::Values<'a, VertexId<Idx>, Node<N, Idx>>,
}
/// [`NodeValuesMut`] is a mutable iterator over the actual hypernodes of the graph, yielding
/// mutable references to each [`Node`].
pub struct NodeValuesMut<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) iter: hash_map::ValuesMut<'a, VertexId<Idx>, Node<N, Idx>>,
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

impl<'a, N, Idx> Iterator for NodeValues<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    type Item = &'a Node<N, Idx>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, N, Idx> Iterator for NodeValuesMut<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    type Item = &'a mut Node<N, Idx>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, N, Idx> Iterator for NodeKeys<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    type Item = &'a VertexId<Idx>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
