/*
    Appellation: indexable <module>
    Contrib: @FL03
*/

pub trait Indexable {
    type Idx;

    fn index(&self) -> &crate::Index<Self::Idx>;
}
