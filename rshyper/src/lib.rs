/*
    appellation: rshyper <library>
    authors: @FL03
*/
#![crate_name = "rshyper"]
#![crate_type = "lib"]
//! # rshyper
//!
//! Welcome to the `rshyper` crate - a Rust library for hypergraphs.
//!
//!
//! ## The Hypergraph
//!
//! A hypergraph is an abstraction of a graph that allows edges to connect any number of
//! vertices. This flexible data-strcture is highly mathematical, yet, extremely useful in
//! many applications such as database design, network analysis, combinatorial optimization,
//! modeling topological spaces, and more.
//!
//! _**definition.**_ A hypergraph is defined to be...
//!
//!
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
    cmp::prelude::*,
    error::{Error, Result},
    ops::prelude::*,
    traits::prelude::*,
    types::prelude::*,
};

pub mod algo;
pub mod error;
#[cfg(feature = "hash")]
pub mod hash_graph;

pub mod cmp {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod edge;
    pub mod node;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::edge::*;
        #[doc(inline)]
        pub use super::node::*;
    }
}

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
    pub mod hyper_graph;
    pub mod nodes;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::convert::*;
        #[doc(inline)]
        pub use super::hyper_graph::*;
        #[doc(inline)]
        pub use super::nodes::*;
    }
}

pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod graph_kind;
    pub mod index;
    pub mod weight;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::graph_kind::*;
        #[doc(inline)]
        pub use super::index::*;
        #[doc(inline)]
        pub use super::weight::*;
    }
}

pub mod prelude {
    #[doc(no_inline)]
    pub use crate::error::*;

    #[doc(no_inline)]
    pub use crate::algo::prelude::*;
    #[doc(no_inline)]
    pub use crate::cmp::prelude::*;
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
