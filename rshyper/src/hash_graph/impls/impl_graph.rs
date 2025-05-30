use crate::cmp::HyperNode;
use crate::hash_graph::HashGraph;
use crate::index::{EdgeId, HashIndex, VertexId};
use std::collections::HashSet;

impl<N, E, Idx> HashGraph<N, E, Idx>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    Idx: HashIndex,
{
    /// clears all vertices and hyperedges, resetting the hypergraph
    pub fn clear(&mut self) -> &mut Self {
        self.nodes_mut().clear();
        self.edges_mut().clear();
        self
    }
    /// check if a hyperedge with the given id exists
    pub fn contains_edge(&self, index: &EdgeId<Idx>) -> bool {
        self.edges().contains_key(index)
    }
    /// check if a vertex with the given id exists
    pub fn contains_node(&self, index: &VertexId<Idx>) -> bool {
        self.nodes().contains_key(index)
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
    /// returns the total number of vertices in the hypergraph
    pub fn total_vertices(&self) -> usize {
        self.nodes().len()
    }
    /// returns the size, or order, of a particular hyperedge
    pub fn get_edge_order(&self, index: &EdgeId<Idx>) -> crate::Result<usize> {
        self.edges()
            .get(index)
            .map(|vertices| vertices.len())
            .ok_or(crate::Error::IndexNotFound)
    }
    /// returns all hyperedges containing a given vertex
    pub fn get_edges_with_vertex(&self, index: &VertexId<Idx>) -> crate::Result<Vec<EdgeId<Idx>>>
    where
        Idx: Copy,
    {
        // handle the case where the vertex does not exist
        if !self.contains_node(index) {
            return Err(crate::Error::NodeNotFound);
        }
        //
        let edges = self
            .edges()
            .iter()
            .filter_map(|(&edge_id, vertices)| {
                if vertices.contains(index) {
                    Some(edge_id)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        Ok(edges)
    }
    /// retrieves a reference to the facet (hyperedge with an associated weight)
    pub fn get_facet(&self, index: &EdgeId<Idx>) -> crate::Result<&E> {
        self.facets()
            .get(index)
            .ok_or_else(|| crate::Error::EdgeNotFound)
    }
    /// retrieves a mutable reference to the facet (hyperedge with an associated weight)
    pub fn get_facet_mut(&mut self, index: &EdgeId<Idx>) -> crate::Result<&mut E> {
        self.facets_mut()
            .get_mut(index)
            .ok_or_else(|| crate::Error::EdgeNotFound)
    }
    /// retrieves the set of nodes composing the given edge
    pub fn get_nodes_for_edge(
        &self,
        index: &EdgeId<Idx>,
    ) -> crate::Result<Vec<&HyperNode<N, Idx>>> {
        let vertices = self.get_vertices_for_edge(&index)?;
        let nodes = vertices
            .iter()
            .map(|v| self.get_vertex_weight(&v).expect("vertex not found"))
            .collect::<Vec<_>>();
        Ok(nodes)
    }
    /// returns the set of vertices composing the given edge
    pub fn get_vertices_for_edge(
        &self,
        index: &EdgeId<Idx>,
    ) -> crate::Result<&HashSet<VertexId<Idx>>> {
        self.edges()
            .get(index)
            .ok_or_else(|| crate::Error::EdgeNotFound)
    }
    /// returns the degree of a given vertex where the degree is the number of hyperedges that
    /// contain the vertex
    pub fn get_vertex_degree(&self, index: &VertexId<Idx>) -> crate::Result<usize> {
        if !self.contains_node(index) {
            return Err(crate::Error::NodeNotFound);
        }

        let degree = self
            .edges()
            .values()
            .filter(|vertices| vertices.contains(index))
            .count();
        Ok(degree)
    }
    /// returns the weight of a particular vertex
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub fn get_vertex_weight(&self, index: &VertexId<Idx>) -> crate::Result<&HyperNode<N, Idx>> {
        self.nodes().get(index).ok_or(crate::Error::NodeNotFound)
    }
    /// returns a mutable reference to the weight of a vertex
    pub fn get_vertex_weight_mut(
        &mut self,
        index: &VertexId<Idx>,
    ) -> crate::Result<&mut HyperNode<N, Idx>> {
        self.nodes_mut()
            .get_mut(index)
            .ok_or(crate::Error::NodeNotFound)
    }
    /// add a new hyperedge with the given vertices and return its ID
    pub fn insert_edge<I>(&mut self, vertices: I) -> crate::Result<EdgeId<Idx>>
    where
        I: IntoIterator<Item = VertexId<Idx>>,
        Idx: Copy + core::ops::Add<Output = Idx> + num_traits::One,
    {
        // collect the vertices into a HashSet to ensure uniqueness
        let vset = HashSet::from_iter(
            vertices
                .into_iter()
                .map(|v| {
                    // ensure the vertex ID is valid
                    if !self.contains_node(&v) {
                        return Err(crate::Error::NodeNotFound);
                    }
                    Ok(v)
                })
                .filter_map(Result::ok),
        );
        // fetch the next edge index
        let eid = self.next_edge_id();
        // handle the case where the edge has no associated vertices
        if vset.is_empty() {
            return Err(crate::Error::EmptyHyperedge);
        }
        // insert the new hyperedge into the adjacency map
        self.edges_mut().insert(eid, vset);
        Ok(eid)
    }
    /// insert a new facet (hyperedge with an associated weight) into the hypergraph;
    /// if the facet, or hyperedge, already exists, it will replace the existing value with
    /// the given
    pub fn insert_edge_weight(&mut self, index: EdgeId<Idx>, facet: E) -> crate::Result<EdgeId<Idx>>
    where
        Idx: Copy,
    {
        if !self.contains_edge(&index) {
            return Err(crate::Error::EdgeNotFound);
        }
        let _prev = self.facets_mut().insert(index, facet);
        Ok(index)
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
        Idx: Copy + core::ops::Add<Output = Idx> + num_traits::One,
    {
        // insert the edge and get its ID
        let index = self.insert_edge(vertices)?;
        // insert the facet with the given weight
        self.insert_edge_weight(index, weight)
    }

    /// insert a new node with the given weight and return its index
    pub fn insert_node(&mut self, weight: N) -> VertexId<Idx>
    where
        Idx: Copy + core::ops::Add<Output = Idx> + num_traits::One,
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
        Idx: Copy + core::ops::Add<Output = Idx> + num_traits::One,
    {
        self.insert_node(N::default())
    }
    /// merges two hyperedges into one (combining their vertices)
    pub fn merge_edges(&mut self, e1: &EdgeId<Idx>, e2: &EdgeId<Idx>) -> crate::Result<EdgeId<Idx>>
    where
        Idx: Copy + core::ops::Add<Output = Idx> + num_traits::One,
    {
        let set1 = self
            .edges_mut()
            .remove(e1)
            .ok_or(crate::Error::IndexNotFound)?;
        let set2 = self
            .edges_mut()
            .remove(e2)
            .ok_or(crate::Error::IndexNotFound)?;
        let merged = set1.union(&set2).copied().collect::<HashSet<_>>();
        let new_edge = self.next_edge_id();
        self.edges_mut().insert(new_edge, merged);
        Ok(new_edge)
    }
    /// returns a set of vertices that are in the hyperedge with the given id
    pub fn neighbors(&self, index: &VertexId<Idx>) -> crate::Result<HashSet<VertexId<Idx>>>
    where
        Idx: Copy,
    {
        if !self.contains_node(index) {
            return Err(crate::Error::IndexNotFound);
        }
        // initialize an empty set to hold the neighbors
        let mut neighbors = HashSet::new();
        // iterate through all the connections
        self.edges().values().for_each(|vertices| {
            if vertices.contains(index) {
                neighbors.extend(vertices.iter().copied().filter(|v| v != index));
            }
        });
        Ok(neighbors)
    }
    /// remove the hyperedge with the given id
    pub fn remove_edge(&mut self, index: &EdgeId<Idx>) -> crate::Result<HashSet<VertexId<Idx>>> {
        self.edges_mut()
            .remove(index)
            .ok_or(crate::Error::IndexNotFound)
    }
    /// removes the vertex with the given id and all of its associated hyperedges
    pub fn remove_vertex(&mut self, index: &VertexId<Idx>) -> crate::Result<HyperNode<N, Idx>> {
        self.nodes_mut()
            .remove(index)
            .map(|node| {
                // Remove all hyperedges containing this vertex
                self.edges_mut()
                    .retain(|_, vertices| !vertices.contains(index));
                node
            })
            .ok_or(crate::Error::IndexNotFound)
    }
    /// update the weight of a given vertex
    pub fn set_vertex_weight(&mut self, index: &VertexId<Idx>, weight: N) -> crate::Result<()>
    where
        N: Clone,
    {
        self.nodes_mut()
            .get_mut(index)
            .map(|node| {
                node.set_weight(weight.clone());
            })
            .ok_or(crate::Error::IndexNotFound)
    }
}
