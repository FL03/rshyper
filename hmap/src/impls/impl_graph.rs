/*
    appellation: impl_graph <module>
    authors: @FL03
*/
use crate::{HashEdge, HyperMap};
use core::hash::{BuildHasher, Hash};
use rshyper::error::{Error, Result};
use rshyper::idx::{EdgeId, RawIndex, VertexId, VertexSet};
use rshyper::{AddStep, GraphProps, GraphType};
use rshyper::{Edge, Node, Weight};

/// private implementations of the [`HyperMap`] providing methods, for convenience and
/// consistency.
impl<N, E, A, K, Idx, S> HyperMap<N, E, A, S>
where
    A: GraphProps<Ix = Idx, Kind = K>,
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "trace", target = "hyper_map")
    )]
    /// this method is responsible for directly registering new surfaces with the system,
    /// implementing additional checks to ensure the validity of the instance. More
    /// specifically, it ensures that:
    ///
    /// - the surface must not be empty
    /// - the associated id must be recorded in the ledger, but not present within the graph
    ///
    /// if **any** of these condition are not met, an error will be thrown.
    pub(crate) fn add_hyperedge(&mut self, surface: HashEdge<E, K, Idx, S>) -> Result<EdgeId<Idx>>
    where
        Idx: Clone,
    {
        // ensure the surface is valid
        if surface.is_empty() {
            #[cfg(feature = "tracing")]
            tracing::error!(
                "attempted to insert an empty hyperedge with id {id}",
                id = surface.id()
            );
            return Err(Error::EmptyHyperedge);
        }
        // verify the edge id is already recorded in the history
        if !self.history().contains_edge(surface.id()) && !self.contains_edge(surface.id()) {
            #[cfg(feature = "tracing")]
            tracing::error!(
                "the surface with id {} is not recorded in the history",
                surface.id()
            );
            return Err(Error::EdgeNotFound);
        }
        // get the id of the surface
        let id = surface.id().clone();
        #[cfg(feature = "tracing")]
        tracing::debug!("inserting a new hyperedge ({id}) into the graph...");
        // insert the new hyperedge into the adjacency map
        self.edges_mut().insert(id.clone(), surface);
        // return the id
        Ok(id)
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "trace", target = "hyper_map")
    )]
    /// this method is responsible for directly registering new nodes with the system,
    /// implementing additional checks to ensure the validity of the instance. More
    /// specifically, it ensures that:
    ///
    /// - the associated is is recorded in the ledger
    /// - the node doesn't already exist in the graph
    ///
    /// if **any** of these condition are not met, an error will be thrown.
    pub(crate) fn add_hypernode(&mut self, data: Node<N, Idx>) -> Result<VertexId<Idx>>
    where
        Idx: Clone,
    {
        // verify the edge id is already recorded in the history
        if !self.history().contains_node(data.id()) && !self.contains_node(data.id()) {
            #[cfg(feature = "tracing")]
            tracing::error!(
                "the surface with id {} is not recorded in the history",
                data.id()
            );
            // self.history_mut().add_edge(surface.id().clone());
            return Err(Error::NodeNotFound);
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
}
/// this implementation of the [`HyperMap`] works to provide fundamental manipulation methods
/// alongside additional functional accessors, validators, and more.
impl<N, E, A, K, Idx, S> HyperMap<N, E, A, S>
where
    A: GraphProps<Ix = Idx, Kind = K>,
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    /// add a new hyperedge, using the given vertices and the logical [`Default`] for the
    /// weight of type `T` and returning the corresponding edge index.
    pub fn add_edge<I>(&mut self, vertices: I) -> Result<EdgeId<Idx>>
    where
        I: IntoIterator<Item = VertexId<Idx>>,
        Idx: AddStep<Output = Idx> + Clone,
        E: Default,
        S: Default,
    {
        self.add_surface(vertices, Default::default())
    }
    /// add a new node with the given weight and return its index
    pub fn add_node(&mut self, Weight(weight): Weight<N>) -> Result<VertexId<Idx>>
    where
        Idx: AddStep<Output = Idx> + Copy,
    {
        // generate a new index to identify the new node
        let ndx = self.next_vertex_id();
        // initialize a new node with the given weight & index
        let node = Node::new(ndx, weight);
        // insert the new node into the vertices map
        self.add_hypernode(node)
    }
    /// given a set of weights, insert a new node into the graph for each and return its id as
    /// an iterator of [`VertexId`]
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
    pub fn add_surface<I>(&mut self, vertices: I, weight: Weight<E>) -> Result<EdgeId<Idx>>
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
                    return Err(Error::NodeNotFound);
                }
                Ok(v)
            })
            .filter_map(Result::ok)
            .collect::<VertexSet<Idx, S>>();
        // fetch the next edge index
        let id = self.next_edge_id();
        // create a new surface
        let surface = Edge::new(id.clone(), verts, weight);
        // add the hyperedge to the graph
        self.add_hyperedge(surface)
    }
    /// add a new hypernode using the logical [`Default`] for the weight of type `N` and
    /// return its index.
    pub fn add_vertex(&mut self) -> Result<VertexId<Idx>>
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
        self.edges_mut().clear();
        self.nodes_mut().clear();
        self
    }
    /// returns a set of edge indices that contain the given vertex
    pub fn find_edges_with_node(&self, index: &VertexId<Idx>) -> Result<Vec<EdgeId<Idx>>>
    where
        Idx: Copy,
    {
        // handle the case where the vertex does not exist
        if !self.contains_node(index) {
            return Err(Error::NodeNotFound);
        }
        //
        let edges = self
            .edges()
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
        tracing::instrument(skip_all, name = "neighbors", target = "hyper_map")
    )]
    pub fn find_node_neighbors(&self, index: &VertexId<Idx>) -> Result<VertexSet<Idx>>
    where
        Idx: Copy,
    {
        if !self.contains_node(index) {
            #[cfg(feature = "tracing")]
            tracing::error!("the vertex {index:?} does not exist in the hypergraph");
            return Err(Error::NodeNotFound);
        }
        // initialize an empty set to hold the neighbors
        let mut neighbors = VertexSet::new();
        // iterate through all the connections
        self.edges().values().for_each(|surface| {
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
        tracing::instrument(skip_all, level = "trace", target = "hyper_map")
    )]
    pub fn get_edge_nodes<Q>(&self, index: &Q) -> Result<Vec<&Node<N, Idx>>>
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
    pub fn get_edge_order(&self, index: &EdgeId<Idx>) -> Result<usize> {
        self.get_surface(index).map(|edge| edge.len())
    }
    /// returns the set of vertices composing the given edge
    pub fn get_edge_domain<Q>(&self, index: &Q) -> Result<&VertexSet<Idx, S>>
    where
        Q: Eq + Hash + ?Sized,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.get_surface(index).map(|edge| edge.domain())
    }
    /// returns a mutable reference to the set of vertices composing the given edge
    pub fn get_edge_domain_mut<Q>(&mut self, index: &Q) -> Result<&mut VertexSet<Idx, S>>
    where
        Q: Eq + Hash + ?Sized,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.get_surface_mut(index).map(|edge| edge.domain_mut())
    }
    /// returns an immutable reference to the weight of a hyperedge
    pub fn get_edge_weight<Q>(&self, index: &Q) -> Result<&Weight<E>>
    where
        Q: Eq + Hash + ?Sized,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.get_surface(index).map(|edge| edge.weight())
    }
    /// returns a mutable reference to the weight of a hyperedge
    pub fn get_edge_weight_mut<Q>(&mut self, index: &Q) -> Result<&mut Weight<E>>
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
        self.edges()
            .values()
            .filter(|facet| facet.edge().domain().contains(index))
            .count()
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, target = "hyper_map", level = "trace")
    )]
    /// returns the weight of a particular vertex
    pub fn get_node<Q>(&self, index: &Q) -> Result<&Node<N, Idx>>
    where
        Q: Eq + Hash + ?Sized,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.nodes().get(index).ok_or(Error::NodeNotFound)
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, target = "hyper_map", level = "trace")
    )]
    /// returns a mutable reference to the weight of a vertex
    pub fn get_node_mut<Q>(&mut self, index: &Q) -> Result<&mut Node<N, Idx>>
    where
        Q: Eq + Hash + ?Sized,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.nodes_mut().get_mut(index).ok_or(Error::NodeNotFound)
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, target = "hyper_map", level = "trace")
    )]
    /// returns an immutable reference to the weight of a vertex
    pub fn get_node_weight<Q>(&self, index: &Q) -> Result<&Weight<N>>
    where
        Q: Eq + Hash + ?Sized,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.get_node(index).map(|node| node.weight())
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, target = "hyper_map", level = "trace")
    )]
    /// returns a mutable reference to the weight of a vertex
    pub fn get_node_weight_mut<Q>(&mut self, index: &Q) -> Result<&mut Weight<N>>
    where
        Q: Eq + Hash + ?Sized,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.get_node_mut(index).map(|node| node.weight_mut())
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, target = "hyper_map", level = "trace")
    )]
    /// returns an immutable reference to the [`HashFacet`] associated with the given index
    pub fn get_surface<Q>(&self, index: &Q) -> Result<&HashEdge<E, K, Idx, S>>
    where
        Q: Eq + Hash + ?Sized,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.edges().get(index).ok_or_else(Error::edge_not_found)
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, target = "hyper_map", level = "trace")
    )]
    /// returns a mutable reference to the [`HashFacet`] associated with the given index
    pub fn get_surface_mut<Q>(&mut self, index: &Q) -> Result<&mut HashEdge<E, K, Idx, S>>
    where
        Q: Eq + Hash + ?Sized,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.edges_mut()
            .get_mut(index)
            .ok_or_else(|| Error::EdgeNotFound)
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, target = "hyper_map", level = "trace")
    )]
    /// merge two edges within the hypergraph into one by combining their vertices and using
    /// the [`Add`](core::ops::Add) trait to merge their weights;
    /// trait
    pub fn merge_edges<Q>(&mut self, e1: &Q, e2: &Q) -> Result<EdgeId<Idx>>
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
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, target = "hyper_map", level = "trace")
    )]
    /// merge two edges within the hypergraph into one by combining their vertices and using
    /// the provided function to merge their weights;
    pub fn merge_edges_with<Q, F>(&mut self, e1: &Q, e2: &Q, f: F) -> Result<EdgeId<Idx>>
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
        let surface = Edge::new(edge_id, vertices, Weight(weight));
        // insert the new hyperedge into the surfaces map
        self.edges_mut().insert(edge_id, surface);
        // return the new edge ID
        Ok(edge_id)
    }
    #[inline]
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, target = "hyper_map", level = "trace")
    )]
    /// removes the vertex with the given id and all of its associated hyperedges
    pub fn remove_node<Q>(&mut self, index: &Q) -> Result<Node<N, Idx>>
    where
        Q: Eq + core::fmt::Debug + Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        #[cfg(feature = "tracing")]
        tracing::debug!("removing the vertex {index:?} from the hypergraph...");

        self.nodes_mut()
            .remove(index)
            .ok_or(Error::NodeNotFound)
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
        tracing::instrument(skip_all, target = "hyper_map", level = "trace")
    )]
    pub fn remove_surface<Q>(&mut self, index: &Q) -> Result<HashEdge<E, K, Idx, S>>
    where
        Q: Eq + Hash + ?Sized,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.edges_mut().remove(index).ok_or(Error::EdgeNotFound)
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, target = "hyper_map", level = "trace")
    )]
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
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, target = "hyper_map", level = "trace")
    )]
    /// retain surfaces in the hypergraph based on a predicate;
    pub fn retain_surfaces<F>(&mut self, f: F) -> &mut Self
    where
        F: FnMut(&EdgeId<Idx>, &mut HashEdge<E, K, Idx, S>) -> bool,
    {
        self.edges_mut().retain(f);
        self
    }
    #[inline]
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, target = "hyper_map", level = "trace")
    )]
    /// update the weight of an edge with the given index
    pub fn set_edge_weight<Q>(&mut self, index: &Q, weight: Weight<E>) -> Result<&mut Self>
    where
        Q: Eq + Hash + ?Sized,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.get_edge_weight_mut(index)
            .map(|w| {
                *w = weight;
            })
            .map_err(|_| Error::EdgeNotFound)?;
        Ok(self)
    }
    #[inline]
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, target = "hyper_map", level = "trace")
    )]
    /// update the weight of a given vertex
    pub fn set_node_weight<Q>(&mut self, index: &Q, weight: Weight<N>) -> Result<&mut Self>
    where
        Q: Eq + Hash + ?Sized,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.get_node_weight_mut(index)
            .map(|w| {
                *w = weight;
            })
            .map_err(|_| Error::NodeNotFound)?;
        Ok(self)
    }
}

/*
 ************* DEPRECATED *************
*/

#[doc(hidden)]
#[allow(deprecated)]
impl<N, E, A, S, K, Idx> HyperMap<N, E, A, S>
where
    A: GraphProps<Ix = Idx, Kind = K>,
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
    S: BuildHasher,
{
    #[deprecated(note = "use `get_edge_domain` instead", since = "0.1.5")]
    pub fn get_edge_vertices<Q>(&self, index: &Q) -> Result<&VertexSet<Idx, S>>
    where
        Q: Eq + Hash + ?Sized,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.get_surface(index).map(|edge| edge.domain())
    }
    #[deprecated(note = "use `get_edge_domain_mut` instead", since = "0.1.5")]
    pub fn get_edge_vertices_mut<Q>(&mut self, index: &Q) -> Result<&mut VertexSet<Idx, S>>
    where
        Q: Eq + Hash + ?Sized,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.get_surface_mut(index).map(|edge| edge.domain_mut())
    }
}
