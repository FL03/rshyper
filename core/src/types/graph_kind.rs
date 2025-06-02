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

macro_rules! impl_kind {
    ($(
        $vis:vis $itype:ident $kind:ident
    );* $(;)?) => {
        $(
            impl_kind!(@impl $vis $itype $kind);
        )*
    };
    (@impl $vis:vis struct $kind:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Deserialize, serde::Serialize)
        )]
        pub enum $kind {}

        impl_kind!(@kind $kind);
    };
    (@impl $vis:vis enum $kind:ident) => {
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Deserialize, serde::Serialize)
        )]
        pub struct $kind;

        impl_kind!(@kind $kind);
    };
    (@kind $kind:ident) => {
        unsafe impl Send for $kind {}

        unsafe impl Sync for $kind {}

        impl ::core::fmt::Display for $kind {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}", ::core::any::type_name::<Self>())
            }
        }

        impl GraphKind for $kind {
            seal!();
        }
    };
}

impl_kind! {
    pub enum Directed;
    pub enum Undirected;
}

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

unsafe impl Send for GraphKinds {}

unsafe impl Sync for GraphKinds {}

impl GraphKind for GraphKinds {
    seal!();
}
