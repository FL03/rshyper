/*
    appellation: container <module>
    authors: @FL03
*/

/// [`Contains`] defines a common interface for types able to verify if they contain a given
/// key or index;
pub trait Contains<T> {
    type Q;
    /// checks if the container contains the given index
    fn contains(&self, key: &Self::Q) -> bool
    where
        T: core::borrow::Borrow<Self::Q>;
}

pub unsafe trait RawData {
    type Item;
}

pub trait RawContainer<T> {
    type Data<U>: RawData<Item = U> + ?Sized;
}

pub trait RawContainerMut<T>: RawContainer<T> {
    fn as_mut(&mut self) -> &mut Self::Data<T>;
}

pub trait Container<T>: RawContainer<T> {
    /// returns an immutable reference to the container
    fn as_ref(&self) -> &Self::Data<T>;
    /// returns a mutable reference to the container
    fn as_mut(&mut self) -> &mut Self::Data<T>;
    /// returns a reference to the container as a slice
    fn as_slice(&self) -> &[T];
    /// returns a mutable slice of the container
    fn as_mut_slice(&mut self) -> &mut [T];
    /// returns the number of elements stored within the container
    fn len(&self) -> usize {
        self.as_slice().len()
    }
    /// check if the container is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

macro_rules! raw_store {
    ($($($name:ident)::*<$T:ident>),* $(,)?) => {
        $(
            raw_store!(@impl $($name)::*<$T>);
        )*
    };
    (@impl $($name:ident)::*<$T:ident>) => {
        unsafe impl<$T> RawData for $($name)::*<$T> {
            type Item = $T;
        }

        impl<$T> RawContainer<$T> for $($name)::*<$T> {
            type Data<_T> = $($name)::*<_T>;
        }
    };
}

raw_store! {
    crate::Weight<T>,
}

#[cfg(feature = "alloc")]
raw_store! {
    alloc::boxed::Box<T>,
    alloc::collections::BTreeSet<T>,
    alloc::collections::LinkedList<T>,
    alloc::rc::Rc<T>,
    alloc::sync::Arc<T>,
    alloc::vec::Vec<T>,
}

#[cfg(feature = "std")]
raw_store! {
    std::cell::Cell<T>,
    std::collections::HashSet<K>,
}

unsafe impl<T> RawData for [T]
where
    T: Sized,
{
    type Item = T;
}

unsafe impl<'a, T> RawData for &'a [T] {
    type Item = T;
}

unsafe impl<'a, T> RawData for &'a mut [T] {
    type Item = T;
}

impl<T> RawContainer<T> for [T]
where
    T: Sized,
{
    type Data<U> = [U];
}

impl<T> Container<T> for [T]
where
    T: Sized,
{
    fn as_ref(&self) -> &Self::Data<T> {
        self
    }

    fn as_mut(&mut self) -> &mut Self::Data<T> {
        self
    }

    fn as_slice(&self) -> &[T] {
        self
    }

    fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }
}
