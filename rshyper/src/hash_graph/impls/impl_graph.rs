/*
    appellation: impl_graph <module>
    authors: @FL03
*/
use crate::GraphKind;
use crate::hash_graph::{HashFacet, HashGraph, VertexSet};
use num_traits::One;
use rshyper_core::Weight;
use rshyper_core::cmp::{HyperFacet, HyperNode};
use rshyper_core::index::{EdgeId, RawIndex, VertexId};

impl<N, E, K, Idx> HashGraph<N, E, K, Idx>
where
    E: Eq + core::hash::Hash,
    N: Eq + core::hash::Hash,
    K: GraphKind,
    Idx: Eq + RawIndex + core::hash::Hash,
{
    /// add a new hyperedge composed of the given vertices, using the default weight, and
    /// returns the corresponding id
    pub fn add_edge<I>(&mut self, vertices: I) -> crate::Result<EdgeId<Idx>>
    where
        I: IntoIterator<Item = VertexId<Idx>>,
        Idx: Copy + core::ops::Add<Output = Idx> + One,
        E: Default,
    {
        self.add_surface(vertices, Weight(E::default()))
    }
    /// add a new hyperedge with the given vertices and weight, returning its ID;
    pub fn add_surface<I>(&mut self, vertices: I, weight: Weight<E>) -> crate::Result<EdgeId<Idx>>
    where
        I: IntoIterator<Item = VertexId<Idx>>,
        E: Eq + core::hash::Hash,
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
        let edge_id = self.next_edge_id();
        // handle the case where the edge has no associated vertices
        if vset.is_empty() {
            return Err(crate::Error::EmptyHyperedge);
        }
        let surface = crate::HyperFacet::new(edge_id, vset, weight);
        // insert the new hyperedge into the adjacency map
        self.surfaces_mut().insert(edge_id, surface);
        Ok(edge_id)
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
        self.surfaces_mut().clear();
        self.nodes_mut().clear();
        self
    }
    /// returns the size, or order, of a particular hyperedge
    pub fn find_order_of_edge(&self, index: &EdgeId<Idx>) -> crate::Result<usize> {
        self.surfaces()
            .get(index)
            .map(|edge| edge.len())
            .ok_or(crate::Error::EdgeNotFound)
    }
    /// returns the degree of a given vertex where the degree is the number of hyperedges that
    /// contain the vertex
    pub fn get_degree_of_node<Q>(&self, index: &Q) -> usize
    where
        Q: Eq + core::hash::Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.surfaces()
            .values()
            .filter(|facet| facet.edge().points().contains(index))
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
            .surfaces()
            .iter()
            .filter_map(|(&edge_id, facet)| {
                if facet.contains_vertex(index) {
                    Some(edge_id)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        Ok(edges)
    }

    /// returns an immutable reference to the hyperedge
    pub fn get_surface<Q>(
        &self,
        index: &Q,
    ) -> crate::Result<&crate::HyperFacet<E, VertexSet<Idx>, K, Idx>>
    where
        Q: Eq + core::hash::Hash,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.surfaces()
            .get(index)
            .ok_or_else(|| crate::Error::EdgeNotFound)
    }

    pub fn get_surface_mut<Q>(
        &mut self,
        index: &Q,
    ) -> crate::Result<&mut crate::HyperFacet<E, VertexSet<Idx>, K, Idx>>
    where
        Q: Eq + core::hash::Hash,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.surfaces_mut()
            .get_mut(index)
            .ok_or_else(|| crate::Error::EdgeNotFound)
    }

    /// retrieves the set of nodes composing the given edge
    pub fn get_edge_nodes<Q>(&self, index: &Q) -> crate::Result<Vec<&HyperNode<N, Idx>>>
    where
        Q: Eq + core::hash::Hash,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        let surface = self.get_surface(&index)?;
        let nodes = surface
            .points()
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
        self.get_surface(index).map(|edge| edge.points())
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
    /// merges two hyperedges into one (combining their vertices;
    ///
    /// **note:** the method requires the edge types `E` to implement the [`Add`](core::ops::Add)
    /// trait
    pub fn merge_edges<Q>(&mut self, e1: &Q, e2: &Q) -> crate::Result<EdgeId<Idx>>
    where
        Idx: Copy + core::ops::Add<Output = Idx> + One,
        Q: Eq + core::hash::Hash,
        E: Clone + core::ops::Add<Output = E>,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        let set1 = self
            .surfaces_mut()
            .remove(e1)
            .ok_or(crate::Error::EdgeNotFound)?;
        let set2 = self
            .surfaces_mut()
            .remove(e2)
            .ok_or(crate::Error::EdgeNotFound)?;
        let merged = set1
            .points()
            .union(set2.points())
            .copied()
            .collect::<VertexSet<_>>();

        let weight = set1.weight().clone() + set2.weight().clone();
        let new_edge = self.next_edge_id();
        // initialize a new facet using the merged vertices, new index, and source weight
        let merged_facet = HyperFacet::new(new_edge, merged, weight);
        self.surfaces_mut().insert(new_edge, merged_facet);
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
        self.surfaces().values().for_each(|surface| {
            if surface.contains_vertex(index) {
                neighbors.extend(
                    surface
                        .edge()
                        .points()
                        .iter()
                        .copied()
                        .filter(|v| v != index),
                );
            }
        });
        Ok(neighbors)
    }
    /// remove the hyperedge with the given id
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, index)))]
    #[inline]
    pub fn remove_surface<Q>(&mut self, index: &Q) -> crate::Result<HashFacet<E, K, Idx>>
    where
        Q: Eq + core::hash::Hash,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.surfaces_mut()
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
                self.surfaces_mut()
                    .retain(|_, facet| !facet.contains_vertex(index));
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
