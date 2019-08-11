use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Get { key: String },
    Set { key: String, val: String },
    Remove { key: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GetResponse {
    Ok(Option<String>),
    Err(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SetResponse {
    Ok(()),
    Err(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RemoveResponse {
    Ok(()),
    Err(String),
}
