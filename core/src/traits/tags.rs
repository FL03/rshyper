/*
    appellation: tags <module>
    authors: @FL03
*/

pub trait RawKind {
    private!();
}
/// This trait is used to define various _kinds_ of types
pub trait Kind: RawKind
where
    Self: Copy + Eq + Ord + core::fmt::Debug + core::fmt::Display + core::hash::Hash,
{
}
