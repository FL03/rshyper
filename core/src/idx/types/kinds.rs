/*
    appellation: kinds <module>
    authors: @FL03
*/
//! this module defines the [`GraphIndex`] trait and the [`IndexKind`] enumeration, which are
//! used to represent various kinds of indices in a hypergraph. Additionally, the module
//! provides two specific implemenations of the `GraphIndex` trait: [`EdgeIndex`] and
//! [`VertexIndex`], each of which are used to represent edge and vertex indices, respectively.
//! The `IndexKind` enumeration is used primarily:
//!
//! - during the initialization process
//! - enables dynamic dispatching of indices
//! - allows for maps and other data structures to be keyed by index kind
use crate::idx::{IndexBase, RawIndex};

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
/// the [`Grid`] enumerate the possible variations of indicies that can be used within the
/// scope of a hypergraph. This entity is useful for enabling "compoite" collections of
/// indicies for iteration, traversal, etc.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    Ord,
    PartialOrd,
    strum::EnumCount,
    strum::EnumDiscriminants,
    strum::EnumIs,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case"),
    strum_discriminants(derive(serde::Deserialize, serde::Serialize))
)]
#[strum_discriminants(
    name(IndexKind),
    derive(
        Hash,
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
    ),
    strum(serialize_all = "snake_case")
)]
pub enum Grid<Idx>
where
    Idx: RawIndex,
{
    #[cfg_attr(
        feature = "serde",
        strum_discriminants(serde(alias = "hyperedge", alias = "facet", alias = "surface"))
    )]
    Edge(IndexBase<Idx, EdgeIndex>),
    #[cfg_attr(
        feature = "serde",
        strum_discriminants(serde(
            alias = "hypernode",
            alias = "node",
            alias = "point",
            alias = "vertex"
        ))
    )]
    Vertex(IndexBase<Idx, VertexIndex>),
    Raw(Idx),
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
