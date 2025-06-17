/*
    appellation: impl_edge <module>
    authors: @FL03
*/
use crate::edge::EdgeLayout;
use crate::idx::{EdgeId, RawIndex, VertexId};
use crate::{Directed, Domain, GraphType, Undirected};

impl<S, Idx> EdgeLayout<S, Directed, Idx>
where
    Idx: RawIndex,
    S: Domain<Idx>,
{
    /// returns a new [`Directed`] hyperedge with the given id and nodes
    pub fn directed(id: EdgeId<Idx>, nodes: S) -> Self {
        Self::new(id, nodes)
    }
}

impl<S, Idx> EdgeLayout<S, Undirected, Idx>
where
    Idx: RawIndex,
    S: Domain<Idx>,
{
    /// creates a new [`Undirected`] hyperedge with the given id and nodes
    pub fn undirected(id: EdgeId<Idx>, nodes: S) -> Self {
        Self::new(id, nodes)
    }
}

impl<S, K, Idx> Default for EdgeLayout<S, K, Idx>
where
    Idx: RawIndex + Default,
    K: GraphType,
    S: Domain<Idx> + Default,
{
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

impl<S, K, Idx> core::fmt::Debug for EdgeLayout<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx> + core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Edge")
            .field("id", &self.id())
            .field("domain", &self.domain())
            .finish()
    }
}

impl<S, K, Idx> core::fmt::Display for EdgeLayout<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx> + core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{{ id: {}, domain: {:?} }}", self.id(), self.domain())
    }
}

impl<S, K, Idx> FromIterator<Idx> for EdgeLayout<S, K, Idx>
where
    Idx: Default + RawIndex,
    K: GraphType,
    S: Domain<Idx> + FromIterator<VertexId<Idx>>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Idx>,
    {
        let iter = iter.into_iter().map(|v| VertexId::from(v));
        Self::from_iter(iter)
    }
}

impl<S, K, Idx> FromIterator<VertexId<Idx>> for EdgeLayout<S, K, Idx>
where
    Idx: Default + RawIndex,
    K: GraphType,
    S: Domain<Idx> + FromIterator<VertexId<Idx>>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = VertexId<Idx>>,
    {
        let store = S::from_iter(iter);
        Self::from_domain(store)
    }
}

impl<S, K, Idx> From<EdgeId<Idx>> for EdgeLayout<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Default + Domain<Idx>,
{
    fn from(from: EdgeId<Idx>) -> Self {
        Self::from_id(from)
    }
}

impl<S, K, Idx> From<EdgeLayout<S, K, Idx>> for EdgeId<Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    fn from(from: EdgeLayout<S, K, Idx>) -> Self {
        from.id
    }
}

impl<S, K, Idx> AsRef<S> for EdgeLayout<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    fn as_ref(&self) -> &S {
        self.domain()
    }
}

impl<S, K, Idx> AsMut<S> for EdgeLayout<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    fn as_mut(&mut self) -> &mut S {
        self.domain_mut()
    }
}

impl<S, K, Idx> core::borrow::Borrow<EdgeId<Idx>> for EdgeLayout<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    fn borrow(&self) -> &EdgeId<Idx> {
        self.id()
    }
}

impl<S, K, Idx> core::borrow::BorrowMut<EdgeId<Idx>> for EdgeLayout<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    fn borrow_mut(&mut self) -> &mut EdgeId<Idx> {
        self.id_mut()
    }
}

impl<S, K, Idx> core::ops::Deref for EdgeLayout<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    type Target = EdgeId<Idx>;

    fn deref(&self) -> &Self::Target {
        self.id()
    }
}

impl<S, K, Idx> core::ops::DerefMut for EdgeLayout<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.id_mut()
    }
}
