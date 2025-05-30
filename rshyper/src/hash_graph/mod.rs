/*
    appellation: hash_graph <module>
    authors: @FL03
*/
//! this module focuses on implementing a hash-based hypergraph, [`HashGraph`]
#[doc(inline)]
pub use self::graph::HashGraph;

pub mod graph;

mod impls {
    pub mod impl_ops;
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::graph::*;
}
