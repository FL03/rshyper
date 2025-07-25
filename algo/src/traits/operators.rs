/*
    appellation: operators <module>
    authors: @FL03
*/
use crate::error::Result;
use rshyper::{GraphProps, HyperGraph};

/// this trait is used to denote an algorithm that can be applied to a hypergraph
pub trait GraphicAlgorithm<H> {
    /// the type of output that this algorithm produces
    type Output;

    /// run the algorithm on the graph and return the output
    fn process(self, graph: H) -> Result<Self::Output>;
}

/// this trait is used to denote an algorithmic operator that can be applied to a hypergraph.
pub trait GraphOperator<N, E, A>
where
    A: GraphProps,
{
    type Graph: HyperGraph<N, E, A>;

    private!();
}
