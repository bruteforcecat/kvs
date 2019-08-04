#![deny(missing_docs)]

use crate::error::Result;
// use crate::Command;
use crate::KvStoreError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::io::Read;
use std::io::{Seek, SeekFrom, Write};
use std::path;
use std::path::PathBuf;

#[derive(Debug)]
struct LogPointer {
    offset: u64,
    length: u64,
}

type Index = HashMap<String, LogPointer>;

/// The `KvStore` stores string key/value pairs.
/// ```
pub struct KvStore {
    index: Index,
    log_file_path: PathBuf,
}

impl KvStore {
    /// Create a `KvStore` from a file path
    pub fn open(path: &path::Path) -> Result<KvStore> {
        let mut path_buf = PathBuf::from(path);
        path_buf.push("kvs_log");
        path_buf.set_extension("wal"); // stand for write ahead log

        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path_buf)
            .expect("failed to create file using path_buf");

        let index = KvStore::build_index(&path_buf)?;

        return Ok(KvStore {
            index,
            log_file_path: path_buf,
        });
    }

    // private function to build an initial index
    fn build_index(path_buf: &PathBuf) -> Result<Index> {
        let mut index = HashMap::new();
        let log_file = KvStore::get_log_file(path_buf)?;
        let mut log_file_buf_reader = BufReader::new(log_file);

        loop {
            let log_file_buf_reader_ref = log_file_buf_reader.by_ref();
            let offset = log_file_buf_reader_ref.seek(SeekFrom::Current(0))?;
            let result = bincode::deserialize_from(log_file_buf_reader_ref);
            let cmd_length = log_file_buf_reader.by_ref().seek(SeekFrom::Current(0))? - offset;
            match result {
                Ok(cmd) => match cmd {
                    Command::Set { key, value: _ } => {
                        index.insert(
                            key,
                            LogPointer {
                                offset,
                                length: cmd_length,
                            },
                        );
                    }
                    Command::Remove { key } => {
                        index.remove(&key);
                    }
                },
                Err(_) => break,
            }
        }

        Ok(index)
    }

    /// get log file buf
    fn get_log_file(path_buf: &PathBuf) -> Result<File> {
        let file = OpenOptions::new().read(true).append(true).open(path_buf)?;
        Ok(file)
    }

    /// Get value by key
    pub fn get(&self, key: String) -> Result<Option<String>> {
        match self.index.get(&key) {
            None => Ok(None),
            Some(LogPointer { offset, length: _ }) => {
                let mut file = self.log_file()?;
                file.seek(SeekFrom::Start(*offset))?;
                if let Command::Set { key: _, value } = bincode::deserialize_from(file)? {
                    return Ok(Some(value));
                }
                Err(KvStoreError::UnknownError)
            }
        }
    }

    /// Set Value for the key and persist it in to log file
    /// If there is value associated with the key, its value wil be overrided
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let set_cmd = Command::Set {
            key: key.clone(),
            value,
        };
        let (offset, length) = self.write_cmd_to_log(set_cmd)?;

        let lp = LogPointer { offset, length };
        self.index.insert(key, lp);

        Ok(())
    }

    /// Remove a given key.
    pub fn remove(&mut self, key: String) -> Result<()> {
        let remove_cmd = Command::Remove { key: key.clone() };
        self.write_cmd_to_log(remove_cmd)?;

        match self.index.remove(&key) {
            None => return Err(KvStoreError::KeyNotFoundError),
            Some(_value) => return Ok(()),
        }
    }

    /// write command to log file
    fn write_cmd_to_log(&self, cmd: Command) -> Result<((u64, u64))> {
        let serialized = bincode::serialize(&cmd)?;
        let mut file = self.log_file()?;
        let offset = file.seek(SeekFrom::End(0))?;
        file.write(&serialized)?;
        let cur_offset = file.seek(SeekFrom::End(0))?;

        Ok((offset, cur_offset - offset))
    }

    /// get log file buf
    fn log_file(&self) -> Result<File> {
        KvStore::get_log_file(&self.log_file_path)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}
