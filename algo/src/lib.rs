/*
    Appellation: algo <library>
    Contrib: @FL03
*/
//! # rshyper-algo
//!
//! this crate provides algorithms and operators for hypergraphs.
//!
//! ## Features
//!
//! - [`astar`]: the A* search algorithm for hypergraphs
//! - [`breadth_first`]: the breadth-first search algorithm for hypergraphs
//! - [`depth_first`]: the depth-first search algorithm for hypergraphs
//! - [`dijkstra`]: Dijkstra's algorithm for finding the shortest path in hypergraphs
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
    clippy::non_canonical_clone_impl,
    clippy::non_canonical_partial_ord_impl,
    clippy::should_implement_trait
)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

extern crate rshyper_core as rshyper;

#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
}

#[doc(inline)]
pub use self::traits::prelude::*;
#[allow(unused_imports)]
pub use self::types::prelude::*;
#[cfg(feature = "std")]
pub use self::{
    astar::AStarSearch, breadth_first::BreadthFirstTraversal, depth_first::DepthFirstTraversal,
    dijkstra::Dijkstra,
};

#[cfg(feature = "std")]
pub mod astar;
#[cfg(feature = "std")]
/// this module implements the breadth-first search algorithm
pub mod breadth_first;
#[cfg(feature = "std")]
/// this module implements the depth-first search algorithm
pub mod depth_first;
#[cfg(feature = "std")]
/// this module implements the Dijkstra's algorithm for finding the shortest path in a hypergraph
pub mod dijkstra;

pub mod error;

pub mod traits {
    //! this module implements additional traits for defining algorithmic operators on
    //! hypergraphs.
    #[doc(inline)]
    pub use self::prelude::*;
    /// this module defines the [`Heuristic`] trait for heuristic functions
    mod heuristic;
    /// this module defines the [`Operator`] trait for establishing a common interface for all
    /// algorithmic operators on a hypergraph.
    mod operators;
    /// this module defines the interface for path-finding algorithms on hypergraphs, [`PathFinder`].
    mod path;
    /// this module defines the [`Search`] trait for all implemented search algorithms on a
    /// hypergraph.
    mod search;
    /// this module defines the [`Traversal`] trait for traversing hypergraphs.
    mod traverse;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::heuristic::*;
        #[doc(inline)]
        pub use super::operators::*;
        #[doc(inline)]
        pub use super::path::*;
        #[doc(inline)]
        pub use super::search::*;
        #[doc(inline)]
        pub use super::traverse::*;
    }
}

pub mod types {
    //! this module define various types and type aliases for the algorithms and operators
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use self::prelude::*;

    pub(crate) mod prelude {
        #[doc(inline)]
        #[allow(unused_imports)]
        pub use super::aliases::*;
    }

    #[allow(unused_imports)]
    mod aliases {
        #[cfg(feature = "hashbrown")]
        use hashbrown::{
            hash_map::{self, HashMap},
            hash_set::{self, HashSet},
        };
        #[cfg(all(feature = "std", not(feature = "hashbrown")))]
        pub(crate) use std::collections::{
            hash_map::{self, HashMap},
            hash_set::{self, HashSet},
        };

        #[cfg(all(feature = "std", not(feature = "hashbrown")))]
        pub(crate) type DefaultHashBuilder = std::hash::RandomState;
        #[cfg(feature = "hashbrown")]
        pub(crate) type DefaultHashBuilder = hashbrown::DefaultHashBuilder;
        /// a type alias for a [`HashSet`] of [`VertexId`] that is generic over  the index type `I`
        pub(crate) type VertexSet<I, S = DefaultHashBuilder> = HashSet<rshyper::VertexId<I>, S>;
    }
}

#[doc(hidden)]
#[allow(missing_docs)]
pub mod prelude {
    #[doc(inline)]
    pub use super::traits::prelude::*;
    #[cfg(feature = "std")]
    pub use super::{
        astar::AStarSearch, breadth_first::BreadthFirstTraversal, depth_first::DepthFirstTraversal,
        dijkstra::Dijkstra,
    };
}
