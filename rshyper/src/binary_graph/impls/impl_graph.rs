/*
    appellation: impl_graph <module>
    authors: @FL03
*/
use crate::binary_graph::{BinaryGraph, aliases::*};
use alloc::vec::Vec;
use rshyper_core::index::{EdgeId, RawIndex, VertexId};
use rshyper_core::node::Node;
use rshyper_core::{GraphAttributes, GraphKind, Weight};

impl<N, E, A, K, Idx> BinaryGraph<N, E, A>
where
    N: Ord,
    E: Ord,
    A: GraphAttributes<Idx = Idx, Kind = K>,
    K: GraphKind,
    Idx: RawIndex + Ord,
{
    /// returns a reference to the node associated with the given key
    pub fn get_node<Q>(&self, key: &Q) -> Option<&Node<N, Idx>>
    where
        Q: ?Sized + Ord,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.nodes().get(key)
    }
    /// returns a mutable reference to the node associated with the given key
    pub fn get_node_mut<Q>(&mut self, key: &Q) -> Option<&mut Node<N, Idx>>
    where
        Q: ?Sized + Ord,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.nodes_mut().get_mut(key)
    }
    /// returns a reference to the weight of the node associated with the given key
    pub fn get_node_weight<Q>(&self, key: &Q) -> Option<&Weight<N>>
    where
        Q: ?Sized + Ord,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.nodes().get(key).map(|node| node.weight())
    }
    /// returns a mutable reference to the weight of the node associated with the given key
    pub fn get_node_weight_mut<Q>(&mut self, key: &Q) -> Option<&mut Weight<N>>
    where
        Q: ?Sized + Ord,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.nodes_mut().get_mut(key).map(|node| node.weight_mut())
    }
    /// returns a reference to the surface associated with the given key
    pub fn get_surface<Q>(&self, key: &Q) -> Option<&BTreeFacet<E, K, Idx>>
    where
        Q: ?Sized + Ord,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.surfaces().get(key)
    }
    /// returns a mutable reference to the surface associated with the given key
    pub fn get_surface_mut<Q>(&mut self, key: &Q) -> Option<&mut BTreeFacet<E, K, Idx>>
    where
        Q: ?Sized + Ord,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.surfaces_mut().get_mut(key)
    }
    /// returns a reference to the weight of the surface associated with the given key
    pub fn get_surface_weight<Q>(&self, key: &Q) -> Option<&Weight<E>>
    where
        Q: ?Sized + Ord,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.surfaces().get(key).map(|surface| surface.weight())
    }
    /// returns a mutable reference to the weight of the surface associated with the given key
    pub fn get_surface_weight_mut<Q>(&mut self, key: &Q) -> Option<&mut Weight<E>>
    where
        Q: ?Sized + Ord,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.surfaces_mut()
            .get_mut(key)
            .map(|surface| surface.weight_mut())
    }
    /// returns the [`HyperNode`] of each vertex that makes up a given edge
    pub fn get_surface_nodes<Q>(&self, key: &Q) -> Vec<&Node<N, Idx>>
    where
        Q: ?Sized + Ord,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        let mut nodes = Vec::new();
        if let Some(surface) = self.surfaces().get(key) {
            for vertex in surface.points() {
                if let Some(node) = self.nodes().get(vertex) {
                    nodes.push(node);
                }
            }
        }
        nodes
    }
    /// returns a reference to the set of vertices that makeup a given edge
    pub fn get_surface_vertices<Q>(&self, key: &Q) -> Option<&VertexBSet<Idx>>
    where
        Q: ?Sized + Ord,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.surfaces().get(key).map(|surface| surface.points())
    }
    /// returns a mutable reference to the set of vertices that makeup a given edge
    pub fn get_surface_vertices_mut<Q>(&mut self, key: &Q) -> Option<&mut VertexBSet<Idx>>
    where
        Q: ?Sized + Ord,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.surfaces_mut()
            .get_mut(key)
            .map(|surface| surface.points_mut())
    }
}
