/*
    appellation: binary_graph <module>
    authors: @FL03
*/
//! this module focuses on implementing a hash-based hypergraph, [`BinaryGraph`]
#[doc(inline)]
pub use self::graph::BinaryGraph;

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
