/*
    Appellation: transform <module>
    Contrib: @FL03
*/
/// The [`Apply`] trait defines a generic, _transformative_ operation that can be applied to an
/// object of type `Rhs`.
pub trait Apply<Rhs> {
    type Output;
    /// apply the transformation defined by `self` onto the object `rhs`
    fn apply(&self, rhs: Rhs) -> Self::Output;
}
/// A trait denoting objects capable of being transformed by another object.
pub trait Transform<T> {
    /// the expected output type after the transformation is applied
    type C<_T>;
    /// transform the object `self` using the transformation defined by `dirac`.
    fn transform<U, F>(&self, dirac: F) -> Self::C<U>
    where
        F: FnMut(&T) -> U;
}
/// The [`TransformInplace`] generically describes objects capable of being transformed
/// in-place by another object.
pub trait TransformInplace<Rhs> {
    fn transform_with(&mut self, with: Rhs) -> &mut Self;
}

/*
 ************* Implementations *************
*/
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "alloc")]
impl<T> Transform<T> for Vec<T> {
    type C<U> = Vec<U>;

    fn transform<U, F>(&self, dirac: F) -> Self::C<U>
    where
        F: FnMut(&T) -> U,
    {
        self.iter().map(dirac).collect::<Vec<_>>()
    }
}
