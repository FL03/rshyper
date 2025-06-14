/*
    appellation: error <module>
    authors: @FL03
*/
//! this module defines the various errors encountered by storage containers.

/// a type alias for a [`Result`] equipped to handle the [`StoreError`] type
#[allow(dead_code)]
pub(crate) type StoreResult<T> = core::result::Result<T, StoreError>;
/// the [`StoreError`] type enumerates the possible errors that can occur in the store module.
#[derive(Clone, Copy, Debug, strum::EnumIs, thiserror::Error)]
pub enum StoreError {
    #[error(transparent)]
    EntryError(#[from] EntryError),
    /// An error indicating that the key was not found in the key-value store.
    #[error("Key not found in the key-value store")]
    KeyNotFound,
}
/// the [`EntryError`] type enumerates the possible errors that can occur when accessing
/// entries in the key-value store.
#[derive(Clone, Copy, Debug, strum::EnumIs, thiserror::Error)]
pub enum EntryError {
    /// An error indicating that the value is not present in the key-value store.
    #[error("The entry is vacant")]
    VacantEntry,
}
