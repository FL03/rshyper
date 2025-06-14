/*
    appellation: container <module>
    authors: @FL03
*/
use super::RawContainer;

/// the [`ContainerBase`] type materializes the [`RawContainer`] trait, providing a physical
/// implementation of a generic container type
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct ContainerBase<S>
where
    S: RawContainer,
{
    /// the underlying store for the container
    pub(crate) repr: S,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub(crate) _marker: core::marker::PhantomData<S::Item>,
}

impl<A, S> ContainerBase<S>
where
    S: RawContainer<Item = A>,
{
    /// returns a new instance using the given store
    pub const fn from_store(store: S) -> Self {
        Self {
            repr: store,
            _marker: core::marker::PhantomData::<A>,
        }
    }
    /// returns a reference to the underlying store
    pub const fn store(&self) -> &S {
        &self.repr
    }
    /// returns a mutable reference to the underlying store
    pub const fn store_mut(&mut self) -> &mut S {
        &mut self.repr
    }
    /// [`replace`](core::mem::replace) the store with another and return the previous value
    pub const fn replace_store(&mut self, store: S) -> S {
        core::mem::replace(self.store_mut(), store)
    }
    /// set the current store and return a mutable reference to the container
    pub fn set_store(&mut self, store: S) -> &mut Self {
        self.repr = store;
        self
    }
    /// [`swap`](core::mem::swap) the store another and return a mutable reference to the
    /// container
    pub const fn swap_store(&mut self, store: &mut S) -> &mut Self {
        core::mem::swap(self.store_mut(), store);
        self
    }
    /// [`take`](core::mem::take) the store and return it, leaving the logical default in its
    /// place
    pub fn take_store(&mut self) -> S
    where
        S: Default,
    {
        core::mem::take(self.store_mut())
    }
    /// consumes the current instance to create another with the given store
    pub fn with_store<T: RawContainer<Item = A>>(self, repr: T) -> ContainerBase<T> {
        ContainerBase {
            repr,
            _marker: core::marker::PhantomData::<A>,
        }
    }
}

impl<S> AsRef<S> for ContainerBase<S>
where
    S: RawContainer,
{
    fn as_ref(&self) -> &S {
        self.store()
    }
}

impl<S> AsMut<S> for ContainerBase<S>
where
    S: RawContainer,
{
    fn as_mut(&mut self) -> &mut S {
        self.store_mut()
    }
}

impl<S> core::borrow::Borrow<S> for ContainerBase<S>
where
    S: RawContainer,
{
    fn borrow(&self) -> &S {
        self.store()
    }
}

impl<S> core::borrow::BorrowMut<S> for ContainerBase<S>
where
    S: RawContainer,
{
    fn borrow_mut(&mut self) -> &mut S {
        self.store_mut()
    }
}

impl<S> core::ops::Deref for ContainerBase<S>
where
    S: RawContainer,
{
    type Target = S;

    fn deref(&self) -> &Self::Target {
        self.store()
    }
}

impl<S> core::ops::DerefMut for ContainerBase<S>
where
    S: RawContainer,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.store_mut()
    }
}

impl<A, I, S> core::ops::Index<I> for ContainerBase<S>
where
    S: RawContainer<Item = A> + core::ops::Index<I, Output = A>,
{
    type Output = S::Item;

    fn index(&self, index: I) -> &Self::Output {
        self.store().index(index)
    }
}

impl<A, I, S> core::ops::IndexMut<I> for ContainerBase<S>
where
    S: RawContainer<Item = A> + core::ops::IndexMut<I, Output = A>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.store_mut().index_mut(index)
    }
}

impl<A, S> IntoIterator for ContainerBase<S>
where
    S: RawContainer<Item = A> + IntoIterator<Item = A>,
{
    type Item = A;
    type IntoIter = S::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.repr.into_iter()
    }
}

impl<'a, A, S> IntoIterator for &'a ContainerBase<S>
where
    S: RawContainer<Item = A>,
    for<'b> &'b S: IntoIterator<Item = A>,
{
    type Item = A;
    type IntoIter = <&'a S as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.store().into_iter()
    }
}

impl<'a, A, S> IntoIterator for &'a mut ContainerBase<S>
where
    S: RawContainer<Item = A>,
    for<'b> &'b mut S: IntoIterator<Item = S::Item>,
{
    type Item = A;
    type IntoIter = <&'a mut S as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.store_mut().into_iter()
    }
}
