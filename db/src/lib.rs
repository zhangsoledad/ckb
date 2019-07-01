//! # The DB Library
//!
//! This Library contains the `KeyValueDB` traits
//! which provides key-value store interface

use failure::Fail;
use std::result;

pub mod config;
pub mod rocksdb;
mod transaction;

pub use crate::config::DBConfig;
pub use crate::rocksdb::{DBPinnableSlice, DBVector, RocksDB};
pub use crate::transaction::{RocksDBTransaction, RocksDBTransactionSnapshot};

pub type Col = &'static str;
pub type Result<T> = result::Result<T, Error>;
// pub type KeyValueIteratorItem = (Box<[u8]>, Box<[u8]>);

#[derive(Clone, Debug, PartialEq, Eq, Fail)]
pub enum Error {
    #[fail(display = "DBError {}", _0)]
    DBError(String),
}
