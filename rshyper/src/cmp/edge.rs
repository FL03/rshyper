/*
    appellation: hyper_edge <module>
    authors: @FL03
*/

use crate::types::EdgeId;

pub struct HyperEdge<S, Idx = usize> {
    pub id: EdgeId<Idx>,
    pub nodes: S,
}
