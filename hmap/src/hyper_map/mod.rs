/*
    appellation: hyper_map <module>
    authors: @FL03
*/

#[doc(inline)]
pub use self::{aliases::*, graph::*, iter::prelude::*};

pub mod graph;

mod impls {
    pub mod impl_graph;
    pub mod impl_hyper_graph;
    pub mod impl_iter;
    pub mod impl_ops;
    pub mod impl_repr;
    #[cfg(feature = "serde")]
    pub mod impl_serde;
}

pub mod iter {
    //! this module implements the iterators for the [`HyperMap`](super::HashGraph)
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod node;
    pub mod seq;
    pub mod surface;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::node::*;
        #[doc(inline)]
        pub use super::seq::*;
        #[doc(inline)]
        pub use super::surface::*;
    }
}
pub(crate) mod aliases {
    
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::graph::*;
    // #[allow(dead_code, deprecated)]
    // #[deprecated(since = "0.1.3", note = "use `HyperMap` instead")]
    // pub use super::{HashGraph, DiHashGraph, UnHashGraph};
}

#[allow(dead_code, deprecated)]
#[deprecated(since = "0.1.3", note = "use `HyperMap` instead")]
pub type HashGraph<N, E, A, S> = HyperMap<N, E, A, S>;
#[allow(dead_code, deprecated)]
#[deprecated(since = "0.1.3", note = "use `DiHyperMap` instead")]
pub type DiHashGraph<N, E, Idx, S> = HyperMap<N, E, crate::attrs::DiAttributes<Idx>, S>;
#[allow(dead_code, deprecated)]
#[deprecated(since = "0.1.3", note = "use `UnHyperMap` instead")]
pub type UnHashGraph<N, E, Idx, S> = HyperMap<N, E, crate::attrs::UnAttributes<Idx>, S>;
