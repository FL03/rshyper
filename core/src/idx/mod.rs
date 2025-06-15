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
pub use self::{error::*, index::*, position::*, traits::prelude::*, types::prelude::*};

/// this module defines the [`IndexError`] type, establishing the various errors encountered by
/// indices in a hypergraph.
pub mod error;
/// this module provides the [`IndexBase`] type, which is a generic index type used to
/// represent various kinds of indices in a hypergraph.
pub mod index;
/// this module implements the [`IndexCursor`] type, which is used to track the current edge
/// and vertex indices in a hypergraph.
pub mod position;
#[cfg(feature = "alloc")]
/// this module provides the [`IndexTracker`] for retaining a history of created indices
pub mod tracker;

#[doc(hidden)]
mod impls {
    pub mod impl_index;
    pub mod impl_ops;
    #[cfg(feature = "rand")]
    pub mod impl_rand;
    pub mod impl_repr;
}

pub mod traits {
    //! this module defines the [`RawIndex`] trait along with its related traits and
    //! implementations.
    #[doc(inline)]
    pub use self::prelude::*;

    /// this module defines various conversion routines for converting types into valid indices
    pub mod convert;
    /// this module provides the [`RawIndex`] trait
    pub mod index;
    /// this module provides the [`Indexed`] trait for defining various representations of a
    /// type that has knowledge of its index.
    pub mod indexed;

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

    pub mod kinds;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::aliases::*;
        #[doc(inline)]
        pub use super::kinds::*;
    }

    mod aliases {
        use crate::idx::{EdgeIndex, IndexBase, VertexIndex};
        /// a type alias for a [`usize`] used to define the default index type throughout the crate.
        pub type Udx = usize;
        /// a type alias for an [`Index`] whose _kind_ is [`EdgeIndex`]
        pub type EdgeId<T = Udx> = IndexBase<T, EdgeIndex>;
        /// a type alias for an [`Index`] whose _kind_ is [`VertexIndex`]
        pub type VertexId<T = Udx> = IndexBase<T, VertexIndex>;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::index::IndexBase;
    #[doc(inline)]
    pub use super::position::*;
    #[doc(inline)]
    pub use super::traits::{NumIndex, RawIndex};
    #[doc(inline)]
    pub use super::types::prelude::{EdgeId, Udx, VertexId};
}
