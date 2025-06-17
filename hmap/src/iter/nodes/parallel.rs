/*
    appellation: parallel <nodes>
    authors: @FL03
*/
use super::iter::*;
use core::hash::Hash;
use hashbrown::hash_map::rayon as hash_map;
use rayon::iter::plumbing::UnindexedConsumer;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use rshyper::idx::{RawIndex, VertexId};
use rshyper::node::Node;

/// [`NodeParIter`] is a parallel iterator over the nodes of a hypergraph, yielding pairs of
/// [`VertexId`] and the corresponding [`Node`].
#[cfg(feature = "rayon")]
pub struct NodeParIter<'a, N, Idx>
where
    N: Send + Sync,
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) iter: hash_map::ParValues<'a, VertexId<Idx>, Node<N, Idx>>,
}

/// [`NodeParIterMut`] is a mutable parallel iterator over the nodes of a hypergraph, yielding
/// pairs of [`VertexId`] and a mutable reference to the corresponding [`Node`].
#[cfg(feature = "rayon")]
pub struct NodeParIterMut<'a, N, Idx>
where
    N: Send + Sync,
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) iter: hash_map::ParValuesMut<'a, VertexId<Idx>, Node<N, Idx>>,
}

/*
 ************* Implementations *************
*/

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
            .into_par_iter()
            .drive_unindexed(consumer)
    }
}
