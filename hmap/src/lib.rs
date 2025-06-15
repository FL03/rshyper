/*
    appellation: rshyper-hmap <library>
    authors: @FL03
*/
//! # rshyper-hmap
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
#![cfg(feature = "std")]
#![crate_type = "lib"]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/FL03/rshyper/main/.artifacts/assets/logo.svg"
)]

/// declare the macros module for use throughout the crate
#[macro_use]
pub(crate) mod macros {

    #[macro_use]
    pub mod seal;
}

/// the `algo` module focuses on implementing algorithms and operators for hypergraphs
extern crate rshyper_algo as algo;
extern crate rshyper_core as rshyper;

#[doc(inline)]
pub use self::{graph::*, types::prelude::*};

pub mod graph;

mod impls {
    pub mod impl_graph;
    pub mod impl_hyper_graph;
    pub mod impl_iter;
    pub mod impl_ops;
    pub mod impl_repr;
    #[cfg(feature = "serde")]
    pub mod impl_serde;
}

pub mod iter {
    //! this module implements the iterators for the [`HyperMap`](super::HashGraph)
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod node;
    pub mod seq;
    pub mod surface;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::node::*;
        #[doc(inline)]
        pub use super::seq::*;
        #[doc(inline)]
        pub use super::surface::*;
    }
}

pub mod types {
    //! this module defines various types and type aliases in support of the [`HyperMap`](super::HyperMap)
    //! implementation
    #[doc(inline)]
    pub use self::prelude::*;

    mod aliases;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::aliases::*;
    }
}

#[doc(hidden)]
pub mod prelude {
    #[doc(inline)]
    pub use super::graph::*;
    #[doc(inline)]
    pub use super::iter::prelude::*;
    #[doc(inline)]
    pub use super::types::prelude::*;

    #[allow(deprecated)]
    #[deprecated(since = "0.1.3", note = "use `HyperMap` instead")]
    pub use super::{DiHashGraph, HashGraph, UnHashGraph};
}
