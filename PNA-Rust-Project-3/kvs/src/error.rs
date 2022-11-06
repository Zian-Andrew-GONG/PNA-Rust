use failure::Fail;
use serde_json;
use std::io;
#[derive(Debug, Fail)]
pub enum KvStoreError {
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),
    #[fail(display = "Key not found")]
    KeyNotFound,
    #[fail(display = "Unexpected command type")]
    UnexpectedCommandType,
}

impl From<serde_json::Error> for KvStoreError {
    fn from(err: serde_json::Error) -> Self {
        KvStoreError::Serde(err)
    }
}

impl From<io::Error> for KvStoreError {
    fn from(err: io::Error) -> Self {
        KvStoreError::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, KvStoreError>;
