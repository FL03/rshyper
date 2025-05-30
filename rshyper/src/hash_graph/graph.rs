/*
    Appellation: hash_graph <module>
    Contrib: @FL03
*/
use rshyper_core::{EdgeId, HyperNode, NumIndex, Position, RawIndex, VertexId};
use std::collections::{HashMap, HashSet};

/// A hash-based hypergraph implementation
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct HashGraph<N = (), E = (), Idx = usize>
where
    Idx: Eq + RawIndex + core::hash::Hash,
{
    /// the `edges` of a hypergraph is a map associating hyperedges (identified by `EdgeId`) to
    /// sets of vertices (identified by `VertexId`).
    pub(crate) edges: HashMap<EdgeId<Idx>, HashSet<VertexId<Idx>>>,
    /// the `facets` of a hypergraph materializes hyperedges by associating them with a weight
    pub(crate) facets: HashMap<EdgeId<Idx>, E>,
    /// the `nodes` of a hypergraph are the vertices, each identified by a `VertexId` and
    /// associated with a weight of type `N`.
    pub(crate) nodes: HashMap<VertexId<Idx>, HyperNode<N, Idx>>,
    pub(crate) position: Position<Idx>,
}

impl<N, E, Idx> HashGraph<N, E, Idx>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    Idx: Eq + RawIndex + core::hash::Hash,
{
    /// initialize a new hypergraph
    pub fn new() -> Self
    where
        Idx: Default,
    {
        HashGraph {
            facets: HashMap::new(),
            edges: HashMap::new(),
            nodes: HashMap::new(),
            position: Position::default(),
        }
    }
    /// returns an immutable reference to the edges of the hypergraph; a mapping of edges to vertices essentially forming a topological space
    /// that enables the data-structure to be traversed, analyzed, and manipulated.
    pub const fn edges(&self) -> &HashMap<EdgeId<Idx>, HashSet<VertexId<Idx>>> {
        &self.edges
    }
    /// returns a mutable reference to the hyperedges
    pub const fn edges_mut(&mut self) -> &mut HashMap<EdgeId<Idx>, HashSet<VertexId<Idx>>> {
        &mut self.edges
    }
    /// returns an immutable reference to the facets of the hypergraph; here, a facet is a
    /// hyperedge with an associated weight
    pub const fn facets(&self) -> &HashMap<EdgeId<Idx>, E> {
        &self.facets
    }
    /// returns a mutable reference to the edges, or facets, of the hypergraph
    pub const fn facets_mut(&mut self) -> &mut HashMap<EdgeId<Idx>, E> {
        &mut self.facets
    }
    /// returns am immutable reference to the nodes
    pub const fn nodes(&self) -> &HashMap<VertexId<Idx>, HyperNode<N, Idx>> {
        &self.nodes
    }
    /// returns a mutable reference to the nodes of the hypergraph
    pub const fn nodes_mut(&mut self) -> &mut HashMap<VertexId<Idx>, HyperNode<N, Idx>> {
        &mut self.nodes
    }
    /// returns a copy of the position of the hypergraph; here, the [`position`](Position) is
    /// used to track the indices (edge & vertex) and define which ones are next to be used
    /// when inserting new hyperedges or vertices
    pub const fn position(&self) -> &Position<Idx> {
        &self.position
    }
    /// returns a mutable reference to the current position of the hypergraph;
    pub fn position_mut(&mut self) -> &mut Position<Idx> {
        &mut self.position
    }
}
/// depreciated implementations for the [`HashGraph`]
impl<N, E, Idx> HashGraph<N, E, Idx>
where
    N: Eq + core::hash::Hash,
    E: Eq + core::hash::Hash,
    Idx: NumIndex,
{
    #[deprecated(since = "v0.0.3", note = "use `merge_edges` instead")]
    pub fn merge_hyperedges(
        &mut self,
        e1: &EdgeId<Idx>,
        e2: &EdgeId<Idx>,
    ) -> crate::Result<EdgeId<Idx>>
    where
        Idx: core::ops::Add<Output = Idx> + num_traits::One,
    {
        self.merge_edges(e1, e2)
    }
    #[deprecated(since = "v0.0.3", note = "use `remove_edge` instead")]
    pub fn remove_hyperedge(
        &mut self,
        index: &EdgeId<Idx>,
    ) -> crate::Result<HashSet<VertexId<Idx>>> {
        self.remove_edge(index)
    }
    #[deprecated(since = "v0.0.3", note = "use `insert_edge` instead")]
    pub fn add_hyperedge<I>(&mut self, vertices: I) -> crate::Result<EdgeId<Idx>>
    where
        I: Clone + IntoIterator<Item = VertexId<Idx>>,
        Idx: core::ops::Add<Output = Idx> + num_traits::One,
    {
        self.insert_edge(vertices)
    }
    #[deprecated(since = "v0.0.3", note = "use `insert_node` instead")]
    pub fn add_vertex(&mut self, weight: N) -> VertexId<Idx>
    where
        Idx: core::ops::Add<Output = Idx> + num_traits::One,
    {
        self.insert_node(weight)
    }
    #[deprecated(since = "v0.0.3", note = "use `insert_node_default` instead")]
    pub fn add_vertex_default(&mut self) -> VertexId<Idx>
    where
        N: Default,
        Idx: core::ops::Add<Output = Idx> + num_traits::One,
    {
        self.insert_node(N::default())
    }
    #[deprecated(
        since = "v0.0.4",
        note = "use `neighbors` instead to get the neighbors of a vertex"
    )]
    pub fn get_neighbors(&self, index: &VertexId<Idx>) -> crate::Result<HashSet<VertexId<Idx>>> {
        self.neighbors(index)
    }
}


impl<N, E, Idx> Default for HashGraph<N, E, Idx>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    Idx: NumIndex,
{
    fn default() -> Self {
        Self::new()
    }
}
