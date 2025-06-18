/*
    appellation: index <module>
    authors: @FL03
*/
/// a simple trait for denoting types compatible with to be used as indices in a hypergraph.
/// **note:** the trait is sealed to prevent external implementations.
pub trait RawIndex: 'static + Send + Sync + core::fmt::Debug + core::fmt::Display {
    private!();

    /// converts the index to a string representation.
    #[cfg(feature = "alloc")]
    fn to_string(&self) -> alloc::string::String {
        alloc::format!("{self}")
    }
}
/// The [`HyperIndex`] trait extends the [`RawIndex`] trait to include additional operations
/// and behaviours commonly expected from indices in a hypergraph.
///
/// **note:** the trait is automatically implemented for all types that implement [`RawIndex`]
/// alongside traits including: [Clone], [Default], [PartialEq], and [PartialOrd]
pub trait HyperIndex: RawIndex
where
    Self: Clone + Default + PartialEq + PartialOrd,
{
    private!();
}
/// The [`HashIndex`] trait extends the [`StdIndex`] trait to include additional operations and
/// behaviours commonly expected from indices in a hypergraph.
///
/// **note:** this trait is automatically implemented for all [`RawIndex`] implementors that
/// also implements the [`Eq`] and [`Hash`](core::hash::Hash) traits and is sealed to prevent
/// any external implementations.
pub trait HashIndex: RawIndex
where
    Self: Eq + core::hash::Hash,
{
    private!();
}
/// The [`NumIndex`] trait extends the [`RawIndex`] trait to include additional operations and
/// behaviours expected from numerical indices in a hypergraph.
///
/// **note:** the trait is automatically implemented for all types that implement [`HashIndex`]
/// alongside additional traits
pub trait NumIndex: HashIndex
where
    Self: Copy
        + Ord
        + crate::AddStep<Output = Self>
        + core::ops::Add<Output = Self>
        + core::ops::Div<Output = Self>
        + core::ops::Mul<Output = Self>
        + core::ops::Rem<Output = Self>
        + core::ops::Shl<Output = Self>
        + core::ops::Shr<Output = Self>
        + core::ops::Sub<Output = Self>
        + core::ops::AddAssign
        + core::ops::DivAssign
        + core::ops::MulAssign
        + core::ops::RemAssign
        + core::ops::ShlAssign
        + core::ops::ShrAssign
        + core::ops::SubAssign
        + core::ops::Not
        + num_traits::FromPrimitive
        + num_traits::ToPrimitive
        + num_traits::One
        + num_traits::Zero
        + num_traits::ToBytes
        + num_traits::Num
        + num_traits::NumRef
        + num_traits::NumAssignRef,
{
}
/*
 ************* Implementations *************
*/

impl<T> HashIndex for T
where
    T: RawIndex + Eq + core::hash::Hash,
{
    seal!();
}

impl<T> HyperIndex for T
where
    T: 'static + RawIndex + Copy + Default + PartialEq + PartialOrd,
{
    seal!();
}

impl<T> NumIndex for T where
    T: HashIndex
        + Copy
        + Default
        + Ord
        + crate::AddStep<Output = Self>
        + core::ops::Add<Output = Self>
        + core::ops::Div<Output = Self>
        + core::ops::Mul<Output = Self>
        + core::ops::Rem<Output = Self>
        + core::ops::Shl<Output = Self>
        + core::ops::Shr<Output = Self>
        + core::ops::Sub<Output = Self>
        + core::ops::AddAssign
        + core::ops::DivAssign
        + core::ops::MulAssign
        + core::ops::RemAssign
        + core::ops::ShlAssign
        + core::ops::ShrAssign
        + core::ops::SubAssign
        + core::ops::Not
        + num_traits::FromPrimitive
        + num_traits::ToPrimitive
        + num_traits::One
        + num_traits::Zero
        + num_traits::ToBytes
        + num_traits::NumRef
        + num_traits::NumAssignRef
{
}

#[cfg(feature = "alloc")]
impl RawIndex for alloc::boxed::Box<dyn RawIndex> {
    seal!();
}

#[cfg(feature = "alloc")]
impl RawIndex for alloc::boxed::Box<dyn RawIndex + Send + Sync + 'static> {
    seal!();
}

macro_rules! impl_raw_index {
    ($($t:ty),* $(,)?) => {
        $(
            impl_raw_index!(@impl $t);
        )*
    };
    (@impl $t:ty) => {
        impl RawIndex for $t {
            seal!();
        }
    };
}

impl_raw_index! {
    usize, u8, u16, u32, u64, u128,
    isize, i8, i16, i32, i64, i128,
    f32, f64, char, str
}

#[cfg(feature = "alloc")]
impl_raw_index! {
    alloc::string::String,
}
