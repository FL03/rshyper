/*
    appellation: rshyper-hmap <library>
    authors: @FL03
*/
//! This crate provides the [`HyperMap`] implementation for the [`rshyper`](https://docs.rs/rshyper)
//! framework. The [`HyperMap`] is a map-based hypergraph structure designed for efficient
//! storage and manipulation. To achieve the maximum flexibility the implementation is generic
//! over 6 total types represented within 4 generic parameters:
//!
//! - `N`: the type of weight associated with a hypernode
//! - `E`: the type of weight associated with a hyperedge
//! - `A`: the attributes of the graph; an implementor of the [`GraphAttributes`](rshyper_core::GraphAttributes)
//!   trait
//!   - `A::Kind`: the _kind_ of hypergraph, either [`Directed`](rshyper_core::Directed) or [`Undirected`](rshyper_core::Undirected)
//!   - `A::Ix`: the type of indices used by the instance; bounded by the [`RawIndex`](rshyper_core::RawIndex) trait
//! - `S`: the type of [`BuildHasher`](core::hash::BuildHasher) used for the underling stores
//!
//! ## Features
//!
//! The crate is heavily feature-gated to maximize compatibility and minimize dependencies,
//! listed below are some of the most important / impactful features:
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
#![allow(clippy::should_implement_trait, clippy::module_inception)]
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
#[cfg(feature = "algo")]
extern crate rshyper_algo as algo;
extern crate rshyper_core as rshyper;

#[doc(inline)]
pub use self::{graph::*, types::prelude::*};

pub mod graph;

mod impls {
    #[cfg(feature = "algo")]
    pub mod impl_algo;
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

    pub mod edges;
    pub mod nodes;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::edges::*;
        #[doc(inline)]
        pub use super::nodes::*;
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
