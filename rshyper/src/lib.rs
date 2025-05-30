/*
    appellation: rshyper <library>
    authors: @FL03
*/
//! # rshyper
//!
//! Welcome to the `rshyper` crate - a Rust library for hypergraphs.
//!
//! ## Background
//!
//! Before diving in to the technical side of things, let's start by defining several terms
//! commonly used in the definition and implementation of hypergraphs.
//!
//! - `edge`: an edge is a connection between two or more vertices.
//! - `facet`: a facet materializes a hyperedge by associating some weight with the edge.
//! - `node`: a node is a complete _vertex_ in that it is considered to be weighted.
//! - `vertex`: a vertex is an _unweighted_ node defining a point within the hypergraph.
//!
//! ### Hypergraphs
//!
//! A hypergraph is an abstraction of a graph that allows edges to connect any number of
//! vertices. This flexible data-strcture is highly mathematical, yet, extremely useful in
//! many applications such as database design, network analysis, combinatorial optimization,
//! modeling topological spaces, and more.
//!
#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "rshyper"]
#![crate_type = "lib"]

#[cfg(feature = "alloc")]
extern crate alloc;
#[doc(inline)]
#[cfg(feature = "alloc")]
pub use self::graphs::*;
#[doc(inline)]
pub use self::{algo::prelude::*, ops::prelude::*};

#[doc(inline)]
pub use rshyper_core::*;

#[doc(hidden)]
#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
}

pub mod algo;

pub(crate) mod graphs {
    //! this module houses all of the hypergraph implementations
    #[doc(inline)]
    #[cfg(feature = "alloc")]
    pub use self::binary_graph::BinaryGraph;
    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use self::hash_graph::HashGraph;

    #[cfg(feature = "alloc")]
    pub mod binary_graph;
    #[cfg(feature = "std")]
    pub mod hash_graph;

    pub(crate) mod prelude {
        #[cfg(feature = "alloc")]
        #[doc(inline)]
        pub use super::binary_graph::*;
        #[cfg(feature = "hash")]
        #[doc(inline)]
        pub use super::hash_graph::*;
    }
}

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
    #[cfg(feature = "alloc")]
    #[doc(no_inline)]
    pub use crate::graphs::prelude::*;
    #[doc(no_inline)]
    pub use crate::ops::prelude::*;
}
