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
//! Welcome to the `rshyper-hmap` crate! This package provides the [`HyperMap`] implementation,
//! a map-based hypergraph structure designed for efficient storage and manipulation using the 
//! native [`HashMap`](std::collections::HashMap) type.
//! 
//! - `N`: the type of weight associated with a hypernode
//! - `E`: the type of weight associated with a hyperedge
//! - `A`: the attributes of the hypergraph
//!   - `A::Kind`: the _kind_ of hypergraph, either [`Directed`](rshyper_core::Directed) or [`Undirected`](rshyper_core::Undirected)
//!   - `A::Ix`: the type of index used by components within the graph
//! - `S`: the type of [`BuildHasher`](core::hash::BuildHasher) used for the underling stores
//!
//! ## Features
//!
//! - `hyper_map`: enables the [`HyperMap`] implementation, a hash-based hypergraph structure
//! - `macros`: enables the implemented macros for streamlining graph management
//!
//! ### _Dependencies_
//!
//! - `rayon`: enables parallel processing capabilities using the `rayon` crate
//! - `serde`: enables serialization and deserialization of hypergraphs using the `serde` crate
//!
//! ## Examples
//!
//! For more detailed examples, please refer to the [examples directory](https://github.com/FL03/rshyper/blob/main/rshyper/examples).
//! 
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
// **** WARNING ****
// the `std` feature is required by the crate, only declared for concistency w.r.t. the
// available features and for ensuring that all the depencies actually implement the `std`
// feature since the workspace naturally imports them with the `default-features = false`
// flag toggled
// **** WARNING ****
#![cfg(feature = "std")]
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
#[allow(missing_docs)]
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
