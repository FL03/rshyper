/*
    Appellation: symbolic <module>
    Contrib: @FL03
*/

pub trait Symbolic:
    Clone + Default + Eq + PartialOrd + core::fmt::Debug + core::fmt::Display + core::hash::Hash
{
}

impl<T> Symbolic for T where
    T: Clone + Default + Eq + PartialOrd + core::fmt::Debug + core::fmt::Display + core::hash::Hash
{
}
