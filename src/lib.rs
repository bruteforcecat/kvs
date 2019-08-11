#![deny(missing_docs)]
//! A simple key/value store.

#[macro_use]
extern crate failure_derive;

extern crate serde_json;

pub use engines::{KvStore, KvsEngine};
pub use error::{KvStoreError, Result};
pub use server::KvsServer;

mod engines;
mod error;
mod protocol;
mod server;
