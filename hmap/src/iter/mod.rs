/*
    Appellation: iter <module>
    Created At: 2026.01.06:20:51:12
    Contrib: @FL03
*/
//! iterators for the hypermap implementation of a hypergraph
//!
#[doc(inline)]
pub use self::{edges::*, nodes::*};

pub mod edges;
pub mod nodes;

pub(crate) mod prelude {
    pub use super::edges::*;
    pub use super::nodes::*;
}
