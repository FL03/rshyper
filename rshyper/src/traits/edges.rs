/*
    Appellation: edges <module>
    Contrib: @FL03
*/
use super::HyperNode;

pub trait HyperEdge<N, Idx>
where
    N: HyperNode<Idx>,
{
    /// Returns true if the hyperedge contains the given vertex
    fn contains<Q>(&self, vertex: &Q) -> bool
    where
        Q: ?Sized,
        N: core::borrow::Borrow<Q>;
}

/*
 ************* Implementations *************
 */
