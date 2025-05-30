/*
    appellation: id <module>
    authors: @FL03
*/
//! this module defines the [`Index`] type and its associated types for representing indices in
//! a hypergraph.
#[doc(inline)]
pub use self::{
    id::Index,
    kinds::{EdgeIndex, GraphIndex, VertexIndex},
    position::Position,
};

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
    pub use super::id::*;
    #[doc(inline)]
    pub use super::kinds::*;
    #[doc(inline)]
    pub use super::position::*;
    #[doc(inline)]
    pub use super::{EdgeId, Indexed, Ix, NumIndex, RawIndex, VertexId};
}

/// a type alias for a [`usize`] used to define the default index type throughout the crate.
pub type Ix = usize;

/// a type alias for an [`Index`] whose _kind_ is [`EdgeIndex`]
pub type EdgeId<T = Ix> = Index<T, EdgeIndex>;
/// a type alias for an [`Index`] whose _kind_ is [`VertexIndex`]
pub type VertexId<T = Ix> = Index<T, VertexIndex>;

/// This trait is used to denote a type that is aware of its own index.
pub trait Indexed<T: RawIndex> {
    type Idx<I: RawIndex>;

    /// Returns the index of the node.
    fn index(&self) -> &Self::Idx<T>;
}
/// a simple trait for denoting types compatible with to be used as indices in a hypergraph.
/// **note:** the trait is sealed to prevent external implementations.
pub trait RawIndex: Clone + PartialEq + PartialOrd {
    private!();
}
/// The [`NumIndex`] trait extends the [`RawIndex`] trait to include additional operations and
/// behaviours expected from numerical indices in a hypergraph.
pub trait NumIndex: RawIndex
where
    Self: Copy
        + Default
        + Eq
        + PartialOrd
        + core::fmt::Debug
        + core::fmt::Display
        + core::hash::Hash
        + core::ops::Add<Output = Self>
        + core::ops::Div<Output = Self>
        + core::ops::Mul<Output = Self>
        + core::ops::Sub<Output = Self>
        + num_traits::One
        + num_traits::Zero,
{
    private!();
}

/*
 ************* Implementations *************
*/

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
    f32, f64, char
}

#[cfg(feature = "alloc")]
impl_raw_index! {
    alloc::string::String,
}

impl<T> NumIndex for T
where
    T: RawIndex
        + Copy
        + Default
        + Eq
        + Ord
        + core::fmt::Debug
        + core::fmt::Display
        + core::hash::Hash
        + core::ops::Add<Output = Self>
        + core::ops::Div<Output = Self>
        + core::ops::Mul<Output = Self>
        + core::ops::Sub<Output = Self>
        + core::ops::AddAssign
        + core::ops::DivAssign
        + core::ops::MulAssign
        + core::ops::SubAssign
        + num_traits::One
        + num_traits::Zero,
{
    seal!();
}

impl<T: RawIndex> Indexed<T> for VertexId<T> {
    type Idx<I: RawIndex> = VertexId<I>;

    fn index(&self) -> &Self::Idx<T> {
        self
    }
}

impl<T, Idx> Indexed<Idx> for crate::HyperNode<T, Idx>
where
    Idx: RawIndex,
{
    type Idx<I: RawIndex> = VertexId<I>;

    fn index(&self) -> &Self::Idx<Idx> {
        &self.index
    }
}
