/*
    appellation: cmp <module>
    authors: @FL03
*/
//! this module contains the various components that makeup a hypergraph.
#[doc(inline)]
pub use self::prelude::*;

pub mod hyper_edge;
pub mod hyper_facet;
pub mod hyper_node;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::aliases::*;
    #[doc(inline)]
    pub use super::hyper_edge::*;
    #[doc(inline)]
    pub use super::hyper_facet::*;
    #[doc(inline)]
    pub use super::hyper_node::*;
}

pub(crate) mod aliases {
    use super::{HyperEdge, HyperFacet};
    use crate::index::VertexId;
    #[cfg(feature = "alloc")]
    use alloc::{collections::BTreeSet, vec::Vec};
    #[cfg(feature = "std")]
    use std::collections::HashSet;

    /// a type alias for a [`HyperEdge`] whose _vertices_ are stored in an array of fixed size
    /// with the size defined by the generic parameter `N`.
    pub type FixedEdge<const N: usize, Idx = usize> = HyperEdge<[VertexId<Idx>; N], Idx>;
    /// a type alias for a [`HyperFacet`] whose _vertices_ are stored in an array of fixed size
    /// with the size defined by the generic parameter `N`.
    pub type FixedFacet<T, const N: usize, Idx = usize> = HyperFacet<T, [VertexId<Idx>; N], Idx>;
    /// a type alias for a [`HyperEdge`] whose _vertices_ are stored in a slice
    pub type SliceEdge<'a, Idx = usize> = HyperEdge<&'a [VertexId<Idx>], Idx>;
    /// a type alias for a [`HyperFacet`] whose _vertices_ are stored in a slice
    pub type SliceFacet<'a, T, Idx = usize> = HyperFacet<T, &'a [VertexId<Idx>], Idx>;

    /// a type alias for an [`HyperEdge`] whose _vertices_ are stored in a [`Vec`]
    #[cfg(feature = "alloc")]
    pub type VecEdge<Idx = usize> = HyperEdge<Vec<VertexId<Idx>>, Idx>;
    /// a type alias for an [`HyperFacet`] whose _vertices_ are stored in a [`Vec`]
    #[cfg(feature = "alloc")]
    pub type VecFacet<T, Idx = usize> = HyperFacet<T, Vec<VertexId<Idx>>, Idx>;
    /// a type alias for an [`HyperEdge`] whose _vertices_ are stored in a [`BTreeSet`]
    #[cfg(feature = "alloc")]
    pub type BinaryEdge<Idx = usize> = HyperEdge<BTreeSet<VertexId<Idx>>, Idx>;
    /// a type alias for an [`HyperFacet`] whose _vertices_ are stored in a [`BTreeSet`]
    #[cfg(feature = "alloc")]
    pub type BinaryFacet<T, Idx = usize> = HyperFacet<T, BTreeSet<VertexId<Idx>>, Idx>;
    /// a type alias for an [`HyperEdge`] whose _vertices_ are stored in a [`HashSet`]
    #[cfg(feature = "std")]
    pub type HashEdge<Idx = usize> = HyperEdge<HashSet<VertexId<Idx>>, Idx>;
    /// a type alias for an [`HyperFacet`] whose _vertices_ are stored in a [`HashSet`]
    #[cfg(feature = "std")]
    pub type HashFacet<T, Idx = usize> = HyperFacet<T, HashSet<VertexId<Idx>>, Idx>;
}

use crate::index::{RawIndex, VertexId};

/// A trait denoting a node within the hypergraph.
pub trait Point<Idx: RawIndex> {
    fn index(&self) -> &VertexId<Idx>;
}

/// Extends the base [HyperNode] trait with the [`Eq`] and [`Hash`](core::hash::Hash) traits
/// for use with hash-related structures.
pub trait HashPoint<Idx: RawIndex>: Point<Idx> + Eq + core::hash::Hash {
    private!();
}

/*
 ************* Implementations *************
*/

impl<T, Idx> HashPoint<Idx> for T
where
    Idx: RawIndex,
    T: Point<Idx> + Eq + core::hash::Hash,
{
    seal!();
}

impl<T, Idx> Point<Idx> for T
where
    Idx: RawIndex,
    T: core::borrow::Borrow<VertexId<Idx>>,
{
    fn index(&self) -> &VertexId<Idx> {
        self.borrow()
    }
}
