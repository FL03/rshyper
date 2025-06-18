/*
    appellation: aliases <module>
    authors: @FL03
*/
//! this module provides various type aliases for the core components of a hypergraph, such as:
//!
//! - [`Udx`], [`EdgeId`], and [`VertexId`]
//! - collections: [`VertexSet`], [`VertexVec`], [`VertexVecDeque`], and others
//!
use crate::idx::{EdgeIndex, IndexBase, VertexIndex};

#[cfg(feature = "alloc")]
pub use self::use_alloc::*;

#[cfg(feature = "hashbrown")]
use hashbrown::{DefaultHashBuilder, HashSet};
#[cfg(all(feature = "std", not(feature = "hashbrown")))]
use std::collections::HashSet;
#[cfg(all(feature = "std", not(feature = "hashbrown")))]
use std::hash::RandomState as DefaultHashBuilder;

/// a type alias for a [`usize`] used to define the default index type throughout the crate.
pub type Udx = usize;
/// a type alias for an [`Index`] whose _kind_ is [`EdgeIndex`]
pub type EdgeId<T = Udx> = IndexBase<T, EdgeIndex>;
/// a type alias for an [`Index`] whose _kind_ is [`VertexIndex`]
pub type VertexId<T = Udx> = IndexBase<T, VertexIndex>;

/// a type alias for a [`HashSet`] of [`VertexId`] that is generic over the index type `I`
pub type VertexSet<I, S = DefaultHashBuilder> = HashSet<VertexId<I>, S>;
/// a type alias for a fixed sized array of [`IndexBase`]
pub type IndexArray<T, const N: usize, K = VertexIndex> = [IndexBase<T, K>; N];
/// a type alias for a slice of [`IndexBase`]
pub type IndexSlice<T, K = VertexIndex> = [IndexBase<T, K>];
/// a type alias for a mutable slice of [`IndexBase`]
pub type IndexSliceMut<'a, T, K = VertexIndex> = &'a mut [IndexBase<T, K>];
/// a type alias for a reference to a slice of [`IndexBase`]
pub type IndexSliceRef<'a, T, K = VertexIndex> = &'a [IndexBase<T, K>];

#[cfg(all(feature = "alloc", not(feature = "nightly")))]
mod use_alloc {
    use crate::idx::VertexId;

    use alloc::collections::{BTreeSet, VecDeque};
    use alloc::vec::Vec;

    /// a type alias for a [`Vec`] of [`VertexId`] that is generic over the index type `Ix`
    pub type VertexVec<Ix> = Vec<VertexId<Ix>>;
    /// a type alias for a [`VertexId`] stored in a [`VecDeque`]
    pub type VertexVecDeque<Ix> = VecDeque<VertexId<Ix>>;
    /// a type alias for a [`VertexId`] stored in a [`BTreeSet`]
    pub type VertexBSet<Ix> = BTreeSet<VertexId<Ix>>;
}

#[cfg(all(feature = "alloc", feature = "nightly"))]
mod use_alloc {
    use crate::idx::VertexId;

    use alloc::collections::{BTreeSet, VecDeque};
    use alloc::vec::Vec;

    /// a type alias for a [`Vec`] of [`VertexId`] that is generic over the index type `I` and
    /// the allocator `A`
    pub type VertexVec<I, A> = Vec<VertexId<I>, A>;
    /// a type alias for a [`VertexId`] stored in a [`VecDeque`]
    pub type VertexVecDeque<Ix, A> = VecDeque<VertexId<Ix>, A>;
    /// a type alias for a [`VertexId`] stored in a [`BTreeSet`]
    pub type VertexBSet<Ix, A> = BTreeSet<VertexId<Ix>, A>;
}
