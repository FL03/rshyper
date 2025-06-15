/*
    appellation: impl_graph <module>
    authors: @FL03
*/
use crate::{HashFacet, HyperMap};
use core::hash::{BuildHasher, Hash};
use rshyper_core::idx::{EdgeId, RawIndex, VertexId, VertexSet};
use rshyper_core::{AddStep, GraphAttributes, GraphType};
use rshyper_core::{HyperError, HyperResult, Node, Surface, Weight};

impl<N, E, A, K, Idx, S> HyperMap<N, E, A, S>
where
    A: GraphAttributes<Ix = Idx, Kind = K>,
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    /// add a new hyperedge, using the given vertices and the logical [`Default`] for the
    /// weight of type `T` and returning the corresponding edge index.
    pub fn add_edge<I>(&mut self, vertices: I) -> HyperResult<EdgeId<Idx>>
    where
        I: IntoIterator<Item = VertexId<Idx>>,
        Idx: AddStep<Output = Idx> + Clone,
        E: Default,
        S: Default,
    {
        self.add_surface(vertices, Default::default())
    }
    /// add a new hyperedge directly using an externally defined surface, returns an error if the
    /// surface is empty or if the associated edge id is not recorded in the history.
    pub(crate) fn add_hyperedge(
        &mut self,
        surface: HashFacet<E, K, Idx, S>,
    ) -> HyperResult<EdgeId<Idx>>
    where
        Idx: Clone,
    {
        // ensure the surface is valid
        if surface.is_empty() {
            return Err(HyperError::EmptyHyperedge);
        }
        // verify the edge id is already recorded in the history
        if !self.history().contains_edge(surface.id()) {
            #[cfg(feature = "tracing")]
            tracing::error!(
                "the surface with id {} is not recorded in the history",
                surface.id()
            );
            // self.history_mut().add_edge(surface.id().clone());
            return Err(HyperError::EdgeNotFound);
        }
        // get the id of the surface
        let id = surface.id().clone();
        #[cfg(feature = "tracing")]
        tracing::debug!("inserting a new hyperedge ({id}) into the graph...");
        // insert the new hyperedge into the adjacency map
        self.surfaces_mut().insert(id.clone(), surface);
        // return the id
        Ok(id)
    }
    /// directly insert a new hypernode
    pub(crate) fn add_hypernode(&mut self, data: Node<N, Idx>) -> HyperResult<VertexId<Idx>>
    where
        Idx: Clone,
    {
        // verify the edge id is already recorded in the history
        if !self.history().contains_node(data.id()) {
            #[cfg(feature = "tracing")]
            tracing::error!(
                "the surface with id {} is not recorded in the history",
                data.id()
            );
            // self.history_mut().add_edge(surface.id().clone());
            return Err(HyperError::NodeNotFound);
        }
        // get the id of the surface
        let id = data.id().clone();
        #[cfg(feature = "tracing")]
        tracing::debug!("inserting a new hypernode ({id}) into the graph...");
        // insert the new hyperedge into the adjacency map
        self.nodes_mut().insert(id.clone(), data);
        // return the id
        Ok(id)
    }
    /// add a new node with the given weight and return its index
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "trace", target = "hash_graph")
    )]
    pub fn add_node(&mut self, Weight(weight): Weight<N>) -> HyperResult<VertexId<Idx>>
    where
        Idx: AddStep<Output = Idx> + Copy,
    {
        // generate a new index to identify the new node
        let ndx = self.next_vertex_id();
        // initialize a new node with the given weight & index
        let node = Node::new(ndx, weight);
        // insert the new node into the vertices map
        self.add_hypernode(node)?;
        Ok(ndx)
    }
    /// add multiple nodes with the given weights and return their indices
    pub fn add_nodes<I>(&mut self, weights: I) -> impl Iterator<Item = VertexId<Idx>>
    where
        I: IntoIterator<Item = N>,
        Idx: AddStep<Output = Idx> + Copy,
    {
        weights
            .into_iter()
            .filter_map(|weight| self.add_node(Weight(weight)).ok())
    }
    /// add a new hyperedge with the given vertices and weight, returning the corresponding
    /// edge index.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "trace", target = "hash_graph")
    )]
    pub fn add_surface<I>(&mut self, vertices: I, weight: Weight<E>) -> HyperResult<EdgeId<Idx>>
    where
        I: IntoIterator<Item = VertexId<Idx>>,
        Idx: AddStep<Output = Idx> + Clone,
        S: Default,
    {
        // collect the vertices into a HashSet to ensure uniqueness
        let verts = vertices
            .into_iter()
            .map(|v| {
                // ensure the vertex ID is valid
                if !self.contains_node(&v) {
                    return Err(HyperError::NodeNotFound);
                }
                Ok(v)
            })
            .filter_map(HyperResult::ok)
            .collect::<VertexSet<Idx, S>>();
        // fetch the next edge index
        let edge_id = self.next_edge_id();
        // create a new surface
        let surface = Surface::new(edge_id.clone(), verts, weight);
        // add the hyperedge to the graph
        self.add_hyperedge(surface)?;
        // log the addition of the new hyperedge
        #[cfg(feature = "tracing")]
        tracing::debug!("added a new hyperedge with id {edge_id}");
        // return the edge id
        Ok(edge_id)
    }
    /// add a new hypernode using the logical [`Default`] for the weight of type `N` and
    /// return its index.
    pub fn add_vertex(&mut self) -> HyperResult<VertexId<Idx>>
    where
        N: Default,
        Idx: AddStep<Output = Idx> + Copy,
    {
        self.add_node(Default::default())
    }
    /// reset the hypergraph by clearing all nodes, edges, and facets
    pub fn clear(&mut self) -> &mut Self {
        // log the addition of the new hyperedge
        #[cfg(feature = "tracing")]
        tracing::info!("clearing the hypergraph...");
        self.surfaces_mut().clear();
        self.nodes_mut().clear();
        self
    }
    /// returns a set of edge indices that contain the given vertex
    pub fn find_edges_with_node(&self, index: &VertexId<Idx>) -> HyperResult<Vec<EdgeId<Idx>>>
    where
        Idx: Copy,
    {
        // handle the case where the vertex does not exist
        if !self.contains_node(index) {
            return Err(HyperError::NodeNotFound);
        }
        //
        let edges = self
            .surfaces()
            .iter()
            .filter_map(|(&edge_id, facet)| {
                if facet.contains(index) {
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
    pub fn find_node_neighbors(&self, index: &VertexId<Idx>) -> HyperResult<VertexSet<Idx>>
    where
        Idx: Copy,
    {
        if !self.contains_node(index) {
            #[cfg(feature = "tracing")]
            tracing::error!("the vertex {index:?} does not exist in the hypergraph");
            return Err(HyperError::NodeNotFound);
        }
        // initialize an empty set to hold the neighbors
        let mut neighbors = VertexSet::new();
        // iterate through all the connections
        self.surfaces().values().for_each(|surface| {
            if surface.contains(index) {
                neighbors.extend(
                    surface
                        .edge()
                        .domain()
                        .iter()
                        .copied()
                        .filter(|v| v != index),
                );
            }
        });
        Ok(neighbors)
    }
    /// returns a set of [`HyperNode`]s that are associated with the given edge id
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "trace", target = "hash_graph")
    )]
    pub fn get_edge_nodes<Q>(&self, index: &Q) -> HyperResult<Vec<&Node<N, Idx>>>
    where
        Q: Eq + Hash + ?Sized,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        let surface = self.get_surface(index)?;
        let nodes = surface
            .domain()
            .iter()
            .map(|v| self.get_node(v).expect("vertex not found"))
            .collect::<Vec<_>>();
        Ok(nodes)
    }
    /// returns the number of vertices within the given edge
    pub fn get_edge_order(&self, index: &EdgeId<Idx>) -> HyperResult<usize> {
        self.get_surface(index).map(|edge| edge.len())
    }
    /// returns the set of vertices composing the given edge
    pub fn get_edge_vertices<Q>(&self, index: &Q) -> HyperResult<&VertexSet<Idx, S>>
    where
        Q: Eq + Hash + ?Sized,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.get_surface(index).map(|edge| edge.domain())
    }
    /// returns a mutable reference to the set of vertices composing the given edge
    pub fn get_edge_vertices_mut<Q>(&mut self, index: &Q) -> HyperResult<&mut VertexSet<Idx, S>>
    where
        Q: Eq + Hash + ?Sized,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.get_surface_mut(index).map(|edge| edge.domain_mut())
    }
    /// returns an immutable reference to the weight of a hyperedge
    pub fn get_edge_weight<Q>(&self, index: &Q) -> HyperResult<&Weight<E>>
    where
        Q: Eq + Hash + ?Sized,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.get_surface(index).map(|edge| edge.weight())
    }
    /// returns a mutable reference to the weight of a hyperedge
    pub fn get_edge_weight_mut<Q>(&mut self, index: &Q) -> HyperResult<&mut Weight<E>>
    where
        Q: Eq + Hash + ?Sized,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.get_surface_mut(index).map(|edge| edge.weight_mut())
    }
    /// returns the degree of a given vertex where the degree is the number of hyperedges that
    /// contain the vertex
    pub fn get_node_degree<Q>(&self, index: &Q) -> usize
    where
        Q: Eq + Hash + ?Sized,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.surfaces()
            .values()
            .filter(|facet| facet.edge().domain().contains(index))
            .count()
    }
    /// returns the weight of a particular vertex
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn get_node<Q>(&self, index: &Q) -> HyperResult<&Node<N, Idx>>
    where
        Q: Eq + Hash + ?Sized,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.nodes().get(index).ok_or(HyperError::NodeNotFound)
    }
    /// returns a mutable reference to the weight of a vertex
    pub fn get_node_mut<Q>(&mut self, index: &Q) -> HyperResult<&mut Node<N, Idx>>
    where
        Q: Eq + Hash + ?Sized,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.nodes_mut()
            .get_mut(index)
            .ok_or(HyperError::NodeNotFound)
    }
    /// returns an immutable reference to the weight of a vertex
    pub fn get_node_weight<Q>(&self, index: &Q) -> HyperResult<&Weight<N>>
    where
        Q: Eq + Hash + ?Sized,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.get_node(index).map(|node| node.weight())
    }
    /// returns a mutable reference to the weight of a vertex
    pub fn get_node_weight_mut<Q>(&mut self, index: &Q) -> HyperResult<&mut Weight<N>>
    where
        Q: Eq + Hash + ?Sized,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.get_node_mut(index).map(|node| node.weight_mut())
    }
    /// returns an immutable reference to the [`HashFacet`] associated with the given index
    pub fn get_surface<Q>(&self, index: &Q) -> HyperResult<&HashFacet<E, K, Idx, S>>
    where
        Q: Eq + Hash + ?Sized,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.surfaces()
            .get(index)
            .ok_or_else(|| HyperError::EdgeNotFound)
    }
    /// returns a mutable reference to the [`HashFacet`] associated with the given index
    pub fn get_surface_mut<Q>(&mut self, index: &Q) -> HyperResult<&mut HashFacet<E, K, Idx, S>>
    where
        Q: Eq + Hash + ?Sized,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.surfaces_mut()
            .get_mut(index)
            .ok_or_else(|| HyperError::EdgeNotFound)
    }
    /// merge two edges within the hypergraph into one by combining their vertices and using
    /// the [`Add`](core::ops::Add) trait to merge their weights;
    /// trait
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, target = "hash_graph", name = "merge_edges")
    )]
    pub fn merge_edges<Q>(&mut self, e1: &Q, e2: &Q) -> HyperResult<EdgeId<Idx>>
    where
        Q: Eq + Hash + core::fmt::Debug,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
        Idx: AddStep<Output = Idx> + Copy,
        S: Default,
        for<'a> &'a E: core::ops::Add<Output = E>,
    {
        self.merge_edges_with(e1, e2, |w1, w2| {
            // use the `Add` trait to merge the weights of the two edges
            w1 + w2
        })
    }
    /// merge two edges within the hypergraph into one by combining their vertices and using
    /// the provided function to merge their weights;
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, target = "hash_graph", name = "merge_edges")
    )]
    pub fn merge_edges_with<Q, F>(&mut self, e1: &Q, e2: &Q, f: F) -> HyperResult<EdgeId<Idx>>
    where
        Q: ?Sized + Eq + Hash + core::fmt::Debug,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
        Idx: AddStep<Output = Idx> + Copy,
        F: FnOnce(&E, &E) -> E,
        S: Default,
    {
        // remove the edges from the surfaces map
        let s1 = self.remove_surface(e1)?;
        #[cfg(feature = "tracing")]
        tracing::debug!("removed edge {e1:?} with vertices {ep:?}", ep = s1.domain());
        let s2 = self.remove_surface(e2)?;
        #[cfg(feature = "tracing")]
        tracing::debug!("removed edge {e2:?} with vertices {ep:?}", ep = s2.domain());
        // merge the vertices of the two edges by unionizing their domains
        let vertices = s1
            .domain()
            .union(s2.domain())
            .copied()
            .collect::<VertexSet<Idx, S>>();
        // merge the two weights using the provided function
        let weight = f(*s1.weight().view(), *s2.weight().view());
        // generate a new edge index
        let edge_id = self.next_edge_id();
        // initialize a new facet using the merged vertices, new index, and source weight
        let surface = Surface::new(edge_id, vertices, Weight(weight));
        // insert the new hyperedge into the surfaces map
        self.surfaces_mut().insert(edge_id, surface);
        // return the new edge ID
        Ok(edge_id)
    }
    /// removes the vertex with the given id and all of its associated hyperedges
    #[inline]
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "remove_node", target = "hash_graph")
    )]
    pub fn remove_node<Q>(&mut self, index: &Q) -> HyperResult<Node<N, Idx>>
    where
        Q: Eq + core::fmt::Debug + Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        #[cfg(feature = "tracing")]
        tracing::debug!("removing the vertex {index:?} from the hypergraph...");

        self.nodes_mut()
            .remove(index)
            .ok_or(HyperError::NodeNotFound)
            .inspect(|node| {
                self.history_mut().remove_node(node.id());
                #[cfg(feature = "tracing")]
                tracing::trace!(
                    "successfully removed the node; removing edges that contained the vertex..."
                );
                // Remove all hyperedges containing this vertex
                self.retain_surfaces(|_, facet| !facet.contains(index));
                #[cfg(feature = "tracing")]
                tracing::trace!(
                    "successfully removed the edges containing the removed vertex {index:?}..."
                );
            })
    }
    /// remove the hyperedge with the given index from the hypergraph
    #[inline]
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "remove_surface", target = "hash_graph")
    )]
    pub fn remove_surface<Q>(&mut self, index: &Q) -> HyperResult<HashFacet<E, K, Idx, S>>
    where
        Q: Eq + Hash + ?Sized,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.surfaces_mut()
            .remove(index)
            .ok_or(HyperError::EdgeNotFound)
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
        F: FnMut(&VertexId<Idx>, &mut Node<N, Idx>) -> bool,
    {
        self.nodes_mut().retain(f);
        self
    }
    /// retain surfaces in the hypergraph based on a predicate;
    pub fn retain_surfaces<F>(&mut self, f: F) -> &mut Self
    where
        F: FnMut(&EdgeId<Idx>, &mut HashFacet<E, K, Idx, S>) -> bool,
    {
        self.surfaces_mut().retain(f);
        self
    }
    /// update the weight of an edge with the given index
    #[inline]
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn set_edge_weight<Q>(&mut self, index: &Q, weight: Weight<E>) -> HyperResult<&mut Self>
    where
        Q: Eq + Hash + ?Sized,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.get_edge_weight_mut(index)
            .map(|w| {
                *w = weight;
            })
            .map_err(|_| HyperError::EdgeNotFound)?;
        Ok(self)
    }
    /// update the weight of a given vertex
    #[inline]
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn set_node_weight<Q>(&mut self, index: &Q, weight: Weight<N>) -> HyperResult<&mut Self>
    where
        Q: Eq + Hash + ?Sized,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.get_node_weight_mut(index)
            .map(|w| {
                *w = weight;
            })
            .map_err(|_| HyperError::NodeNotFound)?;
        Ok(self)
    }
}
