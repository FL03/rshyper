/*
    appellation: edge <module>
    authors: @FL03
*/
#[doc(inline)]
pub use self::hyper_edge::*;

pub mod hyper_edge;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::HashEdge;
    #[doc(inline)]
    pub use super::hyper_edge::*;
}
#[allow(unused_imports)]
use crate::VertexId;

#[cfg(feature = "alloc")]
pub type VecEdge<T, Idx = usize> = Edge<T, alloc::vec::Vec<VertexId<Idx>>, Idx>;
#[cfg(feature = "alloc")]
pub type BinaryEdge<T, Idx = usize> = Edge<T, alloc::collections::BTreeSet<VertexId<Idx>>, Idx>;
#[cfg(feature = "std")]
pub type HashEdge<T, Idx = usize> = Edge<T, std::collections::HashSet<VertexId<Idx>>, Idx>;
