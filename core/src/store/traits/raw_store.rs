/*
    appellation: store <module>
    authors: @FL03
*/
use super::RawContainer;
use crate::idx::{RawIndex, VertexId};

/// [`RawStore`] is a trait that defines the behavior of a store that holds the vertices
/// associated with a hyperedge or hyperfacet. It is used to abstract over different
/// implementations of edge storage, such as arrays, vectors, or sets.
///
/// **note:** The trait is sealed to prevent external implementations, ensuring that only the
/// crate can define how edges are stored. This is to maintain consistency and prevent
/// misuse of the trait in different contexts.
pub trait RawStore: RawContainer {
    type Store<_T>: ?Sized;

    private!();
    /// returns the number of vertices associated with the edge.
    fn len(&self) -> usize;
    /// returns true if there are no points.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
/// An [`EdgeStore`] is a trait is a specialization of the [`RawStore`] trait that represents
/// a store for edges, which are collections of vertices. It is used to define the behavior
pub trait EdgeStore<Idx = usize>: RawStore<Item = VertexId<Idx>>
where
    Idx: RawIndex,
{
}
/// The [`BinaryStore`] trait extends the [`RawStore`] trait to provide specific methods for
/// binary edges, which are edges that connect exactly two vertices.
pub trait BinaryStore<Idx>: RawStore<Item = VertexId<Idx>>
where
    Idx: RawIndex,
{
    /// returns the left-hand side vertex of the edge.
    fn src(&self) -> &Self::Item;
    /// returns the right-hand side vertex of the edge.
    fn tgt(&self) -> &Self::Item;
}
/// The [`StoreIter`] trait extends the [`RawStore`] trait to provide iteration capabilities
/// over the vertices stored in the edge.
pub trait StoreIter<Idx = usize>: RawStore<Item = VertexId<Idx>>
where
    Idx: RawIndex,
{
    type Iter<'a, _T>: Iterator<Item = &'a _T>
    where
        _T: 'a,
        Self: 'a;
    /// returns an iterator over the vertices in the store.
    fn iter(&self) -> Self::Iter<'_, Self::Item>;
}

/*
 ************* Implementations *************
*/
impl<S, Idx> EdgeStore<Idx> for S
where
    Idx: RawIndex,
    S: RawStore<Item = VertexId<Idx>>,
{
}

impl<I> RawStore for &[VertexId<I>]
where
    I: RawIndex,
{
    type Store<_T> = [_T];

    seal!();

    fn len(&self) -> usize {
        <Self::Store<Self::Item>>::len(self)
    }

    fn is_empty(&self) -> bool {
        <Self::Store<Self::Item>>::is_empty(self)
    }
}

impl<I> RawStore for [VertexId<I>]
where
    I: RawIndex,
{
    type Store<_T> = [_T];

    seal!();

    fn len(&self) -> usize {
        <Self::Store<Self::Item>>::len(self)
    }

    fn is_empty(&self) -> bool {
        <Self::Store<Self::Item>>::is_empty(self)
    }
}

impl<const N: usize, I> RawStore for [VertexId<I>; N]
where
    I: RawIndex,
{
    type Store<_T> = [_T; N];

    seal!();

    fn len(&self) -> usize {
        <[Self::Item]>::len(self)
    }

    fn is_empty(&self) -> bool {
        <[Self::Item]>::is_empty(self)
    }
}

impl<I> StoreIter<I> for &[VertexId<I>]
where
    I: RawIndex,
{
    type Iter<'b, _T: 'b>
        = core::slice::Iter<'b, _T>
    where
        Self: 'b;

    fn iter(&self) -> Self::Iter<'_, VertexId<I>> {
        <Self::Store<Self::Item>>::iter(self)
    }
}

impl<I> StoreIter<I> for [VertexId<I>]
where
    I: RawIndex,
{
    type Iter<'a, _T: 'a> = core::slice::Iter<'a, _T>;

    fn iter(&self) -> Self::Iter<'_, Self::Item> {
        <Self::Store<Self::Item>>::iter(self)
    }
}

impl<const N: usize, I> StoreIter<I> for [VertexId<I>; N]
where
    I: RawIndex,
{
    type Iter<'a, _T: 'a> = core::slice::Iter<'a, _T>;

    fn iter(&self) -> Self::Iter<'_, VertexId<I>> {
        <[Self::Item]>::iter(self)
    }
}

impl<I> BinaryStore<I> for [VertexId<I>; 2]
where
    I: RawIndex,
{
    fn src(&self) -> &VertexId<I> {
        &self[0]
    }

    fn tgt(&self) -> &VertexId<I> {
        &self[1]
    }
}

impl<I> RawStore for (VertexId<I>, VertexId<I>)
where
    I: RawIndex,
{
    type Store<_T> = (_T, _T);

    seal!();

    fn len(&self) -> usize {
        2
    }

    fn is_empty(&self) -> bool {
        false
    }
}

impl<I> BinaryStore<I> for (VertexId<I>, VertexId<I>)
where
    I: RawIndex,
{
    fn src(&self) -> &VertexId<I> {
        &self.0
    }

    fn tgt(&self) -> &VertexId<I> {
        &self.1
    }
}

#[allow(unused_macros)]
macro_rules! impl_raw_store {
    (@impl $t:ident<$T:ident>) => {
        impl<$T> $crate::store::RawStore for $t<VertexId<$T>>
        where
            $T: $crate::idx::RawIndex,
        {
            type Store<_T> = $t<_T>;

            seal!();

            fn len(&self) -> usize {
                <Self::Store<Self::Item>>::len(self)
            }

            fn is_empty(&self) -> bool {
                <Self::Store<Self::Item>>::is_empty(self)
            }
        }
    };
    ($($t:ident<$T:ident>),* $(,)?) => {
        $(
            impl_raw_store!(@impl $t<$T>);
        )*
    };
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::StoreIter;
    use crate::idx::{RawIndex, VertexId};
    use alloc::collections::{
        btree_set::{self, BTreeSet},
        vec_deque::{self, VecDeque},
    };
    use alloc::vec::Vec;

    impl_raw_store! {
        BTreeSet<I>,
        Vec<I>,
        VecDeque<I>
    }

    impl<I> StoreIter<I> for BTreeSet<VertexId<I>>
    where
        I: RawIndex,
    {
        type Iter<'a, _T: 'a> = btree_set::Iter<'a, _T>;

        fn iter(&self) -> Self::Iter<'_, VertexId<I>> {
            <Self::Store<Self::Item>>::iter(self)
        }
    }

    impl<I> StoreIter<I> for Vec<VertexId<I>>
    where
        I: RawIndex,
    {
        type Iter<'a, _T: 'a> = core::slice::Iter<'a, _T>;

        fn iter(&self) -> Self::Iter<'_, VertexId<I>> {
            self.as_slice().iter()
        }
    }

    impl<I> StoreIter<I> for VecDeque<VertexId<I>>
    where
        I: RawIndex,
    {
        type Iter<'a, _T: 'a> = vec_deque::Iter<'a, _T>;

        fn iter(&self) -> Self::Iter<'_, VertexId<I>> {
            <Self::Store<Self::Item>>::iter(self)
        }
    }
}

#[cfg(feature = "std")]
mod impl_std {
    use super::{RawStore, StoreIter};
    use crate::idx::{RawIndex, VertexId};
    use core::hash::BuildHasher;
    use std::collections::hash_set::{self, HashSet};

    impl<I, S> RawStore for HashSet<VertexId<I>, S>
    where
        I: RawIndex,
        S: BuildHasher,
    {
        type Store<_T> = HashSet<_T, S>;

        seal!();

        fn len(&self) -> usize {
            <Self::Store<Self::Item>>::len(self)
        }

        fn is_empty(&self) -> bool {
            <Self::Store<Self::Item>>::is_empty(self)
        }
    }

    impl<I, S> StoreIter<I> for HashSet<VertexId<I>, S>
    where
        I: RawIndex,
        S: BuildHasher,
    {
        type Iter<'a, _T: 'a>
            = hash_set::Iter<'a, _T>
        where
            S: 'a;

        fn iter(&self) -> Self::Iter<'_, VertexId<I>> {
            <Self::Store<Self::Item>>::iter(self)
        }
    }
}
