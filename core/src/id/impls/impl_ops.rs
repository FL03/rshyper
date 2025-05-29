/*
    appellation: impl_index <module>
    authors: @FL03
*/
use crate::id::{Index, IndexKind};

impl<T, K> Index<T, K>
where
    K: IndexKind,
{
    /// creates a new index with a value of [`one`](num_traits::One)
    pub fn one() -> Self
    where
        T: num_traits::One,
    {
        Self::from_value(T::one())
    }
    /// creates a new index with a value of [`zero`](num_traits::Zero)
    pub fn zero() -> Self
    where
        T: num_traits::Zero,
    {
        Self::from_value(T::zero())
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

impl<T, K> core::ops::Not for Index<T, K>
where
    K: IndexKind,
    T: core::ops::Not,
{
    type Output = Index<T::Output, K>;

    fn not(self) -> Self::Output {
        self.map(|value| !value)
    }
}

impl<T, K> core::ops::Neg for Index<T, K>
where
    K: IndexKind,
    T: core::ops::Neg,
{
    type Output = Index<T::Output, K>;

    fn neg(self) -> Self::Output {
        self.map(|value| -value)
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
    K: IndexKind + Eq,
    T: num::Num,
{
    type FromStrRadixErr = T::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(str, radix).map(Index::from_value)
    }
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
