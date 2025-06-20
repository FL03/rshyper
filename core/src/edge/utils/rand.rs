/*
    appellation: rand <module>
    authors: @FL03
*/
use crate::{VertexId, Weight};
use alloc::vec::Vec;

/// returns a 2-tuple where:
///
/// - `0`: the first element is a [`Vec`] of randomly generated [`VertexId`]s
/// - `1`: the second element is a randomly generated [`Weight`] for the surface
pub fn generate_random_edge<E>(n: usize) -> (Vec<VertexId>, Weight<E>)
where
    E: Clone + num_traits::FromPrimitive + rand_distr::uniform::SampleUniform,
    rand_distr::StandardUniform: rand_distr::Distribution<E>,
{
    use rand::Rng;
    let mut rng = rand::rng();
    let distr = rand_distr::StandardUniform;
    // generate a random set of vertices containing anywhere between 1 and n vertices
    let verts = (0..(rng.random_range(1..=n)))
        .map(move |_| VertexId::random_between(0..n))
        .collect::<Vec<_>>();
    // generate a random weight for the surface
    let weight = Weight(rng.sample(distr));
    (verts, weight)
}
