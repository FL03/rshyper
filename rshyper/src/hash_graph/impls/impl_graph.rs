/*
    appellation: impl_graph <module>
    authors: @FL03
*/
use crate::hash_graph::{HashFacet, HashGraph, VertexSet};
use crate::{GraphAttributes, GraphKind};
use core::hash::Hash;
use num_traits::One;
use rshyper_core::index::{EdgeId, RawIndex, VertexId};
use rshyper_core::{HyperFacet, HyperNode, Weight};

impl<N, E, A, K, Idx> HashGraph<N, E, A>
where
    E: Eq + Hash,
    N: Eq + Hash,
    A: GraphAttributes<Idx = Idx, Kind = K>,
    K: GraphKind,
    Idx: RawIndex + Eq + Hash,
{
    /// insert a new [`surface`](HyperFacet) into the hypergraph with the given vertices and
    /// using the logical [`Default`] for the weight
    pub fn add_edge<I>(&mut self, vertices: I) -> crate::Result<EdgeId<Idx>>
    where
        I: IntoIterator<Item = VertexId<Idx>>,
        Idx: Copy + core::ops::Add<Output = Idx> + One,
        E: Default,
    {
        self.add_surface(vertices, Weight(E::default()))
    }
    /// insert a new hyperedge composed of the given vertices and weight, returning its unique
    /// index
    pub fn add_surface<I>(&mut self, vertices: I, weight: Weight<E>) -> crate::Result<EdgeId<Idx>>
    where
        I: IntoIterator<Item = VertexId<Idx>>,
        E: Eq + Hash,
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
    /// returns a set of edge indices that contain the given vertex
    pub fn find_edges_with_node(&self, index: &VertexId<Idx>) -> crate::Result<Vec<EdgeId<Idx>>>
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
    /// returns a set of vertices that are in the hyperedge with the given id
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "neighbors", target = "hash_graph")
    )]
    pub fn find_node_neighbors(&self, index: &VertexId<Idx>) -> crate::Result<VertexSet<Idx>>
    where
        Idx: Copy,
    {
        if !self.contains_node(index) {
            #[cfg(feature = "tracing")]
            tracing::error!("the vertex {index:?} does not exist in the hypergraph");
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
    /// retrieves the set of nodes composing the given edge
    pub fn get_edge_nodes<Q>(&self, index: &Q) -> crate::Result<Vec<&HyperNode<N, Idx>>>
    where
        Q: Eq + Hash,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        let surface = self.get_surface(&index)?;
        let nodes = surface
            .points()
            .iter()
            .map(|v| self.get_node(v).expect("vertex not found"))
            .collect::<Vec<_>>();
        Ok(nodes)
    }
    /// returns the number of vertices, or order, composing the hyperedge with the given id
    pub fn get_edge_order(&self, index: &EdgeId<Idx>) -> crate::Result<usize> {
        self.get_surface(index).map(|edge| edge.len())
    }
    /// returns the set of vertices composing the given edge
    pub fn get_edge_vertices<Q>(&self, index: &Q) -> crate::Result<&VertexSet<Idx>>
    where
        Q: Eq + Hash,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.get_surface(index).map(|edge| edge.points())
    }
    /// returns a mutable reference to the set of vertices composing the given edge
    pub fn get_edge_vertices_mut<Q>(&mut self, index: &Q) -> crate::Result<&mut VertexSet<Idx>>
    where
        Q: Eq + Hash,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.get_surface_mut(index).map(|edge| edge.points_mut())
    }
    /// returns an immutable reference to the weight of a hyperedge
    pub fn get_edge_weight<Q>(&self, index: &Q) -> crate::Result<&Weight<E>>
    where
        Q: Eq + Hash,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.get_surface(index).map(|edge| edge.weight())
    }
    /// returns a mutable reference to the weight of a hyperedge
    pub fn get_edge_weight_mut<Q>(&mut self, index: &Q) -> crate::Result<&mut Weight<E>>
    where
        Q: Eq + Hash,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.get_surface_mut(index).map(|edge| edge.weight_mut())
    }
    /// returns the degree of a given vertex where the degree is the number of hyperedges that
    /// contain the vertex
    pub fn get_node_degree<Q>(&self, index: &Q) -> usize
    where
        Q: Eq + Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.surfaces()
            .values()
            .filter(|facet| facet.edge().points().contains(index))
            .count()
    }
    /// returns the weight of a particular vertex
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn get_node<Q>(&self, index: &Q) -> crate::Result<&HyperNode<N, Idx>>
    where
        Q: Eq + Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.nodes().get(index).ok_or(crate::Error::NodeNotFound)
    }
    /// returns a mutable reference to the weight of a vertex
    pub fn get_node_mut<Q>(&mut self, index: &Q) -> crate::Result<&mut HyperNode<N, Idx>>
    where
        Q: Eq + Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.nodes_mut()
            .get_mut(index)
            .ok_or(crate::Error::NodeNotFound)
    }
    /// returns an immutable reference to the weight of a vertex
    pub fn get_node_weight<Q>(&self, index: &Q) -> crate::Result<&Weight<N>>
    where
        Q: Eq + Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.get_node(index).map(|node| node.weight())
    }
    /// returns a mutable reference to the weight of a vertex
    pub fn get_node_weight_mut<Q>(&mut self, index: &Q) -> crate::Result<&mut Weight<N>>
    where
        Q: Eq + Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.get_node_mut(index).map(|node| node.weight_mut())
    }
    /// returns an immutable reference to the [`HashFacet`] associated with the given index
    pub fn get_surface<Q>(&self, index: &Q) -> crate::Result<&HashFacet<E, K, Idx>>
    where
        Q: Eq + Hash,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.surfaces()
            .get(index)
            .ok_or_else(|| crate::Error::EdgeNotFound)
    }
    /// returns a mutable reference to the [`HashFacet`] associated with the given index
    pub fn get_surface_mut<Q>(&mut self, index: &Q) -> crate::Result<&mut HashFacet<E, K, Idx>>
    where
        Q: Eq + Hash,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.surfaces_mut()
            .get_mut(index)
            .ok_or_else(|| crate::Error::EdgeNotFound)
    }
    /// merges two hyperedges into one (combining their vertices;
    ///
    /// **note:** the method requires the edge types `E` to implement the [`Add`](core::ops::Add)
    /// trait
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, target = "hash_graph", name = "merge_edges")
    )]
    pub fn merge_edges<Q>(&mut self, e1: &Q, e2: &Q) -> crate::Result<EdgeId<Idx>>
    where
        Idx: Copy + core::ops::Add<Output = Idx> + One,
        Q: Eq + Hash + core::fmt::Debug,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
        for<'a> &'a E: core::ops::Add<Output = E>,
    {
        // remove the edges from the surfaces map
        let s1 = self.remove_surface(e1)?;
        tracing::debug!("removed edge {e1:?} with vertices {ep:?}", ep = s1.points());
        let s2 = self.remove_surface(e2)?;
        tracing::debug!("removed edge {e2:?} with vertices {ep:?}", ep = s2.points());
        // merge the vertices of the two edges
        let vertices = s1
            .points()
            .union(s2.points())
            .copied()
            .collect::<VertexSet<_>>();
        // sum the weights of the two edges
        let weight = s1.weight().view() + s2.weight().view();
        // generate a new edge index
        let edge_id = self.next_edge_id();
        // initialize a new facet using the merged vertices, new index, and source weight
        let surface = HyperFacet::new(edge_id, vertices, weight);
        // insert the new hyperedge into the surfaces map
        self.surfaces_mut().insert(edge_id, surface);
        // return the new edge ID
        Ok(edge_id)
    }

    /// remove the hyperedge with the given id
    #[inline]
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "remove_surface", target = "hash_graph")
    )]
    pub fn remove_surface<Q>(&mut self, index: &Q) -> crate::Result<HashFacet<E, K, Idx>>
    where
        Q: Eq + Hash,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.surfaces_mut()
            .remove(index)
            .ok_or(crate::Error::EdgeNotFound)
    }
    /// removes the vertex with the given id and all of its associated hyperedges
    #[inline]
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "remove_node", target = "hash_graph")
    )]
    pub fn remove_node<Q>(&mut self, index: &Q) -> crate::Result<HyperNode<N, Idx>>
    where
        Q: Eq + core::fmt::Debug + Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        #[cfg(feature = "tracing")]
        tracing::debug!("removing the vertex {index:?} from the hypergraph...");
        self.nodes_mut()
            .remove(index)
            .map(|node| {
                // Remove all hyperedges containing this vertex
                self.retain_surfaces(|_, facet| !facet.contains_vertex(index));
                node
            })
            .ok_or(crate::Error::NodeNotFound)
    }
    /// returns a mutable reference to the set of hyperedges
    pub fn retain_surfaces<F>(&mut self, f: F) -> &mut Self
    where
        F: FnMut(&EdgeId<Idx>, &mut HashFacet<E, K, Idx>) -> bool,
    {
        self.surfaces_mut().retain(f);
        self
    }
    /// retain nodes in the hypergraph based on a predicate;
    ///
    /// ## Saftey
    ///
    /// This method is unsafe because it allows for the removal of nodes based on a predicate,
    /// without removing the associated hyperedges. This can lead to inconsistencies in the
    /// graph structure if not used carefully. It is the caller's responsibility to ensure that
    /// the predicate does not leave the graph in an invalid state.
    pub unsafe fn retain_nodes<F>(&mut self, f: F) -> &mut Self
    where
        F: FnMut(&VertexId<Idx>, &mut HyperNode<N, Idx>) -> bool,
    {
        self.nodes_mut().retain(f);
        self
    }
    /// update the weight of an edge with the given index
    #[inline]
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn set_edge_weight<Q>(&mut self, index: &Q, weight: Weight<E>) -> crate::Result<&mut Self>
    where
        Q: Eq + Hash,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.get_edge_weight_mut(index)
            .map(|w| {
                *w = weight;
            })
            .map_err(|_| crate::Error::EdgeNotFound)?;
        Ok(self)
    }
    /// update the weight of a given vertex
    #[inline]
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn set_node_weight<Q>(&mut self, index: &Q, weight: Weight<N>) -> crate::Result<&mut Self>
    where
        Q: Eq + Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        let _ = self
            .get_node_weight_mut(index)
            .map(|w| {
                *w = weight;
            })
            .map_err(|_| crate::Error::NodeNotFound)?;
        Ok(self)
    }
}

#[allow(deprecated)]
impl<N, E, A, K, Idx> HashGraph<N, E, A>
where
    E: Eq + Hash,
    N: Eq + Hash,
    A: GraphAttributes<Idx = Idx, Kind = K>,
    K: GraphKind,
    Idx: Eq + RawIndex + Hash,
{
    #[deprecated(
        note = "use `find_edges_with_node` instead; this method will be removed in a future release",
        since = "0.0.10"
    )]
    pub fn get_edges_with_vertex(&self, index: &VertexId<Idx>) -> crate::Result<Vec<EdgeId<Idx>>>
    where
        Idx: Copy,
    {
        self.find_edges_with_node(index)
    }
    #[deprecated(
        note = "use `find_node_neighbors` instead; this method will be removed the next major release",
        since = "0.0.10"
    )]
    pub fn neighbors(&self, index: &VertexId<Idx>) -> crate::Result<VertexSet<Idx>>
    where
        Idx: Copy,
    {
        self.find_node_neighbors(index)
    }
    #[deprecated(
        note = "use `remove_node` instead; this method will be removed the next major release",
        since = "0.0.10"
    )]
    pub fn remove_vertex<Q>(&mut self, index: &Q) -> crate::Result<HyperNode<N, Idx>>
    where
        Q: Eq + core::fmt::Debug + Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.remove_node(index)
    }
    #[deprecated(
        note = "use `set_node_weight` instead; this method will be removed in a future release",
        since = "0.0.10"
    )]
    pub fn update_vertex_weight<Q>(&mut self, index: &Q, weight: N) -> crate::Result<&mut Self>
    where
        Q: Eq + Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.set_node_weight(index, Weight(weight))
    }
}
