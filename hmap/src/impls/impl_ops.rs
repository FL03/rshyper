/*
    appellation: impl_ops <module>
    authors: @FL03
*/
use crate::{HashEdge, HyperMap};
use core::hash::BuildHasher;
use core::ops;
use rshyper::error::Result;
use rshyper::idx::{EdgeId, HashIndex, HyperIndex, VertexId};
use rshyper::node::Node;
use rshyper::{Combine, GraphProps};

impl<N, E, A, S, Ix> Combine<EdgeId<Ix>, EdgeId<Ix>> for HyperMap<N, E, A, S>
where
    A: GraphProps<Ix = Ix>,
    S: BuildHasher + Default,
    Ix: HyperIndex,
    for<'a> &'a E: ops::Add<Output = E>,
{
    type Output = EdgeId<Ix>;

    fn combine(&mut self, src: EdgeId<Ix>, tgt: EdgeId<Ix>) -> Result<Self::Output> {
        self.merge_edges(&src, &tgt)
    }
}

impl<'a, N, E, A, S, Ix> Combine<&'a EdgeId<Ix>, &'a EdgeId<Ix>> for HyperMap<N, E, A, S>
where
    A: GraphProps<Ix = Ix>,
    S: BuildHasher + Default,
    Ix: HyperIndex,
    for<'b> &'b E: ops::Add<Output = E>,
{
    type Output = EdgeId<Ix>;

    fn combine(&mut self, src: &'a EdgeId<Ix>, tgt: &'a EdgeId<Ix>) -> Result<Self::Output> {
        self.merge_edges(src, tgt)
    }
}

impl<N, E, A, S, Ix> ops::Index<&EdgeId<Ix>> for HyperMap<N, E, A, S>
where
    A: GraphProps<Ix = Ix>,
    S: BuildHasher,
    Ix: HashIndex,
{
    type Output = HashEdge<E, A::Kind, Ix, S>;

    fn index(&self, index: &EdgeId<Ix>) -> &Self::Output {
        self.get_edge(index).expect("Edge not found")
    }
}

impl<N, E, A, S, Ix> ops::IndexMut<&EdgeId<Ix>> for HyperMap<N, E, A, S>
where
    A: GraphProps<Ix = Ix>,
    S: BuildHasher,
    Ix: HashIndex,
{
    fn index_mut(&mut self, index: &EdgeId<Ix>) -> &mut Self::Output {
        self.get_edge_mut(index).expect("Edge not found")
    }
}

impl<N, E, A, S, Ix> ops::Index<&VertexId<Ix>> for HyperMap<N, E, A, S>
where
    A: GraphProps<Ix = Ix>,
    S: BuildHasher,
    Ix: HashIndex,
{
    type Output = Node<N, Ix>;

    fn index(&self, index: &VertexId<Ix>) -> &Self::Output {
        self.get_node(index).expect("Node not found")
    }
}

impl<N, E, A, S, Ix> ops::IndexMut<&VertexId<Ix>> for HyperMap<N, E, A, S>
where
    A: GraphProps<Ix = Ix>,
    S: BuildHasher,
    Ix: HashIndex,
{
    fn index_mut(&mut self, index: &VertexId<Ix>) -> &mut Self::Output {
        self.get_node_mut(index).expect("Node not found")
    }
}
