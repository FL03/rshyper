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
//! - [`attrs`]: Contains the [`Attributes`] and [`GraphAttributes`] types for managing graph
//!   attributes.
//! - [`edge`]: implements the [`Edge`] and [`Surface`] types for representing hyperedges
//! - [`node`]: provides the [`Node`] implementation for representing hypernodes
//! - [`weight`]: gives the [`Weight`] type for representing weights in a hypergraph
//!
#![allow(
    clippy::should_implement_trait,
    clippy::module_inception,
    clippy::missing_safety_doc
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/rshyper/rshyper/main/assets/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/rshyper/rshyper/main/assets/logo.svg"
)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
}

#[doc(inline)]
pub use self::{
    attrs::{Attributes, GraphAttributes},
    edge::{Edge, Surface},
    error::{Error, Result},
    idx::prelude::*,
    node::Node,
    traits::prelude::*,
    types::prelude::*,
    weight::prelude::*,
};

pub mod attrs;
pub mod edge;
pub mod error;
pub mod idx;
pub mod node;
/// this module implements the [`Weight`] type, which is used to represent weights in a hypergraph
pub mod weight;

pub mod traits {
    //! this module contains various traits used throughout to establish common interfaces and
    //! behaviors
    #[doc(inline)]
    pub use self::prelude::*;
    /// the [`Contains`] trait provides a way to check if a graph contains a specific component
    pub mod contains;
    /// this module provides various conversion traits and implementations
    pub mod convert;
    /// this module implements the [`RawDomain`] trait for defining the type of collection used
    /// to compose the hyperedge
    pub mod domain;
    /// the [`HyperGraph`] trait defines the core interface for hypergraphs, enabling the
    /// generalization of algorithms constructors, and graphical operators.
    pub mod hyper_graph;
    /// the [`Merge`] trait provides a way to combine two graphs into one
    pub mod merge;
    /// this module defines sequential step generators
    pub mod step;
    /// traits for transformative operations on hypergraphs, such as mapping, are implemented
    /// within this module
    pub mod transform;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::contains::*;
        #[doc(inline)]
        pub use super::convert::*;
        #[doc(inline)]
        pub use super::domain::*;
        #[doc(inline)]
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
    pub mod graph_kind;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::graph_kind::*;
    }
}

pub mod prelude {
    pub use crate::error::*;

    pub use crate::attrs::prelude::*;
    pub use crate::edge::prelude::*;
    pub use crate::idx::prelude::*;
    pub use crate::node::prelude::*;
    pub use crate::traits::prelude::*;
    pub use crate::types::prelude::*;
    pub use crate::weight::prelude::*;
}
