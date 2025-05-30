/*
    appellation: graphs <module>
    authors: @FL03
*/
//! this module houses all of the hypergraph implementations
#[doc(inline)]
#[cfg(feature = "alloc")]
pub use self::binary_graph::BinaryGraph;
#[doc(inline)]
#[cfg(feature = "std")]
pub use self::hash_graph::HashGraph;

#[cfg(feature = "alloc")]
pub mod binary_graph;
#[cfg(feature = "std")]
pub mod hash_graph;

pub(crate) mod prelude {
    #[cfg(feature = "alloc")]
    #[doc(inline)]
    pub use super::binary_graph::*;
    #[cfg(feature = "hash")]
    #[doc(inline)]
    pub use super::hash_graph::*;
}