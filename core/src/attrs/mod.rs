/*
    appellation: attributes <module>
    authors: @FL03
*/
//! this module implements the [`GraphAttributes`] trait and provides a concrete implementation
//! with the [`Attributes`] struct. These objects are used to define the _attributes_ of a
//! hypergraph such as the type of index used to identify vertices and edges as well as the
//! type of graph (directed or undirected).
#[doc(inline)]
pub use self::attributes::Attributes;

mod attributes;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::attributes::Attributes;
    #[doc(inline)]
    pub use super::{DirectedAttributes, GraphAttributes, UndirectedAttributes};
}
use crate::{Directed, GraphKind, RawIndex, Undirected};

/// a type alias for graph [`Attributes`] configured with a [`Directed`] graph type.
pub type DirectedAttributes<Idx> = Attributes<Idx, Directed>;
/// a type alias for graph [`Attributes`] configured with an [`Undirected`] graph type.
pub type UndirectedAttributes<Idx> = Attributes<Idx, Undirected>;

/// The [`GraphAttributes`] trait abstracts several generic types used to define a hyper graph
/// into a single entity.
pub trait GraphAttributes: 'static + Copy + Send + Sync {
    type Idx: RawIndex;
    type Kind: GraphKind;

    private!();

    fn new() -> Self
    where
        Self: Sized;
}
