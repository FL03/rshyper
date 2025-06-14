/*
    appellation: path <module>
    authors: @FL03
*/
//! the `path` implements various pathfinding algorithms for hypergraphs
//!
//! ## Overview
//!
//! - `dijkstra`: implements the Dijkstra's algorithm for shortest path search
//!

#[doc(inline)]
pub use self::dijkstra::*;

/// this module implements the djikstra's algorithm for shortest path search
pub mod dijkstra;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::dijkstra::*;
}

/*
 ************* Implementations *************
*/
