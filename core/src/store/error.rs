/*
    appellation: error <module>
    authors: @FL03
*/

#[derive(Clone, Copy, Debug, strum::EnumIs, thiserror::Error)]
pub enum StoreError {
    #[error(transparent)]
    KeyValueError(#[from] KeyValueError),
}

#[derive(Clone, Copy, Debug, strum::EnumIs, thiserror::Error)]
pub enum KeyValueError {
    /// An error indicating that the key was not found in the key-value store.
    #[error("Key not found in the key-value store")]
    KeyNotFound,
    /// An error indicating that the value is not present in the key-value store.
    #[error("Value not present in the key-value store")]
    ValueNotPresent,
}
