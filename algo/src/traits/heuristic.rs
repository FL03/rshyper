/*
    appellation: heuristic <module>
    authors: @FL03
*/
use rshyper_core::idx::{RawIndex, Udx, VertexId};
/// [`Heuristic`] defines a common interface for heuristic functions compatible with the [`A*`](AStarSearch)
/// search implementation
pub trait Heuristic<T = Udx> {
    type Output;

    fn compute(&self, start: VertexId<T>, goal: VertexId<T>) -> Self::Output;
}

/*
 ************* Implementations *************
*/

impl<F, Idx> Heuristic<Idx> for F
where
    Idx: RawIndex,
    F: Fn(VertexId<Idx>, VertexId<Idx>) -> f64,
{
    type Output = f64;

    fn compute(&self, start: VertexId<Idx>, goal: VertexId<Idx>) -> Self::Output {
        self(start, goal)
    }
}
