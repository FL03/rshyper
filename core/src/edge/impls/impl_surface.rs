/*
    appellation: impl_hyper_facet <module>
    authors: @FL03
*/
use crate::edge::{Edge, EdgeStore, Surface};
use crate::idx::{EdgeId, RawIndex};
use crate::{Directed, GraphType, Undirected, Weight};

impl<T, S, Idx> Surface<T, S, Directed, Idx>
where
    Idx: RawIndex,
    S: EdgeStore<Idx>,
{
    /// returns a new [`Directed`] hypersurface with the given id and nodes
    pub fn directed(id: EdgeId<Idx>, nodes: S, weight: T) -> Self {
        Self::new(id, nodes, Weight::new(weight))
    }
}

impl<T, S, I> Surface<T, S, Undirected, I>
where
    I: RawIndex,
    S: EdgeStore<I>,
{
    /// creates a new [`Undirected`] hypersurface with the given id and nodes
    pub fn undirected(id: EdgeId<I>, nodes: S, weight: T) -> Self {
        Self::new(id, nodes, Weight::new(weight))
    }
}

impl<T, S, K, Idx> Default for Surface<T, S, K, Idx>
where
    Idx: Default + RawIndex,
    K: GraphType,
    T: Default,
    S: EdgeStore<Idx> + Default,
{
    fn default() -> Self {
        Self {
            edge: Edge::default(),
            weight: Weight::init(),
        }
    }
}

impl<T, S, K, Idx> core::fmt::Display for Surface<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    T: core::fmt::Display,
    S: EdgeStore<Idx> + core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{ id: {}, points: {:?}, weight: {} }}",
            self.edge().id(),
            self.edge().points(),
            self.weight()
        )
    }
}

impl<T, S, K, Idx> From<Edge<S, K, Idx>> for Surface<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: EdgeStore<Idx>,
    T: Default,
{
    fn from(edge: Edge<S, K, Idx>) -> Self {
        Self::from_edge(edge)
    }
}

impl<T, S, K, Idx> From<Surface<T, S, K, Idx>> for Edge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: EdgeStore<Idx>,
{
    fn from(facet: Surface<T, S, K, Idx>) -> Self {
        facet.edge
    }
}

impl<T, S, K, Idx> From<EdgeId<Idx>> for Surface<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Default + EdgeStore<Idx>,
    T: Default,
{
    fn from(id: EdgeId<Idx>) -> Self {
        Self::from_id(id)
    }
}

impl<T, S, K, Idx> AsRef<Weight<T>> for Surface<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: EdgeStore<Idx>,
{
    fn as_ref(&self) -> &Weight<T> {
        &self.weight
    }
}

impl<T, S, K, Idx> AsMut<Weight<T>> for Surface<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: EdgeStore<Idx>,
{
    fn as_mut(&mut self) -> &mut Weight<T> {
        &mut self.weight
    }
}

impl<T, S, K, Idx> core::borrow::Borrow<EdgeId<Idx>> for Surface<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: EdgeStore<Idx>,
{
    fn borrow(&self) -> &EdgeId<Idx> {
        self.id()
    }
}

impl<T, S, K, Idx> core::borrow::BorrowMut<EdgeId<Idx>> for Surface<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: EdgeStore<Idx>,
{
    fn borrow_mut(&mut self) -> &mut EdgeId<Idx> {
        self.id_mut()
    }
}

impl<T, S, K, Idx> core::ops::Deref for Surface<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: EdgeStore<Idx>,
{
    type Target = Edge<S, K, Idx>;

    fn deref(&self) -> &Self::Target {
        self.edge()
    }
}

impl<T, S, K, Idx> core::ops::DerefMut for Surface<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: EdgeStore<Idx>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.edge_mut()
    }
}
