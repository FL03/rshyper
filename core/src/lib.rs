/*
    appellation: rshyper-core <library>
    authors: @FL03
*/
//! # rshyper-core
//!
//! This crate provides the core functionality for the rshyper library, implementing various
//! primitives and utilities for working with hypergraphs.
//!
//!
#![crate_name = "rshyper_core"]
#![crate_type = "lib"]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
}

#[doc(inline)]
pub use self::{
    edge::{HyperEdge, HyperFacet},
    error::{Error, Result},
    index::prelude::*,
    node::HyperNode,
    traits::prelude::*,
    types::prelude::*,
};

pub mod edge;
pub mod error;
pub mod index;
pub mod node;

pub mod traits {
    //! this module contains various traits used throughout to establish common interfaces and
    //! behaviors
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod contains;
    pub mod convert;
    pub mod hyper_graph;
    pub mod merge;
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
        pub use super::transform::*;
        #[doc(inline)]
        pub use super::weighted::*;
    }
}

pub mod types {
    //! this module provides various primitive types used throughout the library such as [Weight]
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod attributes;
    pub mod graph_kind;
    pub mod weight;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::attributes::*;
        #[doc(inline)]
        pub use super::graph_kind::*;
        #[doc(inline)]
        pub use super::weight::*;
    }
}

#[doc(hidden)]
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::error::*;

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
}
