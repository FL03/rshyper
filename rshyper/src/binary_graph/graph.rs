/*
    appellation: binary_graph <module>
    authors: @FL03
*/
use super::aliases::*;
use crate::{GraphAttributes, GraphKind};
use rshyper_core::attrs::UnAttributes;
use rshyper_core::{EdgeId, IndexCursor, RawIndex, VertexId};

/// a b-tree based hypergraph implementation
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BinaryGraph<N, E, A = UnAttributes<usize>>
where
    A: GraphAttributes,
    A::Idx: Ord,
{
    /// the `nodes` of a hypergraph are the vertices, each identified by a `VertexId` and
    /// associated with a weight of type `N`.
    pub(crate) nodes: NodeBMap<N, A::Idx>,
    /// `surfaces` represent the hyperedges of the hypergraph, each identified by an `EdgeId`
    pub(crate) surfaces: SurfaceBMap<E, A::Kind, A::Idx>,
    /// tracks the current position of the hypergraph, which is used to determine the next
    /// available indices for edges and vertices.
    pub(crate) position: IndexCursor<A::Idx>,
    /// the attributes of a graph define its _kind_ and the type of index used
    pub(crate) _attrs: A,
}

impl<N, E, A, K, Idx> BinaryGraph<N, E, A>
where
    A: GraphAttributes<Idx = Idx, Kind = K>,
    K: GraphKind,
    Idx: Ord + RawIndex,
{
    /// Creates a new empty [`BinaryGraph`] instance
    pub fn new() -> Self
    where
        Idx: Default,
    {
        Self {
            nodes: NodeBMap::new(),
            surfaces: SurfaceBMap::new(),
            position: IndexCursor::default(),
            _attrs: A::new(),
        }
    }
    /// returns an immutable reference to the connections map
    pub const fn surfaces(&self) -> &SurfaceBMap<E, K, Idx> {
        &self.surfaces
    }
    /// returns a mutable reference to the connections map
    pub const fn surfaces_mut(&mut self) -> &mut SurfaceBMap<E, K, Idx> {
        &mut self.surfaces
    }
    /// returns an immutable reference to the nodes of the hypergraph
    pub const fn nodes(&self) -> &NodeBMap<N, Idx> {
        &self.nodes
    }
    /// returns a mutable reference to the nodes of the hypergraph
    pub const fn nodes_mut(&mut self) -> &mut NodeBMap<N, Idx> {
        &mut self.nodes
    }
    /// returns an immutable reference to the current position in the graph
    pub const fn position(&self) -> &IndexCursor<Idx> {
        &self.position
    }
    /// returns a mutable reference to the current position in the graph
    pub const fn position_mut(&mut self) -> &mut IndexCursor<Idx> {
        &mut self.position
    }
    /// Returns the number of edges in the graph
    pub fn total_edges(&self) -> usize {
        self.surfaces.len()
    }
    /// Returns the number of vertices in the graph
    pub fn total_nodes(&self) -> usize {
        self.nodes().len()
    }
    /// get the next edge index and updates the current position
    pub fn next_edge_id(&mut self) -> EdgeId<Idx>
    where
        Idx: crate::AddStep<Output = Idx>,
    {
        self.position_mut().next_edge().unwrap()
    }
    /// returns the next vertex index and updates the current position
    pub fn next_vertex_id(&mut self) -> VertexId<Idx>
    where
        Idx: crate::AddStep<Output = Idx>,
    {
        self.position_mut().next_vertex().unwrap()
    }
}

impl<N, E, A> Default for BinaryGraph<N, E, A>
where
    A: GraphAttributes,
    A::Idx: Default + Ord,
{
    fn default() -> Self {
        Self::new()
    }
}
