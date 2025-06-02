/*
    appellation: graph_kind <module>
    authors: @FL03
*/
/// [GraphKind] is a marker trait for graph types.
///
/// **note:** This trait is sealed and cannot be implemented outside of this crate.
pub trait GraphKind: 'static + Send + Sync + core::fmt::Debug + core::fmt::Display {
    private!();
}

/// [`GraphKinds`] enumerates the possible graph variants enabling dynamic dispatch features.
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
pub enum GraphKinds {
    Directed = 1,
    #[default]
    Undirected = 0,
}
/*
    ************* Implementations *************
*/
macro_rules! impl_kind {
    ($(
        $(#[doc$($doc:tt)*])?
        $vis:vis $itype:ident $kind:ident
    );* $(;)?) => {
        $(
            impl_kind!(@impl $(#[doc $($doc)*])? $vis $itype $kind);
            impl_kind!(@display $kind);
            impl_kind!(@impls $kind);
        )*
    };
    (@impl $(#[doc$($doc:tt)*])? $vis:vis struct $kind:ident) => {
        $(#[doc $($doc)*])?
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Deserialize, serde::Serialize)
        )]
        pub enum $kind {}
    };
    (@impl $(#[doc$($doc:tt)*])? $vis:vis enum $kind:ident) => {
        $(#[doc $($doc)*])?
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Deserialize, serde::Serialize)
        )]
        pub struct $kind;
    };
    (@display $kind:ident) => {
        impl ::core::fmt::Display for $kind {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}", ::core::any::type_name::<Self>())
            }
        }
    };
    (@impls $kind:ident) => {
        unsafe impl Send for $kind {}

        unsafe impl Sync for $kind {}

        impl GraphKind for $kind {
            seal!();
        }
    };
}

impl_kind! {
    #[doc = "Directed graph type"]
    pub enum Directed;
    #[doc = "Undirected graph type"]
    pub enum Undirected;
}

impl_kind!(@impls GraphKinds);
