/*
    Appellation: graph <module>
    Contrib: @FL03
*/
use alloc::collections::{BTreeMap, BTreeSet};
use rshyper_core::{EdgeId, Node, VertexId};

/// a b-tree based hypergraph implementation
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct BinaryGraph<N, E> {
    pub(crate) connections: BTreeMap<EdgeId, BTreeSet<VertexId>>,
    pub(crate) facets: BTreeMap<EdgeId, E>,
    pub(crate) vertices: BTreeMap<VertexId, Node<N>>,
    pub(crate) next_vertex_id: VertexId,
    pub(crate) next_edge_id: EdgeId,
}

impl<N, E> BinaryGraph<N, E> {
    /// Creates a new empty [`BinaryGraph`] instance
    pub fn new() -> Self {
        Self {
            connections: BTreeMap::new(),
            facets: BTreeMap::new(),
            vertices: BTreeMap::new(),
            next_vertex_id: VertexId::atomic(),
            next_edge_id: EdgeId::atomic(),
        }
    }

    pub const fn connections(&self) -> &BTreeMap<EdgeId, BTreeSet<VertexId>> {
        &self.connections
    }

    pub const fn connections_mut(&mut self) -> &mut BTreeMap<EdgeId, BTreeSet<VertexId>> {
        &mut self.connections
    }

    pub const fn facets(&self) -> &BTreeMap<EdgeId, E> {
        &self.facets
    }

    pub const fn facets_mut(&mut self) -> &mut BTreeMap<EdgeId, E> {
        &mut self.facets
    }

    pub const fn vertices(&self) -> &BTreeMap<VertexId, Node<N>> {
        &self.vertices
    }

    pub const fn vertices_mut(&mut self) -> &mut BTreeMap<VertexId, Node<N>> {
        &mut self.vertices
    }

    pub const fn next_vertex_id(&self) -> VertexId {
        self.next_vertex_id
    }

    /// Returns the number of vertices in the graph
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Returns the number of edges in the graph
    pub fn edge_count(&self) -> usize {
        self.connections.len()
    }
}
