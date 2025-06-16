/*
    appellation: impl_deprecated <module>
    authors: @FL03
*/
use crate::HyperMap;
use core::borrow::Borrow;
use core::hash::{BuildHasher, Hash};
use rshyper::prelude::{EdgeId, GraphProps, GraphType, RawIndex, VertexId};

impl<N, E, A, S, Idx, K> HyperMap<N, E, A, S>
where
    A: GraphProps<Kind = K, Ix = Idx>,
    S: BuildHasher,
    Idx: RawIndex,
    K: GraphType,
{
    #[deprecated(since = "0.1.3", note = "use is_node_in_domain` instead")]
    pub fn contains_node_in_edge<Q, Q2>(&self, index: &Q, vertex: &Q2) -> bool
    where
        A::Ix: Eq + Hash,
        Q: Eq + Hash + ?Sized,
        Q2: Eq + Hash,
        EdgeId<A::Ix>: Borrow<Q>,
        VertexId<A::Ix>: Borrow<Q2>,
    {
        if let Some(surface) = self.surfaces().get(index) {
            return surface.contains(vertex);
        }
        false
    }
    #[deprecated(since = "0.1.2", note = "use `contains_edge` instead")]
    pub fn contains_surface<Q>(&self, index: &Q) -> bool
    where
        A::Ix: Eq + Hash,
        Q: Eq + Hash + ?Sized,
        EdgeId<A::Ix>: Borrow<Q>,
    {
        self.surfaces().contains_key(index)
    }
    #[deprecated(
        note = "use `size` instead; this method will be removed in a future release",
        since = "0.1.2"
    )]
    pub fn total_edges(&self) -> usize {
        self.surfaces().len()
    }
    #[deprecated(
        note = "use `order` instead; this method will be removed in a future release",
        since = "0.1.2"
    )]
    pub fn total_nodes(&self) -> usize {
        self.nodes().len()
    }
    #[deprecated(
        note = "use `order` instead; this method will be removed in a future release",
        since = "0.1.0"
    )]
    pub fn total_vertices(&self) -> usize {
        self.order()
    }
}
