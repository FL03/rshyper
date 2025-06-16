/*
    appellation: aliases <module>
    authors: @FL03
*/
pub(crate) use rshyper::VertexSet;

#[allow(deprecated, unused_imports)]
pub use self::impl_deprecated::*;

use rshyper::idx::{EdgeId, VertexId};
use rshyper::{Edge, Node};
use std::collections::hash_map::{Entry, HashMap};
use std::hash::RandomState;

/// a type alias for a [`Surface`] with [`VertexSet`] as its vertices
pub type HashSurface<E, K, I = usize, S = RandomState> = Edge<E, VertexSet<I, S>, K, I>;

/// a type alias for a [`Entry`] that whose key is a [`VertexId`] and value is a [`Node`]
pub type NodeEntry<'a, N, I = usize> = Entry<'a, VertexId<I>, Node<N, I>>;
/// a type alias for a [`Entry`] that whose key is an [`EdgeId`] and value is a [`HashSurface`]
pub type SurfaceEntry<'a, T, K, I, S = RandomState> = Entry<'a, EdgeId<I>, HashSurface<T, K, I, S>>;
/// a type alias for a [`HashMap`] that maps [`VertexId`] to a [`Node`]
pub type NodeMap<N, I, S = RandomState> = HashMap<VertexId<I>, Node<N, I>, S>;
/// a type alias for a [`HashMap`] that maps [`EdgeId`] to a [`HashFacet`]
pub type SurfaceMap<E, K, I, S = RandomState> = HashMap<EdgeId<I>, HashSurface<E, K, I, S>, S>;

#[allow(deprecated)]
mod impl_deprecated {
    #[deprecated(since = "0.1.3", note = "use `HashSurface` instead")]
    pub type HashFacet<E, K, I, S> = super::HashSurface<E, K, I, S>;
    #[deprecated(since = "0.1.3", note = "use `HyperMap` instead")]
    pub type HashGraph<N, E, A, S> = crate::HyperMap<N, E, A, S>;
    #[deprecated(since = "0.1.3", note = "use `DiHyperMap` instead")]
    pub type DiHashGraph<N, E, Idx, S> = HashGraph<N, E, rshyper::attrs::DiAttrs<Idx>, S>;
    #[deprecated(since = "0.1.3", note = "use `UnHyperMap` instead")]
    pub type UnHashGraph<N, E, Idx, S> = HashGraph<N, E, rshyper::attrs::UnAttrs<Idx>, S>;
}
