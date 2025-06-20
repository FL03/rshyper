/*
    appellation: rshyper-hmap <library>
    authors: @FL03
*/
//! A map-based implementation of a hypergraph providing efficient storage and manipulation of
//! hypernodes and hyperedges.
//!
//! ## Overview
//!
//! This implementation focuses on providing a flexible and efficient representation of a
//! hypergraph. The [`HyperMap`] is parameterized by the following types:
//!
//! - `N`: the type of weight associated with a hypernode
//! - `E`: the type of weight associated with a hyperedge
//! - `A`: the attributes of the graph are implementors of the [`GraphProps`] trait
//!   - `A::Kind`: the _kind_ of hypergraph, either [`Directed`](rshyper::Directed) or [`Undirected`](rshyper::Undirected)
//!   - `A::Ix`: the type of indices used by the instance; bounded by the [`RawIndex` trait
//! - `S`: the type of [`BuildHasher`] used for the underling stores
//!
//! This is done to maximize compatibility and flexibility, allowing users to define their own
//! hypergraphs with custom node and edge types, as well as different index types and hashers.
//!
//! ### Backend
//!
//! The underlying storage mechanics are based on the [`hashbrown`](https://crates.io/crates/hashbrown)
//! crate, which is a Rust implementation of [Google's SwissTable](https://abseil.io/blog/20180927-swisstables)
//! algorithm. This is largely done to benfit from the additional feature set natively
//! available within the crate versus the standard [`HashMap`](std::collections::HashMap)
//! implementation. That being said, it is important to note that for any applications where
//! security it a concerin it is highly recommended to use the [`RandomState`](std::hash::RandomState)
//! as the default hasher in lieu of the [`DefaultHashBuilder`] from [`foldhash`](https://crates.io/crates/foldhash)
//! as it fails to prevent against attacks such as `Hash-DoS`. See the [`hashbrown`](https://crates.io/crates/hashbrown)
//! for more information on the security implications of using a custom hasher.
//!
//! ## Features
//!
//! The [`HyperMap`] supports various features to enhance its functionality:
//!
//! - `algo`: enables the algorithmic operators from the [`rshyper_algo`](https://crates.io/crates/rshyper_algo) crate
//! - `rayon`: enables parallel processing capabilities using the `rayon` crate
//! - `serde`: enables serialization and deserialization of hypergraphs using the `serde` crate
//!
//! ## Examples
//!
//! For more detailed examples, please refer to the [examples directory](https://github.com/FL03/rshyper/blob/main/rshyper/examples).
//!
//! ### _Example #1: Basic Usage_
//!
//! ```rust
//! use rshyper_core::Weight;
//! use rshyper_hmap::HyperMap;
//! // initialize a ne, undirected hypergraph
//! let mut graph = HyperMap::<i32, i32>::undirected();
//! // insert some nodes with and without weights
//! let v0 = graph.add_vertex().expect("failed to add vertex");
//! let v1 = graph.add_node(Weight(42)).expect("failed to add the node");
//! let v2 = graph.add_node(Weight(100)).expect("failed to add the node");
//! // insert an edge between the two nodes
//! let e0 = graph.add_edge([v0, v1, v2], Weight(100)).expect("failed to add edge");
//! // verify the size of the graph; (number of edges)
//! assert_eq!(graph.size(), 1);
//! // verify the order of the graph; (number of nodes)
//! assert_eq!(graph.order(), 3);
//! ```
//!
#![crate_type = "lib"]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/FL03/rshyper/main/.artifacts/assets/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/FL03/rshyper/main/.artifacts/assets/logo.svg"
)]
#![allow(
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::should_implement_trait
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]
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

mod graph;

mod impls {
    pub mod impl_graph;
    pub mod impl_hyper_graph;
    pub mod impl_iter;
    pub mod impl_ops;
    pub mod impl_repr;

    #[cfg(feature = "algo")]
    pub mod impl_algo;
    #[cfg(feature = "serde")]
    pub mod impl_serde;

    #[doc(hidden)]
    #[allow(deprecated)]
    pub mod impl_deprecated;
}

pub mod iter {
    //! this module defines various iterators for the [`HyperMap`](super::HyperMap)
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

mod types {
    //! this module defines various types and type aliases in support of the [`HyperMap`](super::HyperMap)
    //! implementation
    #[doc(inline)]
    pub use self::prelude::*;

    pub(self) mod aliases;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::aliases::*;
    }
}

#[doc(hidden)]
#[allow(missing_docs)]
pub mod prelude {
    pub use super::graph::*;
    pub use super::iter::prelude::*;
    pub use super::types::prelude::*;
}
