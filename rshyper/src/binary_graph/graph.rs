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
    pub(crate) nodes: BTreeMap<VertexId, Node<N>>,
}

impl<N, E> BinaryGraph<N, E> {
    /// Creates a new empty [`BinaryGraph`] instance
    pub fn new() -> Self {
        Self {
            connections: BTreeMap::new(),
            facets: BTreeMap::new(),
            nodes: BTreeMap::new(),
        }
    }
    /// returns an immutable reference to the connections map
    pub const fn connections(&self) -> &BTreeMap<EdgeId, BTreeSet<VertexId>> {
        &self.connections
    }
    /// returns a mutable reference to the connections map
    pub const fn connections_mut(&mut self) -> &mut BTreeMap<EdgeId, BTreeSet<VertexId>> {
        &mut self.connections
    }
    /// returns an immutable reference to the facets map
    pub const fn facets(&self) -> &BTreeMap<EdgeId, E> {
        &self.facets
    }
    /// returns a mutable reference to the facets map
    pub const fn facets_mut(&mut self) -> &mut BTreeMap<EdgeId, E> {
        &mut self.facets
    }
    /// returns an immutable reference to the nodes of the hypergraph
    pub const fn nodes(&self) -> &BTreeMap<VertexId, Node<N>> {
        &self.nodes
    }
    /// returns a mutable reference to the nodes of the hypergraph
    pub const fn nodes_mut(&mut self) -> &mut BTreeMap<VertexId, Node<N>> {
        &mut self.nodes
    }
    /// Returns the number of edges in the graph
    pub fn count_edges(&self) -> usize {
        self.connections.len()
    }
    /// Returns the number of vertices in the graph
    pub fn count_vertices(&self) -> usize {
        self.nodes.len()
    }
}

impl<N, E> Default for BinaryGraph<N, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<N, E> BinaryGraph<N, E> {
    #[deprecated(
        since = "v0.0.4",
        note = "use the creation routines provided by the `EdgeId` instead to ensure uniqueness"
    )]
    pub fn next_edge_id(&self) -> EdgeId {
        EdgeId::atomic()
    }
    #[deprecated(
        since = "v0.0.4",
        note = "use the creation routines provided by the `VertexId` instead to ensure uniqueness"
    )]
    pub fn next_vertex_id(&self) -> VertexId {
        VertexId::atomic()
    }
}
