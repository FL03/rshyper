/*
    appellation: edges <module>
    authors: @FL03
*/
//! this module contains the [`Edge`] and [`Surface`] implementations, which are
//! respectively used to represent unweighted and weighted hyperedges in a hypergraph.
#[doc(inline)]
#[allow(unused_imports)]
pub use self::{
    hyper_edge::Edge, hyper_surface::Surface, traits::prelude::*, types::prelude::*,
    utils::prelude::*,
};

/// here, a [`hyper_edge`] essentially represents an _unweighted_ hyperedge, consisting of an
/// identifier, a domain (i.e. a collection of vertices), and a graph type.
pub mod hyper_edge;
/// a [`hyper_surface`] speaks to a _weighted_ hyperedge, materialized here as a [`Surface`]
pub mod hyper_surface;

mod impls {
    pub mod impl_edge;
    pub mod impl_surface;
}

pub mod traits {
    //! this module contains the traits that define the interface for edges and facets in a
    //! hypergraph
    //!
    #[doc(inline)]
    pub use self::prelude::*;
    /// this module defines the [`RawEdge`] trait for establishing a common interface for
    /// representations of a hyperedge
    pub mod edge;
    /// this module defines the [`RawFacet`] trait
    pub mod surface;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::edge::*;
        #[doc(inline)]
        pub use super::surface::*;
    }
}

pub mod types {
    //! this module contains various type aliases and additional types in support of the edges
    //! of a hypergraph.
    #[doc(inline)]
    pub use self::prelude::*;

    pub(crate) mod aliases;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::aliases::*;
    }
}

pub mod utils {
    //! this module implements various utility functions for working with edges in a hypergraph
    #[allow(unused_imports)]
    #[doc(inline)]
    pub use self::prelude::*;

    mod base;
    #[cfg(feature = "rand")]
    mod rand;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::base::*;
        #[cfg(feature = "rand")]
        #[doc(inline)]
        pub use super::rand::*;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::hyper_edge::*;
    #[doc(inline)]
    pub use super::hyper_surface::*;
    #[doc(inline)]
    pub use super::traits::prelude::*;
    #[doc(inline)]
    pub use super::utils::prelude::*;
}
