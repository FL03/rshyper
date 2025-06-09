/*
    appellation: impl_weight <module>
    authors: @FL03
*/
use crate::weight::Weight;

impl<T> Weight<&T> {
    /// returns a new [`Weight`] using a clone of the current inner value.
    #[inline]
    pub fn cloned(&self) -> Weight<T>
    where
        T: Clone,
    {
        Weight::new(self.0.clone())
    }
    /// returns a new instance of the [`Weight`] using a copy of the current inner value.
    #[inline]
    pub fn copied(&self) -> Weight<T>
    where
        T: Copy,
    {
        Weight::new(*self.0)
    }
}

impl<T> Weight<&mut T> {
    /// returns a new [`Weight`] using a clone of the current inner value.
    #[inline]
    pub fn cloned(&self) -> Weight<T>
    where
        T: Clone,
    {
        Weight::new(self.0.clone())
    }
    /// returns a new instance of the [`Weight`] using a copy of the current inner value.
    #[inline]
    pub const fn copied(&self) -> Weight<T>
    where
        T: Copy,
    {
        Weight::new(*self.0)
    }
}

impl<T> AsRef<T> for Weight<T> {
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<T> AsMut<T> for Weight<T> {
    fn as_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> core::borrow::Borrow<T> for Weight<T> {
    fn borrow(&self) -> &T {
        self.get()
    }
}

impl<T> core::borrow::BorrowMut<T> for Weight<T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> core::ops::Deref for Weight<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> core::ops::DerefMut for Weight<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<T> From<T> for Weight<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> PartialEq<T> for Weight<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        self.get() == other
    }
}

impl<'a, T> PartialEq<&'a T> for Weight<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &&'a T) -> bool {
        self.get() == *other
    }
}

impl<'a, T> PartialEq<&'a mut T> for Weight<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &&'a mut T) -> bool {
        self.get() == *other
    }
}

impl<T> PartialOrd<T> for Weight<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &T) -> Option<core::cmp::Ordering> {
        self.get().partial_cmp(other)
    }
}
