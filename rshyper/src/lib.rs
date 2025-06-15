/*
    appellation: rshyper <library>
    authors: @FL03
*/
//! # rshyper
//!
//! [![crates.io](https://img.shields.io/crates/v/rshyper?style=for-the-badge&logo=rust)](https://crates.io/crates/rshyper)
//! [![docs.rs](https://img.shields.io/docsrs/rshyper?style=for-the-badge&logo=docs.rs)](https://docs.rs/rshyper)
//! [![GitHub License](https://img.shields.io/github/license/FL03/rshyper?style=for-the-badge&logo=github)](https://github.com/FL03/rshyper/blob/main/LICENSE)
//!
//! ***
//!
//! Welcome to the `rshyper` crate - a Rust package providing a comprehensive framework for
//! creating, manipulating, and analyzing hypergraphs using a myriad of mathematical and
//! algorithmic techniques. The crate is designed to be flexible and modular enabled via heavy
//! feature-gating throughout the framework.
//!
//! ## Background
//!
//! Before diving in to the technical side of things, let's start by defining several terms
//! commonly used in the definition and implementation of hypergraphs.
//!
//! ### Terms
//!
//! - **edge:** a hyperedge is a generalization of an edge in a graph, allowing it to connect
//!   any number of vertices.
//! - **facet:** a facet, or surface, defines a weighted hyperedge composed of one or more
//!   vertices.
//! - **node** a node is a complete _vertex_ in that it is considered to be weighted.
//! - **point:** here, a point is a synonym for a vertex, and is used to define the position of
//!   a vertex within the hypergraph.
//! - **surface:** a surface is a synonym for a facet, speaking to an edge and its associated
//!   weight.
//! - **vertex:** a vertex is an _unweighted_ node defining a point within the hypergraph.
//!
//! ### Hypergraphs
//!
//! A hypergraph is an abstraction of a graph that allows edges to connect any number of
//! vertices. This flexible data-strcture is highly mathematical, yet, extremely useful in
//! many applications such as database design, network analysis, combinatorial optimization,
//! modeling topological spaces, and more.
//!
//! #### _Definition 1:_
//! Formally, a directed hypergraph is a pair `H = (V,E)` where `V` is a set of vertices and
//! `E` is a set of hyperedges. Each hyperedge `e ∈ E` is a subset of `V` that can contain
//!
//! ## Features
//!
//! - `hash_graph`: enables the [`HashGraph`] implementation, a hash-based hypergraph structure
//! - `macros`: enables the implemented macros for streamlining graph management
//!
//! ### _Dependencies_
//!
//! **Note:** While the `alloc` and `std` libraries are feature-gated, they are required for
//! anything useful in this crate; both are enabled by default.
//!
//! - `anyhow`: enables the use of the `anyhow` crate for error handling
//! - `rayon`: enables parallel processing capabilities using the `rayon` crate
//! - `serde`: enables serialization and deserialization of hypergraphs using the `serde` crate
//!
//! ## Examples
//!
//! For more detailed examples, please refer to the [examples directory](https://github.com/FL03/rshyper/blob/main/rshyper/examples).
//!
#![allow(
    clippy::should_implement_trait,
    clippy::module_inception,
    clippy::missing_safety_doc,
    clippy::non_canonical_clone_impl,
    clippy::non_canonical_partial_ord_impl
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "rshyper"]
#![crate_type = "lib"]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/FL03/rshyper/main/.artifacts/assets/logo.svg"
)]

#[cfg(feature = "alloc")]
extern crate alloc;
/// declare the macros module for use throughout the crate
#[macro_use]
pub(crate) mod macros;

#[doc(inline)]
pub use rshyper_core::*;
#[doc(inline)]
#[cfg(feature = "macros")]
pub use rshyper_macros::*;

#[doc(inline)]
#[cfg(feature = "hyper_map")]
pub use self::hyper_map::{DiHyperMap, HyperMap, UnHyperMap};

#[doc(inline)]
#[cfg(feature = "algo")]
/// the `algo` module focuses on implementing algorithms and operators for hypergraphs
pub use rshyper_algo as algo;

/// this module contains the [`HashGraph`], a hash-based hypergraph implementation
#[cfg(feature = "hyper_map")]
pub mod hyper_map;

/// the prelude module for the crate contains all commonly used traits, types, and functions
#[allow(missing_docs)]
pub mod prelude {
    // pub use super::error::*;
    pub use rshyper_core::prelude::*;

    #[cfg(feature = "hyper_map")]
    pub use crate::hyper_map::prelude::*;
    #[cfg(feature = "algo")]
    pub use rshyper_algo::prelude::*;
    #[cfg(feature = "macros")]
    pub use rshyper_macros::*;
}
