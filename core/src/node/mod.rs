/*
    appellation: hyper_node <module>
    authors: @FL03
*/
#[doc(inline)]
pub use self::prelude::*;

pub mod hyper_node;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::hyper_node::*;
    #[doc(inline)]
    pub use super::{HashNode, HyperNode};
}

use crate::VertexId;

/// A trait denoting a node within the hypergraph.
pub trait HyperNode<Idx> {
    fn index(&self) -> &VertexId<Idx>;
}

/// Extends the base [HyperNode] trait with the [`Eq`] and [`Hash`](core::hash::Hash) traits
/// for use with hash-related structures.
pub trait HashNode<Idx>: HyperNode<Idx> + Eq + core::hash::Hash {
    private!();
}

/*
 ************* Implementations *************
*/

impl<T, Idx> HashNode<Idx> for T
where
    T: HyperNode<Idx> + Eq + core::hash::Hash,
{
    seal!();
}

impl<T, Idx> HyperNode<Idx> for T
where
    T: core::borrow::Borrow<VertexId<Idx>>,
{
    fn index(&self) -> &VertexId<Idx> {
        self.borrow()
    }
}
