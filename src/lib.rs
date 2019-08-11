#![deny(missing_docs)]
//! A simple key/value store.

#[macro_use]
extern crate failure_derive;
extern crate serde_json;

pub use client::KvsClient;
pub use engines::{KvStore, KvsEngine};
pub use error::{KvsError, Result};
pub use server::KvsServer;

mod client;
mod engines;
mod error;
mod protocol;
mod server;
