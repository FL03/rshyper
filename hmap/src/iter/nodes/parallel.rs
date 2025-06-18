/*
    appellation: parallel <nodes>
    authors: @FL03
*/
use core::hash::Hash;
use hashbrown::hash_map::rayon as hash_map;
use rayon::iter::ParallelIterator;
use rayon::iter::plumbing::UnindexedConsumer;
use rshyper::idx::{RawIndex, VertexId};
use rshyper::node::Node;

/// [`ParNodeValues`] is a parallel iterator over the nodes of a hypergraph, yielding pairs of
/// [`VertexId`] and the corresponding [`Node`].
#[cfg(feature = "rayon")]
pub struct ParNodeValues<'a, N, Idx>
where
    N: Send + Sync,
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) iter: hash_map::ParValues<'a, VertexId<Idx>, Node<N, Idx>>,
}

/// [`ParNodeValuesMut`] is a mutable parallel iterator over the nodes of a hypergraph, yielding
/// pairs of [`VertexId`] and a mutable reference to the corresponding [`Node`].
#[cfg(feature = "rayon")]
pub struct ParNodeValuesMut<'a, N, Idx>
where
    N: Send + Sync,
    Idx: RawIndex + Eq + Hash,
{
    pub(crate) iter: hash_map::ParValuesMut<'a, VertexId<Idx>, Node<N, Idx>>,
}

/*
 ************* Implementations *************
*/
impl<'a, N, Idx> ParallelIterator for ParNodeValues<'a, N, Idx>
where
    N: Send + Sync,
    Idx: RawIndex + Eq + Hash,
{
    type Item = &'a Node<N, Idx>;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.iter.drive_unindexed(consumer)
    }
}

impl<'a, N, Idx> ParallelIterator for ParNodeValuesMut<'a, N, Idx>
where
    N: Send + Sync,
    Idx: RawIndex + Eq + Hash,
{
    type Item = &'a mut Node<N, Idx>;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.iter.drive_unindexed(consumer)
    }
}
