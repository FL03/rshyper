/*
    appellation: edge <module>
    authors: @FL03
*/
use super::RawStore;
use crate::GraphType;
use crate::idx::{EdgeId, RawIndex};

/// [`RawEdge`] establishes a common interface for _hyperedge_ representations.
pub trait RawEdge {
    type Index: RawIndex;
    type Kind: GraphType;
    type Store: RawStore<Self::Index>;

    private!();

    /// returns an immutable reference to the edge index
    fn index(&self) -> &EdgeId<Self::Index>;
    /// returns an immutable reference to the edge data.
    fn vertices(&self) -> &Self::Store;
    /// returns a mutable reference to the edge data.
    fn vertices_mut(&mut self) -> &mut Self::Store;
    /// returns true if the edge is directed, false otherwise.
    fn is_directed(&self) -> bool {
        use core::any::TypeId;
        TypeId::of::<crate::Directed>() == TypeId::of::<Self::Kind>()
    }
    /// returns true if the edge is undirected, false otherwise.
    fn is_undirected(&self) -> bool {
        use core::any::TypeId;
        TypeId::of::<crate::Undirected>() == TypeId::of::<Self::Kind>()
    }
}
/// [`HyperEdge`] extends the behaviour of a [`RawEdge`] to include various constructors and
/// other utilitarian methods.
pub trait HyperEdge: RawEdge {
    fn new(id: EdgeId<Self::Index>, vertices: Self::Store) -> Self;
}
