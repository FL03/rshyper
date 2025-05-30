/*
    Appellation: indexable <module>
    Contrib: @FL03
*/
use crate::id::{EdgeId, RawIndex, VertexId};

/// a trait for converting a type into a valid [`EdgeId`]
pub trait IntoEdgeId<Idx: RawIndex> {
    fn into_edge_index(self) -> EdgeId<Idx>;
}

/// a trait for converting a type into a valid [`VertexId`]
pub trait IntoNodeId<Idx: RawIndex> {
    fn into_node_index(self) -> VertexId<Idx>;
}

/*
 ************* Implementations *************
*/
impl<T> IntoNodeId<T> for T
where
    T: RawIndex + Into<VertexId<T>>,
{
    fn into_node_index(self) -> VertexId<T> {
        self.into()
    }
}

impl<T> IntoEdgeId<T> for T
where
    T: RawIndex + Into<EdgeId<T>>,
{
    fn into_edge_index(self) -> EdgeId<T> {
        self.into()
    }
}
