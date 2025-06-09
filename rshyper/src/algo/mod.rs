/*
    Appellation: algo <module>
    Contrib: @FL03
*/
//! the [`algo`](crate::algo) module focuses on implementing algorithms and operators for
//! hypergraphs.
#[doc(inline)]
pub use self::prelude::*;

/// search algorithms for hypergraphs
#[cfg(feature = "std")]
pub mod search;

pub mod traits {
    //! this module implements additional traits for defining algorithmic operators on
    //! hypergraphs.
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod operators;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::operators::*;
    }
}

pub(crate) mod prelude {
    #[cfg(feature = "std")]
    #[doc(inline)]
    pub use super::search::prelude::*;
}
