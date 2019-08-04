use std::result;

#[derive(Fail, Debug)]
/// Custom Error enum for KVStore
pub enum KvStoreError {
    /// Unknown Error
    #[fail(display = "An unknown error has occurred")]
    UnknownError,

    /// Error for a key that can't be found in KV Store
    #[fail(display = "Key not found")]
    KeyNotFoundError,

    /// IO Error
    #[fail(display = "IOError {}", _0)]
    IOError(#[cause] std::io::Error),

    /// Serde errors.
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),
}

impl From<std::io::Error> for KvStoreError {
    fn from(err: std::io::Error) -> KvStoreError {
        KvStoreError::IOError(err)
    }
}

impl From<serde_json::Error> for KvStoreError {
    fn from(err: serde_json::Error) -> KvStoreError {
        KvStoreError::Serde(err)
    }
}

/// Type alias Result
pub type Result<T> = result::Result<T, KvStoreError>;
