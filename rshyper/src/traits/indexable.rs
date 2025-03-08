/*
    Appellation: indexable <module>
    Contrib: @FL03
*/
use crate::Index;

pub trait IntoIndex<Idx> {
    fn into_index(self) -> Index<Idx>;
}

/// A trait denoting objects that may be identified by an index.
pub trait Indexable<Idx> {
    fn index(&self) -> &Index<Idx>;
}

/*
 ************* Implementations *************
 */

impl<Idx, T> IntoIndex<Idx> for T
where
    T: Into<Index<Idx>>,
{
    fn into_index(self) -> Index<Idx> {
        self.into()
    }
}

impl<T, Idx> Indexable<Idx> for T
where
    T: core::borrow::Borrow<Index<Idx>>,
{
    fn index(&self) -> &Index<Idx> {
        self.borrow()
    }
}
