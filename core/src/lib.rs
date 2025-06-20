/*
    appellation: rshyper-core <library>
    authors: @FL03
*/
//! # rshyper-core
//!
//! This crate provides the core functionality for the rshyper library, implementing various
//! primitives and utilities for working with hypergraphs.
//!
//! ## Components
//!
//! - [`attrs`]: Contains the [`Attrs`] and [`GraphProps`] types for managing graph attributes.
//! - [`edge`]: implements the [`Edge`] and [`Surface`] types for representing hyperedges
//! - [`node`]: provides the [`Node`] implementation for representing hypernodes
//! - [`weight`]: gives the [`Weight`] type for representing weights in a hypergraph
//!
#![crate_type = "lib"]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/FL03/rshyper/main/.artifacts/assets/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/FL03/rshyper/main/.artifacts/assets/logo.svg"
)]
#![allow(
    clippy::should_implement_trait,
    clippy::module_inception,
    clippy::missing_safety_doc
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]
#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
}

#[doc(inline)]
pub use self::{
    attrs::{Attrs, GraphProps},
    edge::Edge,
    error::{Error, Result},
    idx::prelude::*,
    node::Node,
    rel::{Link, RawLayout},
    traits::prelude::*,
    types::*,
    weight::prelude::*,
};

pub mod attrs;
pub mod edge;
pub mod error;
pub mod idx;
pub mod node;
pub mod rel;
pub mod weight;

pub mod traits {
    //! this module contains various traits used throughout to establish common interfaces and
    //! behaviors
    #[doc(inline)]
    pub use self::prelude::*;
    /// the [`Contains`] trait provides a way to check if a graph contains a specific component
    mod contains;
    /// this module implements the [`RawDomain`] trait for defining the type of collection used
    /// to compose the hyperedge
    mod domain;
    /// the [`HyperGraph`] trait defines the core interface for hypergraphs, enabling the
    /// generalization of algorithms constructors, and graphical operators.
    #[cfg(feature = "alloc")]
    mod hyper_graph;
    /// the [`Merge`] trait provides a way to combine two graphs into one
    mod merge;
    /// this module defines sequential step generators
    mod step;
    /// traits for transformative operations on hypergraphs, such as mapping, are implemented
    /// within this module
    mod transform;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::contains::*;
        #[doc(inline)]
        pub use super::domain::*;
        #[doc(inline)]
        #[cfg(feature = "alloc")]
        pub use super::hyper_graph::*;
        #[doc(inline)]
        pub use super::merge::*;
        #[doc(inline)]
        pub use super::step::*;
        #[doc(inline)]
        pub use super::transform::*;
    }
}

pub mod types {
    //! this module provides various types used throughout the library
    #[doc(inline)]
    pub use self::prelude::*;
    /// this module defines the two types of graph kinds: [`Directed`] and [`Undirected`]
    mod graph_kind;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::graph_kind::*;
    }
}

pub mod prelude {
    // pub use crate::error::*;

    pub use crate::attrs::prelude::*;
    pub use crate::edge::prelude::*;
    pub use crate::idx::prelude::*;
    pub use crate::node::prelude::*;
    pub use crate::rel::prelude::*;
    pub use crate::traits::prelude::*;
    pub use crate::types::prelude::*;
    pub use crate::weight::prelude::*;
}
