mod error;
mod kv;
pub use error::{KvStoreError, Result};
pub use kv::KvStore;