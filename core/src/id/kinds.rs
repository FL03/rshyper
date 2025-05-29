/*
    appellation: kinds <module>
    authors: @FL03
*/
/// This trait is used to define various _kinds_ of indices that are used to compose graphical
/// structures.
pub trait IndexKind:
    Copy + Eq + Ord + core::fmt::Debug + core::fmt::Display + core::hash::Hash
{
    private!();
}

macro_rules! impl_index_kind {
    ($($kind:ident),* $(,)?) => {
        $(
            impl_index_kind!(@impl $kind);
        )*
    };
    (@impl $kind:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(
            feature = "serde",
            derive(serde_derive::Deserialize, serde_derive::Serialize)
        )]
        pub enum $kind {}

        impl IndexKind for $kind {
            seal!();
        }

        impl ::core::fmt::Display for $kind {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                // stringify the ident of the kind
                let tag = stringify!($kind);
                // write the tag in lowercase
                write!(f, "{}", tag.to_lowercase())
            }
        }
    }
}

impl_index_kind! {
    EdgeIndex,
    VertexIndex,
}
