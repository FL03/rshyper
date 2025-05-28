/*
    Appellation: hash <module>
    Contrib: @FL03
*/
//! this module implements a hash-based implementation of a hypergraph
#[doc(inline)]
pub use self::prelude::*;

pub mod graph;

pub(crate) mod impls {
    #[doc(hidden)]
    pub mod impl_ops;
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::HashNode;
    #[doc(inline)]
    pub use super::graph::*;
}

use crate::HyperNode;
/// Extends the base [HyperNode] trait with the [core::cmp::Eq] and [core::hash::Hash] traits
/// for use with hash-related structures.
pub trait HashNode<Idx>: HyperNode<Idx> + core::cmp::Eq + core::hash::Hash {}

impl<T, Idx> HashNode<Idx> for T where T: HyperNode<Idx> + core::cmp::Eq + core::hash::Hash {}
