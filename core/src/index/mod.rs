/*
    appellation: index <module>
    authors: @FL03
*/
//! the [`index`](crate::index) module is centered around the [`IndexBase`] implementation.
//! Additional type aliases ([`EdgeId`] and [`VertexId`]) are provided for convenience, as well
//! as traits that define the behaviour of indices in a hypergraph.
#[doc(inline)]
pub use self::{aliases::*, error::*, id::IndexBase, kinds::*, position::IndexCursor};

pub mod error;
pub mod id;
pub mod kinds;
pub mod position;

#[doc(hidden)]
mod impls {
    pub mod impl_ops;
    #[cfg(feature = "rand")]
    pub mod impl_rand;
    pub mod impl_repr;
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::aliases::*;
    #[doc(inline)]
    pub use super::error::*;
    #[doc(inline)]
    pub use super::id::*;
    #[doc(inline)]
    pub use super::position::*;
    #[doc(inline)]
    pub use super::{HashIndex, Indexed, NumIndex, RawIndex};
}

mod aliases {
    use super::{EdgeIndex, IndexBase, VertexIndex};
    /// a type alias for a [`usize`] used to define the default index type throughout the crate.
    pub type Udx = usize;
    /// a type alias for an [`Index`] whose _kind_ is [`EdgeIndex`]
    pub type EdgeId<T = Udx> = IndexBase<T, EdgeIndex>;
    /// a type alias for an [`Index`] whose _kind_ is [`VertexIndex`]
    pub type VertexId<T = Udx> = IndexBase<T, VertexIndex>;
}
///[`Indexed`] describes a common interface for all types which are aware of some associated
/// index. The trait is generic over a type `T` which implements the [`RawIndex`] trait,
/// allowing for flexibility in the type of index used while ensuring that the index type is
/// compatible with the hypergraph's indexing system.
pub trait Indexed<T: RawIndex> {
    type Idx<I>;

    /// Returns the index of the node.
    fn index(&self) -> &Self::Idx<T>;
}
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
/// The [`Index`] trait extends the [`RawIndex`] trait to include additional operations and
/// behaviours commonly expected from indices in a hypergraph.
///
/// **note:** the trait is automatically implemented for all types that implement [`RawIndex`]
/// alongside traits including: [Clone], [Default], [PartialEq], and [PartialOrd]
pub trait Index: RawIndex
where
    Self: Clone + Default + PartialEq + PartialOrd,
{
}
/// The [`HashIndex`] trait extends the [`Index`] trait to include additional operations and
/// behaviours commonly expected from indices in a hypergraph.
///
/// **note:** the trait is automatically implemented for all types that implement [`Idx`]
///  alongside traits including: [Eq] and [Hash](core::hash::Hash)
/// implementations.
pub trait HashIndex: Index
where
    Self: Eq + core::hash::Hash,
{
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

impl<T> Index for T where T: 'static + RawIndex + Clone + Default + PartialEq + PartialOrd {}

impl<T> HashIndex for T where T: Index + Eq + core::hash::Hash {}

impl<T> NumIndex for T where
    T: HashIndex
        + Copy
        + Default
        + Ord
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

impl<T: RawIndex> Indexed<T> for VertexId<T> {
    type Idx<I> = VertexId<I>;

    fn index(&self) -> &Self::Idx<T> {
        self
    }
}

impl<T, Idx> Indexed<Idx> for crate::HyperNode<T, Idx>
where
    Idx: RawIndex,
{
    type Idx<I> = VertexId<I>;

    fn index(&self) -> &Self::Idx<Idx> {
        &self.index
    }
}

/*
 ************* [impl] RawIndex *************
*/
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
