/*
    appellation: impls <module>
    authors: @FL03
*/
#[doc(inline)]
pub use self::prelude::*;

pub mod graph;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::graph::*;
}
