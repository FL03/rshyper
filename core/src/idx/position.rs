/*
    appellation: cursor <module>
    authors: @FL03
*/
use crate::idx::{EdgeId, IndexResult, RawIndex, Udx, VertexId};

/// The [`IndexCursor`] stores the current edge and vertex indices in a hypergraph, allowing
/// for efficient traversal and manipulation of the hypergraph structure. Here, when we say
/// current we mean the next indices used to create a new edge or vertex, respectively. It is
/// designed to be used in conjunction with hypergraph operations that require knowledge of the
/// current position within the hypergraph, such as adding or removing edges and vertices, or
/// iterating over the hypergraph's elements. The cursor is generic over the index type `T`,
/// which must implement the [`RawIndex`] trait. This allows for flexibility in the choice of
/// index representation, enabling the use of different types of indices (e.g., [`Udx`]
/// or some custom index types) while maintaining the same interface for the cursor.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
pub struct IndexCursor<T = Udx>
where
    T: RawIndex,
{
    /// represent the edge index that is "on-deck", or the next id used to create a new edge
    pub(crate) edge: EdgeId<T>,
    /// represent the node index that is "on-deck", or the next id used to create a new vertex
    pub(crate) point: VertexId<T>,
}

impl<T> IndexCursor<T>
where
    T: RawIndex,
{
    /// returns a new instance with the given edge and vertex indices.
    pub const fn new(edge: EdgeId<T>, point: VertexId<T>) -> Self {
        Self { edge, point }
    }
    #[allow(clippy::should_implement_trait)]
    /// initializes a new instance using the lgoical defaults for both the edge and vertex
    /// indices.
    pub fn default() -> Self
    where
        T: Default,
    {
        Self::new(EdgeId::default(), VertexId::default())
    }
    /// initializes a new instance with [`one`](num_traits::One) values for both
    /// edge and vertex indices.
    pub fn one() -> Self
    where
        T: num_traits::One,
    {
        Self::new(EdgeId::one(), VertexId::one())
    }
    /// initializes a new instance with [`zero`](num_traits::Zero) values for both
    /// edge and vertex indices.
    pub fn zero() -> Self
    where
        T: num_traits::Zero,
    {
        Self::new(EdgeId::zero(), VertexId::zero())
    }
    /// returns a new position instance using the given edge index and the logical default for
    /// the vertex index.
    pub fn from_edge(edge: EdgeId<T>) -> Self
    where
        T: Default,
    {
        Self::new(edge, VertexId::default())
    }
    /// creates a new position from a vertex index, initializing the edge index to its default value
    pub fn from_point(point: VertexId<T>) -> Self
    where
        T: Default,
    {
        Self::new(EdgeId::default(), point)
    }
    /// returns an immutable reference to the current edge index
    pub const fn edge(&self) -> &EdgeId<T> {
        &self.edge
    }
    /// returns an mutable reference to the current vertex index
    pub const fn edge_mut(&mut self) -> &mut EdgeId<T> {
        &mut self.edge
    }
    /// returns an immutable reference to the current vertex index
    pub const fn point(&self) -> &VertexId<T> {
        &self.point
    }
    /// returns a mutable reference to the current vertex index
    pub const fn point_mut(&mut self) -> &mut VertexId<T> {
        &mut self.point
    }
    /// update the edge index and returns a mutable reference to the current instance
    #[inline]
    pub fn set_edge(&mut self, edge: EdgeId<T>) -> &mut Self {
        *self.edge_mut() = edge;
        self
    }
    /// update the vertex index and returns a mutable reference to the current instance
    #[inline]
    pub fn set_point(&mut self, vertex: VertexId<T>) -> &mut Self {
        *self.point_mut() = vertex;
        self
    }
    /// consumes the current instance to create another with the given edge index
    #[inline]
    pub fn with_edge(self, edge: EdgeId<T>) -> Self {
        Self { edge, ..self }
    }
    /// consumes the current instance to create another with the given vertex index
    #[inline]
    pub fn with_point(self, vertex: VertexId<T>) -> Self {
        Self {
            point: vertex,
            ..self
        }
    }
    /// increments the current edge index by one and returns the previous value; see
    /// [`step`](EdgeId::step) for more details.
    pub fn next_edge(&mut self) -> IndexResult<EdgeId<T>>
    where
        T: crate::AddStep<Output = T>,
    {
        self.edge_mut().step()
    }
    /// increments the current vertex index by one and returns the previous value; see
    /// [`step`](VertexId::step) for more details.
    pub fn next_point(&mut self) -> IndexResult<VertexId<T>>
    where
        T: crate::AddStep<Output = T>,
    {
        self.point_mut().step()
    }
}

#[allow(deprecated)]
#[doc(hidden)]
impl<T> IndexCursor<T>
where
    T: RawIndex,
{
    #[deprecated(since = "0.1.2", note = "use `from_point` instead")]
    pub fn from_vertex(vertex: VertexId<T>) -> Self
    where
        T: Default,
    {
        Self::from_point(vertex)
    }
    #[deprecated(since = "0.1.2", note = "use `set_point` instead")]
    pub fn set_vertex(&mut self, vertex: VertexId<T>) -> &mut Self {
        self.set_point(vertex)
    }
    #[deprecated(since = "0.1.2", note = "use `with_point` instead")]
    pub fn with_vertex(self, vertex: VertexId<T>) -> Self {
        self.with_point(vertex)
    }
    #[deprecated(since = "0.1.2", note = "use `point` instead")]
    pub const fn vertex(&self) -> &VertexId<T> {
        self.point()
    }
    #[deprecated(since = "0.1.2", note = "use `point_mut` instead")]
    pub const fn vertex_mut(&mut self) -> &mut VertexId<T> {
        self.point_mut()
    }
    #[deprecated(since = "0.1.2", note = "use `next_point` instead")]
    pub fn next_vertex(&mut self) -> IndexResult<VertexId<T>>
    where
        T: crate::AddStep<Output = T>,
    {
        self.next_point()
    }
}
