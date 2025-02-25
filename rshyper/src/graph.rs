/*
    Appellation: graph <module>
    Contrib: @FL03
*/
use crate::types::{EdgeId, VertexId};
use num::Zero;
use std::collections::{HashMap, HashSet};

/// An implementation of a hypergraph;
#[derive(Clone, Debug)]
pub struct HyperGraph {
    pub(crate) vertices: HashSet<VertexId>,
    pub(crate) hyperedges: HashMap<EdgeId, HashSet<VertexId>>,
    pub(crate) next_vertex_id: VertexId,
    pub(crate) next_edge_id: EdgeId,
}

impl HyperGraph {
    // Create a new empty HyperGraph
    pub fn new() -> Self {
        HyperGraph {
            vertices: HashSet::new(),
            hyperedges: HashMap::new(),
            next_vertex_id: VertexId::zero(),
            next_edge_id: EdgeId::zero(),
        }
    }
    /// insert a
    pub fn add_vertex(&mut self) -> VertexId {
        let vertex_id = self.next_vertex_id;
        self.vertices.insert(vertex_id);
        self.next_vertex_id += 1;
        vertex_id
    }
    /// check if a vertex with the given id exists
    pub fn check_vertex(&self, vertex_id: VertexId) -> bool {
        self.vertices.contains(&vertex_id)
    }
    /// check if a hyperedge with the given id exists
    pub fn check_hyperedge(&self, edge_id: EdgeId) -> bool {
        self.hyperedges.contains_key(&edge_id)
    }
    /// add a new hyperedge with the given vertices and return its ID
    pub fn add_hyperedge(&mut self, vertices: Vec<VertexId>) -> crate::Result<EdgeId> {
        // Verify all vertices exist
        for &vertex in &vertices {
            if !self.vertices.contains(&vertex) {
                return Err(crate::Error::VertexDoesNotExist(vertex.to_string()));
            }
        }

        let edge_id = self.next_edge_id;
        let vertex_set: HashSet<VertexId> = vertices.into_iter().collect();

        // Don't allow empty hyperedges
        if vertex_set.is_empty() {
            return Err(crate::Error::EmptyHyperedge);
        }

        self.hyperedges.insert(edge_id, vertex_set);
        self.next_edge_id += 1;
        Ok(edge_id)
    }

    /// removes the vertex with the given id and all of its associated hyperedges
    pub fn remove_vertex(&mut self, id: VertexId) -> crate::Result {
        if !self.vertices.remove(&id) {
            return Err(crate::Error::VertexDoesNotExist(id.to_string()));
        }

        // Remove all hyperedges containing this vertex
        self.hyperedges
            .retain(|_, vertices| !vertices.contains(&id));
        Ok(())
    }

    /// remove the hyperedge with the given id
    pub fn remove_hyperedge(&mut self, id: EdgeId) -> crate::Result {
        if let None = self.hyperedges.remove(&id) {
            return Err(crate::Error::HyperedgeDoesNotExist(id.to_string()));
        }
        Ok(())
    }
    /// returns a set of vertices that are in the hyperedge with the given id
    pub fn get_neighbors(&self, id: VertexId) -> crate::Result<HashSet<VertexId>> {
        if !self.check_vertex(id) {
            return Err(crate::Error::VertexDoesNotExist(id.to_string()));
        }

        let mut neighbors = HashSet::new();
        for vertices in self.hyperedges.values() {
            if vertices.contains(&id) {
                neighbors.extend(vertices.iter().filter(|&&v| v != id));
            }
        }
        Ok(neighbors)
    }
    /// returns the degree of a given vertex where the degree is the number of hyperedges that
    /// contain the vertex
    pub fn vertex_degree(&self, vertex_id: VertexId) -> crate::Result<usize> {
        if !self.check_vertex(vertex_id) {
            return Err(crate::Error::VertexDoesNotExist(vertex_id.to_string()));
        }

        let degree = self
            .hyperedges
            .values()
            .filter(|vertices| vertices.contains(&vertex_id))
            .count();
        Ok(degree)
    }
}
