/*
    Appellation: index <module>
    Contrib: @FL03
*/
use super::IndexKind;
use core::marker::PhantomData;

#[derive(Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Index<Idx, K>
where
    K: IndexKind,
{
    pub(crate) value: Idx,
    pub(crate) _type: PhantomData<K>,
}

impl<T, K> Index<T, K>
where
    K: IndexKind,
{
    pub fn from_value(index: T) -> Self {
        Self {
            value: index,
            _type: PhantomData::<K>,
        }
    }
    /// returns a pointer to the inner value
    pub const fn as_ptr(&self) -> *const T {
        core::ptr::from_ref(&self.value)
    }
    /// returns a mutable pointer to the inner value
    pub fn as_mut_ptr(&mut self) -> *mut T {
        core::ptr::from_mut(&mut self.value)
    }
    /// consumes the index returning the inner value
    #[inline]
    pub fn into_inner(self) -> T {
        self.value
    }
    /// returns an immutable reference to the inner value
    pub const fn get(&self) -> &T {
        &self.value
    }
    /// returns a mutable reference to the inner value
    pub const fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }
    /// apply a function to the inner value and returns a new Index wrapping the result
    #[inline]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Index<U, K> {
        Index {
            value: f(self.value),
            _type: PhantomData::<K>,
        }
    }
    /// [`replace`](core::mem::replace) and return the old value after replacing it with the
    /// given value
    pub const fn replace(&mut self, index: T) -> T {
        core::mem::replace(&mut self.value, index)
    }
    /// set the index to the given value
    pub fn set(&mut self, value: T) -> &mut Self {
        self.value = value;
        self
    }
    /// [`swap`](core::mem::swap) the values of two indices
    pub const fn swap(&mut self, other: &mut Self) {
        core::mem::swap(&mut self.value, &mut other.value)
    }
    /// [`take`](core::mem::take) the value and replace it with the default value
    #[inline]
    pub fn take(&mut self) -> T
    where
        T: Default,
    {
        core::mem::take(&mut self.value)
    }
    /// consumes the current index to create another with the given value
    pub fn with<U>(self, value: U) -> Index<U, K> {
        Index {
            value,
            _type: PhantomData::<K>,
        }
    }
    /// decrements the index value by [one](num_traits::One) and returns a new instance
    pub fn dec(self) -> Self
    where
        T: core::ops::Sub<Output = T> + num_traits::One,
    {
        let value = self.value - T::one();
        Self { value, ..self }
    }
    /// mutably decrements the index value by [one](num_traits::One)
    #[inline]
    pub fn dec_inplace(&mut self)
    where
        T: core::ops::SubAssign + num_traits::One,
    {
        self.value -= T::one();
    }
    /// increments the index value by [one](num_traits::One) and consumes the current instance
    /// to create another with the new value.
    pub fn inc(self) -> Self
    where
        T: core::ops::Add<Output = T> + num_traits::One,
    {
        let value = self.value + T::one();
        Self { value, ..self }
    }
    /// mutably increments the index value by [one](num_traits::One)
    #[inline]
    pub fn inc_inplace(&mut self)
    where
        T: core::ops::AddAssign + num_traits::One,
    {
        self.value += T::one();
    }
}

impl<T, K> AsRef<T> for Index<T, K>
where
    K: IndexKind,
{
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T, K> AsMut<T> for Index<T, K>
where
    K: IndexKind,
{
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T, K> core::borrow::Borrow<T> for Index<T, K>
where
    K: IndexKind,
{
    fn borrow(&self) -> &T {
        &self.value
    }
}
impl<T, K> core::borrow::BorrowMut<T> for Index<T, K>
where
    K: IndexKind,
{
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T, K> Default for Index<T, K>
where
    K: IndexKind,
    T: Default,
{
    fn default() -> Self {
        Self {
            value: T::default(),
            _type: core::marker::PhantomData::<K>,
        }
    }
}

impl<T, K> From<T> for Index<T, K>
where
    K: IndexKind,
{
    fn from(index: T) -> Self {
        Self::from_value(index)
    }
}

impl<T, K> PartialEq<T> for Index<T, K>
where
    K: IndexKind,
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        &self.value == other
    }
}

impl<T, K> core::iter::Iterator for Index<T, K>
where
    K: IndexKind,
    T: for<'a> core::ops::Add<&'a T, Output = T> + num::One,
{
    type Item = Index<T, K>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Index::from_value(T::one() + &self.value))
    }
}

macro_rules! impl_fmt {
    ($($trait:ident),* $(,)?) => {
        $(impl_fmt!(@impl $trait);)*
    };
    (@impl $trait:ident) => {
        impl<T, K> ::core::fmt::$trait for Index<T, K>
        where
            K: IndexKind,
            T: ::core::fmt::$trait,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::$trait::fmt(&self.value, f)
            }
        }
    };
}

impl_fmt! {
    Binary,
    Debug,
    Display,
    LowerExp,
    LowerHex,
    Octal,
    Pointer,
    UpperExp,
    UpperHex,
}
