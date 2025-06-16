/*
    appellation: counter <module>
    authors: @FL03
*/
use num_traits::One;

/// The [`Counter`] is a simple iterator that increments some value, `T`, by one. It
/// accomplishes this by relying on the [`replace`](core::mem::replace) function to
///
///
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, transparent, rename_all = "lowercase")
)]
#[repr(transparent)]
pub struct Counter<T> {
    pub(crate) curr: T,
}

pub struct OwnedCounter<'a, T> {
    pub(crate) counter: Counter<T>,
    pub(crate) _marker: core::marker::PhantomData<&'a T>,
}

#[allow(dead_code)]
impl<T> Counter<T> {
    /// construct a new counter from the given value
    pub const fn new(curr: T) -> Self {
        Self { curr }
    }
    /// returns a reference to the current value of the counter
    pub const fn curr(&self) -> &T {
        &self.curr
    }
    /// returns a mutable reference to the current value of the counter
    pub const fn curr_mut(&mut self) -> &mut T {
        &mut self.curr
    }
    /// [`replace`](core::mem::replace) the current value of the counter with the given value
    /// and return the previous value
    pub const fn replace(&mut self, new: T) -> T {
        core::mem::replace(self.curr_mut(), new)
    }
    /// resets the current counter back to the logical default of the type
    pub fn reset(&mut self) -> &mut Self
    where
        T: Default,
    {
        self.set(T::default())
    }
    /// set the current value and return a mutable reference to the counter
    pub fn set(&mut self, new: T) -> &mut Self {
        *self.curr_mut() = new;
        self
    }
    /// [`swap`](core::mem::swap) the current value of the counter with the given value and
    /// returns the previous destination value
    pub const fn swap(&mut self, rhs: &mut Self) {
        core::mem::swap(self.curr_mut(), rhs.curr_mut())
    }
    /// [`take`](core::mem::take) the current value of the counter and return it, leaving the
    /// logical default of type `T` in its place
    pub fn take(&mut self) -> T
    where
        T: Default,
    {
        core::mem::take(self.curr_mut())
    }
}

impl<T> Iterator for Counter<T>
where
    T: One,
    for<'b> &'b T: core::ops::Add<T, Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // compute the next value
        let next = self.curr() + T::one();
        // replace & return the current value with the next one
        let curr = self.replace(next);
        // return the previous value
        Some(curr)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

impl<'a, T> OwnedCounter<'a, T> {
    /// construct a new owned counter from the given value
    pub const fn new(initial_value: T) -> Self {
        Self {
            counter: Counter::new(initial_value),
            _marker: core::marker::PhantomData,
        }
    }
}

impl<'a, T> core::ops::Deref for OwnedCounter<'a, T> {
    type Target = Counter<T>;

    fn deref(&self) -> &Self::Target {
        &self.counter
    }
}

impl<'a, T> core::ops::DerefMut for OwnedCounter<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.counter
    }
}

impl<'a, T> Iterator for OwnedCounter<'a, T>
where
    T: One,
    for<'b> &'b T: core::ops::Add<T, Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.counter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.counter.size_hint()
    }
}
