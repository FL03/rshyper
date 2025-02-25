/*
    Appellation: rshyper <library>
    Contrib: Joe McCain III <jo3mccain@icloud.com>
*/
//! # rshyper
//!
//! This crates works to implement a hypergraph data structure in Rust. A hypergraph is a
//! generalization of a graph in which an edge can connect any number of vertices.
#![crate_name = "rshyper"]
#![crate_type = "lib"]

#[doc(inline)]
pub use self::{error::*, graph::HyperGraph, traits::prelude::*, types::prelude::*};

pub mod error;
pub mod graph;

pub mod traits {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod edges;
    pub mod indexable;
    pub mod nodes;

    pub(crate) mod prelude {
        pub use super::edges::*;
        pub use super::indexable::*;
        pub use super::nodes::*;
    }
}

pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod index;

    pub(crate) mod prelude {
        pub use super::index::*;
    }
}

pub mod prelude {
    pub use crate::error::*;
    pub use crate::graph::HyperGraph;
    pub use crate::traits::prelude::*;
    pub use crate::types::prelude::*;
}
