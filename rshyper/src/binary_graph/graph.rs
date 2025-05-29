/*
    Appellation: graph <module>
    Contrib: @FL03
*/
use alloc::collections::{BTreeMap, BTreeSet};
use rshyper_core::{EdgeId, Node, VertexId, id::Position};

/// a b-tree based hypergraph implementation
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct BinaryGraph<N, E> {
    pub(crate) connections: BTreeMap<EdgeId, BTreeSet<VertexId>>,
    pub(crate) facets: BTreeMap<EdgeId, E>,
    pub(crate) nodes: BTreeMap<VertexId, Node<N>>,
    pub(crate) position: Position<usize>,
}

impl<N, E> BinaryGraph<N, E> {
    /// Creates a new empty [`BinaryGraph`] instance
    pub fn new() -> Self {
        Self {
            connections: BTreeMap::new(),
            facets: BTreeMap::new(),
            nodes: BTreeMap::new(),
            position: Position::default(),
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

    pub const fn position(&self) -> Position {
        self.position
    }

    pub fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
    /// Returns the number of edges in the graph
    pub fn count_edges(&self) -> usize {
        self.connections.len()
    }
    /// Returns the number of vertices in the graph
    pub fn count_vertices(&self) -> usize {
        self.nodes.len()
    }
    /// get the next edge index and updates the current position
    pub fn next_edge_id(&mut self) -> EdgeId {
        self.position_mut().next_edge().unwrap()
    }
    /// returns the next vertex index and updates the current position
    pub fn next_vertex_id(&mut self) -> VertexId {
        self.position_mut().next_vertex().unwrap()
    }
}

impl<N, E> Default for BinaryGraph<N, E> {
    fn default() -> Self {
        Self::new()
    }
}
