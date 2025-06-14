/*
    appellation: raw_container <module>
    authors: @FL03
*/

/// the [`RawContainer`] trait defines the most basic interface for a container composed of
/// elements of type [`Item`](RawContainer::Item).
pub trait RawContainer {
    type Item;

    private!();
}

/*
 ************* Implementations *************
*/
#[cfg(feature = "alloc")]
impl<T> RawContainer for alloc::boxed::Box<dyn RawContainer<Item = T>> {
    type Item = T;

    seal!();
}

macro_rules! impl_raw_container {
    (@impl $($p:ident)::*<$K:ident, $V:ident, $($rest:ident),*>) => {
        impl<$K, $V, $($rest),*> $crate::store::RawContainer for $($p)::*<$K, $V, $($rest),*> {
            type Item = $V;

            seal!();
        }
    };
    (@impl $($p:ident)::*<$K:ident, $V:ident>) => {
        impl<$K, $V> $crate::store::RawContainer for $($p)::*<$K, $V> {
            type Item = $V;

            seal!();
        }
    };
    (@impl $($p:ident)::*<$I:ident>) => {
        impl<$I> $crate::store::RawContainer for $($p)::*<$I> {
            type Item = $I;

            seal!();
        }
    };
    ($($($p:ident)::*<$($T:ident),*>),* $(,)?) => {
        $(
            impl_raw_container!(@impl $($p)::*<$($T),*>);
        )*
    };
}

impl_raw_container! {
    Option<T>,
    core::cell::Cell<T>,
    core::cell::RefCell<T>,
    core::marker::PhantomData<T>,
}

#[cfg(feature = "alloc")]
impl_raw_container! {
    alloc::boxed::Box<T>,
    alloc::collections::BTreeMap<K, V>,
    alloc::collections::BTreeSet<T>,
    alloc::collections::VecDeque<T>,
    alloc::sync::Arc<T>,
    alloc::vec::Vec<T>,
}

#[cfg(feature = "std")]
impl_raw_container! {
    std::collections::HashMap<K, V, S>,
    std::sync::Mutex<T>,
}

#[cfg(feature = "std")]
impl<K, S> RawContainer for std::collections::HashSet<K, S> {
    type Item = K;

    seal!();
}

impl<T> RawContainer for [T] {
    type Item = T;

    seal!();
}

impl<'a, T> RawContainer for &'a [T] {
    type Item = T;

    seal!();
}

impl<'a, T> RawContainer for &'a mut [T] {
    type Item = T;

    seal!();
}

impl<T, const N: usize> RawContainer for [T; N] {
    type Item = T;

    seal!();
}

impl<'a, T, const N: usize> RawContainer for &'a [T; N] {
    type Item = T;

    seal!();
}

impl<'a, T, const N: usize> RawContainer for &'a mut [T; N] {
    type Item = T;

    seal!();
}

#[allow(unused_macros)]
macro_rules! tuple_cont {
    ($($T:ident),+ $(,)?) => {
        impl<$($T),+> $crate::store::RawContainer for ($($T,)+) {
            type Item =  ($($T,)+);

            seal!();
        }
    };
}

impl<T> RawContainer for (T,) {
    type Item = T;

    seal!();
}

impl<T> RawContainer for (T, T) {
    type Item = T;

    seal!();
}

impl<T> RawContainer for (T, T, T) {
    type Item = T;

    seal!();
}
