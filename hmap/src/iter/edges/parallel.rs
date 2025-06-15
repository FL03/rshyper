/*
    appellation: surface <module>
    authors: @FL03
*/
use super::surface::{Edges, SurfaceIter, SurfaceIterMut};
use crate::types::HashSurface;
use core::hash::{BuildHasher, Hash};
use rshyper_core::GraphType;
use rshyper_core::idx::{EdgeId, RawIndex};
use std::hash::RandomState;

/// [`SurfaceParIter`] is a parallel iterator over the edges of a hypergraph, yielding pairs of
/// [`EdgeId`] and the corresponding [`HashFacet`].
#[cfg(feature = "rayon")]
#[repr(transparent)]
pub struct SurfaceParIter<'a, E, K, Idx, S = RandomState>
where
    K: GraphType + Send + Sync,
    E: Send + Sync,
    Idx: RawIndex + Eq + Hash + Send + Sync,
    S: BuildHasher + Send + Sync,
{
    pub(crate) iter: SurfaceIter<'a, E, K, Idx, S>,
}
/// [`SurfaceParIterMut`] is a mutable parallel iterator over the edges of a hypergraph,
/// yielding pairs of [`EdgeId`] and a mutable reference to the corresponding [`HashFacet`].
#[cfg(feature = "rayon")]
#[repr(transparent)]
pub struct SurfaceParIterMut<'a, E, K, Idx, S = RandomState>
where
    K: GraphType + Send + Sync,
    E: Send + Sync,
    Idx: RawIndex + Eq + Hash + Send + Sync,
    S: BuildHasher + Send + Sync,
{
    pub(crate) iter: SurfaceIterMut<'a, E, K, Idx, S>,
}

/*
 ************* Implementations *************
*/

use rayon::iter::plumbing::UnindexedConsumer;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

impl<'a, E, K, Idx, S> ParallelIterator for Edges<'a, E, K, Idx, S>
where
    K: GraphType + Send + Sync,
    E: Send + Sync,
    Idx: RawIndex + Eq + Hash + Send + Sync,
    S: BuildHasher + Send + Sync,
{
    type Item = &'a EdgeId<Idx>;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.iter.par_bridge().drive_unindexed(consumer)
    }
}

impl<'a, E, K, Idx, S> ParallelIterator for SurfaceIter<'a, E, K, Idx, S>
where
    K: GraphType + Send + Sync,
    E: Send + Sync,
    Idx: RawIndex + Eq + Hash + Send + Sync,
    S: BuildHasher + Send + Sync,
{
    type Item = (&'a EdgeId<Idx>, &'a HashSurface<E, K, Idx, S>);

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.iter.par_bridge().drive_unindexed(consumer)
    }
}

impl<'a, E, K, Idx, S> ParallelIterator for SurfaceIterMut<'a, E, K, Idx, S>
where
    K: GraphType + Send + Sync,
    E: Send + Sync,
    Idx: RawIndex + Eq + Hash + Send + Sync,
    S: BuildHasher + Send + Sync,
{
    type Item = (&'a EdgeId<Idx>, &'a mut HashSurface<E, K, Idx, S>);

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

impl<'a, E, K, Idx, S> ParallelIterator for SurfaceParIter<'a, E, K, Idx, S>
where
    K: GraphType + Send + Sync,
    E: Send + Sync,
    Idx: RawIndex + Eq + Hash + Send + Sync,
    S: BuildHasher + Send + Sync,
{
    type Item = (&'a EdgeId<Idx>, &'a HashSurface<E, K, Idx, S>);

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.iter.par_bridge().drive_unindexed(consumer)
    }
}

impl<'a, E, K, Idx, S> ParallelIterator for SurfaceParIterMut<'a, E, K, Idx, S>
where
    K: GraphType + Send + Sync,
    E: Send + Sync,
    Idx: RawIndex + Eq + Hash + Send + Sync,
    S: BuildHasher + Send + Sync,
{
    type Item = (&'a EdgeId<Idx>, &'a mut HashSurface<E, K, Idx, S>);

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
