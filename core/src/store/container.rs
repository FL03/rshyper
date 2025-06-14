/*
    appellation: container <module>
    authors: @FL03
*/



pub struct ContainerBase<S> {
    /// the underlying store for the container
    pub store: S,
    pub(crate) _marker: core::marker::PhantomData<S::Item>,
}