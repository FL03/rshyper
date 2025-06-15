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
//! - [`astar`]: enables the A* search algorithm for hypergraphs
//! - [`breadth_first`]: enables the breadth-first search algorithm for hypergraphs
//! - [`depth_first`]: enables the depth-first search algorithm for hypergraphs
//! - [`dijkstra`]: enables Dijkstra's algorithm for finding the shortest path in hypergraphs
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

#[cfg(feature = "std")]
pub use self::{
    astar::AStarSearch, breadth_first::BreadthFirstTraversal, depth_first::DepthFirstTraversal,
    dijkstra::Dijkstra,
};
#[doc(inline)]
pub use self::{error::*, traits::prelude::*};

// pub(crate) use rshyper_core

/// this module implements the A* search algorithm
#[cfg(feature = "std")]
pub mod astar;
/// this module implements the breadth-first search algorithm
#[cfg(feature = "std")]
pub mod breadth_first;
/// this module implements the depth-first search algorithm
#[cfg(feature = "std")]
pub mod depth_first;
/// this module implements the Dijkstra's algorithm for finding the shortest path in a hypergraph
#[cfg(feature = "std")]
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
    #[doc(inline)]
    pub use super::error::AlgoError;
    #[doc(inline)]
    pub use super::traits::prelude::*;

    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::astar::AStarSearch;
    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::breadth_first::BreadthFirstTraversal;
    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::depth_first::DepthFirstTraversal;
    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::dijkstra::Dijkstra;
}

/// a type alias for a [`HashSet`](std::collections::HashSet) of [`VertexId`](rshyper::VertexId)'s
#[cfg(feature = "std")]
pub(crate) type VertexSet<Idx> = std::collections::HashSet<rshyper::VertexId<Idx>>;
