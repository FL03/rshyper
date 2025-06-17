/*
    appellation: surface <module>
    authors: @FL03
*/
use super::iter::{EdgeIter, EdgeIterMut, EdgeKeys};
use crate::types::HashEdge;
use core::hash::{BuildHasher, Hash};
use rshyper::GraphType;
use rshyper::idx::{EdgeId, RawIndex};

/// [`ParEdgeIter`] is a parallel iterator over the edges of a hypergraph, yielding pairs of
/// [`EdgeId`] and the corresponding [`HashFacet`].
#[cfg(feature = "rayon")]
#[repr(transparent)]
pub struct ParEdgeIter<'a, E, K, Idx, S>
where
    K: GraphType + Send + Sync,
    E: 'a + Send + Sync,
    Idx: RawIndex + Eq + Hash + Send + Sync,
    S: BuildHasher + Send + Sync,
{
    pub(crate) iter: EdgeIter<'a, E, K, Idx, S>,
}
/// [`ParEdgeIterMut`] is a mutable parallel iterator over the edges of a hypergraph,
/// yielding pairs of [`EdgeId`] and a mutable reference to the corresponding [`HashFacet`].
#[cfg(feature = "rayon")]
#[repr(transparent)]
pub struct ParEdgeIterMut<'a, E, K, Idx, S>
where
    K: GraphType + Send + Sync,
    E: 'a + Send + Sync,
    Idx: RawIndex + Eq + Hash + Send + Sync,
    S: BuildHasher + Send + Sync,
{
    pub(crate) iter: EdgeIterMut<'a, E, K, Idx, S>,
}

/*
 ************* Implementations *************
*/

use rayon::iter::plumbing::UnindexedConsumer;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

impl<'a, E, K, Idx, S> ParallelIterator for EdgeKeys<'a, E, K, Idx, S>
where
    K: GraphType + Send + Sync,
    E: 'a + Send + Sync,
    Idx: RawIndex + Eq + Hash + Send + Sync,
    S: BuildHasher + Send + Sync + 'a,
{
    type Item = &'a EdgeId<Idx>;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.iter.par_bridge().drive_unindexed(consumer)
    }
}

impl<'a, E, K, Idx, S> ParallelIterator for EdgeIter<'a, E, K, Idx, S>
where
    K: GraphType + Send + Sync,
    E: 'a + Send + Sync,
    Idx: RawIndex + Eq + Hash + Send + Sync,
    S: BuildHasher + Send + Sync + 'a,
{
    type Item = (&'a EdgeId<Idx>, &'a HashEdge<E, K, Idx, S>);

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.iter.par_bridge().drive_unindexed(consumer)
    }
}

impl<'a, E, K, Idx, S> ParallelIterator for EdgeIterMut<'a, E, K, Idx, S>
where
    K: GraphType + Send + Sync,
    E: 'a + Send + Sync,
    Idx: RawIndex + Eq + Hash + Send + Sync,
    S: BuildHasher + Send + Sync + 'a,
{
    type Item = (&'a EdgeId<Idx>, &'a mut HashEdge<E, K, Idx, S>);

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

impl<'a, E, K, Idx, S> ParallelIterator for ParEdgeIter<'a, E, K, Idx, S>
where
    K: GraphType + Send + Sync,
    E: 'a + Send + Sync,
    Idx: RawIndex + Eq + Hash + Send + Sync,
    S: BuildHasher + Send + Sync + 'a,
{
    type Item = (&'a EdgeId<Idx>, &'a HashEdge<E, K, Idx, S>);

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.iter.par_bridge().drive_unindexed(consumer)
    }
}

impl<'a, E, K, Idx, S> ParallelIterator for ParEdgeIterMut<'a, E, K, Idx, S>
where
    K: GraphType + Send + Sync,
    E: 'a + Send + Sync,
    Idx: RawIndex + Eq + Hash + Send + Sync,
    S: BuildHasher + Send + Sync + 'a,
{
    type Item = (&'a EdgeId<Idx>, &'a mut HashEdge<E, K, Idx, S>);

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
