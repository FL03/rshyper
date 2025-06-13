/*
    appellation: index <module>
    authors: @FL03
*/
//! the [`index`](crate::index) module is centered around the [`IndexBase`] implementation.
//! Additional type aliases ([`EdgeId`] and [`VertexId`]) are provided for convenience, as well
//! as traits that define the behaviour of indices in a hypergraph.
#[doc(inline)]
#[cfg(feature = "std")]
pub use self::tracker::IndexTracker;
#[doc(inline)]
pub use self::{
    error::*, id::IndexBase, position::IndexCursor, traits::prelude::*, types::prelude::*,
};

pub mod error;
/// this module provides the [`IndexBase`] type, which is a generic index type used to
/// represent various kinds of indices in a hypergraph.
pub mod id;
/// this module implements the [`IndexCursor`] type, which is used to track the current edge
/// and vertex indices in a hypergraph.
pub mod position;
#[cfg(feature = "std")]
/// this module provides the [`IndexTracker`] for retaining a history of created indices
pub mod tracker;

#[doc(hidden)]
mod impls {
    pub mod impl_ops;
    #[cfg(feature = "rand")]
    pub mod impl_rand;
    pub mod impl_repr;
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::error::*;
    #[doc(inline)]
    pub use super::id::*;
    #[doc(inline)]
    pub use super::position::*;
    #[doc(inline)]
    pub use super::traits::prelude::*;
    #[doc(inline)]
    pub use super::types::prelude::*;

    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::tracker::IndexTracker;
}

pub mod traits {
    //! this module defines the [`RawIndex`] trait along with its related traits and
    //! implementations.
    #[doc(inline)]
    pub use self::prelude::*;
    /// this module provides the [`RawIndex`] trait
    pub mod index;
    /// this module provides the [`Indexed`] trait for defining various representations of a
    /// type that has knowledge of its index.
    pub mod indexed;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::index::*;
        #[doc(inline)]
        pub use super::indexed::*;
    }
}

pub mod types {
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
