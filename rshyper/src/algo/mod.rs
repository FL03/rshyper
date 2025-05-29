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
    pub use super::RawGraphAlgorithm;
    #[cfg(feature = "std")]
    #[doc(inline)]
    pub use super::search::prelude::*;
}

/// this trait is used to denote an algorithmic operator that can be applied to a hypergraph.
pub trait RawGraphAlgorithm<N, E> {
    type Graph<N2, E2>;

    private!();
}
