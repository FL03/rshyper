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
    (@impl $($p:ident)::*<$I:ident>) => {
        impl<$I> $crate::store::RawContainer for $($p)::*<$I> {
            type Item = $I;

            seal!();
        }
    };
    (@impl $($p:ident)::*<$K:ident, $V:ident>) => {
        impl<$K, $V> $crate::store::RawContainer for $($p)::*<$K, $V> {
            type Item = $V;

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
    std::collections::HashMap<K, V>,
    std::collections::HashSet<T>,
    std::sync::Mutex<T>,
}
