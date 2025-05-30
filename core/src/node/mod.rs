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
    pub use super::{HashPoint, Point};
}

use crate::id::{RawIndex, VertexId};

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
