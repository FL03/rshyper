/*
    appellation: aliases <module>
    authors: @FL03
*/
#[allow(deprecated, unused_imports)]
pub use self::impl_deprecated::*;

/// privately redeclare the [`VertexSet`] type from [`rshyper_core`]
pub(crate) use rshyper_core::VertexSet;
use rshyper_core::idx::{EdgeId, VertexId};
use rshyper_core::{Edge, Node, Surface};
use std::collections::hash_map::{Entry, HashMap};
use std::hash::RandomState;

/// a type alias for a [`Edge`] with [`VertexSet`] as its vertices
pub type HashEdge<K, I = usize, S = RandomState> = Edge<VertexSet<I, S>, K, I>;
/// a type alias for a [`Surface`] with [`VertexSet`] as its vertices
pub type HashFacet<E, K, I = usize, S = RandomState> = Surface<E, VertexSet<I, S>, K, I>;

/// a type alias for a [`Entry`] that maps [`VertexId`] to a [`HyperNode`]
pub type NodeEntry<'a, N, I = usize> = Entry<'a, VertexId<I>, Node<N, I>>;
/// a type alias for a [`HashMap`] that maps [`VertexId`] to a [`HyperNode`]
pub type NodeMap<N, I, S = RandomState> = HashMap<VertexId<I>, Node<N, I>, S>;

/// a type alias for a [`HashMap`] that maps [`EdgeId`] to a [`HashFacet`]
pub type SurfaceMap<E, K, I, S = RandomState> = HashMap<EdgeId<I>, HashFacet<E, K, I, S>, S>;
/// a type alias for a [`Entry`] that maps [`EdgeId`] to a [`VertexSet`]
pub type SurfaceEntry<'a, T, K, I, S = RandomState> = Entry<'a, EdgeId<I>, HashFacet<T, K, I, S>>;

#[allow(deprecated)]
mod impl_deprecated {
    #[deprecated(since = "0.1.3", note = "use `HyperMap` instead")]
    pub type HashGraph<N, E, A, S> = crate::HyperMap<N, E, A, S>;
    #[deprecated(since = "0.1.3", note = "use `DiHyperMap` instead")]
    pub type DiHashGraph<N, E, Idx, S> = HashGraph<N, E, rshyper_core::attrs::DiAttributes<Idx>, S>;
    #[deprecated(since = "0.1.3", note = "use `UnHyperMap` instead")]
    pub type UnHashGraph<N, E, Idx, S> = HashGraph<N, E, rshyper_core::attrs::UnAttributes<Idx>, S>;
}
