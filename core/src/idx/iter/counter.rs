/*
    appellation: counter <module>
    authors: @FL03
*/
use crate::idx::{IndexBase, NumIndex};

pub struct IndexCounter<T, K> {
    pub(crate) curr: IndexBase<T, K>,
}
