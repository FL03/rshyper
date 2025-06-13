/*
    appellation: aliases <module>
    authors: @FL03
*/
//! this module defines various type aliases, for convenience and clarity, for the
//! [`HyperEdge`] and [`HyperFacet`] types
//!
use crate::edge::{HyperEdge, HyperFacet};
use crate::index::VertexId;
use crate::{Directed, Undirected};

#[cfg(feature = "alloc")]
pub use self::use_alloc::*;
#[cfg(feature = "std")]
pub use self::use_std::*;

/// a type alias for a [`HyperEdge`] whose kind is [`Directed`]
pub type DirectedEdge<S, Idx = usize> = HyperEdge<S, Directed, Idx>;
/// a type alias for an [`Undirected`] [`HyperEdge`]
pub type UndirectedEdge<S, Idx = usize> = HyperEdge<S, Undirected, Idx>;
/// a type alias for a [`HyperFacet`] with a [`Directed`] edge
pub type DirectedFacet<T, S, Idx = usize> = HyperFacet<T, S, Directed, Idx>;
/// a type alias for a [`HyperFacet`] with a [`Undirected`] edge
pub type UndirectedFacet<T, S, Idx = usize> = HyperFacet<T, S, Undirected, Idx>;

/// a type alias for a [`UndirectedEdge`] whose _vertices_ are stored in an array of fixed
/// size with the size defined by the generic parameter `N`.
pub type EdgeArray<const N: usize, K = Undirected, Idx = usize> =
    HyperEdge<[VertexId<Idx>; N], K, Idx>;
/// a type alias for a [`UndirectedFacet`] whose _vertices_ are stored in an array of fixed
/// size with the size defined by the generic parameter `N`.
pub type FacetArray<T, const N: usize, K, Idx = usize> = HyperFacet<T, [VertexId<Idx>; N], K, Idx>;
/// a type alias for a [`UndirectedEdge`] whose _vertices_ are stored in a slice
pub type UnEdgeSlice<'a, Idx = usize> = UndirectedEdge<&'a [VertexId<Idx>], Idx>;
/// a type alias for a [`UndirectedFacet`] whose _vertices_ are stored in a slice
pub type UnFacetSlice<'a, T, Idx = usize> = UndirectedFacet<T, &'a [VertexId<Idx>], Idx>;

#[cfg(feature = "alloc")]
mod use_alloc {
    use crate::edge::{HyperEdge, HyperFacet};
    use crate::index::VertexId;
    use alloc::collections::BTreeSet;
    use alloc::vec::Vec;

    pub type VertexVec<Idx = usize> = Vec<VertexId<Idx>>;

    pub type VertexBSet<Idx = usize> = BTreeSet<VertexId<Idx>>;
    /// a type alias for an [`HyperEdge`] whose _vertices_ are stored in a [`Vec`]
    pub type EdgeVec<K, Idx = usize> = HyperEdge<VertexVec<Idx>, K, Idx>;
    /// a type alias for an [`HyperFacet`] whose _vertices_ are stored in a [`Vec`]
    pub type FacetVec<T, K, Idx = usize> = HyperFacet<T, VertexVec<Idx>, K, Idx>;
    /// a type alias for an [`HyperEdge`] whose _vertices_ are stored in a [`BTreeSet`]
    pub type EdgeBTreeSet<K, Idx = usize> = HyperEdge<VertexBSet<Idx>, K, Idx>;
    /// a type alias for an [`HyperFacet`] whose _vertices_ are stored in a [`BTreeSet`]
    pub type FacetBTreeSet<T, K, Idx = usize> = HyperFacet<T, VertexBSet<Idx>, K, Idx>;
}

#[cfg(feature = "std")]
mod use_std {
    use crate::edge::{HyperEdge, HyperFacet};
    use crate::index::VertexId;

    use std::collections::HashSet;
    use std::hash::RandomState;

    pub type VertexHSet<I = usize, S = RandomState> = HashSet<VertexId<I>, S>;
    /// a type alias for an [`HyperEdge`] whose _vertices_ are stored in a [`HashSet`]
    pub type HashEdge<K, I = usize, S = RandomState> = HyperEdge<VertexHSet<I, S>, K, I>;
    /// a type alias for an [`HyperFacet`] whose _vertices_ are stored in a [`HashSet`]
    pub type HashFacet<T, K, I = usize, S = RandomState> = HyperFacet<T, VertexHSet<I, S>, K, I>;
}
