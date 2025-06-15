/*
    Appellation: algo <module>
    Contrib: @FL03
*/
//! the [`algo`](crate::algo) module focuses on implementing algorithms and operators for
//! hypergraphs.
#[doc(inline)]
pub use self::prelude::*;

/// this module implements the A* search algorithm
pub mod astar;
/// this module implements the breadth-first search algorithm
pub mod breadth_first;
/// this module implements the depth-first search algorithm
pub mod depth_first;
/// this module implements the Dijkstra's algorithm for finding the shortest path in a hypergraph
pub mod dijkstra;

pub mod traits {
    //! this module implements additional traits for defining algorithmic operators on
    //! hypergraphs.
    #[doc(inline)]
    pub use self::prelude::*;
    /// this module defines the [`Heuristic`] trait for heuristic functions
    pub mod heuristic;
    /// this module defines the [`Operator`] trait for establishing a common interface for all
    /// algorithmic operators on a hypergraph.
    pub mod operators;
    /// this module defines the interface for path-finding algorithms on hypergraphs, [`PathFinder`].
    pub mod path;
    /// this module defines the [`Search`] trait for all implemented search algorithms on a
    /// hypergraph.
    pub mod search;
    /// this module defines the [`Traversal`] trait for traversing hypergraphs.
    pub mod traverse;

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

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::traits::prelude::*;

    #[doc(inline)]
    pub use super::astar::AStarSearch;
    #[doc(inline)]
    pub use super::breadth_first::BreadthFirstTraversal;
    #[doc(inline)]
    pub use super::depth_first::DepthFirstTraversal;
    #[doc(inline)]
    pub use super::dijkstra::Dijkstra;
}
