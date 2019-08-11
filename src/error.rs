use std::result;

#[derive(Fail, Debug)]
/// Custom Error enum for KVStore
pub enum KvsError {
    /// Unknown Error
    #[fail(display = "An unknown error has occurred")]
    UnknownError,

    /// Error for a key that can't be found in KV Store
    #[fail(display = "Key not found")]
    KeyNotFoundError,

    // /// Error for a key that can't be found in KV Store
    // #[fail(display = "Key not found")]
    // KeyNotFoundError,
    /// IO Error
    #[fail(display = "IOError {}", _0)]
    IOError(#[cause] std::io::Error),

    /// Serde errors.
    #[fail(display = "serde_json {}", _0)]
    Serde(#[cause] serde_json::Error),

    /// Bincode Errors.
    #[fail(display = "Bincode error: {}", _0)]
    BincodeError(#[fail(cause)] bincode::Error),

    /// Generic String Error
    #[fail(display = "{}", _0)]
    StringError(String),
}

impl From<std::io::Error> for KvsError {
    fn from(err: std::io::Error) -> KvsError {
        KvsError::IOError(err)
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(err: serde_json::Error) -> KvsError {
        KvsError::Serde(err)
    }
}

impl From<bincode::Error> for KvsError {
    fn from(err: bincode::Error) -> KvsError {
        KvsError::BincodeError(err)
    }
}

/// Type alias Result
pub type Result<T> = result::Result<T, KvsError>;
