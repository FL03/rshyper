/*
    Appellation: transform <module>
    Contrib: @FL03
*/
/// this trait is typically used to define particular transformations capable of being applied
/// to various objects.
pub trait Apply<Rhs: ?Sized> {
    type Output;

    fn apply(&self, rhs: &Rhs) -> Self::Output;
}
/// A trait denoting objects capable of being transformed by another object.
pub trait Transform<Rhs> {
    type Output;

    fn transform(&self, dirac: Rhs) -> Self::Output;
}
/// The [`TransformInplace`] generically describes objects capable of being transformed
/// in-place by another object.
pub trait TransformInplace<Rhs> {
    fn transform_with(&self, with: Rhs) -> &mut Self;
}

impl<D, S> Transform<D> for S
where
    D: Apply<S>,
{
    type Output = D::Output;

    fn transform(&self, dirac: D) -> Self::Output {
        dirac.apply(&self)
    }
}
