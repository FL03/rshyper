/*
    Appellation: hash_graph <module>
    Contrib: @FL03
*/

mod impl_ops;

use rshyper_core::{EdgeId, HyperNode, NumIndex, Position, RawIndex, VertexId};
use std::collections::{HashMap, HashSet};

/// A hash-based hypergraph implementation
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct HashGraph<N = (), E = (), Idx = usize>
where
    Idx: Eq + RawIndex + core::hash::Hash,
{
    pub(crate) emap: HashMap<EdgeId<Idx>, HashSet<VertexId<Idx>>>,
    pub(crate) facets: HashMap<EdgeId<Idx>, E>,
    pub(crate) vertices: HashMap<VertexId<Idx>, HyperNode<N, Idx>>,
    pub(crate) position: Position<Idx>,
}

impl<N, E, Idx> HashGraph<N, E, Idx>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    Idx: NumIndex,
{
    /// initialize a new hypergraph
    pub fn new() -> Self
    where
        Idx: Default,
    {
        HashGraph {
            emap: HashMap::new(),
            facets: HashMap::new(),
            vertices: HashMap::new(),
            position: Position::default(),
        }
    }
    /// returns an immutable reference to the _connections_ forming each hyperedge; in other
    /// words, the connections are a map of edges to sets of vertices, where each edge is
    /// represented by an [`EdgeId`] and each vertex by a [`VertexId`].
    pub const fn connections(&self) -> &HashMap<EdgeId<Idx>, HashSet<VertexId<Idx>>> {
        &self.emap
    }
    /// returns a mutable reference to the hyperedges
    pub const fn connections_mut(&mut self) -> &mut HashMap<EdgeId<Idx>, HashSet<VertexId<Idx>>> {
        &mut self.emap
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
        &self.vertices
    }
    /// returns a mutable reference to the nodes of the hypergraph
    pub const fn nodes_mut(&mut self) -> &mut HashMap<VertexId<Idx>, HyperNode<N, Idx>> {
        &mut self.vertices
    }
    /// returns a copy of the position of the hypergraph; here, the [`position`](Position) is
    /// used to track the indices (edge & vertex) and define which ones are next to be used
    /// when inserting new hyperedges or vertices
    pub const fn position(&self) -> Position<Idx> {
        self.position
    }
    /// returns a mutable reference to the current position of the hypergraph;
    pub fn position_mut(&mut self) -> &mut Position<Idx> {
        &mut self.position
    }
    /// clears all vertices and hyperedges, resetting the hypergraph
    pub fn clear(&mut self) {
        self.nodes_mut().clear();
        self.connections_mut().clear();
    }
    /// check if a hyperedge with the given id exists
    pub fn contains_edge(&self, index: &EdgeId<Idx>) -> bool {
        self.connections().contains_key(index)
    }
    /// check if a vertex with the given id exists
    pub fn contains_node(&self, index: &VertexId<Idx>) -> bool {
        self.nodes().contains_key(index)
    }
    /// get the next edge index and updates the current position
    pub fn next_edge_id(&mut self) -> EdgeId<Idx>
    where
        Idx: core::ops::Add<Output = Idx> + num_traits::One,
    {
        self.position_mut().next_edge().unwrap()
    }
    /// returns the next vertex index and updates the current position
    pub fn next_vertex_id(&mut self) -> VertexId<Idx>
    where
        Idx: core::ops::Add<Output = Idx> + num_traits::One,
    {
        self.position_mut().next_vertex().unwrap()
    }
    /// returns the total number of hyperedges in the hypergraph
    pub fn total_edges(&self) -> usize {
        self.connections().len()
    }
    /// returns the total number of vertices in the hypergraph
    pub fn total_vertices(&self) -> usize {
        self.nodes().len()
    }
    /// returns the size, or order, of a particular hyperedge
    pub fn get_edge_order(&self, index: &EdgeId<Idx>) -> crate::Result<usize> {
        self.connections()
            .get(index)
            .map(|vertices| vertices.len())
            .ok_or(crate::Error::IndexNotFound)
    }
    /// returns all hyperedges containing a given vertex
    pub fn get_edges_with_vertex(&self, index: VertexId<Idx>) -> crate::Result<Vec<EdgeId<Idx>>> {
        if !self.contains_node(&index) {
            return Err(crate::Error::IndexNotFound);
        }
        let edges = self
            .connections()
            .iter()
            .filter_map(|(&edge_id, vertices)| {
                if vertices.contains(&index) {
                    Some(edge_id)
                } else {
                    None
                }
            })
            .collect();
        Ok(edges)
    }
    /// retrieves a reference to the facet (hyperedge with an associated weight)
    pub fn get_facet(&self, index: EdgeId<Idx>) -> crate::Result<&E> {
        self.facets()
            .get(&index)
            .ok_or_else(|| crate::Error::IndexNotFound)
    }
    /// retrieves a mutable reference to the facet (hyperedge with an associated weight)
    pub fn get_facet_mut(&mut self, index: EdgeId<Idx>) -> crate::Result<&mut E> {
        self.facets_mut()
            .get_mut(&index)
            .ok_or_else(|| crate::Error::IndexNotFound)
    }
    /// retrieves the set of nodes composing the given edge
    pub fn get_nodes_for_edge(&self, index: EdgeId<Idx>) -> crate::Result<Vec<&HyperNode<N, Idx>>> {
        let vertices = self.get_vertices_for_edge(index)?;
        let nodes = vertices
            .iter()
            .map(|v| self.get_vertex_weight(*v).expect("vertex not found"))
            .collect::<Vec<_>>();
        Ok(nodes)
    }
    /// returns the set of vertices composing the given edge
    pub fn get_vertices_for_edge(
        &self,
        index: EdgeId<Idx>,
    ) -> crate::Result<&HashSet<VertexId<Idx>>> {
        self.connections()
            .get(&index)
            .ok_or_else(|| crate::Error::IndexNotFound)
    }
    /// returns the degree of a given vertex where the degree is the number of hyperedges that
    /// contain the vertex
    pub fn get_vertex_degree(&self, index: VertexId<Idx>) -> crate::Result<usize> {
        if !self.contains_node(&index) {
            return Err(crate::Error::IndexNotFound);
        }

        let degree = self
            .connections()
            .values()
            .filter(|vertices| vertices.contains(&index))
            .count();
        Ok(degree)
    }
    /// returns the weight of a particular vertex
    pub fn get_vertex_weight(&self, index: VertexId<Idx>) -> crate::Result<&HyperNode<N, Idx>> {
        self.nodes().get(&index).ok_or(crate::Error::IndexNotFound)
    }

    /// returns a mutable reference to the weight of a vertex
    pub fn get_vertex_weight_mut(
        &mut self,
        index: VertexId<Idx>,
    ) -> crate::Result<&mut HyperNode<N, Idx>> {
        self.nodes_mut()
            .get_mut(&index)
            .ok_or(crate::Error::IndexNotFound)
    }
    /// add a new hyperedge with the given vertices and return its ID
    pub fn insert_edge<I>(&mut self, vertices: I) -> crate::Result<EdgeId<Idx>>
    where
        I: Clone + IntoIterator<Item = VertexId<Idx>>,
        Idx: core::ops::Add<Output = Idx> + num_traits::One,
    {
        // Verify all vertices exist
        for v in vertices.clone().into_iter() {
            if !self.contains_node(&v) {
                return Err(crate::Error::IndexNotFound);
            }
        }
        // fetch the next edge index
        let eid = self.next_edge_id();
        // collect the vertices into a HashSet to ensure uniqueness
        let vset = vertices.into_iter().collect::<HashSet<_>>();
        // handle the case where the edge has no associated vertices
        if vset.is_empty() {
            return Err(crate::Error::EmptyHyperedge);
        }
        // insert the new hyperedge into the adjacency map
        self.connections_mut().insert(eid, vset);
        Ok(eid)
    }
    /// insert a new hyperedge with the given vertices and weight, returning its ID;
    pub fn insert_edge_with_weight<I>(
        &mut self,
        vertices: I,
        weight: E,
    ) -> crate::Result<EdgeId<Idx>>
    where
        I: Clone + IntoIterator<Item = VertexId<Idx>>,
        E: Eq + core::hash::Hash,
        Idx: core::ops::Add<Output = Idx> + num_traits::One,
    {
        // insert the edge and get its ID
        let index = self.insert_edge(vertices)?;
        // insert the facet with the given weight
        self.insert_facet(index, weight)
    }
    /// insert a new facet (hyperedge with an associated weight) into the hypergraph;
    /// if the facet, or hyperedge, already exists, it will replace the existing value with
    /// the given
    pub fn insert_facet(&mut self, index: EdgeId<Idx>, facet: E) -> crate::Result<EdgeId<Idx>> {
        if !self.contains_edge(&index) {
            return Err(crate::Error::IndexNotFound);
        }
        let _prev = self.facets_mut().insert(index, facet);
        Ok(index)
    }
    /// insert a new node with the given weight and return its index
    pub fn insert_node(&mut self, weight: N) -> VertexId<Idx>
    where
        Idx: core::ops::Add<Output = Idx> + num_traits::One,
    {
        // generate a new vertex ID
        let idx = self.next_vertex_id();
        // initialize a new node with the given weight & index
        let node = HyperNode::new(idx, weight);
        // insert the new node into the vertices map
        self.nodes_mut().insert(idx, node);
        idx
    }
    /// insert a new vertex with the default weight and return its ID
    pub fn insert_node_default(&mut self) -> VertexId<Idx>
    where
        N: Default,
        Idx: core::ops::Add<Output = Idx> + num_traits::One,
    {
        self.insert_node(N::default())
    }
    /// merges two hyperedges into one (combining their vertices)
    pub fn merge_edges(&mut self, e1: EdgeId<Idx>, e2: EdgeId<Idx>) -> crate::Result<EdgeId<Idx>>
    where
        Idx: core::ops::Add<Output = Idx> + num_traits::One,
    {
        let set1 = self
            .connections_mut()
            .remove(&e1)
            .ok_or(crate::Error::IndexNotFound)?;
        let set2 = self
            .connections_mut()
            .remove(&e2)
            .ok_or(crate::Error::IndexNotFound)?;
        let merged = set1.union(&set2).cloned().collect::<HashSet<_>>();
        let new_edge = self.next_edge_id();
        self.connections_mut().insert(new_edge, merged);
        Ok(new_edge)
    }
    /// returns a set of vertices that are in the hyperedge with the given id
    pub fn neighbors(&self, index: VertexId<Idx>) -> crate::Result<HashSet<VertexId<Idx>>> {
        if !self.contains_node(&index) {
            return Err(crate::Error::IndexNotFound);
        }
        // initialize an empty set to hold the neighbors
        let mut neighbors = HashSet::new();
        // iterate through all the connections
        self.connections().values().for_each(|vertices| {
            if vertices.contains(&index) {
                neighbors.extend(vertices.iter().filter(|&&v| v != index));
            }
        });
        Ok(neighbors)
    }
    /// remove the hyperedge with the given id
    pub fn remove_edge(&mut self, index: EdgeId<Idx>) -> crate::Result<HashSet<VertexId<Idx>>> {
        self.connections_mut()
            .remove(&index)
            .ok_or(crate::Error::IndexNotFound)
    }
    /// removes the vertex with the given id and all of its associated hyperedges
    pub fn remove_vertex(&mut self, index: VertexId<Idx>) -> crate::Result<HyperNode<N, Idx>> {
        self.nodes_mut()
            .remove(&index)
            .map(|node| {
                // Remove all hyperedges containing this vertex
                self.connections_mut()
                    .retain(|_, vertices| !vertices.contains(&index));
                node
            })
            .ok_or(crate::Error::IndexNotFound)
    }
    /// update the weight of a given vertex
    pub fn set_vertex_weight(&mut self, index: VertexId<Idx>, weight: N) -> crate::Result<()>
    where
        N: Clone,
    {
        self.nodes_mut()
            .get_mut(&index)
            .map(|node| {
                node.set_weight(weight.clone());
            })
            .ok_or(crate::Error::IndexNotFound)
    }
}

impl<N, E> HashGraph<N, E, usize>
where
    N: Eq + core::hash::Hash,
    E: Eq + core::hash::Hash,
{
    /// search the hypergraph using the A* algorithm with the given heuristic function
    pub fn astar<F>(&self, heuristic: F) -> crate::algo::AStarSearch<'_, N, E, F>
    where
        F: Fn(VertexId, VertexId) -> f64,
    {
        crate::algo::AStarSearch::new(self, heuristic)
    }
    /// search the hypergraph using the breadth-first traversal algorithm
    pub fn bft(&self) -> crate::algo::BreadthFirstTraversal<'_, N, E> {
        crate::algo::BreadthFirstTraversal::from_hypergraph(self)
    }
    /// search the hypergraph using the depth-first traversal algorithm
    pub fn dft(&self) -> crate::algo::DepthFirstTraversal<'_, N, E> {
        crate::algo::DepthFirstTraversal::new(self)
    }
}

impl<N, E, Idx> HashGraph<N, E, Idx>
where
    N: Eq + core::hash::Hash,
    E: Eq + core::hash::Hash,
    Idx: NumIndex,
{
    #[deprecated(since = "v0.0.3", note = "use `connections` instead")]
    pub const fn edges(&self) -> &HashMap<EdgeId<Idx>, HashSet<VertexId<Idx>>> {
        self.connections()
    }
    #[deprecated(since = "v0.0.3", note = "use `connections_mut` instead")]
    pub const fn edges_mut(&mut self) -> &mut HashMap<EdgeId<Idx>, HashSet<VertexId<Idx>>> {
        self.connections_mut()
    }
    #[deprecated(since = "v0.0.3", note = "use `merge_edges` instead")]
    pub fn merge_hyperedges(
        &mut self,
        e1: EdgeId<Idx>,
        e2: EdgeId<Idx>,
    ) -> crate::Result<EdgeId<Idx>>
    where
        Idx: core::ops::Add<Output = Idx> + num_traits::One,
    {
        self.merge_edges(e1, e2)
    }
    #[deprecated(since = "v0.0.3", note = "use `remove_edge` instead")]
    pub fn remove_hyperedge(
        &mut self,
        index: EdgeId<Idx>,
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
    pub fn get_neighbors(&self, index: VertexId<Idx>) -> crate::Result<HashSet<VertexId<Idx>>> {
        self.neighbors(index)
    }
}

impl<E, Idx> HashGraph<(), E, Idx>
where
    E: Eq + core::hash::Hash,
    Idx: NumIndex,
{
    pub fn insert_empty_node(&mut self) -> VertexId<Idx>
    where
        Idx: core::ops::Add<Output = Idx> + num_traits::One,
    {
        self.insert_node(())
    }
}

impl<N, E, Idx> HashGraph<Option<N>, E, Idx>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    Idx: NumIndex,
{
    /// insert [`Some`] vertex with weight `T` and return its ID
    pub fn insert_some_node(&mut self, weight: N) -> VertexId<Idx>
    where
        Idx: core::ops::Add<Output = Idx> + num_traits::One,
    {
        self.insert_node(Some(weight))
    }

    pub fn insert_empty_node(&mut self) -> VertexId<Idx>
    where
        Idx: core::ops::Add<Output = Idx> + num_traits::One,
    {
        self.insert_node(None)
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
