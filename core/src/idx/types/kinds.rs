/*
    appellation: kinds <module>
    authors: @FL03
*/
/// This trait is used to define various _kinds_ of indices that are used to compose graphical
/// structures.
pub trait GraphIndex
where
    Self: Clone
        + Copy
        + Eq
        + Ord
        + PartialEq
        + PartialOrd
        + Send
        + Sync
        + core::fmt::Debug
        + core::fmt::Display
        + core::hash::Hash,
{
    private!();
}
/// [`IndexKind`] is an enumeration that defines the kinds of indices that can be used in a
/// hypergraph. It is used to distinguish between edge and vertex indices, allowing for
/// specialized handling of each type within the hypergraph's indexing system.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    PartialEq,
    Ord,
    PartialOrd,
    scsys::VariantConstructors,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumIter,
    strum::EnumString,
    strum::VariantArray,
    strum::VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
pub enum IndexKind {
    #[cfg_attr(feature = "serde", serde(alias = "facet", alias = "surface"))]
    Edge,
    #[default]
    #[cfg_attr(feature = "serde", serde(alias = "node"))]
    Vertex,
}
macro_rules! impl_type_kind {
    (@impl $(#[doc $($doc:tt)*])? $vis:vis $i:ident $kind:ident) => {
        // create the implementation for the kind
        impl_type_kind!(@branch $(#[doc $($doc)*])? $vis $i $kind);
        // implement the necessary traits for the kind
        impl_type_kind!(@kind $kind);
        // stringify the kind and implement Display
        impl ::core::fmt::Display for $kind {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                // stringify the ident of the kind
                let tag = stringify!($kind);
                // write the tag in lowercase
                write!(f, "{}", tag.to_lowercase())
            }
        }
    };
    (@branch $(#[doc $($doc:tt)*])? $vis:vis enum $kind:ident) => {
        $(#[doc $($doc)*])?
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(
            feature = "serde",
            derive(serde_derive::Deserialize, serde_derive::Serialize),
        )]
        #[repr(transparent)]
        $vis enum $kind {};
    };
    (@branch $(#[doc $($doc:tt)*])? $vis:vis struct $kind:ident) => {
        $(#[doc $($doc)*])?
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(
            feature = "serde",
            derive(serde_derive::Deserialize, serde_derive::Serialize),
        )]
        #[repr(transparent)]
        $vis struct $kind;
    };
    (@kind $kind:ident) => {
        unsafe impl ::core::marker::Send for $kind {}

        unsafe impl ::core::marker::Sync for $kind {}

        impl GraphIndex for $kind {
            seal!();
        }
    };
    ($($(#[doc $($doc:tt)*])? $vis:vis $i:ident $kind:ident);* $(;)?) => {
        $(
            impl_type_kind!(@impl $(#[doc $($doc)*])? $vis $i $kind);
        )*
    };
}

impl_type_kind! {
    #[doc = "A kind of index for edges in a graph"]
    pub struct EdgeIndex;
    #[doc = "A kind of index for vertices in a graph"]
    pub struct VertexIndex;
}

impl_type_kind! {
    @kind IndexKind
}
