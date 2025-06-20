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
pub use self::{error::*, tracker::IndexTracker};
#[doc(inline)]
pub use self::{index::*, iter::*, position::*, traits::prelude::*, types::prelude::*};

#[cfg(feature = "alloc")]
/// this module defines the [`IndexError`] type, establishing the various errors encountered by
/// indices in a hypergraph.
pub(self) mod error;
/// this module provides the [`IndexBase`] type, which is a generic index type used to
/// represent various kinds of indices in a hypergraph.
pub(self) mod index;
/// this module implements the [`IndexCursor`] type, which is used to track the current edge
/// and vertex indices in a hypergraph.
pub(self) mod position;
#[cfg(feature = "alloc")]
/// this module provides the [`IndexTracker`] for retaining a history of created indices
pub(self) mod tracker;

#[doc(hidden)]
mod impls {
    pub(self) mod impl_index;
    pub(self) mod impl_ops;
    #[cfg(feature = "rand")]
    pub(self) mod impl_rand;
    pub(self) mod impl_repr;
}

pub mod iter {
    //! this module provides various iterators for indices, such as [`IndexIter`], which
    //! iterates over the indices in a hypergraph.
    #[doc(inline)]
    pub use self::prelude::*;

    pub(self) mod counter;
    pub(self) mod stepper;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::counter::*;
        #[doc(inline)]
        pub use super::stepper::*;
    }
}

pub mod traits {
    //! this module defines the [`RawIndex`] trait along with its related traits and
    //! implementations.
    #[doc(inline)]
    pub use self::prelude::*;

    /// this module defines various conversion routines for converting types into valid indices
    pub(self) mod convert;
    /// this module provides the [`RawIndex`] trait
    pub(self) mod index;
    /// this module provides the [`Indexed`] trait for defining various representations of a
    /// type that has knowledge of its index.
    pub(self) mod indexed;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::convert::*;
        #[doc(inline)]
        pub use super::index::*;
        #[doc(inline)]
        pub use super::indexed::*;
    }
}

pub mod types {
    //! this module provides various types in support of the [`IndexBase`](super::IndexBase)
    //! type
    //!
    #[doc(inline)]
    pub use self::prelude::*;

    pub(self) mod aliases;
    pub(self) mod kinds;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::aliases::*;
        #[doc(inline)]
        pub use super::kinds::*;
    }
}

pub(crate) mod prelude {
    pub use super::index::IndexBase;
    pub use super::position::*;
    pub use super::traits::prelude::*;
    pub use super::types::prelude::*;
}
