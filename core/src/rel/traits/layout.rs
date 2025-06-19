/*
    appellation: edge <module>
    authors: @FL03
*/
use crate::idx::{EdgeId, RawIndex, VertexId};
use crate::{Domain, GraphType};

/// [`RawLayout`] establishes a common interface for _hyperedge_ representations.
pub trait RawLayout {
    type Index: RawIndex;
    type Kind: GraphType;
    type Store: Domain<Self::Index>;

    private!();

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
/// [`Layout`] extends the behaviour of a [`RawLayout`] to include various constructors
/// and other utilitarian methods.
pub trait Layout: RawLayout {
    fn new(id: EdgeId<Self::Index>, vertices: Self::Store) -> Self;
}
/// A [`BinaryLayout`] represents a specific type of edge that essentially defines the standard
/// edge layout of a normal graph, where each edge connects exactly two vertices.
pub trait BinaryLayout: RawLayout {
    fn lhs(&self) -> &VertexId<Self::Index>;
    fn rhs(&self) -> &VertexId<Self::Index>;
}

/*
 ************* Implementations *************
*/
use crate::BinaryDomain;

impl<S, I, K> BinaryLayout for crate::rel::Link<S, K, I>
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

impl<E, S, I, K> BinaryLayout for crate::edge::Edge<E, S, K, I>
where
    E: BinaryLayout,
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
