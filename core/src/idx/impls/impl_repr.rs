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
