/*
    appellation: graph_kind <module>
    authors: @FL03
*/
/// [`GraphType`] is a marker trait for graph types.
///
/// **note:** This trait is sealed and cannot be implemented outside of this crate.
pub trait GraphType: 'static + Send + Sync + core::fmt::Debug + core::fmt::Display {
    private!();
}

/// [`Mode`] enumerates the possible graph variants enabling dynamic dispatch features.
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
pub enum Mode {
    Directed = 0,
    #[default]
    Undirected = 1,
}

impl Mode {
    /// returns the [`Mode`] corresponding to the given [`GraphType`].
    pub fn from_type<T: GraphType>() -> Self {
        use core::any::TypeId;
        if TypeId::of::<T>() == TypeId::of::<Directed>() {
            Mode::Directed
        } else if TypeId::of::<T>() == TypeId::of::<Undirected>() {
            Mode::Undirected
        } else {
            panic!("Unknown graph type");
        }
    }
    /// returns the [`GraphType`] corresponding to this mode.
    pub fn as_type(&self) -> &'static dyn GraphType {
        match self {
            Mode::Directed => &Directed,
            Mode::Undirected => &Undirected,
        }
    }
}
/*
 ************* Implementations *************
*/
macro_rules! impl_kind {
    (@impl $(#[doc$($doc:tt)*])? $vis:vis enum $kind:ident) => {
        $(#[doc $($doc)*])?
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Deserialize, serde::Serialize)
        )]
        pub enum $kind {}
    };
    (@impl $(#[doc$($doc:tt)*])? $vis:vis struct $kind:ident) => {
        $(#[doc $($doc)*])?
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Deserialize, serde::Serialize)
        )]
        pub struct $kind;

        impl $kind {
            pub const fn new() -> Self {
                Self
            }
        }
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

        impl $crate::types::graph_kind::GraphType for $kind {
            seal!();
        }
    };
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
}

impl_kind! {
    #[doc = "A marker type representing a _directed_ graph type"]
    pub struct Directed;
    #[doc = "A marker type representing an _undirected_ graph type"]
    pub struct Undirected;
}

impl_kind!(@impls Mode);
