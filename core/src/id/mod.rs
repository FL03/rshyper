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

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::index::*;
    #[doc(inline)]
    pub use super::kinds::*;
    #[doc(inline)]
    pub use super::{EdgeId, Idx, VertexId};
}

/// a type alias for an [`Index`] whose _kind_ is [`VertexIndex`]
pub type VertexId<T = usize> = Index<T, VertexIndex>;
/// a type alias for an [`Index`] whose _kind_ is [`EdgeIndex`]
pub type EdgeId<T = usize> = Index<T, EdgeIndex>;
/// a type index defining the default _type_ for indices throughout the crate
pub type Idx = usize;
