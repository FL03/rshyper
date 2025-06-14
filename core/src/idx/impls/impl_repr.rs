/*
    appellation: impl_repr <module>
    authors: @FL03
*/
use crate::idx::{EdgeIndex, GraphIndex, IndexBase, RawIndex, VertexIndex};
use core::sync::atomic::{AtomicUsize, Ordering};

/// a global static counter used to generate unique indices
pub(crate) static INDEX_COUNTER: AtomicUsize = AtomicUsize::new(0);

impl<K: GraphIndex> IndexBase<usize, K> {
    /// returns a new index generated using an [`AtomicUsize`]
    /// This method is useful in that it is `no_std` compatible, thread-safe, and capable of
    /// generating unique indices in a concurrent environment.
    pub fn atomic() -> Self {
        Self::new(INDEX_COUNTER.fetch_add(1, Ordering::Relaxed))
    }
    /// consumes the current index and returns a new, atomic index.
    pub fn atomic_next(&mut self) -> Self {
        let prev = self.replace(INDEX_COUNTER.fetch_add(1, Ordering::Relaxed));
        Self::new(prev)
    }
    /// initialize a new
    pub fn init() -> Self {
        #[cfg(feature = "rand")]
        {
            Self::rand()
        }
        #[cfg(not(feature = "rand"))]
        {
            Self::atomic()
        }
    }
    /// the [`next_step`](IndexBase::next_step) is useful for instances where the type `Idx` of
    /// the [`IndexBase`] is not generalized, automatically using random number generation
    /// if the `rand` feature is enabled, or an atomic counter otherwise. The method is also
    /// useful in that it generates `usize` indices, whcih are the most common instances, yet
    /// aren't direct implementors of the [`SampleUniform`](rand_distr::SampleUniform) trait.
    pub fn next_step(&mut self) -> Self {
        // declare the prev param
        let prev: usize;
        // use random number generation if the `rand` feature is enabled
        #[cfg(feature = "rand")]
        {
            // generate a random value from the standard uniform distribution
            let next = rand::random::<u128>();
            // replace the current value with the next one
            prev = self.replace(next as usize);
        }
        // otherwise use the atomic counter
        #[cfg(not(feature = "rand"))]
        {
            prev = self.replace(INDEX_COUNTER.fetch_add(1, Ordering::Relaxed));
        }
        // return a new instance with the previous value
        Self::new(prev)
    }
}

impl<T: RawIndex> IndexBase<T, EdgeIndex> {
    pub fn vertex(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: RawIndex> IndexBase<T, VertexIndex> {
    pub fn vertex(value: T) -> Self {
        Self::new(value)
    }
}
