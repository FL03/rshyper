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
/// this module implements the core functionality of the `rshyper` crate
#[doc(inline)]
pub use rshyper_core::*;

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
pub use self::{algo::prelude::*, ops::prelude::*};

pub mod algo;
#[cfg(feature = "hash")]
pub mod hash_graph;

pub mod ops {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod transform;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::transform::*;
    }
}

pub mod prelude {
    #[doc(no_inline)]
    pub use rshyper_core::prelude::*;

    #[doc(no_inline)]
    pub use crate::algo::prelude::*;
    #[cfg(feature = "hash")]
    #[doc(no_inline)]
    pub use crate::hash_graph::prelude::*;
    #[doc(no_inline)]
    pub use crate::ops::prelude::*;
}
