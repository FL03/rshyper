/*
    appellation: impl_edge <module>
    authors: @FL03
*/
use crate::edge::{Edge, RawStore};
use crate::idx::{EdgeId, RawIndex, VertexId};
use crate::{Directed, GraphType, Undirected};

impl<S, Idx> Edge<S, Directed, Idx>
where
    Idx: RawIndex,
    S: RawStore<Item = VertexId<Idx>>,
{
    /// returns a new [`Directed`] hyperedge with the given id and nodes
    pub fn directed(id: EdgeId<Idx>, nodes: S) -> Self {
        Self::new(id, nodes)
    }
}

impl<S, Idx> Edge<S, Undirected, Idx>
where
    Idx: RawIndex,
    S: RawStore<Item = VertexId<Idx>>,
{
    /// creates a new [`Undirected`] hyperedge with the given id and nodes
    pub fn undirected(id: EdgeId<Idx>, nodes: S) -> Self {
        Self::new(id, nodes)
    }
}

impl<S, K, Idx> Default for Edge<S, K, Idx>
where
    Idx: RawIndex + Default,
    K: GraphType,
    S: RawStore<Item = VertexId<Idx>> + Default,
{
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

impl<S, K, Idx> core::fmt::Display for Edge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: RawStore<Item = VertexId<Idx>> + core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{{ id: {}, points: {:?} }}", self.id(), self.points())
    }
}

impl<S, K, Idx> FromIterator<Idx> for Edge<S, K, Idx>
where
    Idx: Default + RawIndex,
    K: GraphType,
    S: RawStore<Item = VertexId<Idx>> + FromIterator<VertexId<Idx>>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Idx>,
    {
        let iter = iter.into_iter().map(|v| VertexId::from(v));
        Self::from_iter(iter)
    }
}

impl<S, K, Idx> FromIterator<VertexId<Idx>> for Edge<S, K, Idx>
where
    Idx: Default + RawIndex,
    K: GraphType,
    S: RawStore<Item = VertexId<Idx>> + FromIterator<VertexId<Idx>>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = VertexId<Idx>>,
    {
        let store = S::from_iter(iter);
        Self::from_points(store)
    }
}

impl<S, K, Idx> From<EdgeId<Idx>> for Edge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Default + RawStore<Item = VertexId<Idx>>,
{
    fn from(from: EdgeId<Idx>) -> Self {
        Self::from_id(from)
    }
}

impl<S, K, Idx> From<Edge<S, K, Idx>> for EdgeId<Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: RawStore<Item = VertexId<Idx>>,
{
    fn from(from: Edge<S, K, Idx>) -> Self {
        from.id
    }
}

impl<S, K, Idx> AsRef<S> for Edge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: RawStore<Item = VertexId<Idx>>,
{
    fn as_ref(&self) -> &S {
        self.points()
    }
}

impl<S, K, Idx> AsMut<S> for Edge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: RawStore<Item = VertexId<Idx>>,
{
    fn as_mut(&mut self) -> &mut S {
        self.points_mut()
    }
}

impl<S, K, Idx> core::borrow::Borrow<EdgeId<Idx>> for Edge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: RawStore<Item = VertexId<Idx>>,
{
    fn borrow(&self) -> &EdgeId<Idx> {
        self.id()
    }
}

impl<S, K, Idx> core::borrow::BorrowMut<EdgeId<Idx>> for Edge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: RawStore<Item = VertexId<Idx>>,
{
    fn borrow_mut(&mut self) -> &mut EdgeId<Idx> {
        self.id_mut()
    }
}

impl<S, K, Idx> core::ops::Deref for Edge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: RawStore<Item = VertexId<Idx>>,
{
    type Target = EdgeId<Idx>;

    fn deref(&self) -> &Self::Target {
        self.id()
    }
}

impl<S, K, Idx> core::ops::DerefMut for Edge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: RawStore<Item = VertexId<Idx>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.id_mut()
    }
}
