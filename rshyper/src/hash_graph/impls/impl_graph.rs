/*
    appellation: impl_graph <module>
    authors: @FL03
*/
use crate::GraphKind;
use crate::hash_graph::{HashGraph, VertexSet};
use num_traits::One;
use rshyper_core::index::{EdgeId, HashIndex, RawIndex, VertexId};
use rshyper_core::prelude::{HyperNode, Weight};

impl<N, E, K, Idx> HashGraph<N, E, K, Idx>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    K: GraphKind,
    Idx: Eq + RawIndex + core::hash::Hash,
{
    /// add a new hyperedge with the given vertices and return its ID
    pub fn add_edge<I>(&mut self, vertices: I) -> crate::Result<EdgeId<Idx>>
    where
        I: IntoIterator<Item = VertexId<Idx>>,
        Idx: Copy + core::ops::Add<Output = Idx> + One,
    {
        // collect the vertices into a HashSet to ensure uniqueness
        let vset = vertices
            .into_iter()
            .map(|v| {
                // ensure the vertex ID is valid
                if !self.contains_node(&v) {
                    return Err(crate::Error::NodeNotFound);
                }
                Ok(v)
            })
            .filter_map(Result::ok)
            .collect::<VertexSet<_>>();
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
    /// add a new hyperedge with the given vertices and weight, returning its ID;
    pub fn add_edge_with_weight<I>(&mut self, vertices: I, weight: E) -> crate::Result<EdgeId<Idx>>
    where
        I: Clone + IntoIterator<Item = VertexId<Idx>>,
        E: Eq + core::hash::Hash,
        Idx: Copy + core::ops::Add<Output = Idx> + One,
    {
        // insert the edge and get its ID
        let index = self.add_edge(vertices)?;
        // insert the facet with the given weight
        let _prev = self.add_facet(index, Weight(weight));
        Ok(index)
    }
    /// add a facet associated with the given edge index
    pub fn add_facet(
        &mut self,
        index: EdgeId<Idx>,
        facet: Weight<E>,
    ) -> crate::Result<Option<Weight<E>>>
    where
        Idx: Copy,
    {
        if !self.contains_edge(&index) {
            return Err(crate::Error::EdgeNotFound);
        }
        let _prev = self.facets_mut().insert(index, facet);
        Ok(_prev)
    }
    /// add a new node with the given weight and return its index
    pub fn add_node(&mut self, weight: N) -> VertexId<Idx>
    where
        Idx: Copy + core::ops::Add<Output = Idx> + One,
    {
        // generate a new vertex ID
        let idx = self.next_vertex_id();
        // initialize a new node with the given weight & index
        let node = HyperNode::new(idx, Weight(weight));
        // insert the new node into the vertices map
        self.nodes_mut().insert(idx, node);
        idx
    }
    /// add a new vertex with the default weight and return its ID
    pub fn add_vertex(&mut self) -> VertexId<Idx>
    where
        N: Default,
        Idx: Copy + core::ops::Add<Output = Idx> + One,
    {
        self.add_node(N::default())
    }
    /// reset the hypergraph by clearing all nodes, edges, and facets
    pub fn clear(&mut self) -> &mut Self {
        self.edges_mut().clear();
        self.facets_mut().clear();
        self.nodes_mut().clear();
        self
    }
    /// returns the size, or order, of a particular hyperedge
    pub fn find_order_of_edge(&self, index: &EdgeId<Idx>) -> crate::Result<usize> {
        self.edges()
            .get(index)
            .map(|vertices| vertices.len())
            .ok_or(crate::Error::EdgeNotFound)
    }
    /// returns the degree of a given vertex where the degree is the number of hyperedges that
    /// contain the vertex
    pub fn get_degree_of_node<Q>(&self, index: &Q) -> usize
    where
        Q: Eq + core::hash::Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.edges()
            .values()
            .filter(|vertices| vertices.contains(index))
            .count()
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
    pub fn get_facet<Q>(&self, index: &Q) -> crate::Result<&Weight<E>>
    where
        Q: Eq + core::hash::Hash,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.facets()
            .get(index)
            .ok_or_else(|| crate::Error::EdgeNotFound)
    }
    /// retrieves a mutable reference to the facet (hyperedge with an associated weight)
    pub fn get_facet_mut<Q>(&mut self, index: &Q) -> crate::Result<&mut Weight<E>>
    where
        Q: Eq + core::hash::Hash,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.facets_mut()
            .get_mut(index)
            .ok_or_else(|| crate::Error::EdgeNotFound)
    }
    /// retrieves the set of nodes composing the given edge
    pub fn get_edge_nodes<Q>(&self, index: &Q) -> crate::Result<Vec<&HyperNode<N, Idx>>>
    where
        Q: Eq + core::hash::Hash,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        let vertices = self.get_edge_vertices(&index)?;
        let nodes = vertices
            .iter()
            .map(|v| self.get_node(&v).expect("vertex not found"))
            .collect::<Vec<_>>();
        Ok(nodes)
    }
    /// returns the set of vertices composing the given edge
    pub fn get_edge_vertices<Q>(&self, index: &Q) -> crate::Result<&VertexSet<Idx>>
    where
        Q: Eq + core::hash::Hash,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.edges()
            .get(index)
            .ok_or_else(|| crate::Error::EdgeNotFound)
    }
    /// returns the weight of a particular vertex
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub fn get_node(&self, index: &VertexId<Idx>) -> crate::Result<&HyperNode<N, Idx>> {
        self.nodes().get(index).ok_or(crate::Error::NodeNotFound)
    }
    /// returns a mutable reference to the weight of a vertex
    pub fn get_node_mut<Q>(&mut self, index: &Q) -> crate::Result<&mut HyperNode<N, Idx>>
    where
        Q: Eq + core::hash::Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.nodes_mut()
            .get_mut(index)
            .ok_or(crate::Error::NodeNotFound)
    }
    /// merges two hyperedges into one (combining their vertices)
    pub fn merge_edges<Q>(&mut self, e1: &Q, e2: &Q) -> crate::Result<EdgeId<Idx>>
    where
        Idx: Copy + core::ops::Add<Output = Idx> + One,
        Q: Eq + core::hash::Hash,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        let set1 = self
            .edges_mut()
            .remove(e1)
            .ok_or(crate::Error::EdgeNotFound)?;
        let set2 = self
            .edges_mut()
            .remove(e2)
            .ok_or(crate::Error::EdgeNotFound)?;
        let merged = set1.union(&set2).copied().collect::<VertexSet<_>>();
        let new_edge = self.next_edge_id();
        self.edges_mut().insert(new_edge, merged);
        Ok(new_edge)
    }
    /// returns a set of vertices that are in the hyperedge with the given id
    pub fn neighbors(&self, index: &VertexId<Idx>) -> crate::Result<VertexSet<Idx>>
    where
        Idx: Copy,
    {
        if !self.contains_node(index) {
            return Err(crate::Error::NodeNotFound);
        }
        // initialize an empty set to hold the neighbors
        let mut neighbors = VertexSet::new();
        // iterate through all the connections
        self.edges().values().for_each(|vertices| {
            if vertices.contains(index) {
                neighbors.extend(vertices.iter().copied().filter(|v| v != index));
            }
        });
        Ok(neighbors)
    }
    /// remove the hyperedge with the given id
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, index)))]
    #[inline]
    pub fn remove_edge<Q>(&mut self, index: &Q) -> crate::Result<VertexSet<Idx>>
    where
        Q: Eq + core::hash::Hash,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.edges_mut()
            .remove(index)
            .ok_or(crate::Error::EdgeNotFound)
    }
    /// removes the vertex with the given id and all of its associated hyperedges
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, index)))]
    #[inline]
    pub fn remove_vertex<Q>(&mut self, index: &Q) -> crate::Result<HyperNode<N, Idx>>
    where
        Q: Eq + core::fmt::Debug + core::hash::Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        #[cfg(feature = "tracing")]
        tracing::info!("removing the vertex {index:?} from the hypergraph...");
        self.nodes_mut()
            .remove(index)
            .map(|node| {
                // Remove all hyperedges containing this vertex
                self.edges_mut()
                    .retain(|_, vertices| !vertices.contains(index));
                node
            })
            .ok_or(crate::Error::NodeNotFound)
    }
    /// update the weight of a given vertex
    #[inline]
    pub fn set_vertex_weight<Q>(&mut self, index: &Q, weight: N) -> crate::Result<&mut Self>
    where
        Q: Eq + core::hash::Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        let _ = self
            .nodes_mut()
            .get_mut(index)
            .map(|node| {
                node.set_weight(weight);
            })
            .ok_or(crate::Error::NodeNotFound)?;
        Ok(self)
    }
}

impl<N, E, K, Idx> HashGraph<N, E, K, Idx>
where
    N: Eq + core::hash::Hash,
    E: Eq + core::hash::Hash,
    Idx: HashIndex,
    K: GraphKind,
{
    #[doc(hidden)]
    #[deprecated(note = "use `get_edge_weight")]
    /// retrieves a reference to the facet (hyperedge with an associated weight)
    pub fn _get_facet<Q>(&self, index: &Q) -> crate::Result<&Weight<E>>
    where
        Q: Eq + core::hash::Hash,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.facets()
            .get(index)
            .ok_or_else(|| crate::Error::EdgeNotFound)
    }
    #[doc(hidden)]
    #[deprecated(note = "use `get_edge_weight_mut")]
    /// retrieves a mutable reference to the facet (hyperedge with an associated weight)
    pub fn _get_facet_mut<Q>(&mut self, index: &Q) -> crate::Result<&mut Weight<E>>
    where
        Q: Eq + core::hash::Hash,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.facets_mut()
            .get_mut(index)
            .ok_or_else(|| crate::Error::EdgeNotFound)
    }

    #[deprecated(note = "use `add_edge")]
    /// add a new hyperedge with the given vertices and return its ID
    pub fn insert_edge<I>(&mut self, vertices: I) -> crate::Result<EdgeId<Idx>>
    where
        I: IntoIterator<Item = VertexId<Idx>>,
        Idx: Copy + core::ops::Add<Output = Idx> + One,
    {
        // collect the vertices into a HashSet to ensure uniqueness
        let vset = vertices
            .into_iter()
            .map(|v| {
                // ensure the vertex ID is valid
                if !self.contains_node(&v) {
                    return Err(crate::Error::NodeNotFound);
                }
                Ok(v)
            })
            .filter_map(Result::ok)
            .collect::<VertexSet<_>>();
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

    #[deprecated(note = "use `add_edge_with_weight", since = "0.0.8")]
    /// insert a new hyperedge with the given vertices and weight, returning its ID;
    pub fn insert_edge_with_weight<I>(
        &mut self,
        vertices: I,
        weight: E,
    ) -> crate::Result<EdgeId<Idx>>
    where
        I: Clone + IntoIterator<Item = VertexId<Idx>>,
        E: Eq + core::hash::Hash,
        Idx: Copy + core::ops::Add<Output = Idx> + One,
    {
        self.add_edge_with_weight(vertices, weight)
    }
    #[deprecated(note = "use `add_facet", since = "0.0.8")]
    /// insert a facet associated with the given edge index
    pub fn insert_facet(&mut self, index: EdgeId<Idx>, facet: E) -> crate::Result<EdgeId<Idx>>
    where
        Idx: Copy,
    {
        self.add_facet(index, Weight(facet)).map(|_| index)
    }
    #[deprecated(note = "use `add_node", since = "0.0.8")]
    /// insert a new node with the given weight and return its index
    pub fn insert_node(&mut self, weight: N) -> VertexId<Idx>
    where
        Idx: Copy + core::ops::Add<Output = Idx> + One,
    {
        self.add_node(weight)
    }
    #[deprecated(note = "use `add_vertex", since = "0.0.8")]
    /// insert a new vertex with the default weight and return its ID
    pub fn insert_vertex(&mut self) -> VertexId<Idx>
    where
        N: Default,
        Idx: Copy + core::ops::Add<Output = Idx> + One,
    {
        self.add_vertex()
    }
}
