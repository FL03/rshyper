/*
    appellation: aliases <module>
    authors: @FL03
*/
//! this module provides various type aliases hyperedges such as: [`DiEdge`], [`UnEdge`] and
//! [`HashEdge`]

#[cfg(feature = "alloc")]
pub use self::use_alloc::*;

use crate::edge::Edge;
use crate::idx::{VertexId, VertexSet};
use crate::{Directed, Undirected};

/// a type alias for a [`Directed`] hyperedge
pub type DiEdge<T, S, Idx = usize> = Edge<T, S, Directed, Idx>;
/// a type alias for an [`Undirected`] hyperedge
pub type UnEdge<T, S, Idx = usize> = Edge<T, S, Undirected, Idx>;

/// a type alias for an [`Edge`] whose _vertices_ are stored in a [`VertexSet`]
pub type HashEdge<T, K, I, S> = Edge<T, VertexSet<I, S>, K, I>;
/// a type alias for a [`Edge`] whose _vertices_ are stored in an array of fixed
/// size with the size defined by the generic parameter `N`.
pub type EdgeArray<T, const N: usize, K, Idx = usize> = Edge<T, [VertexId<Idx>; N], K, Idx>;
/// a type alias for a [`Edge`] whose _vertices_ are stored in a slice
pub type EdgeSlice<T, K, Idx = usize> = Edge<T, [VertexId<Idx>], K, Idx>;
/// a type alias for a [`Edge`] whose _vertices_ are stored in an owned slice
pub type EdgeSliceRef<'a, T, K, Idx = usize> = Edge<T, &'a [VertexId<Idx>], K, Idx>;
/// a type alias for a [`Edge`] whose _vertices_ are stored in a mutable slice
pub type EdgeSliceMut<'a, T, K, Idx = usize> = Edge<T, &'a mut [VertexId<Idx>], K, Idx>;

#[cfg(all(feature = "alloc", not(feature = "nightly")))]
mod use_alloc {
    use crate::edge::Edge;
    use crate::{VertexBSet, VertexDeque, VertexVec};

    /// a type alias for an [`Edge`] whose _vertices_ are stored in a [`Vec`]
    pub type EdgeVec<T, K, Idx = usize> = Edge<T, VertexVec<Idx>, K, Idx>;
    /// a type alias for an [`Edge`] whose _vertices_ are stored in a [`VecDeque`]
    pub type EdgeDeque<T, K, Idx = usize> = Edge<T, VertexDeque<Idx>, K, Idx>;
    /// a type alias for an [`Edge`] whose _vertices_ are stored in a [`BTreeSet`]
    pub type EdgeBSet<T, K, Idx = usize> = Edge<T, VertexBSet<Idx>, K, Idx>;
}

#[cfg(all(feature = "alloc", feature = "nightly"))]
mod use_alloc {
    use crate::edge::Edge;
    use crate::{VertexBSet, VertexDeque, VertexVec};

    /// a type alias for an [`Edge`] whose _vertices_ are stored in a [`Vec`]
    pub type EdgeVec<T, K, Idx, A> = Edge<T, VertexVec<Idx, A>, K, Idx>;
    /// a type alias for an [`Edge`] whose _vertices_ are stored in a [`VertexVecDeque`]
    pub type EdgeDeque<T, K, Idx = usize> = Edge<T, VertexDeque<Idx, A>, K, Idx>;
    /// a type alias for an [`Edge`] whose _vertices_ are stored in a [`VertexBSet`]
    pub type EdgeBSet<T, K, Idx = usize> = Edge<T, VertexBSet<Idx, A>, K, Idx>;
}
