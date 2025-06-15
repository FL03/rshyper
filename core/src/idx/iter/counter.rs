/*
    appellation: counter <module>
    authors: @FL03
*/


pub struct IndexCounter<T, K> {
    pub(crate) curr: IndexBase<T, K>,
}