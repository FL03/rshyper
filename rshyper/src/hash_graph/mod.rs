/*
    appellation: hash_graph <module>
    authors: @FL03
*/
//! this module provides a hash-based implementation of a hypergraph, [`HashGraph`], alongside
//! any related representations, traits, types, and utilities.
#[doc(inline)]
pub use self::{aliases::*, graph::*, iter::prelude::*};

pub mod graph;

mod impls {
    pub mod impl_graph;
    pub mod impl_ops;
    pub mod impl_repr;
    #[cfg(feature = "serde")]
    pub mod impl_serde;
}

pub mod iter {
    //! this module implements the iterators for the [`HashGraph`](super::HashGraph)
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod node;
    pub mod surface;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::node::*;
        #[doc(inline)]
        pub use super::surface::*;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::graph::*;
}

pub(crate) mod aliases {
    use rshyper_core::index::{EdgeId, VertexId};
    use rshyper_core::{HyperEdge, HyperFacet, HyperNode, Undirected};
    use std::collections::{HashMap, HashSet, hash_map};
    use std::hash::RandomState;

    /// a type alias for a [`HashSet`] of [`VertexId`]
    pub type VertexSet<Idx = usize, S = RandomState> = HashSet<VertexId<Idx>, S>;

    /// a type alias for a [`HyperEdge`] with [`VertexSet`] as its vertices
    pub type HashEdge<K = Undirected, Idx = usize, S = RandomState> =
        HyperEdge<VertexSet<Idx, S>, K, Idx>;
    /// a type alias for a [`HyperFacet`] with [`VertexSet`] as its vertices
    pub type HashFacet<E, K = Undirected, Idx = usize, S = RandomState> =
        HyperFacet<E, VertexSet<Idx, S>, K, Idx>;

    /// a type alias for a [`HashMap`] that maps [`EdgeId`] to a [`HashEdge`]
    pub type EdgeMap<K, Idx, S = RandomState> = HashMap<EdgeId<Idx>, HashEdge<K, Idx, S>, S>;
    /// a type alias for a [`HashMap`] that maps [`VertexId`] to
    /// [`HyperNode`]
    pub type NodeMap<N, Idx, S = RandomState> = HashMap<VertexId<Idx>, HyperNode<N, Idx>, S>;
    /// a type alias for a [`HashMap`] that maps [`EdgeId`] to a [`HashFacet`]
    pub type SurfaceMap<E, K, Idx, S = RandomState> =
        HashMap<EdgeId<Idx>, HashFacet<E, K, Idx, S>, S>;

    /// a type alias for a [`Entry`](hash_map::Entry) that maps [`EdgeId`] to a [`HashEdge`]
    pub type EdgeEntry<'a, K = Undirected, Idx = usize> =
        hash_map::Entry<'a, EdgeId<Idx>, HashEdge<K, Idx>>;
    /// a type alias for a [`Entry`](hash_map::Entry) that maps [`VertexId`] to a [`HyperNode`]
    pub type NodeEntry<'a, N, Idx = usize> = hash_map::Entry<'a, VertexId<Idx>, HyperNode<N, Idx>>;
    /// a type alias for a [`Entry`](hash_map::Entry) that maps [`EdgeId`] to a [`VertexSet`]
    pub type SurfaceEntry<'a, T, K = Undirected, Idx = usize, S = RandomState> =
        hash_map::Entry<'a, EdgeId<Idx>, HashFacet<T, K, Idx, S>>;
}
