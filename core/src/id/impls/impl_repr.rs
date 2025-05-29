/*
    appellation: impl_repr <module>
    authors: @FL03
*/
use crate::id::{EdgeIndex, Index, IndexKind, VertexIndex};

impl<K> Index<usize, K>
where
    K: IndexKind,
{
    pub fn atomic() -> Self {
        use core::sync::atomic::{AtomicUsize, Ordering::Relaxed};
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        Self::from_value(COUNTER.fetch_add(1, Relaxed))
    }
}

impl<T> Index<T, EdgeIndex> {
    pub fn vertex(value: T) -> Self {
        Self::from_value(value)
    }
}

impl<T> Index<T, VertexIndex> {
    pub fn vertex(value: T) -> Self {
        Self::from_value(value)
    }
}
