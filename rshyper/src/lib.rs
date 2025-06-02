#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "rshyper"]
#![crate_type = "lib"]
//! # rshyper
//!
//! [![crates.io](https://img.shields.io/crates/v/rshyper?style=for-the-badge&logo=rust)](https://crates.io/crates/rshyper)
//! [![docs.rs](https://img.shields.io/docsrs/rshyper?style=for-the-badge&logo=docs.rs)](https://docs.rs/rshyper)
//! [![GitHub License](https://img.shields.io/github/license/FL03/rshyper?style=for-the-badge&logo=github)](https://github.com/FL03/rshyper/blob/main/LICENSE)
//!
//! ***
//!
//! Welcome to the `rshyper` crate - a Rust package providing a comprehensive framework for creating, manipulating, and analyzing hypergraphs using a myriad of mathematical and algorithmic
//! techniques. The crate is designed to be flexible and modular enabled via heavy feature-gating throughout the framework.
//!
//! ## Background
//!
//! Before diving in to the technical side of things, let's start by defining several terms
//! commonly used in the definition and implementation of hypergraphs.
//!
//! ### Terminology
//!
//! - **edge**: an edge is a connection between two or more vertices.
//! - **facet**: a facet materializes a hyperedge by associating some weight with the edge.
//! - **node**: a node is a complete _vertex_ in that it is considered to be weighted.
//! - **vertex**: a vertex is an _unweighted_ node defining a point within the hypergraph.
//!
//! ### Hypergraphs
//!
//! A hypergraph is an abstraction of a graph that allows edges to connect any number of
//! vertices. This flexible data-strcture is highly mathematical, yet, extremely useful in
//! many applications such as database design, network analysis, combinatorial optimization,
//! modeling topological spaces, and more.
//!
#[cfg(feature = "alloc")]
extern crate alloc;
/// declare the macros module for use throughout the crate
#[macro_use]
pub(crate) mod macros;

#[doc(inline)]
pub use self::algo::prelude::*;
#[doc(inline)]
pub use rshyper_core::*;

#[cfg(feature = "binary_graph")]
pub use self::binary_graph::BinaryGraph;
#[doc(inline)]
#[cfg(feature = "hash_graph")]
pub use self::hash_graph::{DirectedHashGraph, HashGraph, UndirectedHashGraph};

/// the `algo` module focuses on implementing algorithms and operators for hypergraphs
pub mod algo;
#[doc(hidden)]
#[cfg(feature = "binary_graph")]
pub mod binary_graph;
/// this module contains the [`HashGraph`], a hash-based hypergraph implementation
#[cfg(feature = "hash_graph")]
pub mod hash_graph;

/// the prelude module for the crate contains all commonly used traits, types, and functions
#[doc(hidden)]
#[allow(missing_docs)]
pub mod prelude {
    #[doc(no_inline)]
    pub use rshyper_core::prelude::*;

    #[doc(no_inline)]
    pub use crate::algo::prelude::*;
    #[doc(hidden)]
    #[cfg(feature = "binary_graph")]
    pub use crate::binary_graph::prelude::*;
    #[cfg(feature = "hash_graph")]
    pub use crate::hash_graph::prelude::*;
}
