/*
    appellation: edges <module>
    authors: @FL03
*/
//! this module contains the [`Edge`] and [`Surface`] implementations, which are
//! respectively used to represent unweighted and weighted hyperedges in a hypergraph.
#[doc(inline)]
#[allow(unused_imports)]
pub use self::utils::prelude::*;
#[doc(inline)]
pub use self::{hyper_edge::*, hyper_facet::*, traits::prelude::*, types::prelude::*};

pub mod hyper_edge;
pub mod hyper_facet;

mod impls {
    pub mod impl_edge;
    pub mod impl_surface;
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::hyper_edge::*;
    #[doc(inline)]
    pub use super::hyper_facet::*;
    #[doc(inline)]
    pub use super::traits::prelude::*;
}

pub mod traits {
    //! this module contains the traits that define the interface for edges and facets in a
    //! hypergraph
    //!
    #[doc(inline)]
    pub use self::prelude::*;
    /// this module defines the [`RawEdge`] trait for establishing a common interface for
    /// representations of a hyperedge
    mod edge;
    /// this module defines the [`RawStore`] trait for establishing a common interface for
    /// representations of a set of [`VertexId`] that compose some edge
    mod store;
    /// this module defines the [`RawFacet`] trait
    mod surface;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::edge::*;
        #[doc(inline)]
        pub use super::store::*;
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
    #[doc(inline)]
    pub use self::prelude::*;

    #[cfg(feature = "rand")]
    mod rand;

    pub(crate) mod prelude {
        #[cfg(feature = "rand")]
        #[doc(inline)]
        pub use super::rand::*;
    }
}
