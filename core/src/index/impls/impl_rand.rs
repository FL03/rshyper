/*
    appellation: impl_rand <module>
    authors: @FL03
*/
#![cfg(feature = "rand")]

use crate::id::{GraphIndex, Index, RawIndex};
use rand_distr::{Distribution, StandardUniform};

/// generic implementations for the [`Index<T, K>`] enabled by the `rand` feature
impl<T, K> Index<T, K>
where
    K: GraphIndex,
    T: RawIndex,
{
    /// generate a random index from a value of type `T`
    pub fn random() -> Self
    where
        StandardUniform: Distribution<T>,
    {
        Self::new(rand::random())
    }
    /// generate a random index from a value of type `T` using the provided [`Rng`](rand::Rng)
    pub fn random_in<R>(rng: &mut R) -> Self
    where
        R: ?Sized + rand::Rng,
        StandardUniform: Distribution<T>,
    {
        Self::new(rng.random())
    }
}

impl<T, K> Distribution<Index<T, K>> for StandardUniform
where
    K: GraphIndex,
    T: RawIndex,
    StandardUniform: Distribution<T>,
{
    fn sample<R>(&self, rng: &mut R) -> Index<T, K>
    where
        R: ?Sized + rand::Rng,
    {
        Index::new(rng.random())
    }
}
