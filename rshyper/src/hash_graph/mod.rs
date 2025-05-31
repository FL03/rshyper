/*
    appellation: hash_graph <module>
    authors: @FL03
*/
//! this module focuses on implementing a hash-based hypergraph, [`HashGraph`]
#[allow(unused_imports)]
pub(crate) use self::aliases::*;
#[doc(inline)]
pub use self::graph::HashGraph;

pub mod graph;

mod impls {
    pub mod impl_graph;
    pub mod impl_ops;
    pub mod impl_repr;
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::graph::*;
}

pub(crate) mod aliases {
    use crate::cmp::HyperNode;
    use crate::index::{EdgeId, VertexId};
    use std::collections::{HashMap, HashSet, hash_map};

    /// a type alias for a [`HashSet`] of [`VertexId`]
    pub(crate) type VertexSet<Idx = usize> = HashSet<VertexId<Idx>>;
    /// a type alias for a [`HashMap`] that maps [`EdgeId`] to a [`VertexSet`]
    pub(crate) type EdgeMap<Idx = usize> = HashMap<EdgeId<Idx>, VertexSet<Idx>>;
    /// a type alias for a [`HashMap`] that maps [`EdgeId`] to some
    /// weight `E`
    pub(crate) type FacetMap<E, Idx = usize> = HashMap<EdgeId<Idx>, E>;
    /// a type alias for a [`HashMap`] that maps [`VertexId`] to
    /// [`HyperNode`]
    pub(crate) type NodeMap<N, Idx = usize> = HashMap<VertexId<Idx>, HyperNode<N, Idx>>;

    /// a type alias for a [`Entry`](hash_map::Entry) that maps [`VertexId`] to a [`HyperNode`]
    pub(crate) type NodeEntry<'a, N, Idx = usize> =
        hash_map::Entry<'a, VertexId<Idx>, HyperNode<N, Idx>>;
    /// a type alias for a [`Entry`](hash_map::Entry) that maps [`EdgeId`] to a [`VertexSet`]
    pub(crate) type EdgeEntry<'a, Idx = usize> = hash_map::Entry<'a, EdgeId<Idx>, VertexSet<Idx>>;
    /// a type alias for a [`Entry`](hash_map::Entry) that maps [`EdgeId`] to some weight `E`
    pub(crate) type FacetEntry<'a, E, Idx = usize> = hash_map::Entry<'a, EdgeId<Idx>, E>;
}
