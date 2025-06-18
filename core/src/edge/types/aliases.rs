/*
    appellation: aliases <module>
    authors: @FL03
*/
//! this module provides various type aliases hyperedges such as: [`DiEdge`], [`UnEdge`] and
//! [`HashEdge`]

#[cfg(feature = "alloc")]
pub use self::use_alloc::*;

use crate::edge::Edge;
use crate::idx::{VertexArray, VertexSet, VertexSlice, VertexSliceMut, VertexSliceRef};
use crate::{Directed, Undirected};

/// a type alias for a [`Directed`] hyperedge
pub type DiEdge<T, S, Idx = usize> = Edge<T, S, Directed, Idx>;
/// a type alias for an [`Undirected`] hyperedge
pub type UnEdge<T, S, Idx = usize> = Edge<T, S, Undirected, Idx>;

/// a type alias for an [`Edge`] whose _vertices_ are stored in a [`VertexSet`]
pub type HashEdge<T, K, Ix, S> = Edge<T, VertexSet<Ix, S>, K, Ix>;

/// a type alias for a [`Edge`] whose _vertices_ are stored in an array of fixed
/// size with the size defined by the generic parameter `N`.
pub type EdgeArray<T, const N: usize, K, Ix = usize> = Edge<T, VertexArray<Ix, N>, K, Ix>;
/// a type alias for a [`Edge`] whose _vertices_ are stored in a slice
pub type EdgeSlice<T, K, Ix = usize> = Edge<T, VertexSlice<Ix>, K, Ix>;
/// a type alias for a [`Edge`] whose _vertices_ are stored in an owned slice
pub type EdgeSliceRef<'a, T, K, Ix = usize> = Edge<T, VertexSliceRef<'a, Ix>, K, Ix>;
/// a type alias for a [`Edge`] whose _vertices_ are stored in a mutable slice
pub type EdgeSliceMut<'a, T, K, Ix = usize> = Edge<T, VertexSliceMut<'a, Ix>, K, Ix>;

#[cfg(all(feature = "alloc", not(feature = "nightly")))]
mod use_alloc {
    use crate::edge::Edge;
    use crate::idx::{VertexBSet, VertexDeque, VertexVec};

    /// a type alias for an [`Edge`] whose _vertices_ are stored in a [`Vec`]
    pub type EdgeVec<T, K, Ix> = Edge<T, VertexVec<Ix>, K, Ix>;
    /// a type alias for an [`Edge`] whose _vertices_ are stored in a [`VecDeque`]
    pub type EdgeDeque<T, K, Ix> = Edge<T, VertexDeque<Ix>, K, Ix>;
    /// a type alias for an [`Edge`] whose _vertices_ are stored in a [`BTreeSet`]
    pub type EdgeBSet<T, K, Ix> = Edge<T, VertexBSet<Ix>, K, Ix>;
}

#[cfg(all(feature = "alloc", feature = "nightly"))]
mod use_alloc {
    use crate::edge::Edge;
    use crate::{VertexBSet, VertexDeque, VertexVec};

    /// a type alias for an [`Edge`] whose _vertices_ are stored in a [`Vec`]
    pub type EdgeVec<T, K, Ix, A> = Edge<T, VertexVec<Ix, A>, K, Ix>;
    /// a type alias for an [`Edge`] whose _vertices_ are stored in a [`VertexDeque`]
    pub type EdgeDeque<T, K, Ix, A> = Edge<T, VertexDeque<Ix, A>, K, Ix>;
    /// a type alias for an [`Edge`] whose _vertices_ are stored in a [`VertexBSet`]
    pub type EdgeBSet<T, K, Ix, A> = Edge<T, VertexBSet<Ix, A>, K, Ix>;
}
