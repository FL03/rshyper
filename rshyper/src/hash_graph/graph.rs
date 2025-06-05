/*
    Appellation: hash_graph <module>
    Contrib: @FL03
*/
use super::aliases::*;

use rshyper_core::attrs::{DirectedAttributes, GraphAttributes, UndirectedAttributes};
use rshyper_core::index::{EdgeId, IndexCursor, NumIndex, RawIndex, VertexId};
use rshyper_core::node::HyperNode;
use rshyper_core::{GraphKind, HyperGraph, RawHyperGraph, Weight};

use core::hash::Hash;

/// a type alias for a [directed](crate::Directed) [`HashGraph`]
pub type DirectedHashGraph<N, E, Idx = usize> = HashGraph<N, E, DirectedAttributes<Idx>>;
/// a type alias for an [undirected](crate::Undirected) [`HashGraph`]
pub type UndirectedHashGraph<N, E, Idx = usize> = HashGraph<N, E, UndirectedAttributes<Idx>>;

/// A hash-based hypergraph implementation
#[derive(Clone, Debug, Default)]
pub struct HashGraph<N = (), E = (), A = UndirectedAttributes<usize>>
where
    A: GraphAttributes,
    A::Idx: Eq + Hash,
{
    /// the `nodes` of a hypergraph are the vertices, each identified by a `VertexId` and
    /// associated with a weight of type `N`.
    pub(crate) nodes: NodeMap<N, A::Idx>,
    /// `surfaces` represent the hyperedges of the hypergraph, each identified by an `EdgeId`
    pub(crate) surfaces: SurfaceMap<E, A::Kind, A::Idx>,
    /// tracks the current position of the hypergraph, which is used to determine the next
    /// available indices for edges and vertices.
    pub(crate) position: IndexCursor<A::Idx>,
    /// the attributes of a graph define its _kind_ and the type of index used
    pub(crate) _attrs: A,
}

impl<N, E, K, Idx, A> HashGraph<N, E, A>
where
    E: Eq + Hash,
    N: Eq + Hash,
    A: GraphAttributes<Idx = Idx, Kind = K>,
    K: GraphKind,
    Idx: Eq + RawIndex + Hash,
{
    /// initialize a new, empty hypergraph
    pub fn new() -> Self
    where
        Idx: Default,
    {
        HashGraph {
            surfaces: SurfaceMap::new(),
            nodes: NodeMap::new(),
            position: IndexCursor::default(),
            _attrs: A::new(),
        }
    }
    /// creates a new instance of the hypergraph with the given capacity for edges and nodes
    pub fn with_capacity(edges: usize, nodes: usize) -> Self
    where
        Idx: Default,
    {
        HashGraph {
            surfaces: SurfaceMap::with_capacity(edges),
            nodes: NodeMap::with_capacity(nodes),
            position: IndexCursor::default(),
            _attrs: A::new(),
        }
    }
    /// returns am immutable reference to the nodes
    pub const fn nodes(&self) -> &NodeMap<N, Idx> {
        &self.nodes
    }
    /// returns a mutable reference to the nodes of the hypergraph
    pub const fn nodes_mut(&mut self) -> &mut NodeMap<N, Idx> {
        &mut self.nodes
    }
    /// returns a copy of the position of the hypergraph; here, the [`position`](Position) is
    /// used to track the indices (edge & vertex) and define which ones are next to be used
    /// when inserting new hyperedges or vertices
    pub const fn position(&self) -> &IndexCursor<Idx> {
        &self.position
    }
    /// returns a mutable reference to the current position of the hypergraph;
    pub fn position_mut(&mut self) -> &mut IndexCursor<Idx> {
        &mut self.position
    }
    /// returns an immutable reference to the surfaces of the hypergraph
    pub const fn surfaces(&self) -> &SurfaceMap<E, K, Idx> {
        &self.surfaces
    }
    /// returns a mutable reference to the surfaces of the hypergraph
    pub const fn surfaces_mut(&mut self) -> &mut SurfaceMap<E, K, Idx> {
        &mut self.surfaces
    }
    /// overrides the current nodes and returns a mutable reference to the hypergraph
    #[inline]
    pub fn set_nodes(&mut self, nodes: NodeMap<N, Idx>) -> &mut Self
    where
        Idx: Default,
    {
        self.nodes = nodes;
        self
    }
    /// overrides the current position and returns a mutable reference to the hypergraph
    #[inline]
    pub fn set_position(&mut self, position: IndexCursor<Idx>) -> &mut Self
    where
        Idx: Default,
    {
        self.position = position;
        self
    }
    #[inline]
    /// overrides the current surfaces and returns a mutable reference to the hypergraph
    pub fn set_surfaces(&mut self, surfaces: SurfaceMap<E, K, Idx>) -> &mut Self
    where
        Idx: Default,
    {
        self.surfaces = surfaces;
        self
    }
    /// consumes the current instance to create another with the given nodes
    #[inline]
    pub fn with_nodes(self, nodes: NodeMap<N, Idx>) -> Self
    where
        Idx: Default,
    {
        Self { nodes, ..self }
    }
    /// consumes the current instance to create another with the given position
    pub fn with_position(self, position: IndexCursor<Idx>) -> Self
    where
        Idx: Default,
    {
        Self { position, ..self }
    }
    /// consumes the current instance to create another with the given edges
    #[inline]
    pub fn with_surfaces(self, surfaces: SurfaceMap<E, K, Idx>) -> Self
    where
        Idx: Default,
    {
        Self { surfaces, ..self }
    }
    /// check if a vertex with the given id exists
    pub fn contains_node<Q>(&self, index: &Q) -> bool
    where
        Q: Eq + Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.nodes().contains_key(index)
    }
    /// returns true if the hypergraph contains an edge with the given index;
    pub fn contains_surface<Q>(&self, index: &Q) -> bool
    where
        Q: Eq + Hash,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.surfaces().contains_key(index)
    }
    /// returns true if the hypergraph is empty, meaning it has no edges, facets, or nodes
    pub fn is_empty(&self) -> bool {
        self.surfaces().is_empty() && self.nodes().is_empty()
    }
    /// returns true if the hypergraph is directed;
    pub fn is_directed(&self) -> bool {
        use core::any::TypeId;
        TypeId::of::<K>() == TypeId::of::<crate::Directed>()
    }
    /// returns true if the hypergraph is undirected;
    pub fn is_undirected(&self) -> bool {
        use core::any::TypeId;
        TypeId::of::<K>() == TypeId::of::<crate::Undirected>()
    }
    /// returns an [`Entry`](std::collections::hash_map::Entry) for the node with the given
    /// index, allowing for modifications or insertions to the mapping
    pub fn node(&mut self, index: VertexId<Idx>) -> NodeEntry<'_, N, Idx> {
        self.nodes_mut().entry(index)
    }
    /// returns a [`SurfaceEntry`] for the surface with the given index, allowing for in-place
    /// mutations to the value associated with the index
    pub fn surface(&mut self, index: EdgeId<Idx>) -> SurfaceEntry<'_, E, K, Idx> {
        self.surfaces_mut().entry(index)
    }
    /// returns an iterator over the nodes of the hypergraph, yielding pairs of [`VertexId`] and
    /// the corresponding [`HyperNode`].
    pub fn node_iter(&self) -> super::iter::NodeIter<'_, N, Idx> {
        super::iter::NodeIter {
            iter: self.nodes().iter(),
        }
    }
    /// returns an iterator over the surfaces of the hypergraph, yielding pairs of [`EdgeId`]
    /// and the corresponding [`HashFacet`].
    pub fn surface_iter(&self) -> super::iter::SurfaceIter<'_, E, K, Idx> {
        super::iter::SurfaceIter {
            iter: self.surfaces().iter(),
        }
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
    /// returns the total number of hyperedges in the hypergraph
    pub fn total_edges(&self) -> usize {
        self.surfaces().len()
    }
    /// returns the total number of vertices in the hypergraph
    pub fn total_vertices(&self) -> usize {
        self.nodes().len()
    }
}

impl<N, E, A, K, Idx> core::fmt::Display for HashGraph<N, E, A>
where
    E: core::fmt::Debug + Eq + Hash,
    N: core::fmt::Debug + Eq + Hash,
    A: GraphAttributes<Idx = Idx, Kind = K>,
    K: GraphKind,
    Idx: NumIndex,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{ nodes: {n:?}, edges: {e:?} }}",
            n = self.nodes(),
            e = self.surfaces()
        )
    }
}

impl<N, E, K, Idx, A> RawHyperGraph<N, E> for HashGraph<N, E, A>
where
    E: Eq + Hash,
    N: Eq + Hash,
    A: GraphAttributes<Idx = Idx, Kind = K>,
    K: GraphKind,
    Idx: Eq + RawIndex + Hash,
{
    type Idx = Idx;
    type Kind = K;
}

impl<N, E, K, Idx, A> HyperGraph<N, E> for HashGraph<N, E, A>
where
    E: Eq + Hash,
    N: Eq + Hash,
    A: GraphAttributes<Idx = Idx, Kind = K>,
    K: GraphKind,
    Idx: NumIndex,
{
    fn add_surface<I>(&mut self, iter: I, weight: Weight<E>) -> crate::Result<EdgeId<Idx>>
    where
        I: IntoIterator<Item = VertexId<Self::Idx>>,
    {
        self.add_surface(iter, weight)
    }

    fn get_edge_vertices<S>(&self, index: &EdgeId<Idx>) -> crate::Result<S>
    where
        for<'a> S: core::iter::FromIterator<&'a VertexId<Self::Idx>>,
    {
        self.get_edge_vertices(index)
            .map(|v| v.iter().collect::<S>())
    }

    fn add_node(&mut self, weight: N) -> crate::Result<VertexId<Idx>> {
        self.add_node(weight)
    }

    fn get_node(&self, index: &VertexId<Idx>) -> crate::Result<&HyperNode<N, Idx>> {
        self.get_node(index)
    }

    fn get_facet(&self, index: &EdgeId<Idx>) -> crate::Result<&Weight<E>> {
        self.get_surface(index).map(|s| s.weight())
    }

    fn contains_edge(&self, index: &EdgeId<Idx>) -> bool {
        self.contains_surface(index)
    }

    fn contains_node(&self, index: &VertexId<Idx>) -> bool {
        self.contains_node(index)
    }
}
