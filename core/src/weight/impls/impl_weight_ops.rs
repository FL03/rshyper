/*
    appellation: impl_weight_ops <module>
    authors: @FL03
*/
use crate::weight::Weight;
use num_traits::{Num, One, Zero};

impl<T> One for Weight<T>
where
    T: One,
{
    fn one() -> Self {
        Weight(T::one())
    }
}

impl<T> Zero for Weight<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Weight(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.get().is_zero()
    }
}

impl<T> Num for Weight<T>
where
    T: Num,
{
    type FromStrRadixErr = T::FromStrRadixErr;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok(Weight(T::from_str_radix(s, radix)?))
    }
}

macro_rules! impl_wrapper_binary_op {
    ($s:ident::<[$($op:ident.$call:ident),* $(,)?]>) => {
        $(
            impl_wrapper_binary_op!(@impl $s::$op.$call);
            impl_wrapper_binary_op!(@mut $s::$op.$call);
        )*
    };
    (@impl $s:ident::$op:ident.$call:ident) => {
        impl<A, B, C> ::core::ops::$op<$s<B>> for $s<A>
        where
            A: ::core::ops::$op<B, Output = C>,
        {
            type Output = $s<C>;

            fn $call(self, rhs: $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(self.into_inner(), rhs.into_inner()))
            }
        }

        impl<'a, A, B, C> ::core::ops::$op<$s<B>> for &'a $s<A>
        where
            &'a A: ::core::ops::$op<B, Output = C>,
        {
            type Output = $s<C>;

            fn $call(self, rhs: $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(self.get(), rhs.into_inner()))
            }
        }

        impl<'a, A, B, C> ::core::ops::$op<&'a $s<B>> for &'a $s<A>
        where
            &'a A: ::core::ops::$op<&'a B, Output = C>,
        {
            type Output = $s<C>;

            fn $call(self, rhs: &'a $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(self.get(), rhs.get()))
            }
        }

        impl<'a, A, B, C> ::core::ops::$op<&'a $s<B>> for $s<A>
        where
            A: ::core::ops::$op<&'a B, Output = C>,
        {
            type Output = $s<C>;

            fn $call(self, rhs: &'a $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(self.into_inner(), rhs.get()))
            }
        }

        impl<'a, A, B, C> ::core::ops::$op<$s<B>> for &'a mut $s<A>
        where
            &'a A: ::core::ops::$op<B, Output = C>,
        {
            type Output = $s<C>;

            fn $call(self, rhs: $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(&self.0, rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$op<&'a mut $s<B>> for $s<A>
        where
            A: ::core::ops::$op<&'a B, Output = C>,
        {
            type Output = $s<C>;

            fn $call(self, rhs: &'a mut $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(self.0, &rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$op<&'a mut $s<B>> for &'a mut $s<A>
        where
            &'a A: ::core::ops::$op<&'a B, Output = C>,
        {
            type Output = $s<C>;

            fn $call(self, rhs: &'a mut $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(&self.0, &rhs.0))
            }
        }
    };
    (@mut $s:ident::$op:ident.$call:ident) => {
        paste::paste! {
            impl_binary_op_mut!(@impl $s::[<$op Assign>].[<$call _assign>]);
        }
    };
}

macro_rules! impl_binary_op_mut {
    ($s:ident::<[$($op:ident.$call:ident),* $(,)?]>) => {
        $(
            impl_wrapper_binary_op!(@impl $s::$op.$call);
        )*
    };
    (@impl $s:ident::$op:ident.$call:ident) => {
        impl<A, B> ::core::ops::$op<$s<B>> for $s<A>
        where
            A: ::core::ops::$op<B>,
        {

            fn $call(&mut self, rhs: $s<B>) {
                core::ops::$op::$call(self.get_mut(), rhs.into_inner())
            }
        }

        impl<A, B> ::core::ops::$op<$s<B>> for &mut $s<A>
        where
            A: ::core::ops::$op<B>,
        {

            fn $call(&mut self, rhs: $s<B>) {
                core::ops::$op::$call(self.get_mut(), rhs.into_inner())
            }
        }
    };
}

macro_rules! impl_unary_op {
    ($s:ident::<[$($op:ident.$call:ident),* $(,)?]>) => {
        $(
            impl_unary_op!(@impl $s::$op.$call);
        )*
    };
    (@impl $s:ident::$op:ident.$call:ident) => {
        impl<A, B> ::core::ops::$op for $s<A>
        where
            A: ::core::ops::$op<Output = B>,
        {
            type Output = $s<B>;

            fn $call(self) -> Self::Output {
                $s(core::ops::$op::$call(self.into_inner()))
            }
        }

        impl<'a, A, B> ::core::ops::$op for &'a $s<A>
        where
            &'a A: ::core::ops::$op<Output = B>,
        {
            type Output = $s<B>;

            fn $call(self) -> Self::Output {
                $s(core::ops::$op::$call(self.get()))
            }
        }

        impl<'a, A, B> ::core::ops::$op for &'a mut $s<A>
        where
            &'a A: ::core::ops::$op<Output = B>,
        {
            type Output = $s<B>;

            fn $call(self) -> Self::Output {
                $s(core::ops::$op::$call(&self.0))
            }
        }
    };
}

impl_wrapper_binary_op! {
    Weight::<[
        Add.add,
        Sub.sub,
        Mul.mul,
        Div.div,
        Rem.rem,
        BitAnd.bitand,
        BitOr.bitor,
        BitXor.bitxor,
        Shl.shl,
        Shr.shr
    ]>
}

impl_unary_op! {
    Weight::<[
        Neg.neg,
        Not.not
    ]>
}
