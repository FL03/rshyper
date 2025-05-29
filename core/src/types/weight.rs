/*
    appellation: weight <module>
    authors: @FL03
*/

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent, rename_all = "lowercase")
)]
#[repr(transparent)]
pub struct Weight<T>(pub T);

impl<T> Weight<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }
}
