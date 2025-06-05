/*
    appellation: cursor <module>
    authors: @FL03
*/
use crate::index::{EdgeId, IndexResult, RawIndex, Udx, VertexId};

/// The [`IndexCursor`] implementation is uses to track the current indexes of edges and vertices
/// within a hypergraph.
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
    pub(crate) edge: EdgeId<T>,
    pub(crate) vertex: VertexId<T>,
}

impl<T> IndexCursor<T>
where
    T: RawIndex,
{
    /// returns a new [`Position`] instance with the given edge and vertex indices.
    pub fn new(edge: EdgeId<T>, vertex: VertexId<T>) -> Self {
        Self { edge, vertex }
    }
    /// initialize a new [`Position`] using the logical default for both the edge and vertex
    /// indices.
    pub fn default() -> Self
    where
        T: Default,
    {
        Self {
            edge: EdgeId::default(),
            vertex: VertexId::default(),
        }
    }
    /// initializes a new [`Position`] instance with [`one`](num_traits::One) values for both
    /// edge and vertex indices.
    pub fn one() -> Self
    where
        T: num_traits::One,
    {
        Self {
            edge: EdgeId::one(),
            vertex: VertexId::one(),
        }
    }
    /// initializes a new [`Position`] instance with [`zero`](num_traits::Zero) values for both
    /// edge and vertex indices.
    pub fn zero() -> Self
    where
        T: num_traits::Zero,
    {
        Self {
            edge: EdgeId::zero(),
            vertex: VertexId::zero(),
        }
    }
    /// returns a new position instance using the given edge index and the logical default for
    /// the vertex index.
    pub fn from_edge(edge: EdgeId<T>) -> Self
    where
        T: Default,
    {
        Self {
            edge,
            vertex: VertexId::default(),
        }
    }
    /// creates a new position from a vertex index, initializing the edge index to its default value
    pub fn from_vertex(vertex: VertexId<T>) -> Self
    where
        T: Default,
    {
        Self {
            edge: EdgeId::default(),
            vertex,
        }
    }
    /// consumes the current instance to create another with the given edge index
    pub fn with_edge(self, edge: EdgeId<T>) -> Self {
        Self { edge, ..self }
    }
    /// consumes the current instance to create another with the given vertex index
    pub fn with_vertex(self, vertex: VertexId<T>) -> Self {
        Self { vertex, ..self }
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
    pub fn vertex(&self) -> &VertexId<T> {
        &self.vertex
    }
    /// returns a mutable reference to the current vertex index
    pub fn vertex_mut(&mut self) -> &mut VertexId<T> {
        &mut self.vertex
    }
    /// increments the current edge index by one and returns the previous value; see
    /// [`step`](crate::index::IndexBase::step) for more details.
    pub fn next_edge(&mut self) -> IndexResult<EdgeId<T>>
    where
        T: Copy + core::ops::Add<T, Output = T> + num_traits::One,
    {
        self.edge_mut().step()
    }
    /// increments the current vertex index by one and returns the previous value; see
    /// [`step`](crate::index::IndexBase::step) for more details.
    pub fn next_vertex(&mut self) -> IndexResult<VertexId<T>>
    where
        T: Copy + core::ops::Add<T, Output = T> + num_traits::One,
    {
        self.vertex_mut().step()
    }
}
