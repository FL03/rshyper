/*
    appellation: aliases <module>
    authors: @FL03
*/
#[allow(deprecated, unused_imports)]
pub use self::impl_deprecated::*;

use hashbrown::DefaultHashBuilder;
use hashbrown::hash_map::{Entry, HashMap};
use rshyper::idx::{EdgeId, VertexId};
use rshyper::{Edge, Node};

/// a type alias for a [`HashSet`](hashbrown::HashSet) of [`VertexId`]
pub type VertexSet<Idx = usize, S = DefaultHashBuilder> = hashbrown::HashSet<VertexId<Idx>, S>;

/// a type alias for a [`Edge`] with [`VertexSet`] as its vertices
pub type HashEdge<E, K, I, S> = Edge<E, VertexSet<I, S>, K, I>;

/// a type alias for a [`Entry`] that whose key is a [`VertexId`] and value is a [`Node`]
pub type NodeEntry<'a, N, I, S> = Entry<'a, VertexId<I>, Node<N, I>, S>;
/// a type alias for a [`Entry`] that whose key is an [`EdgeId`] and value is a [`HashSurface`]
pub type EdgeEntry<'a, T, K, I, S> = Entry<'a, EdgeId<I>, HashEdge<T, K, I, S>, S>;
/// a type alias for a [`HashMap`] that maps [`VertexId`] to a [`Node`]
pub type NodeMap<N, I, S = DefaultHashBuilder> = HashMap<VertexId<I>, Node<N, I>, S>;
/// a type alias for a [`HashMap`] that maps [`EdgeId`] to a [`HashFacet`]
pub type EdgeMap<E, K, I, S = DefaultHashBuilder> = HashMap<EdgeId<I>, HashEdge<E, K, I, S>, S>;

#[allow(deprecated)]
#[doc(hidden)]
mod impl_deprecated {
    #[deprecated(
        since = "0.1.5",
        note = "use `HashEdge` instead; this type will be removed in the next major release."
    )]
    pub type HashSurface<E, K, I, S> = super::HashEdge<E, K, I, S>;
    #[deprecated(
        since = "0.1.5",
        note = "use `EdgeMap` instead; this type will be removed in the next major release."
    )]
    pub type SurfaceMap<E, K, I, S> = super::EdgeMap<E, K, I, S>;
    #[deprecated(
        since = "0.1.3",
        note = "use `HashSurface` instead; this type will be removed in the next major release."
    )]
    pub type HashFacet<E, K, I, S> = super::HashEdge<E, K, I, S>;
    #[deprecated(
        since = "0.1.3",
        note = "use `HyperMap` instead; this type will be removed in the next major release."
    )]
    pub type HashGraph<N, E, A, S> = crate::HyperMap<N, E, A, S>;
    #[deprecated(
        since = "0.1.3",
        note = "use `DiHyperMap` instead; this type will be removed in the next major release."
    )]
    pub type DiHashGraph<N, E, Idx, S> = HashGraph<N, E, rshyper::attrs::DiAttrs<Idx>, S>;
    #[deprecated(
        since = "0.1.3",
        note = "use `UnHyperMap` instead; this type will be removed in the next major release."
    )]
    pub type UnHashGraph<N, E, Idx, S> = HashGraph<N, E, rshyper::attrs::UnAttrs<Idx>, S>;
}
