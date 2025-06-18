/*
    appellation: rel <module>
    authors: @FL03
*/
//! this module establishes relational traits and implementations for components of a
//! hypergraph.
//!
#[doc(inline)]
pub use self::{link::Link, traits::prelude::*};

/// here, a [`link`] essentially represents an _unweighted_ hyperedge, consisting of an
/// identifier, a domain (i.e. a collection of vertices), and a graph type.
pub mod link;

mod impls {
    pub mod impl_link;
}

pub mod traits {
    //! this module contains the traits that define the interface for edges and facets in a
    //! hypergraph
    //!
    #[doc(inline)]
    pub use self::prelude::*;
    /// this module defines the [`RawLayout`] trait for establishing a common interface for
    /// representations of a hyperedge
    mod relationship;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::relationship::*;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::link::*;
    #[doc(inline)]
    pub use super::traits::prelude::*;
}
