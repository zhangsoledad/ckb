use crate::cache::StoreCache;
use crate::config::StoreConfig;
use crate::store::ChainStore;
use crate::transaction::StoreTransaction;
use crate::StoreSnapshot;
use crate::COLUMN_CELL_SET;
use ckb_chain_spec::consensus::Consensus;
use ckb_db::{
    iter::{DBIterator, DBIteratorItem},
    Col, DBPinnableSlice, Direction, RocksDB,
};
use ckb_error::{Error, InternalErrorKind};
use ckb_types::{
    core::{BlockExt, TransactionMeta},
    packed,
    prelude::*,
    utilities::ChainRootMMR,
};
use std::sync::Arc;

pub struct ChainDB {
    db: RocksDB,
    cache: Arc<StoreCache>,
}

impl<'a> ChainStore<'a> for ChainDB {
    type Vector = DBPinnableSlice<'a>;

    fn cache(&'a self) -> Option<&'a StoreCache> {
        Some(&self.cache)
    }

    fn get(&'a self, col: Col, key: &[u8]) -> Option<Self::Vector> {
        self.db
            .get_pinned(col, key)
            .expect("db operation should be ok")
    }

    fn get_iter<'i>(
        &'i self,
        col: Col,
        from_key: &'i [u8],
        direction: Direction,
    ) -> Box<Iterator<Item = DBIteratorItem> + 'i> {
        self.db
            .iter(col, from_key, direction)
            .expect("db operation should be ok")
    }
}

impl ChainDB {
    pub fn new(db: RocksDB, config: Option<StoreConfig>) -> Self {
        let cache = config
            .map(StoreCache::from_config)
            .unwrap_or_else(StoreCache::default);
        ChainDB {
            db,
            cache: Arc::new(cache),
        }
    }

    pub fn traverse_cell_set<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(packed::Byte32, packed::TransactionMeta) -> Result<(), Error>,
    {
        self.db
            .traverse(COLUMN_CELL_SET, |hash_slice, tx_meta_bytes| {
                let tx_hash = packed::Byte32Reader::from_slice_should_be_ok(hash_slice).to_entity();
                let tx_meta = packed::TransactionMetaReader::from_slice_should_be_ok(tx_meta_bytes)
                    .to_entity();
                callback(tx_hash, tx_meta)
            })
    }

    pub fn begin_transaction(&self) -> StoreTransaction {
        StoreTransaction {
            inner: self.db.transaction(),
            cache: Arc::clone(&self.cache),
        }
    }

    pub fn get_snapshot(&self) -> StoreSnapshot {
        StoreSnapshot {
            inner: self.db.get_snapshot(),
            cache: Arc::clone(&self.cache),
        }
    }

    pub fn init(&self, consensus: &Consensus) -> Result<(), Error> {
        let genesis = consensus.genesis_block();
        let epoch = consensus.genesis_epoch_ext();
        let db_txn = self.begin_transaction();
        let genesis_hash = genesis.hash();
        let ext = BlockExt {
            received_at: genesis.timestamp(),
            total_difficulty: genesis.difficulty(),
            total_uncles_count: 0,
            verified: Some(true),
            txs_fees: vec![],
        };

        let block_number = genesis.number();
        let epoch_number = genesis.epoch();
        let block_hash = genesis.hash();

        for tx in genesis.transactions().iter() {
            let outputs_len = tx.outputs().len();
            let tx_meta = if tx.is_cellbase() {
                TransactionMeta::new_cellbase(
                    block_number,
                    epoch_number,
                    block_hash.clone(),
                    outputs_len,
                    false,
                )
            } else {
                TransactionMeta::new(
                    block_number,
                    epoch_number,
                    block_hash.clone(),
                    outputs_len,
                    false,
                )
            };
            db_txn.update_cell_set(&tx.hash(), &tx_meta.pack())?;
        }

        let last_block_hash_in_previous_epoch = epoch.last_block_hash_in_previous_epoch();

        // Init MMR
        let mut mmr = ChainRootMMR::new(0, &db_txn);
        mmr.push(genesis.header().into())
            .map_err(|e| InternalErrorKind::MMR.cause(e))?;
        mmr.commit().map_err(|e| InternalErrorKind::MMR.cause(e))?;

        db_txn.insert_block(genesis)?;
        db_txn.insert_block_ext(&genesis_hash, &ext)?;
        db_txn.insert_tip_header(&genesis.header())?;
        db_txn.insert_current_epoch_ext(epoch)?;
        db_txn.insert_block_epoch_index(&genesis_hash, &last_block_hash_in_previous_epoch)?;
        db_txn.insert_epoch_ext(&last_block_hash_in_previous_epoch, &epoch)?;
        db_txn.attach_block(genesis)?;
        db_txn.commit()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::COLUMNS;
    use super::*;
    use ckb_chain_spec::consensus::Consensus;
    use ckb_db::RocksDB;

    fn setup_db(columns: u32) -> RocksDB {
        RocksDB::open_tmp(columns)
    }

    #[test]
    fn save_and_get_block() {
        let db = setup_db(COLUMNS);
        let store = ChainDB::new(db, None);
        let consensus = Consensus::default();
        let block = consensus.genesis_block();

        let hash = block.hash();
        let txn = store.begin_transaction();
        txn.insert_block(&block).unwrap();
        txn.commit().unwrap();
        assert_eq!(block, &store.get_block(&hash).unwrap());
    }

    #[test]
    fn save_and_get_block_with_transactions() {
        let db = setup_db(COLUMNS);
        let store = ChainDB::new(db, None);
        let block = packed::Block::new_builder()
            .transactions(
                (0..3)
                    .map(|_| packed::Transaction::new_builder().build())
                    .collect::<Vec<_>>()
                    .pack(),
            )
            .build()
            .into_view();

        let hash = block.hash();
        let txn = store.begin_transaction();
        txn.insert_block(&block).unwrap();
        txn.commit().unwrap();
        assert_eq!(block, store.get_block(&hash).unwrap());
    }

    #[test]
    fn save_and_get_block_ext() {
        let db = setup_db(COLUMNS);
        let store = ChainDB::new(db, None);
        let consensus = Consensus::default();
        let block = consensus.genesis_block();

        let ext = BlockExt {
            received_at: block.timestamp(),
            total_difficulty: block.difficulty(),
            total_uncles_count: block.data().uncles().len() as u64,
            verified: Some(true),
            txs_fees: vec![],
        };

        let hash = block.hash();
        let txn = store.begin_transaction();
        txn.insert_block_ext(&hash, &ext).unwrap();
        txn.commit().unwrap();
        assert_eq!(ext, store.get_block_ext(&hash).unwrap());
    }

    #[test]
    fn index_store() {
        let db = RocksDB::open_tmp(COLUMNS);
        let store = ChainDB::new(db, None);
        let consensus = Consensus::default();
        let block = consensus.genesis_block();
        let hash = block.hash();
        store.init(&consensus).unwrap();
        assert_eq!(hash, store.get_block_hash(0).unwrap());

        assert_eq!(
            block.difficulty(),
            store.get_block_ext(&hash).unwrap().total_difficulty
        );

        assert_eq!(block.number(), store.get_block_number(&hash).unwrap());

        assert_eq!(block.header(), store.get_tip_header().unwrap());
    }
}
