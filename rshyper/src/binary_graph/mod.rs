/*
    appellation: binary_graph <module>
    authors: @FL03
*/
#[doc(inline)]
pub use self::graph::BinaryGraph;

pub(crate) mod graph;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::graph::*;
}
