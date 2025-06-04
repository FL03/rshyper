/*
    appellation: impl_edge <module>
    authors: @FL03
*/
use crate::edge::{HyperEdge, RawStore};
use crate::index::{EdgeId, RawIndex, VertexId};
use crate::{Directed, GraphKind, Undirected};

impl<S, Idx> HyperEdge<S, Directed, Idx>
where
    Idx: RawIndex,
    S: RawStore<Idx>,
{
    /// creates a new directed hyperedge with the given id and nodes
    pub fn directed(id: EdgeId<Idx>, nodes: S) -> Self {
        Self::new(id, nodes)
    }
}

impl<S, Idx> HyperEdge<S, Undirected, Idx>
where
    Idx: RawIndex,
    S: RawStore<Idx>,
{
    /// creates a new undirected hyperedge with the given id and nodes
    pub fn undirected(id: EdgeId<Idx>, nodes: S) -> Self {
        Self::new(id, nodes)
    }
}

impl<S, K, Idx> Default for HyperEdge<S, K, Idx>
where
    Idx: RawIndex + Default,
    K: GraphKind,
    S: RawStore<Idx> + Default,
{
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

impl<S, K, Idx> core::fmt::Display for HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx> + core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(id: {}, nodes: {})", self.id, self.points,)
    }
}

impl<S, K, Idx> FromIterator<VertexId<Idx>> for HyperEdge<S, K, Idx>
where
    Idx: Default + RawIndex,
    K: GraphKind,
    S: RawStore<Idx> + FromIterator<VertexId<Idx>>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = VertexId<Idx>>,
    {
        let store = S::from_iter(iter);
        Self::from_points(store)
    }
}

impl<S, K, Idx> From<EdgeId<Idx>> for HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: Default + RawStore<Idx>,
{
    fn from(from: EdgeId<Idx>) -> Self {
        Self::from_id(from)
    }
}

impl<S, K, Idx> From<HyperEdge<S, K, Idx>> for EdgeId<Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    fn from(from: HyperEdge<S, K, Idx>) -> Self {
        from.id
    }
}

impl<S, K, Idx> AsRef<S> for HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    fn as_ref(&self) -> &S {
        self.points()
    }
}

impl<S, K, Idx> AsMut<S> for HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    fn as_mut(&mut self) -> &mut S {
        self.points_mut()
    }
}

impl<S, K, Idx> core::borrow::Borrow<EdgeId<Idx>> for HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    fn borrow(&self) -> &EdgeId<Idx> {
        self.id()
    }
}

impl<S, K, Idx> core::borrow::BorrowMut<EdgeId<Idx>> for HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    fn borrow_mut(&mut self) -> &mut EdgeId<Idx> {
        self.id_mut()
    }
}

impl<S, K, Idx> core::ops::Deref for HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    type Target = EdgeId<Idx>;

    fn deref(&self) -> &Self::Target {
        self.id()
    }
}

impl<S, K, Idx> core::ops::DerefMut for HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.id_mut()
    }
}
