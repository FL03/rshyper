/*
    appellation: hash_graph <module>
    authors: @FL03
*/
//! this module focuses on implementing a hash-based hypergraph, [`HashGraph`]
#[doc(inline)]
pub use self::{aliases::*, graph::*, iter::*};

pub mod graph;
pub mod iter;

mod impls {
    pub mod impl_graph;
    pub mod impl_ops;
    pub mod impl_repr;
    #[cfg(feature = "serde")]
    pub mod impl_serde;
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::graph::*;
    #[doc(inline)]
    pub use super::iter::*;
}

pub(crate) mod aliases {
    use rshyper_core::index::{EdgeId, VertexId};
    use rshyper_core::{HyperEdge, HyperFacet, HyperNode, Undirected};
    use std::collections::{HashMap, HashSet, hash_map};

    /// a type alias for a [`HashSet`] of [`VertexId`]
    pub type VertexSet<Idx = usize> = HashSet<VertexId<Idx>>;

    /// a type alias for a [`HyperEdge`] with [`VertexSet`] as its vertices
    pub type HashEdge<K = Undirected, Idx = usize> = HyperEdge<VertexSet<Idx>, K, Idx>;
    /// a type alias for a [`HyperFacet`] with [`VertexSet`] as its vertices
    pub type HashFacet<E, K = Undirected, Idx = usize> = HyperFacet<E, VertexSet<Idx>, K, Idx>;

    /// a type alias for a [`HashMap`] that maps [`EdgeId`] to a [`HashEdge`]
    pub type EdgeMap<K = Undirected, Idx = usize> = HashMap<EdgeId<Idx>, HashEdge<K, Idx>>;
    /// a type alias for a [`HashMap`] that maps [`VertexId`] to
    /// [`HyperNode`]
    pub type NodeMap<N, Idx = usize> = HashMap<VertexId<Idx>, HyperNode<N, Idx>>;
    /// a type alias for a [`HashMap`] that maps [`EdgeId`] to a [`HashFacet`]
    pub type SurfaceMap<E, K, Idx = usize> = HashMap<EdgeId<Idx>, HashFacet<E, K, Idx>>;

    /// a type alias for a [`Entry`](hash_map::Entry) that maps [`EdgeId`] to a [`HashEdge`]
    pub type EdgeEntry<'a, K = Undirected, Idx = usize> =
        hash_map::Entry<'a, EdgeId<Idx>, HashEdge<K, Idx>>;
    /// a type alias for a [`Entry`](hash_map::Entry) that maps [`VertexId`] to a [`HyperNode`]
    pub type NodeEntry<'a, N, Idx = usize> = hash_map::Entry<'a, VertexId<Idx>, HyperNode<N, Idx>>;
    /// a type alias for a [`Entry`](hash_map::Entry) that maps [`EdgeId`] to a [`VertexSet`]
    pub type SurfaceEntry<'a, T, K = Undirected, Idx = usize> =
        hash_map::Entry<'a, EdgeId<Idx>, HashFacet<T, K, Idx>>;
}
