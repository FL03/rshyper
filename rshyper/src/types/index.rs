/*
    Appellation: index <types>
    Contrib: @FL03
*/

// Define a type alias for Vertex ID (can be any hashable type)
pub type VertexId<T = usize> = Index<T, VertexIndex>;
// Define a type alias for HyperEdge ID
pub type EdgeId<T = usize> = Index<T, EdgeIndex>;

pub type Idx = usize;

use core::marker::PhantomData;

pub trait IndexKind: Eq + core::hash::Hash {
    private!();
}

macro_rules! impl_index_kind {
    ($($kind:ident),* $(,)?) => {
        $(
            impl_index_kind!(@impl $kind);
        )*
    };
    (@impl $kind:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(
            feature = "serde",
            derive(serde_derive::Deserialize, serde_derive::Serialize)
        )]
        pub enum $kind {}

        impl IndexKind for $kind {
            seal!();
        }
    }
}

impl_index_kind! {
    EdgeIndex,
    VertexIndex,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Deserialize, serde_derive::Serialize)
)]
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
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }
    /// apply a function to the inner value and returns a new Index wrapping the result
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
    /// consumes the current index to create another with the given value
    pub fn with<U>(self, value: U) -> Index<U, K> {
        Index {
            value,
            _type: PhantomData::<K>,
        }
    }
    /// [`swap`](core::mem::swap) the values of two indices
    pub const fn swap(&mut self, other: &mut Self) {
        core::mem::swap(&mut self.value, &mut other.value)
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

impl<T> EdgeId<T> {
    pub fn vertex(value: T) -> Self {
        Self::from_value(value)
    }
}

impl<T> VertexId<T> {
    pub fn vertex(value: T) -> Self {
        Self::from_value(value)
    }
}

impl<K: IndexKind> Index<usize, K> {
    pub fn atomic() -> Self {
        use core::sync::atomic::{AtomicUsize, Ordering::Relaxed};
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        Self::from_value(COUNTER.fetch_add(1, Relaxed))
    }
}

impl<T, K> Default for Index<T, K>
where
    K: IndexKind,
    T: Default,
{
    fn default() -> Self {
        Self::from_value(T::default())
    }
}

#[cfg(feature = "rand")]
impl<T, K> Index<T, K>
where
    K: IndexKind,
    rand_distr::StandardUniform: rand_distr::Distribution<T>,
{
    pub fn random() -> Self {
        Self::from_value(rand::random())
    }

    pub fn random_in<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        Self::from_value(rng.random())
    }
}

#[cfg(feature = "rand")]
impl<T, K> rand_distr::Distribution<Index<T, K>> for rand_distr::StandardUniform
where
    K: IndexKind,
    rand_distr::StandardUniform: rand_distr::Distribution<T>,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Index<T, K> {
        Index::from_value(rng.random())
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

impl<T, K> core::convert::AsRef<T> for Index<T, K>
where
    K: IndexKind,
{
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<T, K> core::convert::AsMut<T> for Index<T, K>
where
    K: IndexKind,
{
    fn as_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T, K> core::borrow::Borrow<T> for Index<T, K>
where
    K: IndexKind,
{
    fn borrow(&self) -> &T {
        self.get()
    }
}
impl<T, K> core::borrow::BorrowMut<T> for Index<T, K>
where
    K: IndexKind,
{
    fn borrow_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T, K> core::ops::Not for Index<T, K>
where
    K: IndexKind,
    T: core::ops::Not<Output = T>,
{
    type Output = Index<T, K>;

    fn not(self) -> Self::Output {
        self.map(|value| !value)
    }
}

impl<T, K> core::ops::Neg for Index<T, K>
where
    K: IndexKind,
    T: core::ops::Neg<Output = T>,
{
    type Output = Index<T, K>;

    fn neg(self) -> Self::Output {
        self.map(|value| -value)
    }
}

impl<T, K> core::ops::Deref for Index<T, K>
where
    K: IndexKind,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T, K> core::ops::DerefMut for Index<T, K>
where
    K: IndexKind,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T, K> num_traits::One for Index<T, K>
where
    K: IndexKind,
    T: num_traits::One,
{
    fn one() -> Self {
        Self::from_value(T::one())
    }
}

impl<T, K> num_traits::Zero for Index<T, K>
where
    K: IndexKind,
    T: num_traits::Zero,
{
    fn zero() -> Self {
        Self::from_value(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl<T, K> num::Num for Index<T, K>
where
    K: IndexKind,
    T: num::Num,
{
    type FromStrRadixErr = T::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(str, radix).map(Index::from_value)
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

macro_rules! impl_bin_op {
    (@impl $trait:ident::$method:ident) => {
        impl<K, A, B, C> core::ops::$trait<Index<B, K>> for Index<A, K>
        where
            A: core::ops::$trait<B, Output = C>,
            K: IndexKind,
        {
            type Output = Index<C, K>;

            fn $method(self, rhs: Index<B, K>) -> Self::Output {
                Index::from_value(core::ops::$trait::$method(self.value, rhs.value))
            }
        }
    };

    ($($trait:ident::$method:ident),* $(,)?) => {
        $(impl_bin_op!(@impl $trait::$method);)*
    };
}

macro_rules! impl_assign_op {
    (@impl $trait:ident::$method:ident) => {
        impl<K, A, B> core::ops::$trait<B> for Index<A, K>
        where
            A: core::ops::$trait<B>,
            K: IndexKind,
        {
            fn $method(&mut self, rhs: B) {
                core::ops::$trait::$method(&mut self.value, rhs)
            }
        }
    };

    ($($trait:ident::$method:ident),* $(,)?) => {
        $(impl_assign_op!(@impl $trait::$method);)*
    };
}

impl_assign_op! {
    AddAssign::add_assign,
    SubAssign::sub_assign,
    MulAssign::mul_assign,
    DivAssign::div_assign,
    RemAssign::rem_assign,
    BitAndAssign::bitand_assign,
    BitOrAssign::bitor_assign,
    BitXorAssign::bitxor_assign,
    ShlAssign::shl_assign,
    ShrAssign::shr_assign,
}

impl_bin_op! {
    Add::add,
    Sub::sub,
    Mul::mul,
    Div::div,
    Rem::rem,
    BitAnd::bitand,
    BitOr::bitor,
    BitXor::bitxor,
    Shl::shl,
    Shr::shr,
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
