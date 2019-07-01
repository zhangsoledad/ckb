use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Hash, Debug)]
pub struct StoreConfig {
    pub header_cache_size: usize,
    pub cell_output_cache_size: usize,
}

impl Default for StoreConfig {
    fn default() -> Self {
        Self {
            header_cache_size: 4096,
            cell_output_cache_size: 128,
        }
    }
}
