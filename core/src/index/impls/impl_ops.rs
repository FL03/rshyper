/*
    appellation: impl_index <module>
    authors: @FL03
*/
use crate::index::{GraphIndex, IndexBase, RawIndex};

impl<T, K> core::ops::Deref for IndexBase<T, K>
where
    K: GraphIndex,
    T: RawIndex,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T, K> core::ops::DerefMut for IndexBase<T, K>
where
    K: GraphIndex,
    T: RawIndex,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T, K> core::ops::Not for IndexBase<T, K>
where
    K: GraphIndex,
    T: RawIndex + core::ops::Not,
    T::Output: RawIndex,
{
    type Output = IndexBase<T::Output, K>;

    fn not(self) -> Self::Output {
        self.map(|value| !value)
    }
}

impl<T, K> core::ops::Neg for IndexBase<T, K>
where
    K: GraphIndex,
    T: RawIndex + core::ops::Neg,
    T::Output: RawIndex,
{
    type Output = IndexBase<T::Output, K>;

    fn neg(self) -> Self::Output {
        self.map(|value| -value)
    }
}

impl<T, K> num_traits::One for IndexBase<T, K>
where
    K: GraphIndex,
    T: RawIndex + num_traits::One,
{
    fn one() -> Self {
        Self::new(T::one())
    }
}

impl<T, K> num_traits::Zero for IndexBase<T, K>
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

impl<T, K> num::Num for IndexBase<T, K>
where
    K: GraphIndex + Eq,
    T: RawIndex + num::Num,
{
    type FromStrRadixErr = T::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(str, radix).map(IndexBase::new)
    }
}

macro_rules! impl_binary_op {
    (@impl $trait:ident::$method:ident) => {
        impl<K, A, B, C> ::core::ops::$trait<IndexBase<B, K>> for IndexBase<A, K>
        where
            A: RawIndex + ::core::ops::$trait<B, Output = C>,
            B: RawIndex,
            C: RawIndex,
            K: GraphIndex,
        {
            type Output = IndexBase<C, K>;

            fn $method(self, rhs: IndexBase<B, K>) -> Self::Output {
                IndexBase::new(::core::ops::$trait::$method(self.value, rhs.value))
            }
        }
    };
    (@mut $trait:ident::$method:ident) => {
        paste::paste! {
            impl_assign_op!(@impl [<$trait Assign>]::[<$method _assign>]);
        }
    };
    ($($trait:ident::$method:ident),* $(,)?) => {
        $(
            impl_binary_op!(@impl $trait::$method);
            impl_binary_op!(@mut $trait::$method);
        )*
    };
}

macro_rules! impl_assign_op {
    (@impl $trait:ident::$method:ident) => {
        impl<K, A, B> ::core::ops::$trait<B> for IndexBase<A, K>
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

impl_binary_op! {
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
