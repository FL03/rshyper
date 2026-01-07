/*
    appellation: surface <module>
    authors: @FL03
*/
use core::hash::{BuildHasher, Hash};
use hashbrown::hash_map::rayon as hash_map;
use rayon::iter::ParallelIterator;
use rayon::iter::plumbing::UnindexedConsumer;
use rshyper_core::edge::HashEdge;
use rshyper_core::idx::{EdgeId, RawIndex};
use rshyper_core::GraphType;

/// [`ParEdgeValues`] is a parallel iterator over the edges of a hypergraph, yielding
/// references to the edges as [`HashEdge`]s.
#[repr(transparent)]
pub struct ParEdgeValues<'a, E, K, Idx, S>
where
    K: GraphType + Send + Sync,
    E: 'a + Send + Sync,
    Idx: RawIndex + Eq + Hash + Send + Sync,
    S: BuildHasher + Send + Sync,
{
    pub(crate) iter: hash_map::ParValues<'a, EdgeId<Idx>, HashEdge<E, K, Idx, S>>,
}
/// [`ParEdgeValuesMut`] is a mutable parallel iterator over the edges of a hypergraph,
/// yielding a mutable reference to the corresponding [`HashEdge`].
#[repr(transparent)]
pub struct ParEdgeValuesMut<'a, E, K, Idx, S>
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

impl<'a, E, K, Idx, S> ParallelIterator for ParEdgeValues<'a, E, K, Idx, S>
where
    K: GraphType + Send + Sync,
    E: 'a + Send + Sync,
    Idx: RawIndex + Eq + Hash + Send + Sync,
    S: BuildHasher + Send + Sync + 'a,
{
    type Item = &'a HashEdge<E, K, Idx, S>;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.iter.drive_unindexed(consumer)
    }
}

impl<'a, E, K, Idx, S> ParallelIterator for ParEdgeValuesMut<'a, E, K, Idx, S>
where
    K: GraphType + Send + Sync,
    E: 'a + Send + Sync,
    Idx: RawIndex + Eq + Hash + Send + Sync,
    S: BuildHasher + Send + Sync + 'a,
{
    type Item = &'a mut HashEdge<E, K, Idx, S>;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.iter.drive_unindexed(consumer)
    }
}
