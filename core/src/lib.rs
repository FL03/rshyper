/*
    appellation: rshyper-core <library>
    authors: @FL03
*/
//! This crate provides the core functionality for the rshyper library.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
}

#[doc(inline)]
pub use self::{
    error::{Error, Result},
    id::{EdgeId, VertexId},
    traits::prelude::*, 
    types::prelude::*,
};

pub mod error;
pub mod id;

pub mod traits {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod container;
    pub mod convert;
    pub mod hyper_graph;
    pub mod nodes;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::container::*;
        #[doc(inline)]
        pub use super::convert::*;
        #[doc(inline)]
        pub use super::hyper_graph::*;
        #[doc(inline)]
        pub use super::nodes::*;
    }
}

pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod graph_kind;
    pub mod weight;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::graph_kind::*;
        #[doc(inline)]
        pub use super::weight::*;
    }
}

pub mod prelude {
    #[doc(no_inline)]
    pub use crate::error::*;
    #[doc(no_inline)]
    pub use crate::id::prelude::*;
    #[doc(no_inline)]
    pub use crate::traits::prelude::*;
    #[doc(no_inline)]
    pub use crate::types::prelude::*;
}
