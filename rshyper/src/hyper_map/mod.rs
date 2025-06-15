/*
    appellation: hyper_map <module>
    authors: @FL03
*/
//! this module provides a map-based hypergraph implementation [`HyperMap`] as well as
//! various iterators, representations, and types related to the instance. The [`HyperMap`] is
//! the "flagship" model of the `rshyper` crate, providing a flexible and efficient way to
//! represent hypergraphs using hash maps and sets. Currently, the hypermap is only designed to
//! work for [`std`] implementations due to the reliance upon the `HashMap` and `HashSet` types
//! from the standard library. That being said, there are plans to explore ways of
//! generalizeing the structure to allow for flexibility in the underlying representation.
#[doc(inline)]
pub use self::{aliases::*, graph::*, iter::prelude::*};

pub mod graph;

mod impls {
    pub mod impl_graph;
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
    use rshyper_core::idx::{EdgeId, VertexId};
    use rshyper_core::{Edge, Node, Surface};
    use std::collections::{
        HashSet,
        hash_map::{Entry, HashMap},
    };
    use std::hash::RandomState;

    /// a type alias for a [`HashSet`] of [`VertexId`]
    pub type VertexSet<I = usize, S = RandomState> = HashSet<VertexId<I>, S>;

    /// a type alias for a [`Edge`] with [`VertexSet`] as its vertices
    pub type HashEdge<K, I = usize, S = RandomState> = Edge<VertexSet<I, S>, K, I>;
    /// a type alias for a [`Surface`] with [`VertexSet`] as its vertices
    pub type HashFacet<E, K, I = usize, S = RandomState> = Surface<E, VertexSet<I, S>, K, I>;

    /// a type alias for a [`Entry`] that maps [`VertexId`] to a [`HyperNode`]
    pub type NodeEntry<'a, N, I = usize> = Entry<'a, VertexId<I>, Node<N, I>>;
    /// a type alias for a [`HashMap`] that maps [`VertexId`] to a [`HyperNode`]
    pub type NodeMap<N, I, S = RandomState> = HashMap<VertexId<I>, Node<N, I>, S>;

    /// a type alias for a [`HashMap`] that maps [`EdgeId`] to a [`HashFacet`]
    pub type SurfaceMap<E, K, I, S = RandomState> = HashMap<EdgeId<I>, HashFacet<E, K, I, S>, S>;
    /// a type alias for a [`Entry`] that maps [`EdgeId`] to a [`VertexSet`]
    pub type SurfaceEntry<'a, T, K, I, S = RandomState> =
        Entry<'a, EdgeId<I>, HashFacet<T, K, I, S>>;
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
