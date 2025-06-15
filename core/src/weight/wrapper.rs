/*
    appellation: weight <module>
    authors: @FL03
*/
use super::Weightless;

/// The [`Weight`] type is a wrapper around a generic type `T` that provides additional
/// functionality for working with weights in a graph context.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent, rename_all = "lowercase")
)]
#[repr(transparent)]
pub struct Weight<T>(pub T);
/// the base implemenation of [`Weight`] that is generic over type `T`
impl<T> Weight<T> {
    /// returns a new instance of the [`Weight`] created from the given value.
    pub const fn new(value: T) -> Self {
        Self(value)
    }
    /// generates a new instance of the [`Weight`] using the provided function
    pub fn new_with<F>(value: F) -> Self
    where
        F: FnOnce() -> T,
    {
        Self::new(value())
    }
    #[allow(clippy::should_implement_trait)]
    /// returns a new instance of the [`Weight`] with the default value of the inner type.
    pub fn default() -> Self
    where
        T: Default,
    {
        Self::new_with(Default::default)
    }
    /// returns an immutable reference to the inner value.
    pub const fn get(&self) -> &T {
        &self.0
    }
    /// returns a mutable reference to the inner value.
    pub const fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
    /// consumes the current instance to return the inner value
    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }
    /// applies the provided function onto the inner value and returns a new [`Weight`] with
    /// the result.
    #[inline]
    pub fn map<U, F>(self, f: F) -> Weight<U>
    where
        F: FnOnce(T) -> U,
    {
        Weight::new(f(self.into_inner()))
    }
    /// apply the function onto a mutable reference to the inner value and return a
    /// mutable reference to the current instanc storing the updating weight.
    #[inline]
    pub fn map_mut<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut T),
    {
        // apply the function onto the inner value
        f(self.get_mut());
        self
    }
    /// [`replace`](core::mem::replace) the inner value and return the previous value.
    pub const fn replace(&mut self, value: T) -> T {
        core::mem::replace(self.get_mut(), value)
    }
    /// updates the inner value with the provided value and returns a mutable reference to the
    /// current instance.
    #[inline]
    pub fn set(&mut self, value: T) -> &mut Self {
        *self.get_mut() = value;
        self
    }
    /// [`swap`](core::mem::swap) the inner value with another [`Weight`].
    pub const fn swap(&mut self, other: &mut Self) {
        core::mem::swap(self.get_mut(), other.get_mut());
    }
    /// [`take`](core::mem::take) the inner value, leaving the logical default in its place.
    #[inline]
    pub fn take(&mut self) -> T
    where
        T: Default,
    {
        core::mem::take(self.get_mut())
    }
    /// returns a constant pointer to the inner value; see [`core::ptr::addr_of!`] for more
    /// information
    pub const fn as_ptr(&self) -> *const T {
        core::ptr::addr_of!(self.0)
    }
    /// returns a mutable pointer to the inner value; see [`core::ptr::addr_of_mut!`] for more
    /// information
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        core::ptr::addr_of_mut!(self.0)
    }
    /// returns a _view_ of the weight whose inner value is a reference to the original.
    pub const fn view(&self) -> Weight<&T> {
        Weight::new(self.get())
    }
    /// returns a _view_ of the weight whose inner value is a mutable reference to the original
    pub const fn view_mut(&mut self) -> Weight<&mut T> {
        Weight::new(self.get_mut())
    }
    /// consumes the current instance to create another with the given value
    #[inline]
    pub fn with<U>(self, value: U) -> Weight<U> {
        self.map(|_| value)
    }
    /// returns true if:
    ///
    /// - the weight is not of type [`Weightless`]
    /// - the inner value is not the default value of the type `T`
    ///
    /// rather than simply checking if the weight is not of type [`Weightless`], this method
    /// includes additional assertions for quickly checking the state of a weight.
    pub fn is_weighted(&self) -> bool
    where
        T: 'static + Default + PartialEq,
    {
        !self.is_weightless() && self.get() != &T::default()
    }
    /// returns true if the weight is of type [`Weightless`]
    pub fn is_weightless(&self) -> bool
    where
        T: 'static,
    {
        use core::any::TypeId;
        TypeId::of::<Self>() != TypeId::of::<Weightless<T>>()
    }
}

#[allow(deprecated)]
#[doc(hidden)]
impl<T> Weight<T> {
    #[deprecated(
        note = "use `into_inner` instead, this method will be removed in the next major version",
        since = "0.1.2"
    )]
    #[inline]
    pub fn value(self) -> T {
        self.into_inner()
    }
}

scsys::fmt_wrapper! {
    Weight<T>(
        Binary,
        Debug,
        Display,
        LowerExp,
        LowerHex,
        UpperExp,
        UpperHex,
        Octal,
        Pointer,
    )
}
