/*
    appellation: aliases <module>
    authors: @FL03
*/
//! this module defines various type aliases, for convenience and clarity, for the
//! [`Edge`] and [`Surface`] types
//!
use crate::edge::{Edge, Surface};
use crate::index::VertexId;
use crate::{Directed, Undirected};

#[cfg(feature = "alloc")]
pub use self::use_alloc::*;
#[cfg(feature = "std")]
pub use self::use_std::*;

/// a type alias for a [`Directed`] hyperedge
pub type DiEdge<S, Idx = usize> = Edge<S, Directed, Idx>;
/// a type alias for an [`Undirected`] hyperedge
pub type UnEdge<S, Idx = usize> = Edge<S, Undirected, Idx>;
/// a type alias for a [`Directed`] hypersurface
pub type DiSurface<T, S, Idx = usize> = Surface<T, S, Directed, Idx>;
/// a type alias for an [`Undirected`] hypersurface
pub type UnSurface<T, S, Idx = usize> = Surface<T, S, Undirected, Idx>;

/// a type alias for a [`Edge`] whose _vertices_ are stored in an array of fixed
/// size with the size defined by the generic parameter `N`.
pub type EdgeArray<const N: usize, K = Undirected, Idx = usize> = Edge<[VertexId<Idx>; N], K, Idx>;
/// a type alias for a [`Surface`] whose _vertices_ are stored in an array of fixed
/// size with the size defined by the generic parameter `N`.
pub type SurfaceArray<T, const N: usize, K, Idx = usize> = Surface<T, [VertexId<Idx>; N], K, Idx>;
/// a type alias for a [`Edge`] whose _vertices_ are stored in a slice
pub type EdgeSlice<K, Idx = usize> = Edge<[VertexId<Idx>], K, Idx>;
/// a type alias for a [`Surface`] whose _vertices_ are stored in a slice
pub type SurfaceSlice<T, K, Idx = usize> = Surface<T, [VertexId<Idx>], K, Idx>;
/// a type alias for a [`Edge`] whose _vertices_ are stored in an owned slice
pub type EdgeSliceRef<'a, K, Idx = usize> = Edge<&'a [VertexId<Idx>], K, Idx>;
/// a type alias for a [`Surface`] whose _vertices_ are stored in an owned slice
pub type SurfaceSliceRef<'a, T, K, Idx = usize> = Surface<T, &'a [VertexId<Idx>], K, Idx>;
/// a type alias for a [`Edge`] whose _vertices_ are stored in a mutable slice
pub type EdgeSliceMut<'a, K, Idx = usize> = Edge<&'a mut [VertexId<Idx>], K, Idx>;
/// a type alias for a [`Surface`] whose _vertices_ are stored in a mutable slice
pub type SurfaceSliceMut<'a, T, K, Idx = usize> = Surface<T, &'a mut [VertexId<Idx>], K, Idx>;

#[cfg(feature = "alloc")]
mod use_alloc {
    use crate::edge::{Edge, Surface};
    use crate::index::VertexId;
    use alloc::collections::{BTreeSet, VecDeque};
    use alloc::vec::Vec;

    /// a type alias for a [`VertexId`] stored in a [`Vec`]
    pub type VertexVec<Idx = usize> = Vec<VertexId<Idx>>;
    /// a type alias for a [`VertexId`] stored in a [`VecDeque`]
    pub type VertexVecDeque<Idx = usize> = VecDeque<VertexId<Idx>>;
    /// a type alias for a [`VertexId`] stored in a [`BTreeSet`]
    pub type VertexBSet<Idx> = BTreeSet<VertexId<Idx>>;
    /// a type alias for an [`Edge`] whose _vertices_ are stored in a [`Vec`]
    pub type EdgeVec<K, Idx = usize> = Edge<VertexVec<Idx>, K, Idx>;
    /// a type alias for an [`Surface`] whose _vertices_ are stored in a [`Vec`]
    pub type SurfaceVec<T, K, Idx = usize> = Surface<T, VertexVec<Idx>, K, Idx>;
    /// a type alias for an [`Edge`] whose _vertices_ are stored in a [`VecDeque`]
    pub type EdgeVecDeque<K, Idx = usize> = Edge<VertexVecDeque<Idx>, K, Idx>;
    /// a type alias for an [`Surface`] whose _vertices_ are stored in a [`VecDeque`]
    pub type SurfaceVecDeque<T, K, Idx = usize> = Surface<T, VertexVecDeque<Idx>, K, Idx>;
    /// a type alias for an [`Edge`] whose _vertices_ are stored in a [`BTreeSet`]
    pub type EdgeBTreeSet<K, Idx = usize> = Edge<VertexBSet<Idx>, K, Idx>;
    /// a type alias for an [`Surface`] whose _vertices_ are stored in a [`BTreeSet`]
    pub type SurfaceBTreeSet<T, K, Idx = usize> = Surface<T, VertexBSet<Idx>, K, Idx>;
}

#[cfg(feature = "std")]
mod use_std {
    use crate::edge::{Edge, Surface};
    use crate::index::VertexId;

    use std::collections::HashSet;
    use std::hash::RandomState;
    /// a type alias for a [`VertexId`] stored in a [`HashSet`] that is generic over the
    /// type of the index, `I`, and the hasher used, `S`.
    pub type VertexHashSet<I = usize, S = RandomState> = HashSet<VertexId<I>, S>;
    /// a type alias for an [`Edge`] whose _vertices_ are stored in a [`HashSet`]
    pub type HashEdge<K, I = usize, S = RandomState> = Edge<VertexHashSet<I, S>, K, I>;
    /// a type alias for an [`Surface`] whose _vertices_ are stored in a [`HashSet`]
    pub type HashSurface<T, K, I = usize, S = RandomState> = Surface<T, VertexHashSet<I, S>, K, I>;
}
