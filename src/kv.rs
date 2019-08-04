#![deny(missing_docs)]

use crate::error::Result;
// use crate::Command;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path;
use std::path::{PathBuf};
use crate::KvStoreError;

/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are stored in a `HashMap` in memory and not persisted to disk.
///
/// Example:
///
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
/// ```
pub struct KvStore {
    map: HashMap<String, String>,
    log_file_path: PathBuf,
}

impl KvStore {
    /// Create a `KvStore` from a file path
    pub fn open(path: &path::Path) -> Result<KvStore> {
        let mut path_buf = PathBuf::from(path);
        path_buf.push("kvs_log");
        path_buf.set_extension("txt");

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path_buf)
            .expect("failed to create file using path_buf");

        let mut map = HashMap::new();

        for line in BufReader::new(file).lines() {
            let cmd: Command = serde_json::from_str(&line?)?;

            if let Command::Set { key, value } = &cmd {
                map.insert(key.to_string(), value.to_string());
            };

            if let Command::Remove { key } = &cmd {
                map.remove(&key.to_string());
            };
        }

        return Ok(KvStore {
            map,
            log_file_path: path_buf,
        });
    }

    /// Get value by key
    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.map.get(&key).cloned())
        // match self.map.get(&key).cloned() {
        //     None => Ok(None),
        //     v => Ok(v)
        // }
    }

    /// Set Value for the key and persist it in to log file
    /// If there is value associated with the key, its value wil be overrided
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let set_cmd = Command::Set { key, value };
        self.write_cmd_to_log(&set_cmd)?;

        // to avoid error of "value used here after move"
        if let Command::Set { key, value } = set_cmd {
            self.map.insert(key, value);
        }
        Ok(())
    }

    /// Remove a given key.
    pub fn remove(&mut self, key: String) -> Result<()> {
        let remove_cmd = Command::Remove { key };
        self.write_cmd_to_log(&remove_cmd)?;

        // to avoid error of "value used here after move"
        if let Command::Remove { key } = remove_cmd {
            match self.map.remove(&key) {
                None =>
                    return Err(KvStoreError::KeyNotFoundError),
                Some(_value) =>
                    return Ok(())
            }
        }

        Ok(())
    }

    /// write command to log file
    fn write_cmd_to_log(&self, cmd: &Command) -> Result<()> {
        let mut cmd_serialized = serde_json::to_string(&cmd)?;
        cmd_serialized.push('\n');

        let mut stream = BufWriter::new(self.log_file()?);
        stream.write(cmd_serialized.as_bytes())?;


        Ok(())
    }

    /// get log file buf
    fn log_file(&self) -> Result<File> {
        let file_handler = OpenOptions::new().append(true).open(&self.log_file_path)?;
        Ok(file_handler)
    }
}

#[derive(Serialize, Deserialize)]
pub enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}
