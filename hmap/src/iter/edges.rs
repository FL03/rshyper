/*
    appellation: edges <module>
    authors: @FL03
*/
//! this module provides various iterators over the edges of a hypergraph, allowing users to
//! traverse sequentially, directionally, or even in parallel.
#[doc(inline)]
pub use self::prelude::*;

mod directional;
#[cfg(feature = "rayon")]
mod parallel;
mod sequential;
mod surface;

#[allow(unused_imports)]
mod prelude {
    #[doc(inline)]
    pub use super::directional::*;
    #[doc(inline)]
    #[cfg(feature = "rayon")]
    pub use super::parallel::*;
    #[doc(inline)]
    pub use super::sequential::*;
    #[doc(inline)]
    pub use super::surface::*;
}
