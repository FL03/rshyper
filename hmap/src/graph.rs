/*
    Appellation: hash_graph <module>
    Contrib: @FL03
*/
/// the [`graph`] module is responsible for defining the [`HyperMap`] structure and provides
/// its root implementation(s).
use crate::types::prelude::*;

use core::hash::{BuildHasher, Hash};
use rshyper_core::{
    AddStep, GraphType, Mode,
    attrs::{DiAttrs, GraphProps, UnAttrs},
    idx::{EdgeId, Frame, IndexTracker, RawIndex, VertexId},
};
use std::hash::RandomState;

/// a type alias for a [directed](crate::Directed) [`HyperMap`]
pub type DiHyperMap<N, E, Idx = usize, S = RandomState> = HyperMap<N, E, DiAttrs<Idx>, S>;
/// a type alias for an [undirected](crate::Undirected) [`HyperMap`]
pub type UnHyperMap<N, E, Idx = usize, S = RandomState> = HyperMap<N, E, UnAttrs<Idx>, S>;

/// The [`HyperMap`] is a map-based implementation of a hypergraph that is generic over the
/// types:
///
/// - `N`: the weight of the nodes (vertices)
/// - `E`: the weight of the edges (surfaces)
/// - `A`: the attributes of the hypergraph, which define its kind and index type
///   - `A::Ix`: the index type used for vertices and edges, which must implement the
///     [`RawIndex`] trait
///   - `A::Kind`: the kind of the hypergraph, which must implement the [`GraphType`] trait
/// - `S`: the hasher used for hashing the nodes and edges
///
/// The generic design enables the graph to be used in various contexts and conditions while
/// retaining a familiar interface for users. This implementation focuses on performance and
/// flexibility, leveraging an internal history to maintain a sense of order and enable
/// sequential iterators over the components of the graph w.r.t. the order in which they were
/// created.
///
#[derive(Clone, Default)]
pub struct HyperMap<N = (), E = (), A = UnAttrs<usize>, S = RandomState>
where
    S: BuildHasher,
    A: GraphProps,
{
    /// the attributes of a graph define its _kind_ and the type of index used
    pub(crate) attrs: A,
    /// tracks the current position of the hypergraph, which is used to determine the next
    /// available indices for edges and vertices.
    pub(crate) history: IndexTracker<A::Ix>,
    /// the `nodes` of a hypergraph are the vertices, each identified by a `VertexId` and
    /// associated with a weight of type `N`.
    pub(crate) nodes: NodeMap<N, A::Ix, S>,
    /// `edges` represent the hyperedges of the hypergraph, each identified by an `EdgeId`
    pub(crate) edges: SurfaceMap<E, A::Kind, A::Ix, S>,
}

impl<N, E, A, K, Idx, S> HyperMap<N, E, A, S>
where
    S: BuildHasher,
    A: GraphProps<Ix = Idx, Kind = K>,
    K: GraphType,
    Idx: RawIndex,
{
    /// initialize a new, empty hypergraph
    pub fn new() -> Self
    where
        Idx: Default,
        S: Clone + Default,
    {
        let hasher = S::default();
        HyperMap {
            attrs: A::new(),
            history: IndexTracker::new(),
            edges: SurfaceMap::with_hasher(hasher.clone()),
            nodes: NodeMap::with_hasher(hasher),
        }
    }
    /// creates a new instance of the hypergraph with the given capacity for edges and nodes
    pub fn with_capacity(edges: usize, nodes: usize) -> Self
    where
        Idx: Default,
        S: Clone + Default,
    {
        let hasher = S::default();
        HyperMap {
            edges: SurfaceMap::with_capacity_and_hasher(edges, hasher.clone()),
            nodes: NodeMap::with_capacity_and_hasher(nodes, hasher),
            history: IndexTracker::new(),
            attrs: A::new(),
        }
    }
    /// returns a copy of the graph attributes; almost never used, however, it is useful for
    /// extracting certain truths about the hypergraph.
    pub(crate) const fn attrs(&self) -> A {
        self.attrs
    }
    /// returns the [`Mode`] of the hypergraph
    pub fn mode(&self) -> Mode {
        self.attrs().mode()
    }
    /// returns am immutable reference to the nodes
    pub const fn nodes(&self) -> &NodeMap<N, Idx, S> {
        &self.nodes
    }
    /// returns a mutable reference to the nodes of the hypergraph
    pub const fn nodes_mut(&mut self) -> &mut NodeMap<N, Idx, S> {
        &mut self.nodes
    }
    /// returns an immutable reference to the history of the hypergraph, which is used to track
    /// the indices of edges and vertices that have been created so far.
    pub const fn history(&self) -> &IndexTracker<Idx> {
        &self.history
    }
    /// returns a mutable reference to the history of the hypergraph, which is used to track
    /// the indices of edges and vertices that have been created so far.
    pub const fn history_mut(&mut self) -> &mut IndexTracker<Idx> {
        &mut self.history
    }
    /// returns a copy of the position of the hypergraph; here, the [`position`](Position) is
    /// used to track the indices (edge & vertex) and define which ones are next to be used
    /// when inserting new hyperedges or vertices
    pub const fn position(&self) -> &Frame<Idx> {
        self.history().cursor()
    }
    /// returns a mutable reference to the current position of the hypergraph;
    pub const fn position_mut(&mut self) -> &mut Frame<Idx> {
        self.history_mut().cursor_mut()
    }
    /// returns an immutable reference to the surfaces of the hypergraph
    pub const fn edges(&self) -> &SurfaceMap<E, K, Idx, S> {
        &self.edges
    }
    /// returns a mutable reference to the surfaces of the hypergraph
    pub const fn edges_mut(&mut self) -> &mut SurfaceMap<E, K, Idx, S> {
        &mut self.edges
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
    /// overrides the current history and returns a mutable reference to the hypergraph
    #[inline]
    pub fn set_history(&mut self, history: IndexTracker<Idx>) -> &mut Self
    where
        Idx: Default,
    {
        *self.history_mut() = history;
        self
    }
    /// overrides the current position and returns a mutable reference to the hypergraph
    #[inline]
    pub fn set_position(&mut self, position: Frame<Idx>) -> &mut Self
    where
        Idx: Default,
    {
        self.history_mut().set_cursor(position);
        self
    }
    #[inline]
    /// overrides the current surfaces and returns a mutable reference to the hypergraph
    pub fn set_surfaces(&mut self, surfaces: SurfaceMap<E, K, Idx, S>) -> &mut Self
    where
        Idx: Default,
    {
        self.edges = surfaces;
        self
    }
    /// returns true if the hypergraph contains an edge with the given index;
    pub fn contains_edge<Q>(&self, index: &Q) -> bool
    where
        Idx: Eq + Hash,
        Q: Eq + Hash + ?Sized,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.edges().contains_key(index)
    }
    /// check if a vertex with the given id exists
    pub fn contains_node<Q>(&self, index: &Q) -> bool
    where
        Idx: Eq + Hash,
        Q: Eq + Hash + ?Sized,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.nodes().contains_key(index)
    }
    /// returns true if the vertex is contained in the hyperedge with the given id
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, target = "hash_graph")
    )]
    pub fn is_node_in_domain<Q, Q2>(&self, index: &Q, vertex: &Q2) -> bool
    where
        Idx: Eq + Hash,
        Q: Eq + Hash + ?Sized,
        Q2: Eq + Hash,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
        VertexId<Idx>: core::borrow::Borrow<Q2>,
    {
        if let Some(surface) = self.edges().get(index) {
            return surface.contains(vertex);
        }
        false
    }
    /// returns true if the hypergraph is empty, meaning it has no edges, facets, or nodes
    pub fn is_empty(&self) -> bool {
        self.edges().is_empty() && self.nodes().is_empty()
    }
    /// returns true if the hypergraph is directed;
    pub fn is_directed(&self) -> bool {
        self.attrs().is_directed()
    }
    /// returns true if the hypergraph is undirected;
    pub fn is_undirected(&self) -> bool {
        self.attrs().is_undirected()
    }
    /// returns an [`Entry`](std::collections::hash_map::Entry) for the node with the given
    /// index, allowing for modifications or insertions to the mapping
    pub fn node(&mut self, index: VertexId<Idx>) -> NodeEntry<'_, N, Idx>
    where
        Idx: Eq + Hash,
    {
        self.nodes_mut().entry(index)
    }
    /// returns a [`SurfaceEntry`] for the surface with the given index, allowing for in-place
    /// mutations to the value associated with the index
    pub fn surface(&mut self, index: EdgeId<Idx>) -> SurfaceEntry<'_, E, K, Idx, S>
    where
        Idx: Eq + Hash,
    {
        self.edges_mut().entry(index)
    }
    /// computes the next edge index before replacing and returning the previous value
    pub fn next_edge_id(&mut self) -> EdgeId<Idx>
    where
        Idx: AddStep<Output = Idx> + Clone + PartialEq,
    {
        self.history_mut().next_edge().unwrap()
    }
    /// computes the next node index before replacing and returning the previous value
    pub fn next_vertex_id(&mut self) -> VertexId<Idx>
    where
        Idx: AddStep<Output = Idx> + Clone + PartialEq,
    {
        self.history_mut().next_vertex().unwrap()
    }
    /// returns the order of the hypergraph, which is defined to be the number of nodes in `X`
    /// where `H=(X,E)`.
    pub fn order(&self) -> usize {
        self.nodes().len()
    }
    /// returns the size of the hypergraph, which is defined to be the number of edges in `E`
    /// where `H=(X,E)`.
    /// returns the total number of edges within the hypergraph
    pub fn size(&self) -> usize {
        self.edges().len()
    }
}

impl<N, E, A, S> core::fmt::Debug for HyperMap<N, E, A, S>
where
    A: GraphProps,
    E: core::fmt::Debug,
    N: core::fmt::Debug,
    S: BuildHasher,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("HyperMap")
            .field("history", self.history())
            .field("nodes", self.nodes())
            .field("surfaces", self.edges())
            .finish()
    }
}

impl<N, E, A, S> core::fmt::Display for HyperMap<N, E, A, S>
where
    A: GraphProps,
    E: core::fmt::Debug,
    N: core::fmt::Debug,
    S: BuildHasher,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{ history: {h:?}, edges: {e:?}, nodes: {n:?} }}",
            n = self.nodes(),
            e = self.edges(),
            h = self.history()
        )
    }
}
