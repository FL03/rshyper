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
}
