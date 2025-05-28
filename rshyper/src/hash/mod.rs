/*
    Appellation: hash <module>
    Contrib: @FL03
*/
//! this module implements a hash-based implementation of a hypergraph
#[doc(inline)]
pub use self::prelude::*;

pub mod graph;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::graph::*;
}
