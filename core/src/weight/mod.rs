/*
    appellation: weight <module>
    authors: @FL03
*/
//! this module implements the [`Weight`] wrapper
#[doc(inline)]
pub use self::prelude::*;

pub(crate) mod unweighted;
pub(crate) mod wrapper;

mod impls {
    pub mod impl_weight;
    pub mod impl_weight_ops;
    pub mod impl_weight_repr;
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::unweighted::Unweighted;
    #[doc(inline)]
    pub use super::wrapper::Weight;
}
