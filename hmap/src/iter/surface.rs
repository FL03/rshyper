/*
    appellation: surface <module>
    authors: @FL03
*/
use crate::types::HashFacet;
use core::hash::{BuildHasher, Hash};
use rshyper_core::GraphType;
use rshyper_core::idx::{EdgeId, RawIndex};
use std::collections::hash_map;
use std::hash::RandomState;

/// an iterator over the keys of the surfaces within a hypergraph, yielding the
/// [`EdgeId`]s of the entries.
#[repr(transparent)]
pub struct Edges<'a, E, K, Idx, S = RandomState>
where
    Idx: RawIndex + Eq + Hash,
    K: GraphType,
    S: BuildHasher,
{
    pub(crate) iter: hash_map::Keys<'a, EdgeId<Idx>, HashFacet<E, K, Idx, S>>,
}

/// [`SurfaceIter`] is an iterator over the edges of a hypergraph, yielding pairs of
/// [`EdgeId`] and the corresponding [`HashFacet`].
#[repr(transparent)]
pub struct SurfaceIter<'a, E, K, Idx, S = RandomState>
where
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    pub(crate) iter: hash_map::Iter<'a, EdgeId<Idx>, HashFacet<E, K, Idx, S>>,
}
/// [`SurfaceIterMut`] is a mutable iterator over the edges of a hypergraph, yielding pairs of
/// [`EdgeId`] and a mutable reference to the corresponding [`HashFacet`].
#[repr(transparent)]
pub struct SurfaceIterMut<'a, E, K, Idx, S = RandomState>
where
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    pub(crate) iter: hash_map::IterMut<'a, EdgeId<Idx>, HashFacet<E, K, Idx, S>>,
}

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
impl<'a, E, K, Idx, S> Iterator for Edges<'a, E, K, Idx, S>
where
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    type Item = &'a EdgeId<Idx>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, E, K, Idx, S> Iterator for SurfaceIter<'a, E, K, Idx, S>
where
    K: GraphType,
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
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    type Item = (&'a EdgeId<Idx>, &'a mut HashFacet<E, K, Idx, S>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/*
 ************* Parallel Implementations *************
*/
#[cfg(feature = "rayon")]
mod impl_par {
    use super::*;
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
        type Item = (&'a EdgeId<Idx>, &'a HashFacet<E, K, Idx, S>);

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

    impl<'a, E, K, Idx, S> ParallelIterator for SurfaceParIter<'a, E, K, Idx, S>
    where
        K: GraphType + Send + Sync,
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

    impl<'a, E, K, Idx, S> ParallelIterator for SurfaceParIterMut<'a, E, K, Idx, S>
    where
        K: GraphType + Send + Sync,
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
}
