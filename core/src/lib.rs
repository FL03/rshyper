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
    edge::{HyperEdge, HyperFacet},
    error::{Error, Result},
    index::prelude::*,
    node::HyperNode,
    traits::prelude::*,
    types::prelude::*,
    weight::prelude::*,
};

pub mod attrs;
pub mod edge;
pub mod error;
pub mod index;
pub mod node;
pub mod weight;

pub mod traits {
    //! this module contains various traits used throughout to establish common interfaces and
    //! behaviors
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod contains;
    pub mod convert;
    pub mod hyper_graph;
    pub mod merge;
    pub mod step;
    pub mod transform;
    pub mod weighted;

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
        #[doc(inline)]
        pub use super::weighted::*;
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

#[doc(hidden)]
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::error::*;

    #[doc(inline)]
    pub use crate::attrs::prelude::*;
    #[doc(no_inline)]
    pub use crate::edge::prelude::*;
    #[doc(no_inline)]
    pub use crate::index::prelude::*;
    #[doc(no_inline)]
    pub use crate::node::prelude::*;
    #[doc(no_inline)]
    pub use crate::traits::prelude::*;
    #[doc(no_inline)]
    pub use crate::types::prelude::*;
    #[doc(no_inline)]
    pub use crate::weight::prelude::*;
}
