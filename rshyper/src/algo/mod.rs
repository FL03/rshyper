/*
    Appellation: algo <module>
    Contrib: @FL03
*/
//! this module implements various algorithms for hypergraphs
#[doc(inline)]
pub use self::prelude::*;

/// search algorithms for hypergraphs
#[cfg(feature = "std")]
pub mod search;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::HyperGraphAlgo;
    #[cfg(feature = "std")]
    #[doc(inline)]
    pub use super::search::*;
}

pub trait HyperGraphAlgo<N, E> {
    type Graph<N2, E2>;
}
