/*
    appellation: aliases <module>
    authors: @FL03
*/
use hashbrown::hash_map::{Entry, HashMap};
use rshyper_core::Node;
use rshyper_core::idx::{EdgeId, VertexId};

use rshyper_core::prelude::HashEdge;

/// a type alias for a [`Entry`] that whose key is a [`VertexId`] and value is a [`Node`]
pub type NodeEntry<'a, N, Ix, S> = Entry<'a, VertexId<Ix>, Node<N, Ix>, S>;
/// a type alias for a [`Entry`] that whose key is an [`EdgeId`] and value is a [`HashEdge`]
pub type EdgeEntry<'a, T, K, Ix, S> = Entry<'a, EdgeId<Ix>, HashEdge<T, K, Ix, S>, S>;

/// a type alias for a [`HashMap`] that maps [`VertexId`] to a [`Node`]
pub type NodeMap<N, Ix, S> = HashMap<VertexId<Ix>, Node<N, Ix>, S>;
/// a type alias for a [`HashMap`] that maps [`EdgeId`] to a [`HashEdge`]
pub type EdgeMap<E, K, Ix, S> = HashMap<EdgeId<Ix>, HashEdge<E, K, Ix, S>, S>;
