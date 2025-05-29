/*
    Appellation: graph <module>
    Contrib: @FL03
*/
use rshyper_core::{EdgeId, Node, VertexId, id::Position};
use std::collections::{HashMap, HashSet};

/// A hash-based hypergraph implementation
///
/// ## Features
///
/// - ``
/// - `facet`: a materialized hyperedge with an associated weight
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct HashGraph<N = (), E = ()> {
    pub(crate) connections: HashMap<EdgeId, HashSet<VertexId>>,
    pub(crate) facets: HashMap<EdgeId, E>,
    pub(crate) vertices: HashMap<VertexId, Node<N>>,
    pub(crate) position: Position,
}

impl<N, E> HashGraph<N, E>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
{
    /// initialize a new hypergraph
    pub fn new() -> Self {
        HashGraph {
            facets: HashMap::new(),
            connections: HashMap::new(),
            vertices: HashMap::new(),
            position: Position::zero(),
        }
    }
    /// returns an immutable reference to the hyperedges
    pub const fn connections(&self) -> &HashMap<EdgeId, HashSet<VertexId>> {
        &self.connections
    }
    /// returns a mutable reference to the hyperedges
    pub const fn connections_mut(&mut self) -> &mut HashMap<EdgeId, HashSet<VertexId>> {
        &mut self.connections
    }
    /// returns an immutable reference to the facets of the hypergraph; here, a facet is a
    /// hyperedge with an associated weight
    pub const fn facets(&self) -> &HashMap<EdgeId, E> {
        &self.facets
    }
    /// returns a mutable reference to the edges, or facets, of the hypergraph
    pub const fn facets_mut(&mut self) -> &mut HashMap<EdgeId, E> {
        &mut self.facets
    }
    /// returns am immutable reference to the nodes
    pub const fn nodes(&self) -> &HashMap<VertexId, Node<N>> {
        &self.vertices
    }
    /// returns a mutable reference to the nodes of the hypergraph
    pub const fn nodes_mut(&mut self) -> &mut HashMap<VertexId, Node<N>> {
        &mut self.vertices
    }
    /// returns a copy of the current position of the hypergraph
    pub const fn position(&self) -> Position {
        self.position
    }
    /// returns a mutable reference to the current position of the hypergraph
    pub fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
    /// clears all vertices and hyperedges, resetting the hypergraph
    pub fn clear(&mut self) {
        self.nodes_mut().clear();
        self.connections_mut().clear();
    }
    /// check if a hyperedge with the given id exists
    pub fn contains_edge(&self, index: &EdgeId) -> bool {
        self.connections().contains_key(index)
    }
    /// check if a vertex with the given id exists
    pub fn contains_node(&self, index: &VertexId) -> bool {
        self.nodes().contains_key(index)
    }
    /// get the next edge index and updates the current position
    pub fn next_edge_id(&mut self) -> EdgeId {
        self.position_mut().next_edge().unwrap()
    }
    /// returns the next vertex index and updates the current position
    pub fn next_vertex_id(&mut self) -> VertexId {
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
    pub fn get_edge_order(&self, index: &EdgeId) -> crate::Result<usize> {
        self.connections()
            .get(index)
            .map(|vertices| vertices.len())
            .ok_or(crate::Error::HyperedgeDoesNotExist(*index))
    }
    /// returns all hyperedges containing a given vertex
    pub fn get_edges_with_vertex(&self, index: VertexId) -> crate::Result<Vec<EdgeId>> {
        if !self.contains_node(&index) {
            return Err(crate::Error::VertexDoesNotExist(index));
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
    pub fn get_facet(&self, index: EdgeId) -> crate::Result<&E> {
        self.facets()
            .get(&index)
            .ok_or_else(|| crate::Error::HyperedgeDoesNotExist(index))
    }
    /// retrieves a mutable reference to the facet (hyperedge with an associated weight)
    pub fn get_facet_mut(&mut self, index: EdgeId) -> crate::Result<&mut E> {
        self.facets_mut()
            .get_mut(&index)
            .ok_or_else(|| crate::Error::HyperedgeDoesNotExist(index))
    }
    /// retrieves the set of nodes composing the given edge
    pub fn get_nodes_for_edge(&self, index: EdgeId) -> crate::Result<Vec<&Node<N>>> {
        let vertices = self.get_vertices_for_edge(index)?;
        let nodes = vertices
            .iter()
            .map(|v| self.get_vertex_weight(*v).expect("vertex not found"))
            .collect::<Vec<_>>();
        Ok(nodes)
    }
    /// returns the set of vertices composing the given edge
    pub fn get_vertices_for_edge(&self, index: EdgeId) -> crate::Result<&HashSet<VertexId>> {
        self.connections()
            .get(&index)
            .ok_or_else(|| crate::Error::HyperedgeDoesNotExist(index))
    }
    /// returns the degree of a given vertex where the degree is the number of hyperedges that
    /// contain the vertex
    pub fn get_vertex_degree(&self, index: VertexId) -> crate::Result<usize> {
        if !self.contains_node(&index) {
            return Err(crate::Error::VertexDoesNotExist(index));
        }

        let degree = self
            .connections()
            .values()
            .filter(|vertices| vertices.contains(&index))
            .count();
        Ok(degree)
    }
    /// returns the weight of a particular vertex
    pub fn get_vertex_weight(&self, index: VertexId) -> crate::Result<&Node<N>> {
        self.nodes()
            .get(&index)
            .ok_or(crate::Error::VertexDoesNotExist(index))
    }

    /// returns a mutable reference to the weight of a vertex
    pub fn get_vertex_weight_mut(&mut self, index: VertexId) -> crate::Result<&mut Node<N>> {
        self.nodes_mut()
            .get_mut(&index)
            .ok_or(crate::Error::VertexDoesNotExist(index))
    }
    /// add a new hyperedge with the given vertices and return its ID
    pub fn insert_edge<I>(&mut self, vertices: I) -> crate::Result<EdgeId>
    where
        I: Clone + IntoIterator<Item = VertexId>,
    {
        // Verify all vertices exist
        for v in vertices.clone().into_iter() {
            if !self.contains_node(&v) {
                return Err(crate::Error::VertexDoesNotExist(v));
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
    /// insert a new facet (hyperedge with an associated weight) into the hypergraph;
    /// if the facet, or hyperedge, already exists, it will be replaced and returned
    pub fn insert_facet(&mut self, edge_id: EdgeId, facet: E) -> crate::Result<()> {
        if !self.contains_edge(&edge_id) {
            return Err(crate::Error::HyperedgeDoesNotExist(edge_id));
        }
        let _prev = self.facets_mut().insert(edge_id, facet);
        Ok(())
    }
    /// insert a new node with the given weight and return its index
    pub fn insert_node(&mut self, weight: N) -> VertexId {
        // generate a new vertex ID
        let idx = self.next_vertex_id();
        // initialize a new node with the given weight & index
        let node = Node::new(idx, weight);
        // insert the new node into the vertices map
        self.nodes_mut().insert(idx, node);
        idx
    }
    /// insert a new vertex with the default weight and return its ID
    pub fn insert_vertex_default(&mut self) -> VertexId
    where
        N: Default,
    {
        self.insert_node(N::default())
    }
    /// merges two hyperedges into one (combining their vertices)
    pub fn merge_edges(&mut self, e1: EdgeId, e2: EdgeId) -> crate::Result<EdgeId> {
        use crate::Error::HyperedgeDoesNotExist;

        let set1 = self
            .connections_mut()
            .remove(&e1)
            .ok_or(HyperedgeDoesNotExist(e1))?;
        let set2 = self
            .connections_mut()
            .remove(&e2)
            .ok_or(HyperedgeDoesNotExist(e2))?;
        let merged: HashSet<VertexId> = set1.union(&set2).cloned().collect();
        let new_edge = self.next_edge_id();
        self.connections_mut().insert(new_edge, merged);
        Ok(new_edge)
    }
    /// returns a set of vertices that are in the hyperedge with the given id
    pub fn neighbors(&self, index: VertexId) -> crate::Result<HashSet<VertexId>> {
        if !self.contains_node(&index) {
            return Err(crate::Error::VertexDoesNotExist(index));
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
    pub fn remove_edge(&mut self, index: EdgeId) -> crate::Result<HashSet<VertexId>> {
        self.connections_mut()
            .remove(&index)
            .ok_or(crate::Error::HyperedgeDoesNotExist(index))
    }
    /// removes the vertex with the given id and all of its associated hyperedges
    pub fn remove_vertex(&mut self, index: VertexId) -> crate::Result<Node<N>> {
        self.nodes_mut()
            .remove(&index)
            .map(|node| {
                // Remove all hyperedges containing this vertex
                self.connections_mut()
                    .retain(|_, vertices| !vertices.contains(&index));
                node
            })
            .ok_or(crate::Error::VertexDoesNotExist(index))
    }
    /// update the weight of a given vertex
    pub fn set_vertex_weight(&mut self, index: VertexId, weight: N) -> crate::Result<()>
    where
        N: Clone,
    {
        self.nodes_mut()
            .get_mut(&index)
            .map(|node| {
                node.set_weight(weight.clone());
            })
            .ok_or(crate::Error::VertexDoesNotExist(index))
    }
    /// search the hypergraph using the A* algorithm with the given heuristic function
    pub fn astar<F>(&self, heuristic: F) -> crate::algo::AStarSearch<'_, N, E, F>
    where
        F: Fn(VertexId, VertexId) -> f64,
    {
        crate::algo::AStarSearch::new(self, heuristic)
    }
    /// search the hypergraph using the breadth-first traversal algorithm
    pub fn bft(&self) -> crate::algo::BreadthFirstTraversal<'_, N, E> {
        crate::algo::BreadthFirstTraversal::new(self)
    }
    /// search the hypergraph using the depth-first traversal algorithm
    pub fn dft(&self) -> crate::algo::DepthFirstTraversal<'_, N, E> {
        crate::algo::DepthFirstTraversal::new(self)
    }
}

impl<N, E> HashGraph<N, E>
where
    N: Eq + core::hash::Hash,
    E: Eq + core::hash::Hash,
{
    #[deprecated(since = "v0.0.3", note = "use `connections` instead")]
    pub const fn edges(&self) -> &HashMap<EdgeId, HashSet<VertexId>> {
        self.connections()
    }
    #[deprecated(since = "v0.0.3", note = "use `connections_mut` instead")]
    pub const fn edges_mut(&mut self) -> &mut HashMap<EdgeId, HashSet<VertexId>> {
        self.connections_mut()
    }
    #[deprecated(since = "v0.0.3", note = "use `merge_edges` instead")]
    pub fn merge_hyperedges(&mut self, e1: EdgeId, e2: EdgeId) -> crate::Result<EdgeId> {
        self.merge_edges(e1, e2)
    }
    #[deprecated(since = "v0.0.3", note = "use `remove_edge` instead")]
    pub fn remove_hyperedge(&mut self, index: EdgeId) -> crate::Result<HashSet<VertexId>> {
        self.remove_edge(index)
    }
    #[deprecated(since = "v0.0.3", note = "use `insert_edge` instead")]
    pub fn add_hyperedge<I>(&mut self, vertices: I) -> crate::Result<EdgeId>
    where
        I: Clone + IntoIterator<Item = VertexId>,
    {
        self.insert_edge(vertices)
    }
    #[deprecated(since = "v0.0.3", note = "use `insert_vertex` instead")]
    pub fn add_vertex(&mut self, weight: N) -> VertexId {
        self.insert_node(weight)
    }
    #[deprecated(since = "v0.0.3", note = "use `insert_vertex_default` instead")]
    pub fn add_vertex_default(&mut self) -> VertexId
    where
        N: Default,
    {
        self.insert_node(N::default())
    }
    #[deprecated(
        since = "v0.0.4",
        note = "use `neighbors` instead to get the neighbors of a vertex"
    )]
    pub fn get_neighbors(&self, index: VertexId) -> crate::Result<HashSet<VertexId>> {
        self.neighbors(index)
    }
}

impl HashGraph<()> {
    pub fn insert_empty_node(&mut self) -> VertexId {
        self.insert_node(())
    }
}

impl<T, E> HashGraph<Option<T>, E>
where
    E: Eq + core::hash::Hash,
    T: Eq + core::hash::Hash,
{
    /// insert [`Some`] vertex with weight `T` and return its ID
    pub fn insert_some_node(&mut self, weight: T) -> VertexId {
        self.insert_node(Some(weight))
    }

    pub fn insert_empty_node(&mut self) -> VertexId {
        self.insert_node(None)
    }
}

impl<N> Default for HashGraph<N>
where
    N: core::cmp::Eq + core::hash::Hash,
{
    fn default() -> Self {
        Self::new()
    }
}
