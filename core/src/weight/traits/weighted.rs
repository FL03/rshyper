/*
    Appellation: nodes <module>
    Contrib: @FL03
*/
use crate::Weight;

/// [`Weighted`] is used to define common behaviours for types that have an associated weight.
pub trait Weighted<T> {
    /// returns an immutable reference to the weight
    fn weight(&self) -> &Weight<T>;
    /// returns a mutable reference to the weight
    fn weight_mut(&mut self) -> &mut Weight<T>;
    /// [`replace`](core::mem::replace) the weight of the current instance with the given
    /// weight and return the previous weight.
    fn replace_weight(&mut self, other: Weight<T>) -> Weight<T> {
        core::mem::replace(self.weight_mut(), other)
    }
    /// mutably update the weight and return a mutable reference to the current instance.
    fn set_weight(&mut self, Weight(weight): Weight<T>) -> &mut Self {
        self.weight_mut().set(weight);
        self
    }
    /// [`swap`](core::mem::swap) the weight of the current instance with the weight of
    /// another instance.
    fn swap_weight(&mut self, other: &mut Self) {
        core::mem::swap(self.weight_mut(), other.weight_mut());
    }
    /// [`take`](core::mem::take) the weight of the current instance, leaving the logical
    /// default for the type in its place and returning the previous weight.
    fn take_weight(&mut self) -> Weight<T>
    where
        T: Default,
    {
        core::mem::take(self.weight_mut())
    }
}

/*
 ************* Implementations *************
*/
impl<T> Weighted<T> for T
where
    T: AsRef<Weight<T>> + AsMut<Weight<T>>,
{
    fn weight(&self) -> &Weight<T> {
        self.as_ref()
    }

    fn weight_mut(&mut self) -> &mut Weight<T> {
        self.as_mut()
    }
}
