/*
    appellation: impl_index <module>
    authors: @FL03
*/
use crate::index::{GraphIndex, Index, RawIndex};

impl<T, K> core::ops::Deref for Index<T, K>
where
    K: GraphIndex,
    T: RawIndex,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T, K> core::ops::DerefMut for Index<T, K>
where
    K: GraphIndex,
    T: RawIndex,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T, K> core::ops::Not for Index<T, K>
where
    K: GraphIndex,
    T: RawIndex + core::ops::Not,
    T::Output: RawIndex,
{
    type Output = Index<T::Output, K>;

    fn not(self) -> Self::Output {
        self.map(|value| !value)
    }
}

impl<T, K> core::ops::Neg for Index<T, K>
where
    K: GraphIndex,
    T: RawIndex + core::ops::Neg,
    T::Output: RawIndex,
{
    type Output = Index<T::Output, K>;

    fn neg(self) -> Self::Output {
        self.map(|value| -value)
    }
}

impl<T, K> num_traits::One for Index<T, K>
where
    K: GraphIndex,
    T: RawIndex + num_traits::One,
{
    fn one() -> Self {
        Self::new(T::one())
    }
}

impl<T, K> num_traits::Zero for Index<T, K>
where
    K: GraphIndex,
    T: RawIndex + num_traits::Zero,
{
    fn zero() -> Self {
        Self::new(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl<T, K> num::Num for Index<T, K>
where
    K: GraphIndex + Eq,
    T: RawIndex + num::Num,
{
    type FromStrRadixErr = T::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(str, radix).map(Index::new)
    }
}

macro_rules! impl_bin_op {
    (@impl $trait:ident::$method:ident) => {
        impl<K, A, B, C> ::core::ops::$trait<Index<B, K>> for Index<A, K>
        where
            A: RawIndex + ::core::ops::$trait<B, Output = C>,
            B: RawIndex,
            C: RawIndex,
            K: GraphIndex,
        {
            type Output = Index<C, K>;

            fn $method(self, rhs: Index<B, K>) -> Self::Output {
                Index::new(::core::ops::$trait::$method(self.value, rhs.value))
            }
        }
    };

    ($($trait:ident::$method:ident),* $(,)?) => {
        $(impl_bin_op!(@impl $trait::$method);)*
    };
}

macro_rules! impl_assign_op {
    (@impl $trait:ident::$method:ident) => {
        impl<K, A, B> ::core::ops::$trait<B> for Index<A, K>
        where
            A: RawIndex + ::core::ops::$trait<B>,
            K: GraphIndex,
        {
            fn $method(&mut self, rhs: B) {
                ::core::ops::$trait::$method(&mut self.value, rhs)
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
