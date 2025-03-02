/*
    Appellation: modulus <module>
    Contrib: @FL03
*/

// The pymod function you provided
pub fn pymod<T>(lhs: T, rhs: T) -> T
where
    T: Copy + num::Num + PartialOrd,
{
    let r = lhs % rhs;
    if (r < T::zero() && rhs > T::zero()) || (r > T::zero() && rhs < T::zero()) {
        r + rhs
    } else {
        r
    }
}