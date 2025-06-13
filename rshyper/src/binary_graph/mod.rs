/*
    appellation: binary_graph <module>
    authors: @FL03
*/
//! this module focuses on implementing a hash-based hypergraph, [`BinaryGraph`]
#[doc(inline)]
pub use self::{aliases::*, graph::BinaryGraph};

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
    use alloc::collections::{BTreeMap, BTreeSet};
    use rshyper_core::idx::{EdgeId, VertexId};
    use rshyper_core::{Node, Surface};

    /// a type alias for a [`BTreeSet`] storing vertices in a hypergraph
    pub type VertexBSet<Idx> = BTreeSet<VertexId<Idx>>;
    /// a type alias for a [`HyperFacet`] that uses a [`BTreeSet`] to store its vertices
    pub type BTreeFacet<E, K, Idx> = Surface<E, BTreeSet<VertexId<Idx>>, K, Idx>;
    /// a type alias for a [`BTreeMap`] that uses a [`VertexId`] as a key for a [`HyperNode`]
    pub type NodeBMap<N, Idx> = BTreeMap<VertexId<Idx>, Node<N, Idx>>;
    /// a type alias for a [`BTreeMap`] that uses an [`EdgeId`] as a key for a [`BTreeFacet`]
    pub type SurfaceBMap<E, K, Idx> = BTreeMap<EdgeId<Idx>, BTreeFacet<E, K, Idx>>;
}
