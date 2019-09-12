pub mod shared;
mod snapshot;
pub mod tx_pool;
mod tx_pool_ext;

pub use crate::snapshot::{Snapshot, SnapshotMgr};

pub(crate) const LOG_TARGET_TX_POOL: &str = "ckb-tx-pool";
pub(crate) const LOG_TARGET_CHAIN: &str = "ckb-chain";
