#![deny(missing_docs)]
//! A simple key/value store.

#[macro_use]
extern crate failure_derive;

pub use error::{KvStoreError, Result};
pub use kv::KvStore;

mod error;
mod kv;
