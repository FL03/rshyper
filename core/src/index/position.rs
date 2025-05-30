/*
    appellation: cursor <module>
    authors: @FL03
*/
use crate::id::{EdgeId, Idx, RawIndex, VertexId};

/// The [`Position`] implementation is uses to track the current indexes of edges and vertices
/// within a hypergraph.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
pub struct Position<T = Idx>
where
    T: RawIndex,
{
    pub(crate) edge: EdgeId<T>,
    pub(crate) vertex: VertexId<T>,
}

impl<T> Position<T>
where
    T: RawIndex,
{
    pub fn new(edge: EdgeId<T>, vertex: VertexId<T>) -> Self {
        Self { edge, vertex }
    }

    pub fn default() -> Self
    where
        T: Default,
    {
        Self {
            edge: EdgeId::default(),
            vertex: VertexId::default(),
        }
    }

    pub fn zero() -> Self
    where
        T: num_traits::Zero,
    {
        Self {
            edge: EdgeId::zero(),
            vertex: VertexId::zero(),
        }
    }

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

    pub fn next_edge(&mut self) -> crate::Result<EdgeId<T>>
    where
        T: Copy + core::ops::Add<T, Output = T> + num_traits::One,
    {
        self.edge_mut().next().ok_or(crate::Error::IndexOutOfBounds)
    }

    pub fn next_vertex(&mut self) -> crate::Result<VertexId<T>>
    where
        T: Copy + core::ops::Add<T, Output = T> + num_traits::One,
    {
        self.vertex_mut()
            .next()
            .ok_or(crate::Error::IndexOutOfBounds)
    }
}
