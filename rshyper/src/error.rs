/*
    Appellation: error <module>
    Contrib: @FL03
*/

/// A type alias for a [Result] with the crate-specific error type [Error]
pub type Result<T = ()> = core::result::Result<T, Error>;

/// The error type for this crate
#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("Vertex {0} does not exist")]
    VertexDoesNotExist(String),
    #[error("Cannot create empty hyperedge")]
    EmptyHyperedge,
    #[error("Hyperedge {0} does not exist")]
    HyperedgeDoesNotExist(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}
