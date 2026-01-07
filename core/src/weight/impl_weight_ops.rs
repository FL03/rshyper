/*
    appellation: impl_weight_ops <module>
    authors: @FL03
*/
use crate::weight::Weight;
use num_traits::{Num, One, Zero};

contained::fmt_wrapper! {
    impl Weight<T> {
        Binary,
        Debug,
        Display,
        LowerExp,
        LowerHex,
        UpperExp,
        UpperHex,
        Octal,
        Pointer,
    }
}

contained::binary_wrapper! {
    impl Weight {
        Add.add,
        Div.div,
        Mul.mul,
        Sub.sub,
        Rem.rem,
        BitAnd.bitand,
        BitOr.bitor,
        BitXor.bitxor,
        Shl.shl,
        Shr.shr,

    }
}

contained::unary_wrapper! {
    impl Weight {
        Neg.neg,
        Not.not,
    }
}

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
