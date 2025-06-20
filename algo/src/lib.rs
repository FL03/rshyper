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
#![cfg_attr(feature = "nightly", feature(allocator_api))]

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

#[doc(hidden)]
pub mod prelude {
    pub use crate::traits::prelude::*;

    #[cfg(feature = "alloc")]
    pub use crate::astar::AStarSearch;
    #[cfg(feature = "alloc")]
    pub use crate::breadth_first::BreadthFirstTraversal;
    #[cfg(feature = "alloc")]
    pub use crate::depth_first::DepthFirstTraversal;
    #[cfg(feature = "alloc")]
    pub use crate::dijkstra::Dijkstra;
}
