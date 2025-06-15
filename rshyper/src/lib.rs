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
//! - `hyper_map`: enables the [`HyperMap`] implementation, a hash-based hypergraph structure
//! - `macros`: enables the implemented macros for streamlining graph management
//!
//! ### _Dependencies_
//!
//! **Note:** While the `alloc` and `std` libraries are feature-gated, they are required for
//! anything useful in this crate; both are enabled by default.
//!
//! - `rayon`: enables parallel processing capabilities using the `rayon` crate
//! - `serde`: enables serialization and deserialization of hypergraphs using the `serde` crate
//!
//! ## Examples
//!
//! For more detailed examples, please refer to the [examples directory](https://github.com/FL03/rshyper/blob/main/rshyper/examples).
//!
//! ### _Example 1: Basic Usage_
//!
//! ```rust
//! use rshyper::{HyperMap, IntoWeight};
//!
//! let mut graph = HyperMap::<usize, usize>::undirected();
//! // add some unweighted vertices
//! let v0 = graph.add_vertex().expect("failed to add vertex");
//! let v1 = graph.add_vertex().expect("failed to add vertex");
//! // add a weighted node
//! let v2 = graph.add_node(10.into_weight()).expect("failed to add node");
//! // create some edges using those nodes
//! let e0 = graph.add_edge([v0, v1]).expect("failed to add edge");
//! let e1 = graph.add_edge([v1, v2]).expect("failed to add edge");
//! // create a surface (weighted edge) using the nodes
//! let e3 = graph.add_surface([v0, v2], 5.into_weight()).expect("failed to add surface");
//! ```
//!
#![crate_name = "rshyper"]
#![crate_type = "lib"]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/FL03/rshyper/main/.artifacts/assets/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/FL03/rshyper/main/.artifacts/assets/logo.svg"
)]
#![allow(
    clippy::should_implement_trait,
    clippy::module_inception,
    clippy::missing_safety_doc,
    clippy::non_canonical_clone_impl,
    clippy::non_canonical_partial_ord_impl
)]
#![cfg_attr(not(feature = "std"), no_std)]

/*
 ************* ROOT *************
*/

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
#[cfg(feature = "macros")]
pub(crate) mod macros {
    //! this module defines the various macros used throughout the crate to streamline the
    //! creation and maniuplation of hypergraphs and their constituent components.
    #[macro_use]
    pub mod hypergraph;
}

/*
 ************* REIMPORTS *************
*/

#[doc(inline)]
#[cfg(feature = "hyper_map")]
pub use self::hyper_map::{DiHyperMap, HyperMap, UnHyperMap};
#[doc(inline)]
pub use rshyper_core::*;

#[allow(deprecated)]
#[cfg(feature = "hyper_map")]
#[deprecated(since = "0.1.3", note = "use `HyperMap` instead")]
pub use self::hyper_map::{DiHashGraph, HashGraph, UnHashGraph};

/*
 ************* FEATURE-GATED MODULES *************
*/

#[doc(inline)]
#[cfg(feature = "algo")]
/// the `algo` module focuses on implementing algorithms and operators for hypergraphs
pub use rshyper_algo as algo;
#[doc(inline)]
#[cfg(feature = "hyper_map")]
/// this module contains the [`HyperMap`](rshyper_hmap::HyperMap), a hash-based hypergraph
/// implementation
pub use rshyper_hmap as hyper_map;

/*
 ************* PRELUDE *************
*/

#[doc(hidden)]
#[allow(missing_docs)]
pub mod prelude {
    pub use rshyper_core::prelude::*;

    #[cfg(feature = "macros")]
    pub use crate::{hyperedge, hypergraph, hypernode};
    #[cfg(feature = "algo")]
    pub use rshyper_algo::prelude::*;
    #[cfg(feature = "hyper_map")]
    pub use rshyper_hmap::prelude::*;
}
