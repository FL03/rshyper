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

#[cfg(feature = "std")]
pub type HashEdge<T, Idx = usize> = Edge<T, std::collections::HashSet<crate::VertexId<Idx>>, Idx>;
