/*
    appellation: hyper_edge <module>
    authors: @FL03
*/
use super::{RawEdge, RawStore};
use crate::GraphKind;
use crate::index::{EdgeId, RawIndex};

/// [`HyperEdge`] is the base type for hyperedges in a graph. These edges are generic over the
/// edge store type `S`, the graph kind `K`, and the index type `Idx`. This allows for
/// flexibility in how edges store their vertices and how they are identified within the graph.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[repr(C)]
pub struct HyperEdge<S, K, Idx = usize>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    pub(crate) id: EdgeId<Idx>,
    pub(crate) points: S,
    pub(crate) _kind: core::marker::PhantomData<K>,
}

/*
 ************* Implementations *************
*/

impl<S, Idx, K> RawEdge for HyperEdge<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphKind,
    S: RawStore<Idx>,
{
    type Kind = K;
    type Idx = Idx;
    type Store = S;

    seal!();

    fn index(&self) -> &EdgeId<Idx> {
        self.id()
    }

    fn vertices(&self) -> &S {
        self.points()
    }
}
