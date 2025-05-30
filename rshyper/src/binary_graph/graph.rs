/*
    appellation: binary_graph <module>
    authors: @FL03
*/
use alloc::collections::{BTreeMap, BTreeSet};
use rshyper_core::{EdgeId, HyperNode, Position, RawIndex, VertexId};

/// a b-tree based hypergraph implementation
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
pub struct BinaryGraph<N, E, Idx = usize>
where
    Idx: RawIndex + Ord,
{
    pub(crate) connections: BTreeMap<EdgeId<Idx>, BTreeSet<VertexId<Idx>>>,
    pub(crate) facets: BTreeMap<EdgeId<Idx>, E>,
    pub(crate) nodes: BTreeMap<VertexId<Idx>, HyperNode<N, Idx>>,
    pub(crate) position: Position<Idx>,
}

impl<N, E, Idx> BinaryGraph<N, E, Idx>
where
    Idx: Ord + RawIndex,
{
    /// Creates a new empty [`BinaryGraph`] instance
    pub fn new() -> Self
    where
        Idx: Default,
    {
        Self {
            connections: BTreeMap::new(),
            facets: BTreeMap::new(),
            nodes: BTreeMap::new(),
            position: Position::default(),
        }
    }
    /// returns an immutable reference to the connections map
    pub const fn connections(&self) -> &BTreeMap<EdgeId<Idx>, BTreeSet<VertexId<Idx>>> {
        &self.connections
    }
    /// returns a mutable reference to the connections map
    pub const fn connections_mut(&mut self) -> &mut BTreeMap<EdgeId<Idx>, BTreeSet<VertexId<Idx>>> {
        &mut self.connections
    }
    /// returns an immutable reference to the facets map
    pub const fn facets(&self) -> &BTreeMap<EdgeId<Idx>, E> {
        &self.facets
    }
    /// returns a mutable reference to the facets map
    pub const fn facets_mut(&mut self) -> &mut BTreeMap<EdgeId<Idx>, E> {
        &mut self.facets
    }
    /// returns an immutable reference to the nodes of the hypergraph
    pub const fn nodes(&self) -> &BTreeMap<VertexId<Idx>, HyperNode<N, Idx>> {
        &self.nodes
    }
    /// returns a mutable reference to the nodes of the hypergraph
    pub const fn nodes_mut(&mut self) -> &mut BTreeMap<VertexId<Idx>, HyperNode<N, Idx>> {
        &mut self.nodes
    }
    /// returns an immutable reference to the current position in the graph
    pub const fn position(&self) -> &Position<Idx> {
        &self.position
    }
    /// returns a mutable reference to the current position in the graph
    pub fn position_mut(&mut self) -> &mut Position<Idx> {
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
    pub fn next_edge_id(&mut self) -> EdgeId<Idx>
    where
        Idx: Copy + core::ops::Add<Output = Idx> + num_traits::One,
    {
        self.position_mut().next_edge().unwrap()
    }
    /// returns the next vertex index and updates the current position
    pub fn next_vertex_id(&mut self) -> VertexId<Idx>
    where
        Idx: Copy + core::ops::Add<Output = Idx> + num_traits::One,
    {
        self.position_mut().next_vertex().unwrap()
    }
}

impl<N, E, Idx> Default for BinaryGraph<N, E, Idx>
where
    Idx: Default + RawIndex + Ord,
{
    fn default() -> Self {
        Self::new()
    }
}
