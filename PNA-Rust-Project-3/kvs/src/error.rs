use failure::Fail;
use serde_json;
use std::{io, string::FromUtf8Error};
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
    #[fail(display = "sled error: {}", _0)]
    Sled(#[cause] sled::Error),
    /// Key or value is invalid UTF-8 sequence
    #[fail(display = "UTF-8 error: {}", _0)]
    Utf8(#[cause] FromUtf8Error),
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

impl From<FromUtf8Error> for KvStoreError {
    fn from(err: FromUtf8Error) -> KvStoreError {
        KvStoreError::Utf8(err)
    }
}

impl From<sled::Error> for KvStoreError {
    fn from(err: sled::Error) -> KvStoreError {
        KvStoreError::Sled(err)
    }
}

pub type Result<T> = std::result::Result<T, KvStoreError>;
