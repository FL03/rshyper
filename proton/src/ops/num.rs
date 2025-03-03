/*
    Appellation: num <module>
    Contrib: @FL03
*/

/// A trait for modular arithmetic; the algorithm is based on the Python `%` operator which
/// uses the sign of the denominator rather than the numerator.
pub trait PyMod<Rhs> {
    type Output;

    fn pymod(self, rhs: Rhs) -> Self::Output;
}

pub trait PitchMod {
    type Output;

    fn pmod(self) -> Self::Output;
}

impl<A, B, C> PyMod<B> for A
where
    A: core::ops::Rem<B, Output = C>,
    B: Copy + num::Zero + PartialOrd,
    C: core::ops::Add<B, Output = C> + num::Zero + PartialOrd,
{
    type Output = C;

    fn pymod(self, rhs: B) -> Self::Output {
        let r = self % rhs;
        if (r < C::zero() && rhs > B::zero()) || (r > C::zero() && rhs < B::zero()) {
            r + rhs
        } else {
            r
        }
    }
}

impl<A> PitchMod for A
where
    A: PyMod<A> + num::FromPrimitive,
{
    type Output = A::Output;

    fn pmod(self) -> Self::Output {
        self.pymod(A::from_usize(12).unwrap())
    }
}
