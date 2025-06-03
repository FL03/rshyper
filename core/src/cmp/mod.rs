/*
    appellation: cmp <module>
    authors: @FL03
*/
//! this module contains the various components that makeup a hypergraph.
#[doc(inline)]
pub use self::prelude::*;

pub mod hyper_edge;
pub mod hyper_node;

mod impls {
    pub mod impl_hyper_edge;
    pub mod impl_hyper_facet;
    pub mod impl_hyper_node;
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::aliases::*;
    #[doc(inline)]
    pub use super::hyper_edge::*;
    #[doc(inline)]
    pub use super::hyper_node::*;
}

pub(crate) mod aliases {
    #[cfg(feature = "alloc")]
    pub use self::use_alloc::*;
    #[cfg(feature = "std")]
    pub use self::use_std::*;

    use super::{HyperEdge, HyperFacet};
    use crate::index::VertexId;
    use crate::{Directed, Undirected};

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
    pub type FacetArray<T, const N: usize, K, Idx = usize> =
        HyperFacet<T, [VertexId<Idx>; N], K, Idx>;
    /// a type alias for a [`UndirectedEdge`] whose _vertices_ are stored in a slice
    pub type UnEdgeSlice<'a, Idx = usize> = UndirectedEdge<&'a [VertexId<Idx>], Idx>;
    /// a type alias for a [`UndirectedFacet`] whose _vertices_ are stored in a slice
    pub type UnFacetSlice<'a, T, Idx = usize> = UndirectedFacet<T, &'a [VertexId<Idx>], Idx>;

    #[cfg(feature = "alloc")]
    mod use_alloc {
        use crate::VertexId;
        use crate::cmp::{HyperEdge, HyperFacet};
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
        use crate::cmp::{HyperEdge, HyperFacet};
        use std::collections::HashSet;

        pub type VertexHSet<Idx = usize> = HashSet<crate::VertexId<Idx>>;
        /// a type alias for an [`HyperEdge`] whose _vertices_ are stored in a [`HashSet`]
        pub type EdgeHashSet<K, Idx = usize> = HyperEdge<VertexHSet<Idx>, K, Idx>;
        /// a type alias for an [`HyperFacet`] whose _vertices_ are stored in a [`HashSet`]
        pub type FacetHashSet<T, K, Idx = usize> = HyperFacet<T, VertexHSet<Idx>, K, Idx>;
    }
}

use crate::index::{EdgeId, RawIndex, VertexId};
use crate::{GraphKind, Weight};

/// [`RawStore`] is a trait that defines the behavior of a store that holds the vertices
/// associated with a hyperedge or hyperfacet. It is used to abstract over different
/// implementations of edge storage, such as arrays, vectors, or sets.
///
/// **note:** The trait is sealed to prevent external implementations, ensuring that only the
/// crate can define how edges are stored. This is to maintain consistency and prevent
/// misuse of the trait in different contexts.
pub trait RawStore<Idx = usize>
where
    Idx: RawIndex,
{
    type Store<_T>: ?Sized;

    private!();
    /// returns the number of vertices associated with the edge.
    fn len(&self) -> usize;
    /// returns true if there are no points.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
/// [`RawNode`] is a trait that defines the behavior of a node in a hypergraph.
pub trait RawNode<T> {
    type Idx: RawIndex;

    private!();

    /// returns an immutable reference to the node index
    fn index(&self) -> &VertexId<Self::Idx>;
    /// returns an immutable reference to the node data
    fn weight(&self) -> &Weight<T>;
    /// returns a mutable reference to the node data
    fn weight_mut(&mut self) -> &mut Weight<T>;
    /// [`replace`](core::mem::replace) the weight of the node with a new one, returning the
    /// previous value
    fn replace_weight(&mut self, weight: Weight<T>) -> Weight<T> {
        core::mem::replace(self.weight_mut(), weight)
    }
    /// overwrites the weight of the node with a new one and returns a mutable reference to
    /// the edge.
    fn set_weight(&mut self, weight: T) -> &mut Self {
        self.weight_mut().set(weight);
        self
    }
    /// [`swap`](core::mem::swap) the weight of the node with another weight
    fn swap_weight(&mut self, weight: &mut Weight<T>) {
        core::mem::swap(self.weight_mut(), weight)
    }
    /// [`take`](core::mem::take) the weight of the node, replacing it with a default value
    fn take_weight(&mut self) -> Weight<T>
    where
        T: Default,
    {
        core::mem::take(self.weight_mut())
    }
}
/// [`RawEdge`] is a trait that defines the behavior of an edge in a hypergraph.
pub trait RawEdge {
    type Idx: RawIndex;
    type Kind: GraphKind;
    type Store: RawStore<Self::Idx>;

    private!();

    /// returns an immutable reference to the edge index
    fn index(&self) -> &EdgeId<Self::Idx>;
    /// Returns an immutable reference to the edge data.
    fn vertices(&self) -> &Self::Store;
}
/// [`RawFacet`] extends the behaviour of a [`RawEdge`] to include a weight
pub trait RawFacet<T>: RawEdge {
    private!();
    /// Returns the index of the edge.
    fn weight(&self) -> &Weight<T>;
    /// returns a mutable reference to the edge data.
    fn weight_mut(&mut self) -> &mut Weight<T>;
    /// [`replace`](core::mem::replace) the weight of the edge with a new one, returning the
    /// previous value
    fn replace_weight(&mut self, weight: Weight<T>) -> Weight<T> {
        core::mem::replace(self.weight_mut(), weight)
    }
    /// overwrites the weight of the edge with a new one and returns a mutable reference to
    /// the edge.
    fn set_weight(&mut self, weight: T) -> &mut Self {
        self.weight_mut().set(weight);
        self
    }
    /// [`swap`](core::mem::swap) the weight of the edge with another weight
    fn swap_weight(&mut self, weight: &mut Weight<T>) {
        core::mem::swap(self.weight_mut(), weight)
    }
    /// [`take`](core::mem::take) the weight of the edge, replacing it with a default value
    fn take_weight(&mut self) -> Weight<T>
    where
        T: Default,
    {
        core::mem::take(self.weight_mut())
    }
}
/*
 ************* Implementations *************
*/
impl<T, Idx> RawNode<T> for HyperNode<T, Idx>
where
    Idx: RawIndex,
{
    type Idx = Idx;

    seal!();

    fn index(&self) -> &VertexId<Idx> {
        &self.index
    }
    fn weight(&self) -> &Weight<T> {
        self.weight()
    }
    fn weight_mut(&mut self) -> &mut Weight<T> {
        self.weight_mut()
    }
}

macro_rules! impl_raw_store {
    (
        $(
            $p:ident
        );* $(;)?
    ) => {
        $(
            impl_raw_store!(@impl $p);
        )*
    };
    (@impl $p:ident) => {
        impl<Idx> RawStore<Idx> for $p<VertexId<Idx>>
        where
            Idx: RawIndex,
        {
            type Store<_T> = $p<_T>;

            seal!();

            fn len(&self) -> usize {
                <$p<VertexId<Idx>>>::len(self)
            }

            fn is_empty(&self) -> bool {
                <$p<VertexId<Idx>>>::is_empty(self)
            }
        }
    };
    (@impl $($p:ident)::+) => {
        impl<Idx> RawStore<Idx> for $($p)::*<VertexId<Idx>>
        where
            Idx: RawIndex,
        {
            type Store<_T> = $($p)::*<_T>;

            seal!();

            fn len(&self) -> usize {
                <$($p)::*<VertexId<Idx>>>::len(self)
            }

            fn is_empty(&self) -> bool {
                <$($p)::*<VertexId<Idx>>>::is_empty(self)
            }
        }
    };
}
#[cfg(feature = "alloc")]
use alloc::{collections::BTreeSet, vec::Vec};
#[cfg(feature = "std")]
use std::collections::HashSet;

#[cfg(feature = "std")]
impl_raw_store! {
    HashSet;
}

#[cfg(feature = "alloc")]
impl_raw_store! {
    BTreeSet;
    Vec;
}

impl<'a, Idx> RawStore<Idx> for &'a [VertexId<Idx>]
where
    Idx: RawIndex,
{
    type Store<_T> = [_T];

    seal!();

    fn len(&self) -> usize {
        <[VertexId<Idx>]>::len(self)
    }

    fn is_empty(&self) -> bool {
        <[VertexId<Idx>]>::is_empty(self)
    }
}

impl<'a, Idx> RawStore<Idx> for &'a mut [VertexId<Idx>]
where
    Idx: RawIndex,
{
    type Store<_T> = [_T];

    seal!();

    fn len(&self) -> usize {
        <[VertexId<Idx>]>::len(self)
    }
}

impl<Idx> RawStore<Idx> for [VertexId<Idx>]
where
    Idx: RawIndex,
{
    type Store<_T> = [_T];

    seal!();

    fn len(&self) -> usize {
        self.len()
    }
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<const N: usize, Idx> RawStore<Idx> for [VertexId<Idx>; N]
where
    Idx: RawIndex,
{
    type Store<_T> = [_T; N];

    seal!();

    fn len(&self) -> usize {
        N
    }
    fn is_empty(&self) -> bool {
        N == 0
    }
}
