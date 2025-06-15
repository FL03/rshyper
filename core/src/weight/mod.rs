/*
    appellation: weight <module>
    authors: @FL03
*/
//! this module implements a generic [`Weight`] wrapper type for representing the weights of
//! entries within the hypergraph. Additionally, the module provides the [`Weightless`] type
//! alias for cases where there is no associated weight.
#[doc(inline)]
pub use self::{traits::prelude::*, unweighted::*, wrapper::*};

pub mod unweighted;
pub mod wrapper;

mod impls {
    pub mod impl_weight;
    pub mod impl_weight_ops;
    pub mod impl_weight_repr;
}

pub mod traits {
    //! this module defines various traits related to weights
    #[doc(inline)]
    pub use self::prelude::*;
    /// this module provides the [`AsWeight`] and [`IntoWeight`] traits for converting types to
    /// [`Weight`]
    pub mod convert;
    /// this module implements the [`Weighted`] trait for types that have an associated weight
    pub mod weighted;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::convert::*;
        #[doc(inline)]
        pub use super::weighted::*;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::traits::prelude::*;
    #[doc(inline)]
    pub use super::unweighted::{UnWeight, Weightless};
    #[doc(inline)]
    pub use super::wrapper::Weight;
}
