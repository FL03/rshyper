/*
    appellation: weight <module>
    authors: @FL03
*/
//! this module implements a generic [`Weight`] wrapper type for representing the weights of
//! entries within the hypergraph. Additionally, the module provides the [`Weightless`] type
//! alias for cases where there is no associated weight.
#[doc(inline)]
pub use self::{traits::*, types::*};

mod impls {
    mod impl_weight;
    mod impl_weight_ops;
    mod impl_weight_repr;

    #[doc(hidden)]
    #[allow(deprecated)]
    pub mod impl_weight_deprecated;
}

mod traits {
    //! this module defines various traits related to weights
    #[doc(inline)]
    pub use self::prelude::*;
    /// this module provides the [`AsWeight`] and [`IntoWeight`] traits for converting types to
    /// [`Weight`]
    mod convert;
    /// this module implements the [`Weighted`] trait for types that have an associated weight
    mod weighted;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::convert::*;
        #[doc(inline)]
        pub use super::weighted::*;
    }
}

mod types {
    //! this implements addtional types related to weights
    #[doc(inline)]
    pub use self::prelude::*;
    /// this module provides two distinct marker types for indicating the state of a weight
    mod kinds;
    /// this module implements the [`UnWeight`] marker type
    mod unweighted;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::kinds::*;
        #[doc(inline)]
        pub use super::unweighted::*;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::Weight;
    #[doc(inline)]
    pub use super::traits::prelude::*;
    #[doc(inline)]
    pub use super::types::prelude::*;
}

/// The [`Weight`] type is a wrapper around a generic type `T` that provides additional
/// functionality for working with weights in a graph context.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent, rename_all = "lowercase")
)]
#[repr(transparent)]
pub struct Weight<T>(pub T);

scsys::fmt_wrapper! {
    Weight<T>(
        Binary,
        Debug,
        Display,
        LowerExp,
        LowerHex,
        UpperExp,
        UpperHex,
        Octal,
        Pointer,
    )
}
