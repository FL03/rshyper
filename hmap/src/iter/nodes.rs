/*
    appellation: node <module>
    authors: @FL03
*/
//! this module provides various iterators over the nodes of a hypergraph, allowing users to
//! traverse sequentially, directionally, or even in parallel.
#[doc(inline)]
#[cfg(feature = "rayon")]
pub use self::parallel::*;
#[doc(inline)]
pub use self::{directional::*, node::*, sequential::*};

mod directional;
mod parallel;
mod sequential;

mod node;
