/*
    Appellation: algo <module>
    Contrib: @FL03
*/
//! this module implements various algorithms for hypergraphs
#[doc(inline)]
pub use self::prelude::*;

/// search algorithms for hypergraphs
#[cfg(feature = "std")]
pub mod search;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::GraphOperator;
    #[cfg(feature = "std")]
    #[doc(inline)]
    pub use super::search::prelude::*;
}
/// this trait is used to denote an algorithm that can be applied to a hypergraph
pub trait GraphicAlgorithm<H> {
    /// the type of output that this algorithm produces
    type Output;

    /// run the algorithm on the graph and return the output
    fn process(self, graph: H) -> crate::Result<Self::Output>;
}

/// this trait is used to denote an algorithmic operator that can be applied to a hypergraph.
pub trait GraphOperator<'a> {
    type Graph<N, E, Idx>
    where
        N: 'a,
        E: 'a,
        Idx: 'a + crate::RawIndex;

    private!();
}
