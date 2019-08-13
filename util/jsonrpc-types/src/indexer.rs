use crate::{BlockNumber, CellOutput, Unsigned};
use ckb_types::H256;
use serde_derive::{Deserialize, Serialize};

// This is used as return value of get_live_cells_by_lock_hash RPC
#[derive(Serialize, Deserialize)]
pub struct LiveCell {
    pub created_by: TransactionPoint,
    pub cell_output: CellOutput,
}

// This is used as return value of get_transactions_by_lock_hash RPC
#[derive(Serialize, Deserialize)]
pub struct CellTransaction {
    pub created_by: TransactionPoint,
    pub consumed_by: Option<TransactionPoint>,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionPoint {
    pub block_number: BlockNumber,
    pub tx_hash: H256,
    pub index: Unsigned,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LockHashIndexState {
    pub lock_hash: H256,
    pub block_number: BlockNumber,
    pub block_hash: H256,
}
