/*
    Appellation: index <module>
    Contrib: @FL03
*/
use super::{GraphIndex, IndexError, IndexResult, RawIndex};
/// A generic [`IndexBase`] implementation used to represent various [_kinds_](GraphIndex) of
/// indices
#[derive(Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct IndexBase<Idx, K> {
    pub(crate) value: Idx,
    pub(crate) _type: core::marker::PhantomData<K>,
}

impl<T, K> IndexBase<T, K>
where
    K: GraphIndex,
    T: RawIndex,
{
    /// returns a new instance of [`Index`] with the given value.
    pub fn new(index: T) -> Self {
        Self {
            value: index,
            _type: core::marker::PhantomData::<K>,
        }
    }
    /// initializes a new instance of [`Index`] using the logical default for the type `T`
    pub fn default() -> Self
    where
        T: Default,
    {
        Self::new(T::default())
    }
    /// creates a new index with a value of [`one`](num_traits::One)
    pub fn one() -> Self
    where
        T: num_traits::One,
    {
        Self::new(T::one())
    }
    /// creates a new index with a value of [`zero`](num_traits::Zero)
    pub fn zero() -> Self
    where
        T: num_traits::Zero,
    {
        Self::new(T::zero())
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
    pub fn map<U, F>(self, f: F) -> IndexBase<U, K>
    where
        F: FnOnce(T) -> U,
        U: RawIndex,
    {
        IndexBase::new(f(self.value))
    }
    /// [`replace`](core::mem::replace) and return the old value after replacing it with the
    /// given value
    pub const fn replace(&mut self, index: T) -> T {
        core::mem::replace(&mut self.value, index)
    }
    /// set the index to the given value
    #[inline]
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
    #[inline]
    pub fn with<U: RawIndex>(self, value: U) -> IndexBase<U, K> {
        IndexBase {
            value,
            _type: core::marker::PhantomData::<K>,
        }
    }
    /// decrements the index value by [one](num_traits::One) and returns a new instance
    #[inline]
    pub fn dec(self) -> IndexBase<<T as core::ops::Sub>::Output, K>
    where
        T: core::ops::Sub + num_traits::One,
        <T as core::ops::Sub>::Output: RawIndex,
    {
        let value = self.value - T::one();
        IndexBase::new(value)
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
    #[inline]
    pub fn inc(self) -> IndexBase<<T as core::ops::Add>::Output, K>
    where
        T: core::ops::Add + num_traits::One,
        <T as core::ops::Add>::Output: RawIndex,
    {
        let value = self.value + T::one();
        IndexBase::new(value)
    }
    /// mutably increments the index value by [one](num_traits::One)
    #[inline]
    pub fn inc_inplace(&mut self)
    where
        T: Copy + core::ops::AddAssign + num_traits::One,
    {
        self.value += T::one();
    }
    /// increments the current index and returns the previous instance of the index.
    ///
    /// ```rust
    ///     use rshyper_core::EdgeId;
    ///     let mut edge_id = EdgeId::<usize>::zero();
    ///     let e0 = edge_id.step()?;
    ///     let e1 = edge_id.step()?;
    ///     let e2 = edge_id.step()?;
    ///     assert_eq!(e0.get(), &0);
    ///     assert_eq!(e1.get(), &1);
    ///     assert_eq!(e2.get(), &2);
    /// ```
    #[inline]
    pub fn step(&mut self) -> IndexResult<Self, T>
    where
        T: Copy + core::ops::Add<T, Output = T> + num_traits::One,
    {
        self.next().ok_or(IndexError::IndexOutOfBounds(*self.get()))
    }
}

impl<T, K> AsRef<T> for IndexBase<T, K>
where
    K: GraphIndex,
    T: RawIndex,
{
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T, K> AsMut<T> for IndexBase<T, K>
where
    K: GraphIndex,
    T: RawIndex,
{
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T, K> core::borrow::Borrow<T> for IndexBase<T, K>
where
    K: GraphIndex,
    T: RawIndex,
{
    fn borrow(&self) -> &T {
        &self.value
    }
}
impl<T, K> core::borrow::BorrowMut<T> for IndexBase<T, K>
where
    K: GraphIndex,
    T: RawIndex,
{
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T, K> Default for IndexBase<T, K>
where
    K: GraphIndex,
    T: Default + RawIndex,
{
    fn default() -> Self {
        Self {
            value: T::default(),
            _type: core::marker::PhantomData::<K>,
        }
    }
}

impl<T, K> From<T> for IndexBase<T, K>
where
    K: GraphIndex,
    T: RawIndex,
{
    fn from(index: T) -> Self {
        Self::new(index)
    }
}

impl<T, K> PartialEq<T> for IndexBase<T, K>
where
    K: GraphIndex,
    T: PartialEq + RawIndex,
{
    fn eq(&self, other: &T) -> bool {
        &self.value == other
    }
}

impl<T, K> core::iter::Iterator for IndexBase<T, K>
where
    K: GraphIndex,
    T: Copy + RawIndex + core::ops::Add<T, Output = T> + num_traits::One,
{
    type Item = IndexBase<T, K>;

    fn next(&mut self) -> Option<Self::Item> {
        // compute the next value
        let next = self.value + T::one();
        // replace the current value with the next one
        let prev = core::mem::replace(&mut self.value, next);
        // return the previous instance
        Some(Self::new(prev))
    }
}

macro_rules! impl_fmt {
    (
        $s:ident(
            $($trait:ident),* $(,)?
        )
    ) => {
        $(
            impl_fmt!(@impl $s($trait));
        )*
    };
    (@impl $s:ident($trait:ident)) => {
        impl<T, K> ::core::fmt::$trait for $s<T, K>
        where
            K: GraphIndex,
            T: RawIndex + ::core::fmt::$trait,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::$trait::fmt(&self.value, f)
            }
        }
    };
}

impl_fmt! {
    IndexBase(
        Binary,
        Debug,
        Display,
        LowerExp,
        LowerHex,
        Octal,
        Pointer,
        UpperExp,
        UpperHex,
    )
}
