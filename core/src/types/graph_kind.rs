/*
    appellation: graph_kind <module>
    authors: @FL03
*/
/// [GraphKind] is a marker trait for graph types.
pub trait GraphKind {
    private!();
}

macro_rules! impl_kind {
    ($($kind:ident),* $(,)?) => {
        $(
            impl_kind!(@impl $kind);
        )*
    };
    (@impl $kind:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
        #[cfg_attr(
            feature = "serde",
            derive(serde_derive::Deserialize, serde_derive::Serialize)
        )]
        pub enum $kind {}

        impl GraphKind for $kind {
            seal!();
        }
    }
}

impl_kind! {
    Directed,
    Undirected,
}
