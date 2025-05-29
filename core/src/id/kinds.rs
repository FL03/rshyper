/*
    appellation: kinds <module>
    authors: @FL03
*/

pub trait IndexKind: Eq + core::hash::Hash {
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
    }
}

impl_index_kind! {
    EdgeIndex,
    VertexIndex,
}