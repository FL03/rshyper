/*
    Appellation: algo <module>
    Contrib: @FL03
*/
//! the [`algo`](crate::algo) module focuses on implementing algorithms and operators for
//! hypergraphs.
#[doc(inline)]
pub use self::prelude::*;

#[cfg(feature = "std")]
/// path-finding algorithms for hypergraphs
pub mod path;
/// search algorithms for hypergraphs
#[cfg(feature = "std")]
pub mod search;

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
    #[cfg(feature = "std")]
    pub use super::path::prelude::*;
    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::search::prelude::*;
}
