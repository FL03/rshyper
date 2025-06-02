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

mod impls {}

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
    use crate::{Directed, Undirected};
    #[cfg(feature = "alloc")]
    use alloc::{collections::BTreeSet, vec::Vec};
    #[cfg(feature = "std")]
    use std::collections::HashSet;
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
    pub type UnEdgeFixed<const N: usize, Idx = usize> = UndirectedEdge<[VertexId<Idx>; N], Idx>;
    /// a type alias for a [`UndirectedFacet`] whose _vertices_ are stored in an array of fixed
    /// size with the size defined by the generic parameter `N`.
    pub type UnFacetFixed<T, const N: usize, Idx = usize> =
        UndirectedFacet<T, [VertexId<Idx>; N], Idx>;
    /// a type alias for a [`UndirectedEdge`] whose _vertices_ are stored in a slice
    pub type UnEdgeSlice<'a, Idx = usize> = UndirectedEdge<&'a [VertexId<Idx>], Idx>;
    /// a type alias for a [`UndirectedFacet`] whose _vertices_ are stored in a slice
    pub type UnFacetSlice<'a, T, Idx = usize> = UndirectedFacet<T, &'a [VertexId<Idx>], Idx>;

    /// a type alias for an [`UndirectedEdge`] whose _vertices_ are stored in a [`Vec`]
    #[cfg(feature = "alloc")]
    pub type UnEdgeVec<Idx = usize> = UndirectedEdge<Vec<VertexId<Idx>>, Idx>;
    /// a type alias for an [`UndirectedFacet`] whose _vertices_ are stored in a [`Vec`]
    #[cfg(feature = "alloc")]
    pub type VecFacet<T, Idx = usize> = UndirectedFacet<T, Vec<VertexId<Idx>>, Idx>;
    /// a type alias for an [`UndirectedEdge`] whose _vertices_ are stored in a [`BTreeSet`]
    #[cfg(feature = "alloc")]
    pub type UnEdgeBTree<Idx = usize> = UndirectedEdge<BTreeSet<VertexId<Idx>>, Idx>;
    /// a type alias for an [`UndirectedFacet`] whose _vertices_ are stored in a [`BTreeSet`]
    #[cfg(feature = "alloc")]
    pub type UnFacetBTree<T, Idx = usize> = UndirectedFacet<T, BTreeSet<VertexId<Idx>>, Idx>;
    /// a type alias for an [`UndirectedEdge`] whose _vertices_ are stored in a [`HashSet`]
    #[cfg(feature = "std")]
    pub type UnEdgeHash<Idx = usize> = UndirectedEdge<HashSet<VertexId<Idx>>, Idx>;
    /// a type alias for an [`UndirectedFacet`] whose _vertices_ are stored in a [`HashSet`]
    #[cfg(feature = "std")]
    pub type UnFacetHash<T, Idx = usize> = UndirectedFacet<T, HashSet<VertexId<Idx>>, Idx>;
}

use crate::index::{EdgeId, RawIndex, VertexId};
use crate::{GraphKind, Weight};

/// [`RawEdgeStore`] is a trait that defines the behavior of a store that holds the vertices
/// associated with a hyperedge or hyperfacet. It is used to abstract over different
/// implementations of edge storage, such as arrays, vectors, or sets.
///
/// **note:** The trait is sealed to prevent external implementations, ensuring that only the
/// crate can define how edges are stored. This is to maintain consistency and prevent
/// misuse of the trait in different contexts.
pub trait RawEdgeStore<Idx = usize>
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
pub trait RawNode<T> {
    type Idx: RawIndex;

    private!();

    /// returns an immutable reference to the node index
    fn index(&self) -> &VertexId<Self::Idx>;
    /// returns an immutable reference to the node data
    fn weight(&self) -> Weight<&T>;
}

pub trait RawEdge {
    type Idx: RawIndex;
    type Kind: GraphKind;
    type Store: RawEdgeStore<Self::Idx>;

    private!();

    /// returns an immutable reference to the edge index
    fn index(&self) -> &EdgeId<Self::Idx>;
    /// Returns an immutable reference to the edge data.
    fn vertices(&self) -> &Self::Store;
}

pub trait RawFacet<T>: RawEdge {
    private!();
    /// Returns the index of the edge.
    fn weight(&self) -> Weight<&T>;
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
    fn weight(&self) -> Weight<&T> {
        self.weight().view()
    }
}

impl<S, Idx, K> RawEdge for HyperEdge<S, K, Idx>
where
    Idx: Copy + RawIndex,
    K: GraphKind,
    S: RawEdgeStore<Idx>,
{
    type Kind = K;
    type Idx = Idx;
    type Store = S;

    seal!();

    fn index(&self) -> &EdgeId<Idx> {
        self.id()
    }

    fn vertices(&self) -> &S {
        self.points()
    }
}

impl<T, S, Idx, K> RawEdge for HyperFacet<T, S, K, Idx>
where
    Idx: Copy + RawIndex,
    K: GraphKind,
    S: RawEdgeStore<Idx>,
{
    type Kind = K;
    type Idx = Idx;
    type Store = S;

    seal!();

    fn index(&self) -> &EdgeId<Idx> {
        self.edge().id()
    }

    fn vertices(&self) -> &S {
        self.edge().points()
    }
}

impl<T, S, Idx, K> RawFacet<T> for HyperFacet<T, S, K, Idx>
where
    Idx: Copy + RawIndex,
    K: GraphKind,
    S: RawEdgeStore<Idx>,
{
    seal!();

    fn weight(&self) -> Weight<&T> {
        self.weight().view()
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
        impl<Idx> RawEdgeStore<Idx> for $p<VertexId<Idx>>
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
        impl<Idx> RawEdgeStore<Idx> for $($p)::*<VertexId<Idx>>
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

impl<'a, Idx> RawEdgeStore<Idx> for &'a [VertexId<Idx>]
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

impl<'a, Idx> RawEdgeStore<Idx> for &'a mut [VertexId<Idx>]
where
    Idx: RawIndex,
{
    type Store<_T> = [_T];

    seal!();

    fn len(&self) -> usize {
        <[VertexId<Idx>]>::len(self)
    }
}

impl<Idx> RawEdgeStore<Idx> for [VertexId<Idx>]
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

impl<const N: usize, Idx> RawEdgeStore<Idx> for [VertexId<Idx>; N]
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
