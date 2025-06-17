/*
    appellation: edges <module>
    authors: @FL03
*/
//! this module focuses on the [`Edge`] implementation, providing additional types, traits, and
//! representations for edges in a hypergraph.
#[doc(inline)]
pub use self::{hyper_edge::Edge, layout::EdgeLayout, traits::prelude::*, utils::prelude::*};

/// a [`hyper_surface`] speaks to a _weighted_ hyperedge, materialized here as a [`Surface`]
pub mod hyper_edge;
/// here, a [`hyper_edge`] essentially represents an _unweighted_ hyperedge, consisting of an
/// identifier, a domain (i.e. a collection of vertices), and a graph type.
pub mod layout;

mod impls {
    pub mod impl_edge;
    pub mod impl_layout;
}

pub mod traits {
    //! this module contains the traits that define the interface for edges and facets in a
    //! hypergraph
    //!
    #[doc(inline)]
    pub use self::prelude::*;
    /// this module defines the [`RawLayout`] trait for establishing a common interface for
    /// representations of a hyperedge
    mod layout;
    /// this module defines the [`RawSurface`] trait
    mod surface;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::layout::*;
        #[doc(inline)]
        pub use super::surface::*;
    }
}

pub mod types {
    //! this module contains various type aliases and additional types in support of the edges
    //! of a hypergraph.
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use self::prelude::*;

    pub(crate) mod prelude {}
}

pub mod utils {
    //! this module implements various utility functions for working with edges in a hypergraph
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use self::prelude::*;

    mod base;
    #[cfg(feature = "rand")]
    mod rand;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::base::*;
        #[doc(inline)]
        #[cfg(feature = "rand")]
        pub use super::rand::*;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::hyper_edge::*;
    #[doc(inline)]
    pub use super::layout::*;
    #[doc(inline)]
    pub use super::traits::prelude::*;
    #[doc(inline)]
    pub use super::utils::prelude::*;
}
