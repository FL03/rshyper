/*
    Appellation: graph <module>
    Contrib: @FL03
*/
use crate::types::{EdgeId, VertexId};
use num::Zero;
use std::collections::{HashMap, HashSet};

/// A hash-based hypergraph implementation
#[derive(Clone, Debug)]
pub struct HyperGraph<N = ()> {
    pub(crate) hyperedges: HashMap<EdgeId, HashSet<VertexId>>,
    pub(crate) vertices: HashMap<VertexId, N>,
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
            hyperedges: HashMap::new(),
            next_vertex_id: VertexId::zero(),
            next_edge_id: EdgeId::zero(),
        }
    }
    /// add a new hyperedge with the given vertices and return its ID
    pub fn add_hyperedge(&mut self, vertices: Vec<VertexId>) -> crate::Result<EdgeId> {
        // Verify all vertices exist
        for &vertex in &vertices {
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

        self.hyperedges.insert(edge_id, vertex_set);
        self.next_edge_id += 1;
        Ok(edge_id)
    }
    /// insert a new vertex with the given weight and return its ID
    pub fn add_vertex(&mut self, weight: N) -> VertexId {
        let vertex_id = self.next_vertex_id;
        self.vertices.insert(vertex_id, weight);
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
    pub fn check_vertex(&self, vertex_id: &VertexId) -> bool {
        self.vertices.contains_key(vertex_id)
    }
    /// check if a hyperedge with the given id exists
    pub fn check_hyperedge(&self, edge_id: &EdgeId) -> bool {
        self.hyperedges.contains_key(edge_id)
    }
    /// returns a set of vertices that are in the hyperedge with the given id
    pub fn get_neighbors(&self, id: VertexId) -> crate::Result<HashSet<VertexId>> {
        if !self.check_vertex(&id) {
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
    /// returns all hyperedges containing a given vertex
    pub fn get_vertex_edges(&self, id: VertexId) -> crate::Result<Vec<EdgeId>> {
        if !self.check_vertex(&id) {
            return Err(crate::Error::VertexDoesNotExist(id.to_string()));
        }
        let edges = self
            .hyperedges
            .iter()
            .filter_map(|(edge_id, vertices)| {
                if vertices.contains(&id) {
                    Some(*edge_id)
                } else {
                    None
                }
            })
            .collect();
        Ok(edges)
    }
    /// returns the weight of a particular vertex
    pub fn get_vertex_weight(&self, id: VertexId) -> crate::Result<&N> {
        match self.vertices.get(&id) {
            Some(weight) => Ok(weight),
            None => Err(crate::Error::VertexDoesNotExist(id.to_string())),
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
        let set1 = self.hyperedges.remove(&e1).unwrap();
        let set2 = self.hyperedges.remove(&e2).unwrap();
        let merged: HashSet<VertexId> = set1.union(&set2).cloned().collect();
        let new_edge = self.next_edge_id;
        self.hyperedges.insert(new_edge, merged);
        self.next_edge_id += 1;
        Ok(new_edge)
    }
    /// remove the hyperedge with the given id
    pub fn remove_hyperedge(&mut self, id: EdgeId) -> crate::Result<HashSet<VertexId>> {
        match self.hyperedges.remove(&id) {
            Some(v) => Ok(v),
            None => Err(crate::Error::HyperedgeDoesNotExist(id.to_string())),
        }
    }
    /// removes the vertex with the given id and all of its associated hyperedges
    pub fn remove_vertex(&mut self, id: VertexId) -> crate::Result<N> {
        match self.vertices.remove(&id) {
            Some(node) => {
                // Remove all hyperedges containing this vertex
                self.hyperedges
                    .retain(|_, vertices| !vertices.contains(&id));
                return Ok(node);
            }
            None => Err(crate::Error::VertexDoesNotExist(id.to_string())),
        }
    }
    /// returns the degree of a given vertex where the degree is the number of hyperedges that
    /// contain the vertex
    pub fn vertex_degree(&self, vid: VertexId) -> crate::Result<usize> {
        if !self.check_vertex(&vid) {
            return Err(crate::Error::VertexDoesNotExist(vid.to_string()));
        }

        let degree = self
            .hyperedges
            .values()
            .filter(|vertices| vertices.contains(&vid))
            .count();
        Ok(degree)
    }
    /// update the weight of a given vertex
    pub fn update_vertex_weight(&mut self, id: VertexId, new_weight: N) -> crate::Result<()> {
        if self.check_vertex(&id) {
            self.vertices.insert(id, new_weight);
            Ok(())
        } else {
            Err(crate::Error::VertexDoesNotExist(id.to_string()))
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
