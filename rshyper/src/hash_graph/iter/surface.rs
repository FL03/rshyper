/*
    appellation: surface <module>
    authors: @FL03
*/
use crate::hash_graph::HashFacet;
use core::hash::{BuildHasher, Hash};
use rshyper_core::GraphKind;
use rshyper_core::index::{EdgeId, RawIndex};
use std::collections::hash_map;
use std::hash::RandomState;

/// [`SurfaceIter`] is an iterator over the edges of a hypergraph, yielding pairs of
/// [`EdgeId`] and the corresponding [`HashFacet`].
pub struct SurfaceIter<'a, E, K, Idx, S = RandomState>
where
    K: GraphKind,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    pub(crate) iter: hash_map::Iter<'a, EdgeId<Idx>, HashFacet<E, K, Idx, S>>,
}
/// [`SurfaceIterMut`] is a mutable iterator over the edges of a hypergraph, yielding pairs of
/// [`EdgeId`] and a mutable reference to the corresponding [`HashFacet`].
pub struct SurfaceIterMut<'a, E, K, Idx, S = RandomState>
where
    K: GraphKind,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    pub(crate) iter: hash_map::IterMut<'a, EdgeId<Idx>, HashFacet<E, K, Idx, S>>,
}

/// [`SurfaceParIter`] is a parallel iterator over the edges of a hypergraph, yielding pairs of
/// [`EdgeId`] and the corresponding [`HashFacet`].
#[cfg(feature = "rayon")]
pub struct SurfaceParIter<'a, E, K, Idx, S = RandomState>
where
    K: GraphKind + Send + Sync,
    E: Send + Sync,
    Idx: RawIndex + Eq + Hash + Send + Sync,
    S: BuildHasher + Send + Sync,
{
    pub(crate) iter: SurfaceIter<'a, E, K, Idx, S>,
}
//// [`SurfaceParIterMut`] is a mutable parallel iterator over the edges of a hypergraph,
/// yielding pairs of [`EdgeId`] and a mutable reference to the corresponding [`HashFacet`].
#[cfg(feature = "rayon")]
pub struct SurfaceParIterMut<'a, E, K, Idx, S = RandomState>
where
    K: GraphKind + Send + Sync,
    E: Send + Sync,
    Idx: RawIndex + Eq + Hash + Send + Sync,
    S: BuildHasher + Send + Sync,
{
    pub(crate) iter: SurfaceIterMut<'a, E, K, Idx, S>,
}

/*
 ************* Implementations *************
*/
#[cfg(feature = "rayon")]
use rayon::iter::{
    IntoParallelIterator, ParallelBridge, ParallelIterator, plumbing::UnindexedConsumer,
};

impl<'a, E, K, Idx, S> Iterator for SurfaceIter<'a, E, K, Idx, S>
where
    K: GraphKind,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    type Item = (&'a EdgeId<Idx>, &'a HashFacet<E, K, Idx, S>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, E, K, Idx, S> Iterator for SurfaceIterMut<'a, E, K, Idx, S>
where
    K: GraphKind,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    type Item = (&'a EdgeId<Idx>, &'a mut HashFacet<E, K, Idx, S>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[cfg(feature = "rayon")]
impl<'a, E, K, Idx, S> ParallelIterator for SurfaceParIter<'a, E, K, Idx, S>
where
    K: GraphKind + Send + Sync,
    E: Send + Sync,
    Idx: RawIndex + Eq + Hash + Send + Sync,
    S: BuildHasher + Send + Sync,
{
    type Item = (&'a EdgeId<Idx>, &'a HashFacet<E, K, Idx, S>);

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.iter.par_bridge().drive_unindexed(consumer)
    }
}

#[cfg(feature = "rayon")]
impl<'a, E, K, Idx, S> ParallelIterator for SurfaceParIterMut<'a, E, K, Idx, S>
where
    K: GraphKind + Send + Sync,
    E: Send + Sync,
    Idx: RawIndex + Eq + Hash + Send + Sync,
    S: BuildHasher + Send + Sync,
{
    type Item = (&'a EdgeId<Idx>, &'a mut HashFacet<E, K, Idx, S>);

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
