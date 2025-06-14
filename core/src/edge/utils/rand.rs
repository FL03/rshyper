/*
    appellation: rand <module>
    authors: @FL03
*/
use crate::{VertexId, Weight};
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// returns a 2-tuple where:
///
/// - `0`: the first element is a [`Vec`] containing anywhere between 2 and `n` [`VertexId`]
/// - `1`: the second element is a randomly generated [`Weight`] for the surface
#[cfg(feature = "alloc")]
pub fn generate_random_edge<E>(n: usize) -> (Vec<VertexId>, Weight<E>)
where
    E: Clone,
    rand_distr::StandardUniform: rand_distr::Distribution<E>,
{
    use rand::Rng;
    let mut rng = rand::rng();
    // generate a random set of vertices containing anywhere between 2 and n vertices
    let verts = (0..(rng.random_range(2..=n)))
        .map(|_| VertexId::from(rng.random_range(0..n)))
        .collect::<Vec<_>>();
    let weight = Weight(rng.random());
    (verts, weight)
}
