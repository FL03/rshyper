/*
    Appellation: indexable <module>
    Contrib: @FL03
*/
use crate::Index;

/// A trait denoting objects that may be identified by an index.
pub trait Indexable<Idx> {
    fn index(&self) -> &Index<Idx>;
}

/*
 ************* Implementations *************
 */

impl<T, Idx> Indexable<Idx> for T
where
    T: core::borrow::Borrow<Index<Idx>>,
{
    fn index(&self) -> &Index<Idx> {
        self.borrow()
    }
}
