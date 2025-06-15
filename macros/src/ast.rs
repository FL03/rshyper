/*
    appellation: ast <module>
    authors: @FL03
*/
#[doc(inline)]
pub use self::prelude::*;

pub mod edge;
pub mod graph;
pub mod node;
pub mod weight;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::edge::*;
    #[doc(inline)]
    pub use super::graph::*;
    #[doc(inline)]
    pub use super::node::*;
    #[doc(inline)]
    pub use super::weight::*;
}
