#[doc(inline)]
pub use self::prelude::*;

pub mod hyper_edge;
pub mod hyper_node;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::hyper_edge::*;
    #[doc(inline)]
    pub use super::hyper_node::*;
}

use crate::index::{RawIndex, VertexId};

/// a type alias for an [`HyperEdge`] whose _vertices_ are stored in a [`Vec`](alloc::vec::Vec)
#[cfg(feature = "alloc")]
pub type VecEdge<T, Idx = usize> = HyperEdge<T, alloc::vec::Vec<VertexId<Idx>>, Idx>;
/// a type alias for an [`HyperEdge`] whose _vertices_ are stored in a [`BTreeSet`](alloc::collections::BTreeSet)
#[cfg(feature = "alloc")]
pub type BinaryEdge<T, Idx = usize> =
    HyperEdge<T, alloc::collections::BTreeSet<VertexId<Idx>>, Idx>;
/// a type alias for an [`HyperEdge`] whose _vertices_ are stored in a [`HashSet`](std::collections::HashSet)
#[cfg(feature = "std")]
pub type HashEdge<T, Idx = usize> = HyperEdge<T, std::collections::HashSet<VertexId<Idx>>, Idx>;

/// A trait denoting a node within the hypergraph.
pub trait Point<Idx: RawIndex> {
    fn index(&self) -> &VertexId<Idx>;
}

/// Extends the base [HyperNode] trait with the [`Eq`] and [`Hash`](core::hash::Hash) traits
/// for use with hash-related structures.
pub trait HashPoint<Idx: RawIndex>: Point<Idx> + Eq + core::hash::Hash {
    private!();
}

/*
 ************* Implementations *************
*/

impl<T, Idx> HashPoint<Idx> for T
where
    Idx: RawIndex,
    T: Point<Idx> + Eq + core::hash::Hash,
{
    seal!();
}

impl<T, Idx> Point<Idx> for T
where
    Idx: RawIndex,
    T: core::borrow::Borrow<VertexId<Idx>>,
{
    fn index(&self) -> &VertexId<Idx> {
        self.borrow()
    }
}
