#![deny(missing_docs)]
//! A simple key/value store.

#[macro_use]
extern crate failure_derive;

extern crate serde_json;

pub use error::{KvStoreError, Result};
pub use kv::KvStore;

mod error;
mod kv;
