/*
    appellation: rel <module>
    authors: @FL03
*/
//! this module establishes relational traits and implementations for components of a
//! hypergraph.
//!
#[doc(inline)]
pub use self::{link::Link, traits::*, types::*};

/// here, a [`link`] essentially represents an _unweighted_ hyperedge, consisting of an
/// identifier, a domain (i.e. a collection of vertices), and a graph type.
mod link;

mod impls {
    mod impl_link;
    #[doc(hidden)]
    mod impl_link_deprecated;
    mod impl_link_ext;
}

mod traits {
    //! this module contains the traits that define the interface for edges and facets in a
    //! hypergraph
    //!
    #[doc(inline)]
    pub use self::prelude::*;
    /// this module defines the [`RawLayout`] trait for establishing a common interface for
    /// representations of a hyperedge
    mod layout;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::layout::*;
    }
}

mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    mod aliases;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::aliases::*;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::link::*;
    #[doc(inline)]
    pub use super::traits::prelude::*;
}
