/*
    appellation: hash_graph <module>
    authors: @FL03
*/
//! this module focuses on implementing a hash-based hypergraph, [`HashGraph`]
#[doc(inline)]
pub use self::graph::HashGraph;
#[allow(unused_imports)]
pub(crate) use self::aliases::*;

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
    use crate::index::{EdgeId, VertexId};
    use crate::cmp::HyperNode;
    /// a type alias for a [`HashSet`](std::collections::HashSet) of [`VertexId`]
    pub(crate) type VertexSet<Idx = usize> = std::collections::HashSet<VertexId<Idx>>;
    /// a type alias for a [`HashMap`](std::collections::HashMap) that maps [`EdgeId`] to a [`VertexSet`]
    pub(crate) type EdgeMap<Idx = usize> = std::collections::HashMap<EdgeId<Idx>, VertexSet<Idx>>;
    /// a type alias for a [`HashMap`](std::collections::HashMap) that maps [`EdgeId`] to some
    /// weight `E`
    pub(crate) type FacetMap<E, Idx = usize> = std::collections::HashMap<EdgeId<Idx>, E>;
    /// a type alias for a [`HashMap`](std::collections::HashMap) that maps [`VertexId`] to
    /// [`HyperNode`]
    pub(crate) type NodeMap<N, Idx = usize> =
        std::collections::HashMap<VertexId<Idx>, HyperNode<N, Idx>>;
}