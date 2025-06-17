/*
    appellation: impl_hyper_facet <module>
    authors: @FL03
*/
use crate::edge::{Edge, EdgeLayout};
use crate::idx::{EdgeId, RawIndex};
use crate::{Directed, Domain, GraphType, Undirected, Weight};

impl<T, S, Idx> Edge<T, S, Directed, Idx>
where
    Idx: RawIndex,
    S: Domain<Idx>,
{
    /// returns a new [`Directed`] hypersurface with the given id and nodes
    pub fn directed(id: EdgeId<Idx>, nodes: S, weight: T) -> Self {
        Self::new(id, nodes, Weight::new(weight))
    }
}

impl<T, S, I> Edge<T, S, Undirected, I>
where
    I: RawIndex,
    S: Domain<I>,
{
    /// creates a new [`Undirected`] hypersurface with the given id and nodes
    pub fn undirected(id: EdgeId<I>, nodes: S, weight: T) -> Self {
        Self::new(id, nodes, Weight::new(weight))
    }
}

impl<T, S, K, Idx> Default for Edge<T, S, K, Idx>
where
    Idx: Default + RawIndex,
    K: GraphType,
    T: Default,
    S: Domain<Idx> + Default,
{
    fn default() -> Self {
        Self {
            edge: EdgeLayout::default(),
            weight: Weight::default(),
        }
    }
}

impl<T, S, K, Idx> core::fmt::Debug for Edge<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    T: core::fmt::Debug,
    S: Domain<Idx> + core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Surface")
            .field("edge", &self.edge())
            .field("weight", &self.weight())
            .finish()
    }
}

impl<T, S, K, Idx> core::fmt::Display for Edge<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    T: core::fmt::Display,
    S: Domain<Idx> + core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{ edge: {e}, weight: {w} }}",
            e = self.edge(),
            w = self.weight(),
        )
    }
}

impl<T, S, K, Idx> From<EdgeLayout<S, K, Idx>> for Edge<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
    T: Default,
{
    fn from(edge: EdgeLayout<S, K, Idx>) -> Self {
        Self::from_edge(edge)
    }
}

impl<T, S, K, Idx> From<Edge<T, S, K, Idx>> for EdgeLayout<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    fn from(facet: Edge<T, S, K, Idx>) -> Self {
        facet.edge
    }
}

impl<T, S, K, Idx> From<EdgeId<Idx>> for Edge<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Default + Domain<Idx>,
    T: Default,
{
    fn from(id: EdgeId<Idx>) -> Self {
        Self::from_id(id)
    }
}

impl<T, S, K, Idx> AsRef<Weight<T>> for Edge<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    fn as_ref(&self) -> &Weight<T> {
        &self.weight
    }
}

impl<T, S, K, Idx> AsMut<Weight<T>> for Edge<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    fn as_mut(&mut self) -> &mut Weight<T> {
        &mut self.weight
    }
}

impl<T, S, K, Idx> core::borrow::Borrow<EdgeId<Idx>> for Edge<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    fn borrow(&self) -> &EdgeId<Idx> {
        self.id()
    }
}

impl<T, S, K, Idx> core::borrow::BorrowMut<EdgeId<Idx>> for Edge<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    fn borrow_mut(&mut self) -> &mut EdgeId<Idx> {
        self.id_mut()
    }
}

impl<T, S, K, Idx> core::ops::Deref for Edge<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    type Target = EdgeLayout<S, K, Idx>;

    fn deref(&self) -> &Self::Target {
        self.edge()
    }
}

impl<T, S, K, Idx> core::ops::DerefMut for Edge<T, S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.edge_mut()
    }
}
