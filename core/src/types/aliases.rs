/*
    appellation: aliases <module>
    authors: @FL03
*/
//! this module provides various type aliases for the core components of a hypergraph, such as:
//!
//! - [`DiEdge`], [`UnEdge`], [`DiSurface`], [`UnSurface`], and others.
use crate::edge::{Edge, EdgeLayout};
use crate::idx::VertexId;
use crate::{Directed, Undirected};

#[cfg(feature = "alloc")]
pub use self::use_alloc::*;
#[cfg(feature = "std")]
pub use self::use_std::*;

#[cfg(all(feature = "std", not(feature = "hashbrown")))]
/// a type alias for a [`HashSet`](std::collections::HashSet) of [`VertexId`] that is generic over
/// the index type `I`
pub type VertexHashSet<I, S = std::hash::RandomState> = std::collections::HashSet<VertexId<I>, S>;
#[cfg(feature = "hashbrown")]
/// a type alias for a [`HashSet`](hashbrown::HashSet) of [`VertexId`] that is generic over
/// the index type `I`
pub type VertexHashSet<I, S = hashbrown::DefaultHashBuilder> = hashbrown::HashSet<VertexId<I>, S>;

/// a type alias for a [`Directed`] hyperedge
pub type DiEdge<S, Idx = usize> = EdgeLayout<S, Directed, Idx>;
/// a type alias for an [`Undirected`] hyperedge
pub type UnEdge<S, Idx = usize> = EdgeLayout<S, Undirected, Idx>;
/// a type alias for a [`Directed`] hypersurface
pub type DiSurface<T, S, Idx = usize> = Edge<T, S, Directed, Idx>;
/// a type alias for an [`Undirected`] hypersurface
pub type UnSurface<T, S, Idx = usize> = Edge<T, S, Undirected, Idx>;

/// a type alias for a [`Edge`] whose _vertices_ are stored in an array of fixed
/// size with the size defined by the generic parameter `N`.
pub type EdgeArray<const N: usize, K = Undirected, Idx = usize> =
    EdgeLayout<[VertexId<Idx>; N], K, Idx>;
/// a type alias for a [`Surface`] whose _vertices_ are stored in an array of fixed
/// size with the size defined by the generic parameter `N`.
pub type SurfaceArray<T, const N: usize, K, Idx = usize> = Edge<T, [VertexId<Idx>; N], K, Idx>;
/// a type alias for a [`Edge`] whose _vertices_ are stored in a slice
pub type EdgeSlice<K, Idx = usize> = EdgeLayout<[VertexId<Idx>], K, Idx>;
/// a type alias for a [`Surface`] whose _vertices_ are stored in a slice
pub type SurfaceSlice<T, K, Idx = usize> = Edge<T, [VertexId<Idx>], K, Idx>;
/// a type alias for a [`Edge`] whose _vertices_ are stored in an owned slice
pub type EdgeSliceRef<'a, K, Idx = usize> = EdgeLayout<&'a [VertexId<Idx>], K, Idx>;
/// a type alias for a [`Surface`] whose _vertices_ are stored in an owned slice
pub type SurfaceSliceRef<'a, T, K, Idx = usize> = Edge<T, &'a [VertexId<Idx>], K, Idx>;
/// a type alias for a [`Edge`] whose _vertices_ are stored in a mutable slice
pub type EdgeSliceMut<'a, K, Idx = usize> = EdgeLayout<&'a mut [VertexId<Idx>], K, Idx>;
/// a type alias for a [`Surface`] whose _vertices_ are stored in a mutable slice
pub type SurfaceSliceMut<'a, T, K, Idx = usize> = Edge<T, &'a mut [VertexId<Idx>], K, Idx>;

#[cfg(feature = "alloc")]
mod use_alloc {
    use crate::edge::{Edge, EdgeLayout};
    use crate::idx::VertexId;
    use alloc::collections::{BTreeSet, VecDeque};
    use alloc::vec::Vec;

    #[cfg(feature = "nightly")]
    /// a type alias for a [`Vec`] of [`VertexId`] that is generic over the index type `I`
    pub type VertexVec<I, A> = alloc::vec::Vec<VertexId<I>, A>;
    #[cfg(not(feature = "nightly"))]
    /// a type alias for a [`Vec`] of [`VertexId`] that is generic over the index type `I`
    pub type VertexVec<I> = alloc::vec::Vec<VertexId<I>>;
    /// a type alias for a [`VertexId`] stored in a [`VecDeque`]
    pub type VertexVecDeque<Idx = usize> = VecDeque<VertexId<Idx>>;
    /// a type alias for a [`VertexId`] stored in a [`BTreeSet`]
    pub type VertexBSet<Idx> = BTreeSet<VertexId<Idx>>;
    /// a type alias for an [`Edge`] whose _vertices_ are stored in a [`Vec`]
    pub type EdgeVec<K, Idx = usize> = EdgeLayout<VertexVec<Idx>, K, Idx>;
    /// a type alias for an [`Surface`] whose _vertices_ are stored in a [`Vec`]
    pub type SurfaceVec<T, K, Idx = usize> = Edge<T, VertexVec<Idx>, K, Idx>;
    /// a type alias for an [`Edge`] whose _vertices_ are stored in a [`VecDeque`]
    pub type EdgeVecDeque<K, Idx = usize> = EdgeLayout<VertexVecDeque<Idx>, K, Idx>;
    /// a type alias for an [`Surface`] whose _vertices_ are stored in a [`VecDeque`]
    pub type SurfaceVecDeque<T, K, Idx = usize> = Edge<T, VertexVecDeque<Idx>, K, Idx>;
    /// a type alias for an [`Edge`] whose _vertices_ are stored in a [`BTreeSet`]
    pub type EdgeBTreeSet<K, Idx = usize> = EdgeLayout<VertexBSet<Idx>, K, Idx>;
    /// a type alias for an [`Surface`] whose _vertices_ are stored in a [`BTreeSet`]
    pub type SurfaceBTreeSet<T, K, Idx = usize> = Edge<T, VertexBSet<Idx>, K, Idx>;
}

#[cfg(feature = "std")]
mod use_std {
    use crate::edge::{Edge, EdgeLayout};
    use crate::idx::VertexId;

    use std::collections::HashSet;
    use std::hash::RandomState;
    /// a type alias for a [`VertexId`] stored in a [`HashSet`] that is generic over the
    /// type of the index, `I`, and the hasher used, `S`.
    pub type VertexHashSet<I = usize, S = RandomState> = HashSet<VertexId<I>, S>;
    /// a type alias for an [`Edge`] whose _vertices_ are stored in a [`HashSet`]
    pub type HashEdge<K, I = usize, S = RandomState> = EdgeLayout<VertexHashSet<I, S>, K, I>;
    /// a type alias for an [`Surface`] whose _vertices_ are stored in a [`HashSet`]
    pub type HashSurface<T, K, I = usize, S = RandomState> = Edge<T, VertexHashSet<I, S>, K, I>;
}
