/*
    appellation: impl_iter <module>
    authors: @FL03
*/
use crate::hash_graph::HashGraph;
use crate::hash_graph::iter::*;

use core::hash::{BuildHasher, Hash};
use rshyper_core::{GraphAttributes, GraphType, RawIndex};

impl<N, E, A, S, K, Idx> HashGraph<N, E, A, S>
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
    /// returns an iterator over the keys of the nodes, yielding the indices of the entries.
    pub fn vertices(&self) -> Vertices<'_, N, Idx> {
        Vertices {
            iter: self.nodes().keys(),
        }
    }
    /// returns an iterator over the keys of the surfaces, yielding the indices of the entries.
    pub fn surface_keys(&self) -> SurfaceKeys<'_, E, K, Idx, S> {
        SurfaceKeys {
            iter: self.surfaces().keys(),
        }
    }
}
