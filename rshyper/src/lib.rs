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

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(hidden)]
#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
}

#[cfg(feature = "hash")]
#[doc(inline)]
pub use self::hash_graph::HashGraph;
#[doc(inline)]
pub use self::{
    error::{Error, Result},
    ops::prelude::*,
    traits::prelude::*,
    types::prelude::*,
};

pub mod algo;
pub mod error;
#[cfg(feature = "hash")]
pub mod hash_graph;

pub mod ops {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod search;
    pub mod transform;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::search::*;
        #[doc(inline)]
        pub use super::transform::*;
    }
}

pub mod traits {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod convert;
    pub mod edges;
    pub mod hyper_graph;
    pub mod nodes;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::convert::*;
        #[doc(inline)]
        pub use super::edges::*;
        #[doc(inline)]
        pub use super::hyper_graph::*;
        #[doc(inline)]
        pub use super::nodes::*;
    }
}

pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod index;
    pub mod node;

    pub(crate) mod prelude {
        pub use super::index::*;
        pub use super::node::*;
    }
}

pub mod prelude {
    #[doc(no_inline)]
    pub use crate::error::*;

    #[doc(no_inline)]
    pub use crate::algo::prelude::*;
    #[cfg(feature = "hash")]
    #[doc(no_inline)]
    pub use crate::hash_graph::prelude::*;
    #[doc(no_inline)]
    pub use crate::ops::prelude::*;
    #[doc(no_inline)]
    pub use crate::traits::prelude::*;
    #[doc(no_inline)]
    pub use crate::types::prelude::*;
}
