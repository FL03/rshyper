/*
    Appellation: indexable <module>
    Contrib: @FL03
*/
use crate::{Index, IndexKind};

pub trait IntoIndex<Idx> {
    type Kind: IndexKind;

    fn into_index(self) -> Index<Idx, Self::Kind>;
}

/// A trait denoting objects that may be identified by an index.
pub trait Indexable<Idx> {
    type Kind: IndexKind;

    fn index(&self) -> &Index<Idx, Self::Kind>;
}


