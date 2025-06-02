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
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::graph::*;
    #[doc(inline)]
    pub use super::iter::*;
}

pub(crate) mod aliases {
    use crate::Undirected;
    use crate::cmp::{HyperEdge, HyperFacet, HyperNode};
    use crate::index::{EdgeId, VertexId};
    use std::collections::{HashMap, HashSet, hash_map};

    pub type HyperEdgeMap<K = Undirected, Idx = usize> =
        HashMap<EdgeId<Idx>, HyperEdge<VertexSet<Idx>, K, Idx>>;

    pub type HyperFacetMap<E, K = Undirected, Idx = usize> =
        HashMap<EdgeId<Idx>, HyperFacet<E, VertexSet<Idx>, K, Idx>>;

    /// a type alias for a [`HashSet`] of [`VertexId`]
    pub type VertexSet<Idx = usize> = HashSet<VertexId<Idx>>;
    /// a type alias for a [`HashMap`] that maps [`EdgeId`] to a [`VertexSet`]
    pub type EdgeMap<Idx = usize> = HashMap<EdgeId<Idx>, VertexSet<Idx>>;
    /// a type alias for a [`HashMap`] that maps [`EdgeId`] to some
    /// weight `E`
    pub type FacetMap<Idx, E> = HashMap<EdgeId<Idx>, crate::Weight<E>>;
    /// a type alias for a [`HashMap`] that maps [`VertexId`] to
    /// [`HyperNode`]
    pub type NodeMap<N, Idx = usize> = HashMap<VertexId<Idx>, HyperNode<N, Idx>>;

    /// a type alias for a [`Entry`](hash_map::Entry) that maps [`VertexId`] to a [`HyperNode`]
    pub type NodeEntry<'a, N, Idx = usize> = hash_map::Entry<'a, VertexId<Idx>, HyperNode<N, Idx>>;
    /// a type alias for a [`Entry`](hash_map::Entry) that maps [`EdgeId`] to a [`VertexSet`]
    pub type EdgeEntry<'a, Idx = usize> = hash_map::Entry<'a, EdgeId<Idx>, VertexSet<Idx>>;
    /// a type alias for a [`Entry`](hash_map::Entry) that maps [`EdgeId`] to some weight `E`
    pub type FacetEntry<'a, E, Idx = usize> = hash_map::Entry<'a, EdgeId<Idx>, crate::Weight<E>>;
}
