/*
    appellation: step <module>
    authors: @FL03
*/

/// [`StepWith`] is a trait defining an interface that can be best described as a more flexible
/// [`take`](core::mem::take) method, however, instead of leaving the default value in place
/// of the previous one, it allows for a generator function to be provided.
pub trait StepWith<T> {
    type Output;

    fn step_with<F>(&mut self, f: F) -> Option<Self::Output>
    where
        F: FnOnce(&T) -> T;
}
