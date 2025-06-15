/*
    appellation: impl_iter <module>
    authors: @FL03
*/
use crate::HyperMap;
use crate::iter::*;

use core::hash::{BuildHasher, Hash};
use rshyper_core::{GraphAttributes, GraphType, RawIndex};

impl<N, E, A, S, K, Idx> HyperMap<N, E, A, S>
where
    N: Eq + Hash,
    E: Eq + Hash,
    S: BuildHasher + Default,
    A: GraphAttributes<Kind = K, Ix = Idx>,
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
{
    /// returns an iterator over the nodes of the hypergraph, yielding pairs of [`VertexId`](rshyper_core::VertexId)
    /// and the corresponding [`Node`](rshyper_core::Node).
    pub fn node_iter(&self) -> NodeIter<'_, N, Idx> {
        NodeIter {
            iter: self.nodes().iter(),
        }
    }
    /// returns a mutable iterator over the nodes of the hypergraph, yielding pairs of
    /// [`VertexId`](rshyper_core::VertexId) and a mutable reference to the corresponding
    /// [`Node`](rshyper_core::Node).
    pub fn node_iter_mut(&mut self) -> NodeIterMut<'_, N, Idx> {
        NodeIterMut {
            iter: self.nodes_mut().iter_mut(),
        }
    }
    /// returns an iterator over the surfaces of the hypergraph, yielding pairs of [`EdgeId`](rshyper_core::EdgeId)
    /// and the corresponding [`Surface`](rshyper_core::Surface).
    pub fn surface_iter(&self) -> SurfaceIter<'_, E, K, Idx, S> {
        SurfaceIter {
            iter: self.surfaces().iter(),
        }
    }
    /// returns a mutable iterator over the surfaces of the hypergraph, yielding pairs of
    /// [`EdgeId`](rshyper_core::EdgeId) and a mutable reference to the corresponding
    /// [`Surface`](rshyper_core::Surface).
    pub fn surface_iter_mut(&mut self) -> SurfaceIterMut<'_, E, K, Idx, S> {
        SurfaceIterMut {
            iter: self.surfaces_mut().iter_mut(),
        }
    }
    /// returns an iterator over the keys of the surfaces, yielding the indices of the entries.
    pub fn edges(&self) -> Edges<'_, E, K, Idx, S> {
        Edges {
            iter: self.surfaces().keys(),
        }
    }
    /// returns an iterator producing references to the surfaces of the graph
    pub fn facets(&self) -> Facets<'_, E, K, Idx, S> {
        Facets {
            iter: self.surfaces().values(),
        }
    }
    /// returns a mutable iterator producing mutable references to the surfaces of the graph
    pub fn facets_mut(&mut self) -> FacetsMut<'_, E, K, Idx, S> {
        FacetsMut {
            iter: self.surfaces_mut().values_mut(),
        }
    }
    /// returns an iterator over the keys of the nodes, yielding the indices of the entries.
    pub fn points(&self) -> Points<'_, N, Idx> {
        Points {
            iter: self.nodes().keys(),
        }
    }
    /// returns an iterator over all the nodes of the hypergraph, producing [`Node`](rshyper_core::Node)
    /// until exhausted.
    pub fn vertices(&self) -> Vertices<'_, N, Idx> {
        Vertices {
            iter: self.nodes().values(),
        }
    }
    /// returns a mutable iterator over all the nodes of the hypergraph, producing mutable
    /// references to [`Node`](rshyper_core::Node) until exhausted.
    pub fn vertices_mut(&mut self) -> VerticesMut<'_, N, Idx> {
        VerticesMut {
            iter: self.nodes_mut().values_mut(),
        }
    }
    /// returns a sequential iterator over the edges of the hypergraph
    pub fn iter_edges_seq(&self) -> SeqEdgeIter<'_, E, K, Idx, S> {
        SeqEdgeIter {
            values: self.facets(),
            keys: self.history().edges().iter(),
        }
    }
    /// returns a sequential iterator over the nodes of the hypergraph, yielding pairs of
    /// [`VertexId`](rshyper_core::VertexId) and the corresponding [`Node`](rshyper_core::Node).
    pub fn iter_nodes_seq(&self) -> SeqNodeIter<'_, N, Idx> {
        SeqNodeIter {
            values: self.vertices(),
            keys: self.history().nodes().iter(),
        }
    }
    /// returns a parallel iterator over the nodes of the hypergraph, yielding pairs of
    /// [`VertexId`](rshyper_core::VertexId) and the corresponding [`Node`](rshyper_core::Node).
    #[cfg(feature = "rayon")]
    pub fn node_par_iter(&self) -> NodeParIter<'_, N, Idx>
    where
        N: Send + Sync,
        Idx: Send + Sync,
    {
        NodeParIter {
            iter: self.node_iter(),
        }
    }
    /// returns a mutable parallel iterator over the nodes of the hypergraph, yielding pairs of
    /// [`VertexId`](rshyper_core::VertexId) and a mutable reference to the corresponding
    /// [`Node`](rshyper_core::Node).
    #[cfg(feature = "rayon")]
    pub fn node_par_iter_mut(&mut self) -> NodeParIterMut<'_, N, Idx>
    where
        N: Send + Sync,
        Idx: Send + Sync,
    {
        NodeParIterMut {
            iter: self.node_iter_mut(),
        }
    }
    /// returns a parallel iterator over the surfaces of the hypergraph, yielding pairs of
    /// [`EdgeId`](rshyper_core::EdgeId) and the corresponding [`Surface`](rshyper_core::Surface).
    #[cfg(feature = "rayon")]
    pub fn surface_par_iter(&self) -> SurfaceParIter<'_, E, K, Idx, S>
    where
        E: Send + Sync,
        K: Send + Sync,
        Idx: Send + Sync,
        S: Send + Sync,
    {
        SurfaceParIter {
            iter: self.surface_iter(),
        }
    }
    /// returns a mutable parallel iterator over the surfaces of the hypergraph, yielding pairs of
    /// [`EdgeId`](rshyper_core::EdgeId) and a mutable reference to the corresponding [`Surface`](rshyper_core::Surface).
    #[cfg(feature = "rayon")]
    pub fn surface_par_iter_mut(&mut self) -> SurfaceParIterMut<'_, E, K, Idx, S>
    where
        E: Send + Sync,
        K: Send + Sync,
        Idx: Send + Sync,
        S: Send + Sync,
    {
        SurfaceParIterMut {
            iter: self.surface_iter_mut(),
        }
    }
}
