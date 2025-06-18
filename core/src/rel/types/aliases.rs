/*
    appellation: aliases <rel>
    authors: @FL03
*/
//! this module provides various type aliases relationships with hypergraphs such as:
//! [`DiLink`], [`UnLink`] and [`HashLink`]
#[cfg(feature = "alloc")]
pub use self::use_alloc::*;

use crate::rel::Link;
use crate::{Directed, Undirected, VertexId};

#[cfg(feature = "hashbrown")]
use hashbrown::{DefaultHashBuilder, HashSet};
#[cfg(all(feature = "std", not(feature = "hashbrown")))]
use std::collections::HashSet;
#[cfg(all(feature = "std", not(feature = "hashbrown")))]
use std::hash::RandomState as DefaultHashBuilder;

/// a type alias for a [`HashSet`] of [`VertexId`] that is generic over the index type `I`
pub type VertexSet<I, S = DefaultHashBuilder> = HashSet<VertexId<I>, S>;

/// a type alias for a [`Directed`] relationship
pub type DiLink<S, Ix> = Link<S, Directed, Ix>;
/// a type alias for an [`Undirected`] relationship
pub type UnLink<S, Ix> = Link<S, Undirected, Ix>;

/// a type alias for an [`Edge`] whose _vertices_ are stored in a [`VertexSet`]
pub type HashLink<K, I, S> = Link<VertexSet<I, S>, K, I>;

/// a type alias for a [`Link`] whose _vertices_ are stored in an array of fixed
/// size with the size defined by the generic parameter `N`.
pub type LinkArray<const N: usize, K, Ix> = Link<[VertexId<Ix>; N], K, Ix>;
/// a type alias for a [`Link`] whose _vertices_ are stored in a slice
pub type LinkSlice<K, Ix> = Link<[VertexId<Ix>], K, Ix>;
/// a type alias for a [`Link`] whose _vertices_ are stored in an owned slice
pub type LinkSliceRef<'a, K, Ix> = Link<&'a [VertexId<Ix>], K, Ix>;
/// a type alias for a [`Link`] whose _vertices_ are stored in a mutable slice
pub type LinkSliceMut<'a, K, Ix> = Link<&'a mut [VertexId<Ix>], K, Ix>;

#[cfg(all(feature = "alloc", not(feature = "nightly")))]
mod use_alloc {
    use crate::{Edge, Link, VertexId};

    use alloc::collections::{BTreeSet, VecDeque};
    use alloc::vec::Vec;

    /// a type alias for an [`Link`] whose _vertices_ are stored in a [`Vec`]
    pub type LinkVec<K, Ix> = Link<VertexVec<Ix>, K, Ix>;
    /// a type alias for an [`Link`] whose _vertices_ are stored in a [`VecDeque`]
    pub type LinkDeque<K, Ix> = Link<VertexVecDeque<Ix>, K, Ix>;
    /// a type alias for an [`Link`] whose _vertices_ are stored in a [`BTreeSet`]
    pub type LinkBSet<K, Ix> = Link<VertexBSet<Ix>, K, Ix>;
}

#[cfg(all(feature = "alloc", feature = "nightly"))]
mod use_alloc {
    use crate::{Edge, Link, VertexId};

    use alloc::collections::{BTreeSet, VecDeque};
    use alloc::vec::Vec;

    /// a type alias for an [`Link`] whose _vertices_ are stored in a [`Vec`]
    pub type LinkVec<K, Ix, A> = Link<VertexVec<Ix, A>, K, Ix>;
    /// a type alias for an [`Link`] whose _vertices_ are stored in a [`VertexVecDeque`]
    pub type LinkDeque<K, Ix, A> = Link<VertexVecDeque<Ix, A>, K, Ix>;
    /// a type alias for an [`Link`] whose _vertices_ are stored in a [`VertexBSet`]
    pub type LinkBSet<K, Ix, A> = Link<VertexBSet<Ix, A>, K, Ix>;
}
