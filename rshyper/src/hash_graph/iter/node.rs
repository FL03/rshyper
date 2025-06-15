/*
    appellation: node <module>
    authors: @FL03
*/
use core::hash::Hash;
use rshyper_core::Node;
use rshyper_core::idx::{RawIndex, VertexId};
use std::collections::hash_map;

#[doc(hidden)]
/// returns an iterator that starts at the "first" vertex and follows the path until the end
pub struct DirectedNodeIter<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    #[allow(dead_code)]
    pub(crate) iter: hash_map::Iter<'a, VertexId<Idx>, Node<N, Idx>>,
}


/// returns an interators over the vertices of a hypergraph, yielding the _keys_ of the nodes.
pub struct Vertices<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) iter: hash_map::Keys<'a, VertexId<Idx>, Node<N, Idx>>,
}

/// [`NodeIter`] is an iterator over the nodes of a hypergraph, yielding pairs of
/// [`VertexId`] and the corresponding [`HyperNode`].
pub struct NodeIter<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) iter: hash_map::Iter<'a, VertexId<Idx>, Node<N, Idx>>,
}

/// [`NodeIterMut`] is a mutable iterator over the nodes of a hypergraph, yielding pairs of
/// [`VertexId`] and a mutable reference to the corresponding [`HyperNode`].
pub struct NodeIterMut<'a, N, Idx>
where
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) iter: hash_map::IterMut<'a, VertexId<Idx>, Node<N, Idx>>,
}

/// [`NodeParIter`] is a parallel iterator over the nodes of a hypergraph, yielding pairs of
/// [`VertexId`] and the corresponding [`HyperNode`].
#[cfg(feature = "rayon")]
pub struct NodeParIter<'a, N, Idx>
where
    N: Send + Sync,
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) iter: NodeIter<'a, N, Idx>,
}

/// [`NodeParIterMut`] is a mutable parallel iterator over the nodes of a hypergraph, yielding
/// pairs of [`VertexId`] and a mutable reference to the corresponding [`HyperNode`].
#[cfg(feature = "rayon")]
pub struct NodeParIterMut<'a, N, Idx>
where
    N: Send + Sync,
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) iter: NodeIterMut<'a, N, Idx>,
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

#[cfg(feature = "rayon")]
mod impl_par {
    use super::*;
    use rayon::iter::plumbing::UnindexedConsumer;
    use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

    impl<'a, N, Idx> ParallelIterator for NodeIter<'a, N, Idx>
    where
        N: Send + Sync,
        Idx: RawIndex + Eq + Hash,
    {
        type Item = (&'a VertexId<Idx>, &'a Node<N, Idx>);

        fn drive_unindexed<C>(self, consumer: C) -> C::Result
        where
            C: UnindexedConsumer<Self::Item>,
        {
            self.iter
                .par_bridge()
                .into_par_iter()
                .drive_unindexed(consumer)
        }
    }

    impl<'a, N, Idx> ParallelIterator for NodeIterMut<'a, N, Idx>
    where
        N: Send + Sync,
        Idx: RawIndex + Eq + Hash,
    {
        type Item = (&'a VertexId<Idx>, &'a mut Node<N, Idx>);

        fn drive_unindexed<C>(self, consumer: C) -> C::Result
        where
            C: UnindexedConsumer<Self::Item>,
        {
            self.iter
                .par_bridge()
                .into_par_iter()
                .drive_unindexed(consumer)
        }
    }

    impl<'a, N, Idx> ParallelIterator for NodeParIter<'a, N, Idx>
    where
        N: Send + Sync,
        Idx: RawIndex + Eq + Hash,
    {
        type Item = (&'a VertexId<Idx>, &'a Node<N, Idx>);

        fn drive_unindexed<C>(self, consumer: C) -> C::Result
        where
            C: UnindexedConsumer<Self::Item>,
        {
            self.iter
                .par_bridge()
                .into_par_iter()
                .drive_unindexed(consumer)
        }
    }

    impl<'a, N, Idx> ParallelIterator for NodeParIterMut<'a, N, Idx>
    where
        N: Send + Sync,
        Idx: RawIndex + Eq + Hash,
    {
        type Item = (&'a VertexId<Idx>, &'a mut Node<N, Idx>);

        fn drive_unindexed<C>(self, consumer: C) -> C::Result
        where
            C: UnindexedConsumer<Self::Item>,
        {
            self.iter
                .par_bridge()
                .into_par_iter()
                .drive_unindexed(consumer)
        }
    }
}
