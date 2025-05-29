/*
    appellation: index <module>
    authors: @FL03
*/
//! this module defines the [`Index`] type and its associated types for representing indices in
//! a hypergraph.
#[doc(inline)]
pub use self::{
    index::Index,
    kinds::{EdgeIndex, IndexKind, VertexIndex},
};

pub mod index;
pub mod kinds;

#[doc(hidden)]
mod impls {
    pub mod impl_ops;
    #[cfg(feature = "rand")]
    pub mod impl_rand;
    pub mod impl_repr;
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::index::*;
    #[doc(inline)]
    pub use super::kinds::*;
    #[doc(inline)]
    pub use super::{EdgeId, Idx, VertexId};
}

/// a type alias for a [`usize`] used to define the default index type throughout the crate.
pub type Idx = usize;

/// a type alias for an [`Index`] whose _kind_ is [`EdgeIndex`]
pub type EdgeId<T = Idx> = Index<T, EdgeIndex>;
/// a type alias for an [`Index`] whose _kind_ is [`VertexIndex`]
pub type VertexId<T = Idx> = Index<T, VertexIndex>;
