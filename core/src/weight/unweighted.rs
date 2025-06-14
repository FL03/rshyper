/*
    appellation: unweighted <module>
    authors: @FL03
*/
//! this module provides the [`UnWeight`] marker type and the [`Weightless`] type alias for
//! types that are said to have no weight.
use super::Weight;

/// An [`Weightless`] types is a type alias for a [`Weight`] that uses the [`UnWeight`] marker
/// type to indicate that it has no weight.
pub type Weightless<T> = Weight<UnWeight<T>>;

/// the [`Weightless`] struct is a marker type that represents the absence of a weight.
#[doc(hidden)]
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default)
)]
#[repr(transparent)]
pub struct UnWeight<T> {
    #[cfg_attr(feature = "serde", serde(skip))]
    /// A marker field to indicate the type of the weight.
    _marker: core::marker::PhantomData<T>,
}

impl<T> UnWeight<T> {
    /// Creates a new instance of `UnWeighted`.
    pub const fn new() -> Self {
        Self {
            _marker: core::marker::PhantomData::<T>,
        }
    }
    /// returns true if the types are the same.
    pub fn is<U>(&self) -> bool
    where
        U: 'static,
        T: 'static,
    {
        use core::any::TypeId;
        // Compare the TypeId of T and U to check if they are the same type.
        TypeId::of::<T>() == TypeId::of::<U>()
    }
}

unsafe impl<T> Send for UnWeight<T> {}

unsafe impl<T> Sync for UnWeight<T> {}
