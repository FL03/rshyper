/*
    appellation: contains <module>
    authors: @FL03
*/
/// [`Contains`] defines a common interface for types able to verify if they contain a given
/// key or index;
pub trait Contains<T> {
    type Q;
    /// checks if the container contains the given index
    fn contains(&self, key: &Self::Q) -> bool
    where
        T: core::borrow::Borrow<Self::Q>;
}
