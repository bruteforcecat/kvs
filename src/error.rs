use std::result;

#[derive(Fail, Debug)]
/// Custom Error enum for KVStore
pub enum KvStoreError {
    /// Used for errors that are miscellaneous and/or cannot be explained.
    #[fail(display = "An unknown error has occurred")]
    UnknownError,

    /// Error for a key not found in the key value store.
    #[fail(display = "Key not found")]
    KeyNotFoundError,
}

/// Type alias Result
pub type Result<T> = result::Result<T, KvStoreError>;
