/*
    appellation: surface <module>
    authors: @FL03
*/
use super::iter::{EdgeIter, EdgeIterMut, EdgeKeys};
use crate::types::HashEdge;
use core::hash::{BuildHasher, Hash};
use hashbrown::hash_map::rayon as hash_map;
use rshyper::GraphType;
use rshyper::idx::{EdgeId, RawIndex};

/// [`ParFacets`] is a parallel iterator over the edges of a hypergraph, yielding pairs of
/// [`EdgeId`] and the corresponding [`Edge`](rshyper::Edge).
#[cfg(feature = "rayon")]
#[repr(transparent)]
pub struct ParFacets<'a, E, K, Idx, S>
where
    K: GraphType + Send + Sync,
    E: 'a + Send + Sync,
    Idx: RawIndex + Eq + Hash + Send + Sync,
    S: BuildHasher + Send + Sync,
{
    pub(crate) iter: hash_map::ParValues<'a, EdgeId<Idx>, HashEdge<E, K, Idx, S>>,
}
/// [`ParFacetsMut`] is a mutable parallel iterator over the edges of a hypergraph,
/// yielding pairs of [`EdgeId`] and a mutable reference to the corresponding [`Edge`](rshyper::Edge).
#[cfg(feature = "rayon")]
#[repr(transparent)]
pub struct ParFacetsMut<'a, E, K, Idx, S>
where
    K: GraphType + Send + Sync,
    E: 'a + Send + Sync,
    Idx: RawIndex + Eq + Hash + Send + Sync,
    S: BuildHasher + Send + Sync,
{
    pub(crate) iter: hash_map::ParValuesMut<'a, EdgeId<Idx>, HashEdge<E, K, Idx, S>>,
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

impl<'a, E, K, Idx, S> ParallelIterator for ParFacets<'a, E, K, Idx, S>
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
        self.iter.into_par_iter().drive_unindexed(consumer)
    }
}

impl<'a, E, K, Idx, S> ParallelIterator for ParFacetsMut<'a, E, K, Idx, S>
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
            .into_par_iter()
            .drive_unindexed(consumer)
    }
}
