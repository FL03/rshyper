/*
    Appellation: index <types>
    Contrib: @FL03
*/

// Define a type alias for Vertex ID (can be any hashable type)
pub type VertexId<T = usize> = Index<T>;
// Define a type alias for HyperEdge ID
pub type EdgeId<T = usize> = Index<T>;

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Deserialize, serde_derive::Serialize)
)]
pub struct Index<T = usize>(pub T);

impl<T> Index<T> {
    pub fn new(index: T) -> Self {
        Index(index)
    }
    /// returns a pointer to the inner value
    pub const fn as_ptr(&self) -> *const T {
        core::ptr::from_ref(&self.0)
    }
    /// returns a mutable pointer to the inner value
    pub fn as_mut_ptr(&mut self) -> *mut T {
        core::ptr::from_mut(&mut self.0)
    }
    /// consumes the index returning the inner value
    pub fn into_inner(self) -> T {
        self.0
    }
    /// returns an immutable reference to the inner value
    pub const fn get(&self) -> &T {
        &self.0
    }
    /// returns a mutable reference to the inner value
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
    /// apply a function to the inner value and returns a new Index wrapping the result
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Index<U> {
        Index(f(self.0))
    }
    /// replaces the inner value with the given one and returns the old value
    pub const fn replace(&mut self, index: T) -> T {
        core::mem::replace(&mut self.0, index)
    }
    /// set the index to the given value
    pub fn set(&mut self, index: T) {
        self.0 = index;
    }
    /// swap the values of two indices
    pub const fn swap(&mut self, other: &mut Self) {
        core::mem::swap(&mut self.0, &mut other.0)
    }
}

impl<T> PartialEq<T> for Index<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        &self.0 == other
    }
}

#[cfg(feature = "rand")]
impl<T> Index<T>
where
    rand_distr::StandardUniform: rand_distr::Distribution<T>,
{
    pub fn random() -> Self {
        Index(rand::random())
    }
    pub fn random_in<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        Index(rng.random())
    }
}

#[cfg(feature = "rand")]
impl<T> rand_distr::Distribution<Index<T>> for rand_distr::StandardUniform
where
    rand_distr::StandardUniform: rand_distr::Distribution<T>,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Index<T> {
        Index(rng.random())
    }
}

impl<T> core::iter::Iterator for Index<T>
where
    T: for<'a> core::ops::Add<&'a T, Output = T> + num::One,
{
    type Item = Index<T>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Index(T::one() + &self.0))
    }
}

impl<T> core::convert::AsRef<T> for Index<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> core::convert::AsMut<T> for Index<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> core::borrow::Borrow<T> for Index<T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T> core::borrow::BorrowMut<T> for Index<T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> core::ops::Deref for Index<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> core::ops::DerefMut for Index<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> core::ops::Neg for Index<T>
where
    T: core::ops::Neg,
{
    type Output = Index<<T as core::ops::Neg>::Output>;

    fn neg(self) -> Self::Output {
        Index(-self.0)
    }
}

impl<T> core::ops::Not for Index<T>
where
    T: core::ops::Not,
{
    type Output = Index<<T as core::ops::Not>::Output>;

    fn not(self) -> Self::Output {
        Index(!self.0)
    }
}

impl<T> num::One for Index<T>
where
    T: num::One,
{
    fn one() -> Self {
        Index(T::one())
    }
}

impl<T> num::Zero for Index<T>
where
    T: num::Zero,
{
    fn zero() -> Self {
        Index(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<T> num::Num for Index<T>
where
    T: num::Num,
{
    type FromStrRadixErr = T::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(str, radix).map(Index)
    }
}

macro_rules! impl_fmt {
    ($($trait:ident),* $(,)?) => {
        $(impl<T: core::fmt::$trait> core::fmt::$trait for Index<T> {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                core::fmt::$trait::fmt(&self.0, f)
            }
        })*
    };
}

macro_rules! impl_bin_op {
    (@impl $trait:ident::$method:ident) => {
        impl<A, B, C> core::ops::$trait<Index<B>> for Index<A> where A: core::ops::$trait<B, Output = C>{
            type Output = Index<C>;

            fn $method(self, rhs: Index<B>) -> Self::Output {
                Index(core::ops::$trait::$method(self.0, rhs.0))
            }
        }
    };

    ($($trait:ident::$method:ident),* $(,)?) => {
        $(impl_bin_op!(@impl $trait::$method);)*
    };
}

macro_rules! impl_assign_op {
    (@impl $trait:ident::$method:ident) => {
        impl<A, B> core::ops::$trait<B> for Index<A> where A: core::ops::$trait<B> {
            fn $method(&mut self, rhs: B) {
                core::ops::$trait::$method(&mut self.0, rhs)
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
