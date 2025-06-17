/*
    appellation: impl_iter <module>
    authors: @FL03
*/
use crate::HyperMap;
use crate::iter::*;

use core::hash::{BuildHasher, Hash};
use rshyper::{GraphProps, GraphType, RawIndex};

/// implements various iterators for the [`HyperMap`]
impl<N, E, A, S, K, Idx> HyperMap<N, E, A, S>
where
    S: BuildHasher,
    A: GraphProps<Kind = K, Ix = Idx>,
    K: GraphType,
    Idx: RawIndex + Eq + Hash,
{
    /// returns an iterator over the node entries within the hypergraph, yielding a 2-tuple
    /// consisting of:
    ///
    /// - `0`: a reference to the [`VertexId`](rshyper::VertexId)
    /// - `1`: a reference to the corresponding [`Node`](rshyper::Node).
    pub fn iter_nodes(&self) -> NodeIter<'_, N, Idx> {
        NodeIter {
            iter: self.nodes().iter(),
        }
    }
    /// returns a mutable iterator over the node entries within the hypergraph,yielding a
    /// 2-tuple consisting of:
    ///
    /// - `0`: a reference to the [`VertexId`](rshyper::VertexId)
    /// - `1mutable reference to the corresponding [`Node`](rshyper::Node).
    pub fn iter_nodes_mut(&mut self) -> NodeIterMut<'_, N, Idx> {
        NodeIterMut {
            iter: self.nodes_mut().iter_mut(),
        }
    }
    /// returns an iterator over the edge entries of the hypergraph, yielding a 2-tuple
    /// consisting of:
    ///
    /// - `0`: a reference to the [`EdgeId`](rshyper::EdgeId)
    /// - `1`: a reference to the corresponding [`Edge`](rshyper::Edge).
    pub fn iter_edges(&self) -> EdgeIter<'_, E, K, Idx, S> {
        EdgeIter {
            iter: self.edges().iter(),
        }
    }
    /// returns a mutable iterator over the edge entries of the hypergraph, yielding a 2-tuple
    /// consisting of:
    ///
    ///  - `0`: a reference to the [`EdgeId`](rshyper::EdgeId)
    ///  - `1`: a mutable reference to the corresponding [`Edge`](rshyper::Edge).
    pub fn iter_edges_mut(&mut self) -> EdgeIterMut<'_, E, K, Idx, S> {
        EdgeIterMut {
            iter: self.edges_mut().iter_mut(),
        }
    }
    /// returns an immutable iterator over each of the associated identifiers of the edges
    /// within the graph.
    pub fn iter_edge_ids(&self) -> EdgeKeys<'_, E, K, Idx, S> {
        EdgeKeys {
            iter: self.edges().keys(),
        }
    }
    /// returns a mutable iterator over each of the _values_, or [`Edge`](rshyper::Edge),
    /// associated with the edges of the graph.
    pub fn facets(&self) -> Facets<'_, E, K, Idx, S> {
        Facets {
            iter: self.edges().values(),
        }
    }
    /// returns a mutable iterator over each of the _values_, or [`Edge`](rshyper::Edge),
    /// associated with the edges of the graph, yielding mutable references to the surfaces.
    pub fn facets_mut(&mut self) -> FacetsMut<'_, E, K, Idx, S> {
        FacetsMut {
            iter: self.edges_mut().values_mut(),
        }
    }
    /// returns an iterator over the keys of the nodes, yielding the indices of the entries.
    pub fn points(&self) -> Points<'_, N, Idx> {
        Points {
            iter: self.nodes().keys(),
        }
    }
    /// returns an iterator over all the nodes of the hypergraph, producing [`Node`](rshyper::Node)
    /// until exhausted.
    pub fn vertices(&self) -> Vertices<'_, N, Idx> {
        Vertices {
            iter: self.nodes().values(),
        }
    }
    /// returns a mutable iterator over all the nodes of the hypergraph, producing mutable
    /// references to [`Node`](rshyper::Node) until exhausted.
    pub fn vertices_mut(&mut self) -> VerticesMut<'_, N, Idx> {
        VerticesMut {
            iter: self.nodes_mut().values_mut(),
        }
    }
    /// returns a sequential iterator over the edges of the hypergraph
    pub fn iter_seq_facets(&self) -> SeqFacetIter<'_, E, K, Idx, S> {
        SeqFacetIter {
            values: self.facets(),
            keys: self.history().edges().iter(),
        }
    }
    /// returns a sequential iterator over the nodes of the hypergraph producing items of type
    /// [`Node`](rshyper::Node) in the order they were inserted.
    pub fn iter_seq_vertices(&self) -> SeqVertexIter<'_, N, Idx> {
        SeqVertexIter {
            values: self.vertices(),
            keys: self.history().nodes().iter(),
        }
    }
    /// returns a parallel iterator over the nodes of the hypergraph, yielding pairs of
    /// [`VertexId`](rshyper::VertexId) and the corresponding [`Node`](rshyper::Node).
    #[cfg(feature = "rayon")]
    pub fn par_iter_nodes(&self) -> ParNodeValues<'_, N, Idx>
    where
        N: Send + Sync,
        Idx: Send + Sync,
    {
        ParNodeValues {
            iter: self.nodes().par_values(),
        }
    }
    /// returns a mutable parallel iterator over the nodes of the hypergraph, yielding pairs of
    /// references to the [`Node`](rshyper::Node) in the hypergraph.
    #[cfg(feature = "rayon")]
    pub fn par_iter_nodes_mut(&mut self) -> ParNodeValuesMut<'_, N, Idx>
    where
        N: Send + Sync,
        Idx: Send + Sync,
    {
        ParNodeValuesMut {
            iter: self.nodes_mut().par_values_mut(),
        }
    }
    /// returns a parallel iterator over the surfaces of the hypergraph, yielding pairs of
    /// [`EdgeId`](rshyper::EdgeId) and the corresponding [`Edge`](rshyper::Edge).
    #[cfg(feature = "rayon")]
    pub fn par_iter_facets(&self) -> ParEdgeValues<'_, E, K, Idx, S>
    where
        E: Send + Sync,
        K: Send + Sync,
        Idx: Send + Sync,
        S: Send + Sync,
    {
        ParEdgeValues {
            iter: self.edges().par_values(),
        }
    }
    /// returns a mutable parallel iterator over the surfaces of the hypergraph, yielding pairs of
    /// [`EdgeId`](rshyper::EdgeId) and a mutable reference to the corresponding [`Edge`](rshyper::Edge).
    #[cfg(feature = "rayon")]
    pub fn par_iter_facets_mut(&mut self) -> ParEdgeValuesMut<'_, E, K, Idx, S>
    where
        E: Send + Sync,
        K: Send + Sync,
        Idx: Send + Sync,
        S: Send + Sync,
    {
        ParEdgeValuesMut {
            iter: self.edges_mut().par_values_mut(),
        }
    }
}
