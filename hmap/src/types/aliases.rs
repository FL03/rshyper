/*
    appellation: aliases <module>
    authors: @FL03
*/
#[doc(hidden)]
#[allow(deprecated, unused_imports)]
pub use self::impl_deprecated::*;

use hashbrown::HashSet;
use hashbrown::hash_map::{Entry, HashMap};
use rshyper::idx::{EdgeId, VertexId};
use rshyper::{Edge, Node};

pub(crate) use hashbrown::DefaultHashBuilder;

/// a type alias for a [`HashSet`]
pub type VertexSet<Ix, S> = HashSet<VertexId<Ix>, S>;
/// a type alias for a [`Edge`] with [`VertexSet`] as its vertices
pub type HashEdge<E, K, Ix, S> = Edge<E, VertexSet<Ix, S>, K, Ix>;

/// a type alias for a [`Entry`] that whose key is a [`VertexId`] and value is a [`Node`]
pub type NodeEntry<'a, N, Ix, S> = Entry<'a, VertexId<Ix>, Node<N, Ix>, S>;
/// a type alias for a [`Entry`] that whose key is an [`EdgeId`] and value is a [`HashSurface`]
pub type EdgeEntry<'a, T, K, Ix, S> = Entry<'a, EdgeId<Ix>, HashEdge<T, K, Ix, S>, S>;

/// a type alias for a [`HashMap`] that maps [`VertexId`] to a [`Node`]
pub type NodeMap<N, Ix, S> = HashMap<VertexId<Ix>, Node<N, Ix>, S>;
/// a type alias for a [`HashMap`] that maps [`EdgeId`] to a [`HashFacet`]
pub type EdgeMap<E, K, Ix, S> = HashMap<EdgeId<Ix>, HashEdge<E, K, Ix, S>, S>;

#[allow(deprecated)]
mod impl_deprecated {
    #[deprecated(
        since = "0.1.5",
        note = "use `HashEdge` instead; this type will be removed in the next major release."
    )]
    #[doc(hidden)]
    pub type HashSurface<E, K, I, S> = super::HashEdge<E, K, I, S>;
    #[deprecated(
        since = "0.1.5",
        note = "use `EdgeMap` instead; this type will be removed in the next major release."
    )]
    #[doc(hidden)]
    pub type SurfaceMap<E, K, I, S> = super::EdgeMap<E, K, I, S>;
    #[deprecated(
        since = "0.1.3",
        note = "use `HashSurface` instead; this type will be removed in the next major release."
    )]
    #[doc(hidden)]
    pub type HashFacet<E, K, I, S> = super::HashEdge<E, K, I, S>;
    #[deprecated(
        since = "0.1.3",
        note = "use `HyperMap` instead; this type will be removed in the next major release."
    )]
    #[doc(hidden)]
    pub type HashGraph<N, E, A, S> = crate::HyperMap<N, E, A, S>;
    #[deprecated(
        since = "0.1.3",
        note = "use `DiHyperMap` instead; this type will be removed in the next major release."
    )]
    #[doc(hidden)]
    pub type DiHashGraph<N, E, Idx, S> = HashGraph<N, E, rshyper::attrs::DiAttrs<Idx>, S>;
    #[deprecated(
        since = "0.1.3",
        note = "use `UnHyperMap` instead; this type will be removed in the next major release."
    )]
    #[doc(hidden)]
    pub type UnHashGraph<N, E, Idx, S> = HashGraph<N, E, rshyper::attrs::UnAttrs<Idx>, S>;
}
