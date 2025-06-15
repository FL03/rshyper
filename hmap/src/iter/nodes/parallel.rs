/*
    appellation: parallel <nodes>
    authors: @FL03
*/
use super::node::*;
use core::hash::Hash;
use rayon::iter::plumbing::UnindexedConsumer;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use rshyper_core::Node;
use rshyper_core::idx::{RawIndex, VertexId};

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
