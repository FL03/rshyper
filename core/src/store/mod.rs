/*
    appellation: store <module>
    authors: @FL03
*/
//! this module is focused on defining a set of traits and types for abstracting the behaviourds
//! of an entity capable of storing some data.
#[doc(inline)]
pub use self::{container::*, error::*, traits::prelude::*};

/// this module implements the [`ContainerBase`] type, which is a base type for containers that
/// use a store to manage their data.
pub mod container;
/// this module defiens the [`StoreError`] enum for handling various errors that can occur with
/// stores
pub mod error;

pub mod traits {
    #[doc(inline)]
    pub use self::prelude::*;

    /// this module defines the [`KeyValue`] trait and its associated types for key-value
    /// stores
    mod key_value;
    /// this module defines the [`RawContainer`] trait for establishing a core interface for
    /// various representations of a container.
    mod raw_container;
    /// this module defines the [`RawStore`] trait for establishing a common interface for
    /// representations of a set of [`VertexId`] that compose some edge
    mod raw_store;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::key_value::*;
        #[doc(inline)]
        pub use super::raw_container::*;
        #[doc(inline)]
        pub use super::raw_store::*;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::traits::prelude::*;
}
