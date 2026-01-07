/*
    Appellation: nodes <module>
    Contrib: @FL03
*/
use crate::Weight;

/// [`Weighted`] is used to define common behaviours for types that have an associated weight.
pub trait Weighted<T> {
    type Cont<_T>;
    /// returns an immutable reference to the weight
    fn weight(&self) -> &Self::Cont<T>;
    /// returns a mutable reference to the weight
    fn weight_mut(&mut self) -> &mut Self::Cont<T>;
    /// [`replace`](core::mem::replace) the weight of the current instance with the given
    /// weight and return the previous weight.
    fn replace_weight(&mut self, other: Self::Cont<T>) -> Self::Cont<T> {
        core::mem::replace(self.weight_mut(), other)
    }
    /// mutably update the weight and return a mutable reference to the current instance.
    fn set_weight(&mut self, weight: Self::Cont<T>) {
        *self.weight_mut() = weight;
    }
    /// [`swap`](core::mem::swap) the weight of the current instance with the weight of
    /// another instance.
    fn swap_weight(&mut self, other: &mut Self) {
        core::mem::swap(self.weight_mut(), other.weight_mut());
    }
    /// [`take`](core::mem::take) the weight of the current instance, leaving the logical
    /// default for the type in its place and returning the previous weight.
    fn take_weight(&mut self) -> Self::Cont<T>
    where
        Self::Cont<T>: Default,
    {
        core::mem::take(self.weight_mut())
    }
}

/*
 ************* Implementations *************
*/
impl<T> Weighted<T> for Weight<T> {
    type Cont<_U> = Weight<_U>;

    fn weight(&self) -> &Self::Cont<T> {
        self
    }

    fn weight_mut(&mut self) -> &mut Self::Cont<T> {
        self
    }
}
