/*
    Appellation: graph <module>
    Contrib: @FL03
*/
use crate::types::{EdgeId, VertexId};
use std::collections::{HashMap, HashSet};
use num::Zero;


/// An implementation of a hypergraph;
#[derive(Clone, Debug,)]
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

    // Add a new vertex and return its ID
    pub fn add_vertex(&mut self) -> VertexId {
        let vertex_id = self.next_vertex_id;
        self.vertices.insert(vertex_id);
        self.next_vertex_id += 1;
        vertex_id
    }

    // Add a hyperedge connecting multiple vertices
    pub fn add_hyperedge(&mut self, vertices: Vec<VertexId>) -> Result<EdgeId, String> {
        // Verify all vertices exist
        for &vertex in &vertices {
            if !self.vertices.contains(&vertex) {
                return Err(format!("Vertex {} does not exist", vertex));
            }
        }

        let edge_id = self.next_edge_id;
        let vertex_set: HashSet<VertexId> = vertices.into_iter().collect();
        
        // Don't allow empty hyperedges
        if vertex_set.is_empty() {
            return Err("Cannot create empty hyperedge".to_string());
        }

        self.hyperedges.insert(edge_id, vertex_set);
        self.next_edge_id += 1;
        Ok(edge_id)
    }

    // Remove a vertex and all associated hyperedges
    pub fn remove_vertex(&mut self, vertex_id: VertexId) -> Result<(), String> {
        if !self.vertices.remove(&vertex_id) {
            return Err(format!("Vertex {} does not exist", vertex_id));
        }

        // Remove all hyperedges containing this vertex
        self.hyperedges.retain(|_, vertices| !vertices.contains(&vertex_id));
        Ok(())
    }

    // Remove a hyperedge
    pub fn remove_hyperedge(&mut self, edge_id: EdgeId) -> Result<(), String> {
        if self.hyperedges.remove(&edge_id).is_none() {
            return Err(format!("Hyperedge {} does not exist", edge_id));
        }
        Ok(())
    }

    // Get all vertices connected to a given vertex through any hyperedge
    pub fn get_neighbors(&self, vertex_id: VertexId) -> Result<HashSet<VertexId>, String> {
        if !self.vertices.contains(&vertex_id) {
            return Err(format!("Vertex {} does not exist", vertex_id));
        }

        let mut neighbors = HashSet::new();
        for vertices in self.hyperedges.values() {
            if vertices.contains(&vertex_id) {
                neighbors.extend(vertices.iter().filter(|&&v| v != vertex_id));
            }
        }
        Ok(neighbors)
    }

    // Get degree of a vertex (number of hyperedges it belongs to)
    pub fn vertex_degree(&self, vertex_id: VertexId) -> Result<usize, String> {
        if !self.vertices.contains(&vertex_id) {
            return Err(format!("Vertex {} does not exist", vertex_id));
        }

        let degree = self.hyperedges.values()
            .filter(|vertices| vertices.contains(&vertex_id))
            .count();
        Ok(degree)
    }
}
