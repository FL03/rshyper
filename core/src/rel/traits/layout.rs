/*
    appellation: edge <module>
    authors: @FL03
*/
use crate::idx::{EdgeId, RawIndex, VertexId};
use crate::{Domain, GraphType};

/// [`RawEdge`] establishes a common interface for _hyperedge_ representations.
pub trait RawEdge {
    type Index: RawIndex;
    type Kind: GraphType;
    type Store: Domain<Self::Index>;

    private! {}

    /// returns an immutable reference to the edge index
    fn index(&self) -> &EdgeId<Self::Index>;
    /// returns a reference to the domain of the edge
    fn domain(&self) -> &Self::Store;
    /// returns a mutable reference to the domain of the edge
    fn domain_mut(&mut self) -> &mut Self::Store;
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
/// [`HyperEdge`] extends the behaviour of a [`RawEdge`] to include various constructors
/// and other utilitarian methods.
pub trait HyperEdge: RawEdge {
    fn new(id: EdgeId<Self::Index>, vertices: Self::Store) -> Self;
}
/// A [`BinaryEdge`] represents a specific type of edge that essentially defines the standard
/// edge layout of a normal graph, where each edge connects exactly two vertices.
pub trait BinaryEdge: RawEdge {
    fn lhs(&self) -> &VertexId<Self::Index>;
    fn rhs(&self) -> &VertexId<Self::Index>;
}

/*
 ************* Implementations *************
*/
use crate::BinaryDomain;

impl<S, I, K> BinaryEdge for crate::rel::Link<S, K, I>
where
    S: BinaryDomain<I>,
    I: RawIndex,
    K: GraphType,
{
    fn lhs(&self) -> &VertexId<I> {
        self.domain().src()
    }

    fn rhs(&self) -> &VertexId<I> {
        self.domain().tgt()
    }
}

impl<E, S, I, K> BinaryEdge for crate::edge::Edge<E, S, K, I>
where
    E: BinaryEdge,
    S: BinaryDomain<I>,
    I: RawIndex,
    K: GraphType,
{
    fn lhs(&self) -> &VertexId<I> {
        self.link().lhs()
    }

    fn rhs(&self) -> &VertexId<I> {
        self.link().rhs()
    }
}
