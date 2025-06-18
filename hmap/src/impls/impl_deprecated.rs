/*
    appellation: impl_deprecated <module>
    authors: @FL03
*/
use crate::HyperMap;
use crate::types::{EdgeEntry, EdgeMap};
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
    #[deprecated(note = "use `get_domain` instead", since = "0.1.5")]
    pub fn get_edge_vertices<Q>(&self, index: &Q) -> Result<&VertexSet<Idx, S>>
    where
        Q: Eq + Hash + ?Sized,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.get_domain(index)
    }
    #[deprecated(note = "use `get_domain_mut` instead", since = "0.1.5")]
    pub fn get_edge_vertices_mut<Q>(&mut self, index: &Q) -> Result<&mut VertexSet<Idx, S>>
    where
        Q: Eq + Hash + ?Sized,
        EdgeId<Idx>: core::borrow::Borrow<Q>,
    {
        self.get_domain_mut(index)
    }
    #[deprecated(
        since = "0.1.5",
        note = "use `edge` instead; this method will be removed in the next major release."
    )]
    pub fn surface(&mut self, index: EdgeId<Idx>) -> EdgeEntry<'_, E, K, Idx, S>
    where
        Idx: Eq + Hash,
    {
        self.edge(index)
    }
    #[deprecated(
        since = "0.1.5",
        note = "use `edges` instead; this method will be removed in the next major release."
    )]
    pub const fn surfaces(&self) -> &EdgeMap<E, K, Idx, S> {
        self.edges()
    }
    #[deprecated(
        since = "0.1.5",
        note = "use `edges_mut` instead; this method will be removed in the next major release."
    )]
    pub const fn surfaces_mut(&mut self) -> &mut EdgeMap<E, K, Idx, S> {
        self.edges_mut()
    }
    #[deprecated(
        since = "0.1.3",
        note = "use `is_node_in_domain` instead; this method will be removed in the next major release."
    )]
    pub fn contains_node_in_edge<Q, Q2>(&self, index: &Q, vertex: &Q2) -> bool
    where
        A::Ix: Eq + Hash,
        Q: Eq + Hash + ?Sized,
        Q2: Eq + Hash,
        EdgeId<A::Ix>: Borrow<Q>,
        VertexId<A::Ix>: Borrow<Q2>,
    {
        if let Some(surface) = self.edges().get(index) {
            return surface.contains(vertex);
        }
        false
    }
    #[deprecated(
        since = "0.1.2",
        note = "use `contains_edge` instead; this method will be removed in the next major release."
    )]
    pub fn contains_surface<Q>(&self, index: &Q) -> bool
    where
        A::Ix: Eq + Hash,
        Q: Eq + Hash + ?Sized,
        EdgeId<A::Ix>: Borrow<Q>,
    {
        self.edges().contains_key(index)
    }
    #[deprecated(
        note = "use `size` instead; this method will be removed in the next major release.",
        since = "0.1.2"
    )]
    pub fn total_edges(&self) -> usize {
        self.edges().len()
    }
    #[deprecated(
        note = "use `order` instead; this method will be removed in the next major release.",
        since = "0.1.2"
    )]
    pub fn total_nodes(&self) -> usize {
        self.nodes().len()
    }
    #[deprecated(
        note = "use `order` instead; this method will be removed in the next major release.",
        since = "0.1.0"
    )]
    pub fn total_vertices(&self) -> usize {
        self.order()
    }
}
