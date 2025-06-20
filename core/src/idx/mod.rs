/*
    appellation: index <module>
    authors: @FL03
*/
//! the [`idx`](crate::idx) module provides the [`IndexBase`], a generic index type used to
//! establish a solid foundation for all indices used by the hypergraph. Type aliases, such as
//! [`EdgeId`] and [`VertexId`], are provided for convenience, reducing the need to continually
//! specify the index type when working with hypergraphs.
#[doc(inline)]
#[cfg(feature = "alloc")]
pub use self::tracker::IndexTracker;
#[doc(inline)]
pub use self::{error::*, frame::*, index::*, iter::*, traits::*, types::*};

/// this module defines the [`IndexError`] type, establishing the various errors encountered by
/// indices in a hypergraph.
pub mod error;
/// this module implements the [`IndexFrame`] type, which is used to track the current edge
/// and vertex indices in a hypergraph.
pub mod frame;
/// this module provides the [`IndexBase`] type, which is a generic index type used to
/// represent various kinds of indices in a hypergraph.
mod index;
#[cfg(feature = "alloc")]
/// this module provides the [`IndexTracker`] for retaining a history of created indices
pub mod tracker;

#[doc(hidden)]
mod impls {
    mod impl_index;
    mod impl_ops;
    #[cfg(feature = "rand")]
    pub(self) mod impl_rand;
    mod impl_repr;
}

pub mod iter {
    //! this module provides various iterators for indices, such as [`Counter`] and
    //! [`Stepper`].
    #[doc(inline)]
    pub use self::prelude::*;

    mod counter;
    mod stepper;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::counter::*;
        #[doc(inline)]
        pub use super::stepper::*;
    }
}

mod traits {
    //! this module defines the [`RawIndex`] trait along with its related traits and
    //! implementations.
    #[doc(inline)]
    pub use self::prelude::*;

    /// this module defines various conversion routines for converting types into valid indices
    mod convert;
    /// this module provides the [`RawIndex`] trait
    mod index;
    /// this module provides the [`Indexed`] trait for defining various representations of a
    /// type that has knowledge of its index.
    mod indexed;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::convert::*;
        #[doc(inline)]
        pub use super::index::*;
        #[doc(inline)]
        pub use super::indexed::*;
    }
}

mod types {
    //! this module provides various types in support of the [`IndexBase`](super::IndexBase)
    //! type
    //!
    #[doc(inline)]
    pub use self::prelude::*;

    mod aliases;
    mod kinds;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::aliases::*;
        #[doc(inline)]
        pub use super::kinds::*;
    }
}

pub(crate) mod prelude {
    pub use super::frame::*;
    pub use super::index::IndexBase;
    pub use super::traits::*;
    pub use super::types::*;
}
