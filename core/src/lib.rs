//! # rshyper-core
//!
//! This crate provides the core functionality for the rshyper library, implementing various
//! primitives and utilities for working with hypergraphs.
//!
//! ## Features
//!
//! - `alloc`: enables the use of the `alloc` crate, allowing for dynamic memory allocation.
//! - `std`: enables the use of the standard library, providing additional functionality and
//!   types.
//!
#![allow(
    clippy::should_implement_trait,
    clippy::module_inception,
    clippy::missing_safety_doc
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "rshyper_core"]
#![crate_type = "lib"]

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
    store::prelude::*,
    traits::prelude::*,
    types::prelude::*,
    weight::prelude::*,
};

pub mod attrs;
pub mod edge;
pub mod error;
pub mod idx;
pub mod node;
/// this module provides various traits and implementations for containers capable of acting as
/// storage for different components of a hypergraph, such as edges.
pub mod store;
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
    /// the [`HyperGraph`] trait defines the core interface for hypergraphs, enabling the
    /// generalization of algorithms constructors, and graphical operators.
    pub mod hyper_graph;
    /// the [`Merge`] trait provides a way to combine two graphs into one
    pub mod merge;
    /// this module defines sequential step generators
    pub mod step;
    pub mod transform;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::contains::*;
        #[doc(inline)]
        pub use super::convert::*;
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
    //! this module provides various primitive types used throughout the library such as [Weight]
    #[doc(inline)]
    pub use self::prelude::*;

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
    pub use crate::store::prelude::*;
    pub use crate::traits::prelude::*;
    pub use crate::types::prelude::*;
    pub use crate::weight::prelude::*;
}
