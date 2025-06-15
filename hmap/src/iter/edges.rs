/*
    appellation: edges <module>
    authors: @FL03
*/
//! this module provides various iterators over the edges of a hypergraph, allowing users to
//! traverse sequentially, directionally, or even in parallel.
#[doc(inline)]
#[cfg(feature = "rayon")]
pub use self::parallel::*;
#[doc(inline)]
#[allow(unused_imports)]
pub use self::{directional::*, sequential::*, surface::*};

mod directional;
mod parallel;
mod sequential;

mod surface;
