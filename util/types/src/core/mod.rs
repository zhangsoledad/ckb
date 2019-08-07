pub mod cell;
pub mod service;

mod advanced_builders;
mod blockchain;
mod extras;
mod reward;
mod views;
pub use advanced_builders::{BlockBuilder, HeaderBuilder, TransactionBuilder};
pub use blockchain::*;
pub use extras::*;
pub use reward::*;
pub use views::{
    BlockBodyView, BlockView, HeaderView, TransactionView, UncleBlockVecView, UncleBlockView,
};

pub use ckb_occupied_capacity::{capacity_bytes, Capacity};
pub type PublicKey = numext_fixed_hash::H512;
pub type BlockNumber = u64;
pub type EpochNumber = u64;
pub type Cycle = u64;
pub type Version = u32;

pub const TX_VERSION: Version = 0;
pub const HEADER_VERSION: Version = 0;
