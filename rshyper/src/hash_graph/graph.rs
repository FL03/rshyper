/*
    Appellation: hash_graph <module>
    Contrib: @FL03
*/
use super::aliases::*;

use rshyper_core::GraphKind;
use rshyper_core::index::{EdgeId, IndexCursor, RawIndex, VertexId};

pub type DirectedHashGraph<N, E, Idx = usize> = HashGraph<N, E, crate::Directed, Idx>;
/// a t
pub type UndirectedHashGraph<N, E, Idx = usize> = HashGraph<N, E, crate::Undirected, Idx>;

/// A hash-based hypergraph implementation
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct HashGraph<N, E, K, Idx = usize>
where
    Idx: Eq + RawIndex + core::hash::Hash,
    K: GraphKind,
{
    /// the `edges` of a hypergraph is a map associating hyperedges (identified by `EdgeId`) to
    /// sets of vertices (identified by `VertexId`).
    pub(crate) edges: EdgeMap<Idx>,
    /// the `facets` of a hypergraph materializes hyperedges by associating them with a weight
    pub(crate) facets: FacetMap<Idx, E>,
    /// the `nodes` of a hypergraph are the vertices, each identified by a `VertexId` and
    /// associated with a weight of type `N`.
    pub(crate) nodes: NodeMap<N, Idx>,
    /// tracks the current position of the hypergraph, which is used to determine the next
    /// available indices for edges and vertices.
    pub(crate) position: IndexCursor<Idx>,
    /// the kind of the hypergraph, which can be either directed or undirected
    #[cfg_attr(feature = "serde", serde(skip))]
    pub(crate) _kind: core::marker::PhantomData<K>,
}

impl<N, E, K, Idx> HashGraph<N, E, K, Idx>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    K: GraphKind,
    Idx: Eq + RawIndex + core::hash::Hash,
{
    /// initialize a new, empty hypergraph
    pub fn new() -> Self
    where
        Idx: Default,
    {
        HashGraph {
            facets: FacetMap::new(),
            edges: EdgeMap::new(),
            nodes: NodeMap::new(),
            position: IndexCursor::default(),
            _kind: core::marker::PhantomData::<K>,
        }
    }
    /// creates a new instance of the hypergraph with the given capacity for edges and nodes
    pub fn with_capacity(edges: usize, nodes: usize) -> Self
    where
        Idx: Default,
    {
        HashGraph {
            facets: FacetMap::with_capacity(edges),
            edges: EdgeMap::with_capacity(edges),
            nodes: NodeMap::with_capacity(nodes),
            position: IndexCursor::default(),
            _kind: core::marker::PhantomData::<K>,
        }
    }
    /// returns an immutable reference to the edges of the hypergraph; a mapping of edges to vertices essentially forming a topological space
    /// that enables the data-structure to be traversed, analyzed, and manipulated.
    pub const fn edges(&self) -> &EdgeMap<Idx> {
        &self.edges
    }
    /// returns a mutable reference to the hyperedges
    pub const fn edges_mut(&mut self) -> &mut EdgeMap<Idx> {
        &mut self.edges
    }
    /// returns an immutable reference to the facets of the hypergraph; here, a facet is a
    /// hyperedge with an associated weight
    pub const fn facets(&self) -> &FacetMap<Idx, E> {
        &self.facets
    }
    /// returns a mutable reference to the edges, or facets, of the hypergraph
    pub const fn facets_mut(&mut self) -> &mut FacetMap<Idx, E> {
        &mut self.facets
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
    /// overrides the current edges and returns a mutable reference to the hypergraph
    #[inline]
    pub fn set_edges(&mut self, edges: EdgeMap<Idx>) -> &mut Self
    where
        Idx: Default,
    {
        self.edges = edges;
        self
    }
    /// overrides the current facets and returns a mutable reference to the hypergraph
    #[inline]
    pub fn set_facets(&mut self, facets: FacetMap<Idx, E>) -> &mut Self
    where
        Idx: Default,
    {
        self.facets = facets;
        self
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
    /// consumes the current instance to create another with the given edges
    #[inline]
    pub fn with_edges(self, edges: EdgeMap<Idx>) -> Self
    where
        Idx: Default,
    {
        Self { edges, ..self }
    }
    /// consumes the current instance to create another with the given facets
    #[inline]
    pub fn with_facets(self, facets: FacetMap<Idx, E>) -> Self
    where
        Idx: Default,
    {
        Self { facets, ..self }
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
    /// returns true if the hypergraph contains an edge with the given index;
    pub fn contains_edge<Q>(&self, index: &Q) -> bool
    where
        Q: Eq + core::hash::Hash,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.edges().contains_key(index)
    }
    /// check if a facet with the given id exists; this method is a little heavier since it
    /// checks both the facets and edges fields to ensure the index points to a valid facet.
    pub fn contains_facet<Q>(&self, index: &Q) -> bool
    where
        Q: Eq + core::hash::Hash,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.facets().contains_key(index) && self.edges().contains_key(index)
    }
    /// check if a vertex with the given id exists
    pub fn contains_node<Q>(&self, index: &Q) -> bool
    where
        Q: Eq + core::hash::Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.nodes().contains_key(index)
    }
    /// returns true if the hypergraph is empty, meaning it has no edges, facets, or nodes
    pub fn is_empty(&self) -> bool {
        self.edges().is_empty() && self.facets().is_empty() && self.nodes().is_empty()
    }
    /// returns an [`Entry`](std::collections::hash_map::Entry) for the edge with the given
    /// index, allowing for modifications or insertions to the mapping
    pub fn edge(&mut self, index: EdgeId<Idx>) -> EdgeEntry<'_, Idx> {
        self.edges_mut().entry(index)
    }
    /// returns an [`Entry`](std::collections::hash_map::Entry) for the weight of the edge with
    /// the given index, allowing for modifications or insertions to the mapping
    pub fn facet(&mut self, index: EdgeId<Idx>) -> FacetEntry<'_, E, Idx> {
        self.facets_mut().entry(index)
    }
    /// returns an [`Entry`](std::collections::hash_map::Entry) for the node with the given
    /// index, allowing for modifications or insertions to the mapping
    pub fn node(&mut self, index: VertexId<Idx>) -> NodeEntry<'_, N, Idx> {
        self.nodes_mut().entry(index)
    }
    /// returns an iterator over the edges of the hypergraph, yielding pairs of [`EdgeId`] and
    /// the corresponding [`VertexSet`].
    pub fn edge_iter(&self) -> super::iter::EdgeIter<'_, Idx> {
        super::iter::EdgeIter {
            iter: self.edges().iter(),
        }
    }
    /// returns an iterator over the facets of the hypergraph, yielding pairs of [`EdgeId`] and
    /// the corresponding weight `E`.
    pub fn facet_iter(&self) -> super::iter::FacetIter<'_, E, Idx> {
        super::iter::FacetIter {
            iter: self.facets().iter(),
        }
    }
    /// returns an iterator over the nodes of the hypergraph, yielding pairs of [`VertexId`] and
    /// the corresponding [`HyperNode`].
    pub fn node_iter(&self) -> super::iter::NodeIter<'_, N, Idx> {
        super::iter::NodeIter {
            iter: self.nodes().iter(),
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
        self.edges().len()
    }
    /// returns the total number of facets in the hypergraph
    pub fn total_facets(&self) -> usize {
        self.facets().len()
    }
    /// returns the total number of vertices in the hypergraph
    pub fn total_vertices(&self) -> usize {
        self.nodes().len()
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
}

use crate::{HyperNode, Weight};

impl<N, E, K, Idx> rshyper_core::RawHyperGraph<N, E> for HashGraph<N, E, K, Idx>
where
    N: Eq + core::hash::Hash,
    E: Eq + core::hash::Hash,
    K: GraphKind,
    Idx: RawIndex + Eq + core::hash::Hash,
{
    type Idx = Idx;
    type Kind = K;
}

impl<N, E, K, Idx> rshyper_core::HyperGraph<N, E> for HashGraph<N, E, K, Idx>
where
    N: Eq + core::hash::Hash + Default,
    E: Eq + core::hash::Hash,
    K: GraphKind,
    Idx: crate::NumIndex,
{
    fn add_edge<I>(&mut self, iter: I) -> rshyper_core::Result<EdgeId<Self::Idx>>
    where
        I: IntoIterator<Item = VertexId<Self::Idx>>,
    {
        self.add_edge(iter)
    }

    fn add_facet(
        &mut self,
        index: EdgeId<Self::Idx>,
        weight: Weight<E>,
    ) -> crate::Result<Option<Weight<E>>> {
        self.add_facet(index, weight)
    }

    fn get_edge_vertices<S>(&self, index: &EdgeId<Self::Idx>) -> crate::Result<S>
    where
        for<'a> S: core::iter::FromIterator<&'a VertexId<Self::Idx>>,
    {
        self.get_edge_vertices(index)
            .map(|v| v.iter().collect::<S>())
    }

    fn add_node(&mut self, weight: N) -> VertexId<Self::Idx> {
        self.add_node(weight)
    }

    fn get_node(&self, index: &VertexId<Self::Idx>) -> crate::Result<&HyperNode<N, Self::Idx>> {
        self.get_node(index)
    }

    fn get_facet(&self, index: &EdgeId<Self::Idx>) -> crate::Result<&Weight<E>> {
        self.get_facet(index)
    }

    fn contains_edge(&self, index: &EdgeId<Self::Idx>) -> bool {
        self.contains_edge(index)
    }

    fn contains_facet(&self, index: &EdgeId<Self::Idx>) -> bool {
        self.contains_facet(index)
    }

    fn contains_node(&self, index: &VertexId<Self::Idx>) -> bool {
        self.contains_node(index)
    }
}
