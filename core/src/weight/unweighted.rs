/*
    appellation: unweighted <module>
    authors: @FL03
*/
use super::Weight;

/// The `Weight` module provides a generic wrapper type for weights in a graph context,
pub type Unweighted<T> = Weight<UnWeighted<T>>;

/// the [`UnWeighted`] struct is a marker type that represents the absence of a weight.
#[doc(hidden)]
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default)
)]
#[repr(transparent)]
pub struct UnWeighted<T> {
    #[cfg_attr(feature = "serde", serde(skip))]
    /// A marker field to indicate the type of the weight.
    _marker: core::marker::PhantomData<T>,
}

impl<T> UnWeighted<T> {
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

impl<T> Unweighted<T> {
    ///
    pub const fn unweighted() -> Self {
        Self::new(UnWeighted::new())
    }
}

unsafe impl<T> Send for UnWeighted<T> {}

unsafe impl<T> Sync for UnWeighted<T> {}
