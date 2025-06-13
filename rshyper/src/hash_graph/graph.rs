/*
    Appellation: hash_graph <module>
    Contrib: @FL03
*/
use super::aliases::*;

use rshyper_core::attrs::{DiAttributes, GraphAttributes, UnAttributes};
use rshyper_core::index::{EdgeId, IndexCursor, NumIndex, RawIndex, VertexId};
use rshyper_core::node::Node;
use rshyper_core::{GraphKind, HyperGraph, RawHyperGraph, Weight};

use core::hash::{BuildHasher, Hash};
use std::hash::RandomState;

/// a type alias for a [directed](crate::Directed) [`HashGraph`]
pub type DiHashGraph<N, E, Idx = usize, S = RandomState> = HashGraph<N, E, DiAttributes<Idx>, S>;
/// a type alias for an [undirected](crate::Undirected) [`HashGraph`]
pub type UnHashGraph<N, E, Idx = usize, S = RandomState> = HashGraph<N, E, UnAttributes<Idx>, S>;

/// A hash-based hypergraph implementation
#[derive(Clone, Debug, Default)]
pub struct HashGraph<N = (), E = (), A = UnAttributes<usize>, S = RandomState>
where
    A: GraphAttributes,
    A::Idx: Eq + Hash,
    S: BuildHasher,
{
    /// the `nodes` of a hypergraph are the vertices, each identified by a `VertexId` and
    /// associated with a weight of type `N`.
    pub(crate) nodes: NodeMap<N, A::Idx, S>,
    /// `surfaces` represent the hyperedges of the hypergraph, each identified by an `EdgeId`
    pub(crate) surfaces: SurfaceMap<E, A::Kind, A::Idx, S>,
    /// tracks the current position of the hypergraph, which is used to determine the next
    /// available indices for edges and vertices.
    pub(crate) position: IndexCursor<A::Idx>,
    /// the attributes of a graph define its _kind_ and the type of index used
    pub(crate) _attrs: A,
}

impl<N, E, A, K, Idx, S> HashGraph<N, E, A, S>
where
    E: Eq + Hash,
    N: Eq + Hash,
    A: GraphAttributes<Idx = Idx, Kind = K>,
    S: BuildHasher,
    K: GraphKind,
    Idx: Eq + RawIndex + Hash,
{
    /// initialize a new, empty hypergraph
    pub fn new() -> Self
    where
        Idx: Default,
        S: Clone + Default,
    {
        let hasher = S::default();
        HashGraph {
            surfaces: SurfaceMap::with_hasher(hasher.clone()),
            nodes: NodeMap::with_hasher(hasher),
            position: IndexCursor::default(),
            _attrs: A::new(),
        }
    }
    /// creates a new instance of the hypergraph with the given capacity for edges and nodes
    pub fn with_capacity(edges: usize, nodes: usize) -> Self
    where
        Idx: Default,
        S: Clone + Default,
    {
        let hasher = S::default();
        HashGraph {
            surfaces: SurfaceMap::with_capacity_and_hasher(edges, hasher.clone()),
            nodes: NodeMap::with_capacity_and_hasher(nodes, hasher),
            position: IndexCursor::default(),
            _attrs: A::new(),
        }
    }
    /// returns am immutable reference to the nodes
    pub const fn nodes(&self) -> &NodeMap<N, Idx, S> {
        &self.nodes
    }
    /// returns a mutable reference to the nodes of the hypergraph
    pub const fn nodes_mut(&mut self) -> &mut NodeMap<N, Idx, S> {
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
    pub const fn surfaces(&self) -> &SurfaceMap<E, K, Idx, S> {
        &self.surfaces
    }
    /// returns a mutable reference to the surfaces of the hypergraph
    pub const fn surfaces_mut(&mut self) -> &mut SurfaceMap<E, K, Idx, S> {
        &mut self.surfaces
    }
    /// overrides the current nodes and returns a mutable reference to the hypergraph
    #[inline]
    pub fn set_nodes(&mut self, nodes: NodeMap<N, Idx, S>) -> &mut Self
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
    pub fn set_surfaces(&mut self, surfaces: SurfaceMap<E, K, Idx, S>) -> &mut Self
    where
        Idx: Default,
    {
        self.surfaces = surfaces;
        self
    }
    /// consumes the current instance to create another with the given nodes
    #[inline]
    pub fn with_nodes(self, nodes: NodeMap<N, Idx, S>) -> Self
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
    pub fn with_surfaces(self, surfaces: SurfaceMap<E, K, Idx, S>) -> Self
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
    /// returns true if the vertex is contained in the hyperedge with the given id
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "is_vertex_in_edge", target = "hash_graph")
    )]
    pub fn contains_node_in_edge<Q, Q2>(&self, index: &Q, vertex: &Q2) -> bool
    where
        Q: Eq + Hash,
        Q2: Eq + Hash,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
        VertexId<Idx>: core::borrow::Borrow<Q2>,
    {
        if let Some(surface) = self.surfaces().get(index) {
            return surface.contains(vertex);
        }
        false
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
    pub fn surface(&mut self, index: EdgeId<Idx>) -> SurfaceEntry<'_, E, K, Idx, S> {
        self.surfaces_mut().entry(index)
    }
    /// returns an iterator over the nodes of the hypergraph, yielding pairs of [`VertexId`] and
    /// the corresponding [`HyperNode`].
    pub fn node_iter(&self) -> super::NodeIter<'_, N, Idx> {
        super::NodeIter {
            iter: self.nodes().iter(),
        }
    }
    /// returns a mutable iterator over the nodes of the hypergraph, yielding pairs of
    /// [`VertexId`] and a mutable reference to the corresponding [`HyperNode`].
    pub fn node_iter_mut(&mut self) -> super::NodeIterMut<'_, N, Idx> {
        super::NodeIterMut {
            iter: self.nodes_mut().iter_mut(),
        }
    }
    /// returns an iterator over the surfaces of the hypergraph, yielding pairs of [`EdgeId`]
    /// and the corresponding [`HashFacet`].
    pub fn surface_iter(&self) -> super::SurfaceIter<'_, E, K, Idx, S> {
        super::SurfaceIter {
            iter: self.surfaces().iter(),
        }
    }
    /// returns a mutable iterator over the surfaces of the hypergraph, yielding pairs of
    /// [`EdgeId`] and a mutable reference to the corresponding [`HashFacet`].
    pub fn surface_iter_mut(&mut self) -> super::SurfaceIterMut<'_, E, K, Idx, S> {
        super::SurfaceIterMut {
            iter: self.surfaces_mut().iter_mut(),
        }
    }
    /// returns a parallel iterator over the nodes of the hypergraph, yielding pairs of
    /// [`VertexId`] and the corresponding [`HyperNode`].
    #[cfg(feature = "rayon")]
    pub fn node_par_iter(&self) -> super::NodeParIter<'_, N, Idx>
    where
        N: Send + Sync,
        Idx: Send + Sync,
    {
        super::NodeParIter {
            iter: self.node_iter(),
        }
    }
    /// returns a mutable parallel iterator over the nodes of the hypergraph, yielding pairs of
    /// [`VertexId`] and a mutable reference to the corresponding [`HyperNode`].
    #[cfg(feature = "rayon")]
    pub fn node_par_iter_mut(&mut self) -> super::NodeParIterMut<'_, N, Idx>
    where
        N: Send + Sync,
        Idx: Send + Sync,
    {
        super::NodeParIterMut {
            iter: self.node_iter_mut(),
        }
    }
    /// returns a parallel iterator over the surfaces of the hypergraph, yielding pairs of
    /// [`EdgeId`] and the corresponding [`HashFacet`].
    #[cfg(feature = "rayon")]
    pub fn surface_par_iter(&self) -> super::SurfaceParIter<'_, E, K, Idx, S>
    where
        E: Send + Sync,
        K: Send + Sync,
        Idx: Send + Sync,
        S: Send + Sync,
    {
        super::SurfaceParIter {
            iter: self.surface_iter(),
        }
    }
    /// returns a mutable parallel iterator over the surfaces of the hypergraph, yielding pairs of
    /// [`EdgeId`] and a mutable reference to the corresponding [`HashFacet`].
    #[cfg(feature = "rayon")]
    pub fn surface_par_iter_mut(&mut self) -> super::SurfaceParIterMut<'_, E, K, Idx, S>
    where
        E: Send + Sync,
        K: Send + Sync,
        Idx: Send + Sync,
        S: Send + Sync,
    {
        super::SurfaceParIterMut {
            iter: self.surface_iter_mut(),
        }
    }
    /// computes the next edge index before replacing and returning the previous value
    pub fn next_edge_id(&mut self) -> EdgeId<Idx>
    where
        Idx: Copy + core::ops::Add<Output = Idx> + num_traits::One,
    {
        self.position_mut().next_edge().unwrap()
    }
    /// computes the next node index before replacing and returning the previous value
    pub fn next_vertex_id(&mut self) -> VertexId<Idx>
    where
        Idx: Copy + core::ops::Add<Output = Idx> + num_traits::One,
    {
        self.position_mut().next_vertex().unwrap()
    }
    /// returns the total number of edges within the hypergraph
    pub fn total_edges(&self) -> usize {
        self.surfaces().len()
    }
    /// returns the total number of nodes within the hypergraph
    pub fn total_nodes(&self) -> usize {
        self.nodes().len()
    }
}

impl<N, E, A, K, Idx, S> core::fmt::Display for HashGraph<N, E, A, S>
where
    E: core::fmt::Debug + Eq + Hash,
    N: core::fmt::Debug + Eq + Hash,
    A: GraphAttributes<Idx = Idx, Kind = K>,
    S: BuildHasher,
    K: GraphKind,
    Idx: NumIndex,
    S: BuildHasher,
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

impl<N, E, A, K, Idx, S> RawHyperGraph<A> for HashGraph<N, E, A, S>
where
    A: GraphAttributes<Idx = Idx, Kind = K>,
    E: Eq + Hash,
    N: Eq + Hash,
    K: GraphKind,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    type Node<N2> = Node<N2, Idx>;
    type Edge<E2> = HashFacet<E2, K, Idx, S>;
}

impl<N, E, A, K, Idx, S> HyperGraph<N, E, A> for HashGraph<N, E, A, S>
where
    E: Eq + Hash,
    N: Eq + Hash,
    A: GraphAttributes<Idx = Idx, Kind = K>,
    K: GraphKind,
    Idx: NumIndex,
    S: BuildHasher + Default,
{
    fn add_node(&mut self, weight: Weight<N>) -> crate::Result<VertexId<Idx>> {
        self.add_node(weight)
    }

    fn add_surface<I>(&mut self, iter: I, weight: Weight<E>) -> crate::Result<EdgeId<Idx>>
    where
        I: IntoIterator<Item = VertexId<Idx>>,
    {
        self.add_surface(iter, weight)
    }

    fn get_edge_vertices(&self, index: &EdgeId<Idx>) -> crate::Result<&VertexSet<Idx, S>> {
        self.get_edge_vertices(index)
    }

    fn get_edge_vertices_mut(
        &mut self,
        index: &EdgeId<Idx>,
    ) -> crate::Result<&mut VertexSet<Idx, S>> {
        self.get_edge_vertices_mut(index)
    }

    fn get_edge_weight(&self, index: &EdgeId<Idx>) -> crate::Result<&Weight<E>> {
        self.get_edge_weight(index)
    }

    fn get_edge_weight_mut(&mut self, index: &EdgeId<Idx>) -> crate::Result<&mut Weight<E>> {
        self.get_edge_weight_mut(index)
    }

    fn get_node(&self, index: &VertexId<Idx>) -> crate::Result<&Node<N, Idx>> {
        self.get_node(index)
    }

    fn get_node_mut(&mut self, index: &VertexId<Idx>) -> crate::Result<&mut Node<N, Idx>> {
        self.get_node_mut(index)
    }

    fn get_surface(&self, index: &EdgeId<Idx>) -> crate::Result<&HashFacet<E, K, Idx, S>> {
        self.get_surface(index)
    }

    fn get_surface_mut(
        &mut self,
        index: &EdgeId<Idx>,
    ) -> crate::Result<&mut HashFacet<E, K, Idx, S>> {
        self.get_surface_mut(index)
    }

    fn contains_edge(&self, index: &EdgeId<Idx>) -> bool {
        self.contains_surface(index)
    }

    fn contains_node(&self, index: &VertexId<Idx>) -> bool {
        self.contains_node(index)
    }

    fn find_edges_with_node(
        &self,
        index: &VertexId<Idx>,
    ) -> crate::Result<impl Iterator<Item = EdgeId<Idx>>> {
        match self.find_edges_with_node(index) {
            Ok(edges) => Ok(edges.into_iter()),
            Err(e) => Err(e),
        }
    }
}
