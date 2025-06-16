/*
    Appellation: index <module>
    Contrib: @FL03
*/
use crate::idx::RawIndex;
use crate::idx::error::IndexResult;
use crate::{AddStep, StepWith};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
use num_traits::{One, Zero};

/// A generic [`IndexBase`] implementation used to represent various [_kinds_](GraphIndex) of
/// indices
#[derive(Clone, Copy, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct IndexBase<Idx = super::Udx, K = super::VertexIndex> {
    pub(crate) value: Idx,
    pub(crate) _type: core::marker::PhantomData<K>,
}

impl<T, K> IndexBase<T, K>
where
    T: RawIndex,
{
    /// returns a new instance of [`IndexBase`] with the given value.
    pub fn new(index: T) -> Self {
        Self {
            value: index,
            _type: core::marker::PhantomData::<K>,
        }
    }
    /// creates a new instance of [`IndexBase`] using the given function to generate the value
    pub fn new_with<F>(index: F) -> Self
    where
        F: FnOnce() -> T,
    {
        Self::new(index())
    }
    /// creates a new index with a value of [`one`](One::one)
    pub fn one() -> Self
    where
        T: One,
    {
        Self::new_with(T::one)
    }
    /// creates a new index with a value of [`zero`](Zero::zero)
    pub fn zero() -> Self
    where
        T: Zero,
    {
        Self::new_with(T::zero)
    }
    /// returns a pointer to the inner value
    pub const fn as_ptr(&self) -> *const T {
        core::ptr::from_ref(&self.value)
    }
    /// returns a mutable pointer to the inner value
    pub fn as_mut_ptr(&mut self) -> *mut T {
        core::ptr::from_mut(&mut self.value)
    }
    #[cfg(feature = "alloc")]
    /// converts the a reference to a boxed raw index trait object
    pub fn as_raw_box(&self) -> Box<dyn RawIndex>
    where
        T: Clone,
    {
        Box::new(self.value.clone())
    }
    #[cfg(feature = "alloc")]
    /// boxes up the raw index value for generic use
    pub fn into_raw_box(self) -> Box<dyn RawIndex> {
        Box::new(self.value)
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
        self.next_with(|prev| prev + T::one())
            .expect("Failed to increment index")
    }
    /// mutably increments the index value by [`1`](One)
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
        T: AddStep<Output = T>,
    {
        let prev = self.get_mut().add_step();
        Ok(IndexBase {
            value: prev,
            _type: core::marker::PhantomData::<K>,
        })
    }
    /// replaces the current value with the next one computed using the provided function and
    /// returns the previous instance of the index.
    pub fn step_with<F>(&mut self, f: F) -> IndexResult<Self>
    where
        F: FnOnce(&T) -> T,
    {
        // step the index with the provided function
        let prev = StepWith::step_with(self, f);
        // return the previous value
        Ok(prev)
    }
    /// similar to [`step_with`](IndexBase::step_with), however, rather than replacing the
    /// current value with the computed value, it returns a new instance of the index
    /// containing the computed value.
    pub fn next_with<U, F>(self, f: F) -> IndexResult<IndexBase<U, K>>
    where
        F: FnOnce(T) -> U,
        U: RawIndex,
    {
        // compute the next value using the provided function
        let next = f(self.value);
        // return the previous instance
        Ok(IndexBase::new(next))
    }
    #[deprecated(since = "0.0.10", note = "use `value` instead")]
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T, K> StepWith<T> for IndexBase<T, K>
where
    T: RawIndex,
{
    type Output = IndexBase<T, K>;

    fn step_with<F>(&mut self, f: F) -> Self::Output
    where
        F: FnOnce(&T) -> T,
    {
        // compute the next value using the provided function
        let next = f(self.get());
        // replace the current value with the next one
        let prev = self.replace(next);
        // return the previous instance
        Self::new(prev)
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
            T: ::core::fmt::$trait,
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
