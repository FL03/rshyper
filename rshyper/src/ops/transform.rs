/*
    Appellation: transform <module>
    Contrib: @FL03
*/

/// A trait denoting objects capable of being transformed by another object.
pub trait TransformWith<Rhs> {
    type Output;

    fn transform_with(&self, with: Rhs) -> Self::Output;
}
