use serde_derive::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, Debug)]
pub struct StoreConfig {
    pub header_cache_size: usize,
    pub cell_data_cache_size: usize,
    pub block_proposals_cache_size: usize,
    pub block_tx_hashes_cache_size: usize,
    pub block_uncles_cache_size: usize,
    pub cellbase_cache_size: usize,
}
