/*
    appellation: kinds <module>
    authors: @FL03
*/
/// This trait is used to define various _kinds_ of indices that are used to compose graphical
/// structures.
pub trait GraphIndex:
    Copy + Eq + Ord + core::fmt::Debug + core::fmt::Display + core::hash::Hash
{
    private!();
}

macro_rules! impl_type_kind {
    ($($(#[doc $($doc:tt)*])? $vis:vis $i:ident $kind:ident);* $(;)?) => {
        $(
            impl_type_kind!(@impl $(#[doc $($doc)*])? $vis $i $kind);
        )*
    };
    (@impl $(#[doc $($doc:tt)*])? $vis:vis enum $kind:ident) => {
        $(#[doc $($doc)*])?
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(
            feature = "serde",
            derive(serde_derive::Deserialize, serde_derive::Serialize),
        )]
        #[repr(transparent)]
        $vis enum $kind {};

        impl_type_kind!(@impls $kind);
    };
    (@impl $(#[doc $($doc:tt)*])? $vis:vis struct $kind:ident) => {
        $(#[doc $($doc)*])?
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(
            feature = "serde",
            derive(serde_derive::Deserialize, serde_derive::Serialize),
        )]
        #[repr(transparent)]
        $vis struct $kind;

        impl_type_kind!(@impls $kind);
    };
    (@impls $kind:ident) => {
        impl $crate::index::GraphIndex for $kind {
            seal!();
        }

        impl ::core::fmt::Display for $kind {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                // stringify the ident of the kind
                let tag = stringify!($kind);
                // write the tag in lowercase
                write!(f, "{}", tag.to_lowercase())
            }
        }
    };
}

impl_type_kind! {
    #[doc = "A kind of index for edges in a graph"]
    pub struct EdgeIndex;
    #[doc = "A kind of index for vertices in a graph"]
    pub struct VertexIndex;
}
