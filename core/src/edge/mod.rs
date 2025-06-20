/*
    appellation: edges <module>
    authors: @FL03
*/
//! this module focuses on the [`Edge`] implementation, providing additional types, traits, and
//! representations for edges in a hypergraph.
#[doc(inline)]
pub use self::{hyper_edge::Edge, traits::*, types::*, utils::*};

/// the [`hyper_edge`] is responsible for defining the [`Edge`] struct
pub mod hyper_edge;

mod impls {
    pub mod impl_edge;
}

mod traits {
    //! this module contains the traits that define the interface for edges and facets in a
    //! hypergraph
    //!
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use self::prelude::*;
    /// this module defines the [`RawSurface`] trait
    mod surface;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::surface::*;
    }
}

mod types {
    //! this module contains various type aliases and additional types in support of the edges
    //! of a hypergraph.
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use self::prelude::*;

    mod aliases;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::aliases::*;
    }
}

mod utils {
    //! this module implements various utility functions for working with edges in a hypergraph
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use self::prelude::*;

    mod base;
    #[cfg(all(feature = "alloc", feature = "rand"))]
    pub(self) mod rand;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::base::*;
        #[doc(inline)]
        #[cfg(all(feature = "alloc", feature = "rand"))]
        pub use super::rand::*;
    }
}

pub(crate) mod prelude {
    pub use super::hyper_edge::*;
    pub use super::traits::*;
    pub use super::types::*;
    pub use super::utils::*;
}
