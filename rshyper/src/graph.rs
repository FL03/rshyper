/*
    Appellation: graph <module>
    Contrib: @FL03
*/
use crate::types::{EdgeId, Index, Node, VertexId};
use num::Zero;
use std::collections::{HashMap, HashSet};

/// A hash-based hypergraph implementation
#[derive(Clone, Debug)]
pub struct HyperGraph<N = ()> {
    pub(crate) edges: HashMap<EdgeId, HashSet<VertexId>>,
    pub(crate) vertices: HashMap<VertexId, Node<N>>,
    pub(crate) next_vertex_id: VertexId,
    pub(crate) next_edge_id: EdgeId,
}

impl<N> HyperGraph<N>
where
    N: core::cmp::Eq + core::hash::Hash,
{
    /// initialize a new hypergraph
    pub fn new() -> Self {
        HyperGraph {
            vertices: HashMap::new(),
            edges: HashMap::new(),
            next_vertex_id: VertexId::zero(),
            next_edge_id: EdgeId::zero(),
        }
    }
    /// returns an immutable reference to the hyperedges
    pub const fn edges(&self) -> &HashMap<EdgeId, HashSet<VertexId>> {
        &self.edges
    }
    /// returns a mutable reference to the hyperedges
    pub fn edges_mut(&mut self) -> &mut HashMap<EdgeId, HashSet<VertexId>> {
        &mut self.edges
    }
    /// returns am immutable reference to the vertices
    pub const fn vertices(&self) -> &HashMap<VertexId, Node<N>> {
        &self.vertices
    }
    /// returns a mutable reference to the vertices
    pub fn vertices_mut(&mut self) -> &mut HashMap<VertexId, Node<N>> {
        &mut self.vertices
    }
    /// add a new hyperedge with the given vertices and return its ID
    pub fn add_hyperedge<I>(&mut self, vertices: I) -> crate::Result<EdgeId>
    where
        I: Clone + IntoIterator<Item = VertexId>,
    {
        // Verify all vertices exist
        for vertex in vertices.clone().into_iter() {
            if !self.check_vertex(&vertex) {
                return Err(crate::Error::VertexDoesNotExist(vertex.to_string()));
            }
        }

        let edge_id = self.next_edge_id;
        let vertex_set: HashSet<VertexId> = vertices.into_iter().collect();

        // Don't allow empty hyperedges
        if vertex_set.is_empty() {
            return Err(crate::Error::EmptyHyperedge);
        }

        self.edges.insert(edge_id, vertex_set);
        self.next_edge_id += 1;
        Ok(edge_id)
    }
    /// insert a new vertex with the given weight and return its ID
    pub fn add_vertex(&mut self, weight: N) -> VertexId {
        let vertex_id = self.next_vertex_id;
        self.vertices
            .insert(vertex_id, Node::new(vertex_id, weight));
        self.next_vertex_id += 1;
        vertex_id
    }
    /// insert a new vertex with the default weight and return its ID
    pub fn add_vertex_default(&mut self) -> VertexId
    where
        N: Default,
    {
        self.add_vertex(N::default())
    }

    /// check if a vertex with the given id exists
    pub fn check_vertex(&self, Index(index): &VertexId) -> bool {
        self.vertices.contains_key(index)
    }
    /// check if a hyperedge with the given id exists
    pub fn check_hyperedge(&self, edge_id: &EdgeId) -> bool {
        self.edges.contains_key(edge_id)
    }
    /// clears all vertices and hyperedges, resetting the hypergraph
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.edges.clear();
        self.next_vertex_id = VertexId::zero();
        self.next_edge_id = EdgeId::zero();
    }
    /// returns a set of vertices that are in the hyperedge with the given id
    pub fn get_neighbors(&self, Index(index): VertexId) -> crate::Result<HashSet<VertexId>> {
        if !self.check_vertex(&Index(index)) {
            return Err(crate::Error::VertexDoesNotExist(index.to_string()));
        }

        let mut neighbors = HashSet::new();
        for vertices in self.edges.values() {
            if vertices.contains(&index) {
                neighbors.extend(vertices.iter().filter(|&&v| v != index));
            }
        }
        Ok(neighbors)
    }
    /// retrieves the set of vertices that make up a specific hyperedge
    pub fn get_edge_vertices(&self, Index(index): EdgeId) -> crate::Result<&HashSet<VertexId>> {
        self.edges
            .get(&index)
            .ok_or_else(|| crate::Error::HyperedgeDoesNotExist(index.to_string()))
    }

    pub fn get_edge_nodes(&self, Index(index): EdgeId) -> crate::Result<Vec<&Node<N>>> {
        let vertices = self.get_edge_vertices(Index(index))?;
        let nodes = vertices
            .iter()
            .map(|v| self.get_vertex_weight(*v).expect("vertex not found"))
            .collect::<Vec<_>>();
        Ok(nodes)
    }

    /// returns the size of a given hyperedge (number of vertices in it)
    pub fn edge_cardinality(&self, Index(index): EdgeId) -> crate::Result<usize> {
        match self.edges.get(&index) {
            Some(vertices) => Ok(vertices.len()),
            None => Err(crate::Error::HyperedgeDoesNotExist(index.to_string())),
        }
    }
    /// returns all hyperedges containing a given vertex
    pub fn get_vertex_edges(&self, Index(index): VertexId) -> crate::Result<Vec<EdgeId>> {
        if !self.check_vertex(&Index(index)) {
            return Err(crate::Error::VertexDoesNotExist(index.to_string()));
        }
        let edges = self
            .edges
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
    /// returns the weight of a particular vertex
    pub fn get_vertex_weight(&self, Index(index): VertexId) -> crate::Result<&Node<N>> {
        match self.vertices.get(&Index(index)) {
            Some(weight) => Ok(weight),
            None => Err(crate::Error::VertexDoesNotExist(index.to_string())),
        }
    }
    /// merges two hyperedges into one (combining their vertices)
    pub fn merge_hyperedges(&mut self, e1: EdgeId, e2: EdgeId) -> crate::Result<EdgeId> {
        if !self.check_hyperedge(&e1) {
            return Err(crate::Error::HyperedgeDoesNotExist(e1.to_string()));
        }
        if !self.check_hyperedge(&e2) {
            return Err(crate::Error::HyperedgeDoesNotExist(e2.to_string()));
        }
        let set1 = self.edges.remove(&e1).unwrap();
        let set2 = self.edges.remove(&e2).unwrap();
        let merged: HashSet<VertexId> = set1.union(&set2).cloned().collect();
        let new_edge = self.next_edge_id;
        self.edges.insert(new_edge, merged);
        self.next_edge_id += 1;
        Ok(new_edge)
    }
    /// remove the hyperedge with the given id
    pub fn remove_hyperedge(&mut self, Index(index): EdgeId) -> crate::Result<HashSet<VertexId>> {
        match self.edges.remove(&index) {
            Some(v) => Ok(v),
            None => Err(crate::Error::HyperedgeDoesNotExist(index.to_string())),
        }
    }
    /// removes the vertex with the given id and all of its associated hyperedges
    pub fn remove_vertex(&mut self, Index(index): VertexId) -> crate::Result<Node<N>> {
        match self.vertices.remove(&index) {
            Some(node) => {
                // Remove all hyperedges containing this vertex
                self.edges.retain(|_, vertices| !vertices.contains(&index));
                return Ok(node);
            }
            None => Err(crate::Error::VertexDoesNotExist(index.to_string())),
        }
    }
    /// returns the total number of hyperedges in the hypergraph
    pub fn total_edges(&self) -> usize {
        self.edges.len()
    }
    /// returns the total number of vertices in the hypergraph
    pub fn total_vertices(&self) -> usize {
        self.vertices.len()
    }
    /// returns the degree of a given vertex where the degree is the number of hyperedges that
    /// contain the vertex
    pub fn vertex_degree(&self, Index(index): VertexId) -> crate::Result<usize> {
        if !self.check_vertex(&Index(index)) {
            return Err(crate::Error::VertexDoesNotExist(index.to_string()));
        }

        let degree = self
            .edges
            .values()
            .filter(|vertices| vertices.contains(&index))
            .count();
        Ok(degree)
    }
    /// update the weight of a given vertex
    pub fn update_vertex_weight(
        &mut self,
        Index(index): VertexId,
        new_weight: N,
    ) -> crate::Result<()> {
        if self.check_vertex(&Index(index)) {
            self.vertices
                .insert(Index(index), Node::new(Index(index), new_weight));
            Ok(())
        } else {
            Err(crate::Error::VertexDoesNotExist(index.to_string()))
        }
    }
}

impl HyperGraph<()> {
    pub fn add_vertex_empty(&mut self) -> VertexId {
        self.add_vertex(())
    }
}

impl<N> Default for HyperGraph<N>
where
    N: core::cmp::Eq + core::hash::Hash,
{
    fn default() -> Self {
        Self::new()
    }
}
