/*
    Appellation: index <module>
    Contrib: @FL03
*/
use super::{GraphIndex, IndexError, IndexResult, RawIndex};
use num_traits::{One, Zero};

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
    /// creates a new instance of [`Index`] using the given function to generate the value
    pub fn create<F>(index: F) -> Self
    where
        F: FnOnce() -> T,
    {
        Self::new(index())
    }
    /// initializes a new instance of [`Index`] using the logical default for the type `T`
    pub fn default() -> Self
    where
        T: Default,
    {
        Self::new(T::default())
    }
    /// creates a new index with a value of [`one`](One::one)
    pub fn one() -> Self
    where
        T: One,
    {
        Self::new(T::one())
    }
    /// creates a new index with a value of [`zero`](Zero::zero)
    pub fn zero() -> Self
    where
        T: Zero,
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
    /// returns an immutable reference to the inner value
    pub const fn get(&self) -> &T {
        &self.value
    }
    /// returns a mutable reference to the inner value
    pub const fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }
    /// consumes the current instance to return the inner value
    #[inline]
    pub fn value(self) -> T {
        self.value
    }
    /// apply a function to the inner value and returns a new Index wrapping the result
    #[inline]
    pub fn map<U, F>(self, f: F) -> IndexBase<U, K>
    where
        F: FnOnce(T) -> U,
        U: RawIndex,
    {
        IndexBase::new(f(self.value()))
    }
    /// [`replace`](core::mem::replace) and return the old value after replacing it with the
    /// given value
    pub const fn replace(&mut self, index: T) -> T {
        core::mem::replace(self.get_mut(), index)
    }
    /// set the index to the given value
    #[inline]
    pub fn set(&mut self, value: T) -> &mut Self {
        *self.get_mut() = value;
        self
    }
    /// [`swap`](core::mem::swap) the values of two indices
    pub const fn swap(&mut self, other: &mut Self) {
        core::mem::swap(self.get_mut(), other.get_mut())
    }
    /// [`take`](core::mem::take) the value and replace it with the default value
    #[inline]
    pub fn take(&mut self) -> T
    where
        T: Default,
    {
        core::mem::take(self.get_mut())
    }
    /// consumes the current index to create another with the given value
    #[inline]
    pub fn with<U: RawIndex>(self, value: U) -> IndexBase<U, K> {
        IndexBase {
            value,
            _type: core::marker::PhantomData::<K>,
        }
    }
    /// decrements the index value by [one](One) and returns a new instance
    #[inline]
    pub fn dec(self) -> IndexBase<<T as core::ops::Sub>::Output, K>
    where
        T: core::ops::Sub + One,
        <T as core::ops::Sub>::Output: RawIndex,
    {
        let value = self.value - T::one();
        IndexBase::new(value)
    }
    /// mutably decrements the index value by [one](One)
    #[inline]
    pub fn dec_inplace(&mut self)
    where
        T: core::ops::SubAssign + One,
    {
        self.value -= T::one();
    }
    /// increments the index value by [one](One) and consumes the current instance
    /// to create another with the new value.
    #[inline]
    pub fn inc(self) -> IndexBase<<T as core::ops::Add>::Output, K>
    where
        T: core::ops::Add + One,
        <T as core::ops::Add>::Output: RawIndex,
    {
        let value = self.value + T::one();
        IndexBase::new(value)
    }
    /// mutably increments the index value by [one](One)
    #[inline]
    pub fn inc_inplace(&mut self)
    where
        T: Copy + core::ops::AddAssign + One,
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
    pub fn step(&mut self) -> IndexResult<Self>
    where
        T: Copy + core::ops::Add<T, Output = T> + One,
    {
        self.next()
            .ok_or(IndexError::IndexOutOfBounds)
    }

    #[deprecated(since = "0.0.10", note = "use `value` instead")]
    pub fn into_inner(self) -> T {
        self.value
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
    T: RawIndex + Default,
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
    T: RawIndex + PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        &self.value == other
    }
}

impl<T, K> core::iter::Iterator for IndexBase<T, K>
where
    K: GraphIndex,
    T: RawIndex + Copy + core::ops::Add<T, Output = T> + One,
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
