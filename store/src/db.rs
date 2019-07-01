use crate::store::ChainStore;
use crate::transaction::StoreTransaction;
use crate::{StoreConfig, COLUMN_CELL_SET};
use crate::{COLUMN_BLOCK_BODY, COLUMN_BLOCK_HEADER};
use ckb_chain_spec::consensus::Consensus;
use ckb_core::extras::BlockExt;
use ckb_core::header::Header;
use ckb_core::transaction::CellOutput;
use ckb_core::transaction_meta::TransactionMeta;
use ckb_db::{Col, DBPinnableSlice, Error, RocksDB};
use ckb_protos as protos;
use lru_cache::LruCache;
use numext_fixed_hash::H256;
use std::convert::TryInto;
use std::sync::Mutex;

pub struct ChainDB {
    db: RocksDB,
    header_cache: Mutex<LruCache<H256, Header>>,
    cell_output_cache: Mutex<LruCache<(H256, u32), CellOutput>>,
}

impl<'a> ChainStore<'a> for ChainDB {
    type Vector = DBPinnableSlice<'a>;

    fn get(&'a self, col: Col, key: &[u8]) -> Option<Self::Vector> {
        self.db
            .get_pinned(col, key)
            .expect("db operation should be ok")
    }

    fn get_block_header(&self, hash: &H256) -> Option<Header> {
        let mut header_cache_unlocked = self
            .header_cache
            .lock()
            .expect("poisoned header cache lock");
        if let Some(header) = header_cache_unlocked.get_refresh(hash) {
            return Some(header.clone());
        }
        // release lock asap
        drop(header_cache_unlocked);

        self.get(COLUMN_BLOCK_HEADER, hash.as_bytes())
            .map(|slice| {
                protos::StoredHeader::from_slice(&slice.as_ref())
                    .try_into()
                    .expect("deserialize")
            })
            .and_then(|header: Header| {
                let mut header_cache_unlocked = self
                    .header_cache
                    .lock()
                    .expect("poisoned header cache lock");
                header_cache_unlocked.insert(hash.clone(), header.clone());
                Some(header)
            })
    }

    fn get_cell_output(&self, tx_hash: &H256, index: u32) -> Option<CellOutput> {
        let mut cell_output_cache_unlocked = self
            .cell_output_cache
            .lock()
            .expect("poisoned cell output cache lock");
        if let Some(cell_output) = cell_output_cache_unlocked.get_refresh(&(tx_hash.clone(), index))
        {
            return Some(cell_output.clone());
        }
        // release lock asap
        drop(cell_output_cache_unlocked);

        self.get_transaction_info(&tx_hash)
            .and_then(|info| {
                self.get(COLUMN_BLOCK_BODY, info.block_hash.as_bytes())
                    .and_then(|slice| {
                        protos::StoredBlockBody::from_slice(&slice.as_ref())
                            .output(info.index, index as usize)
                            .expect("deserialize")
                    })
            })
            .map(|cell_output: CellOutput| {
                let mut cell_output_cache_unlocked = self
                    .cell_output_cache
                    .lock()
                    .expect("poisoned cell output cache lock");
                cell_output_cache_unlocked.insert((tx_hash.clone(), index), cell_output.clone());
                cell_output
            })
    }
}

impl ChainDB {
    pub fn new(db: RocksDB) -> Self {
        Self::with_config(db, StoreConfig::default())
    }

    pub fn with_config(db: RocksDB, config: StoreConfig) -> Self {
        ChainDB {
            db,
            header_cache: Mutex::new(LruCache::new(config.header_cache_size)),
            cell_output_cache: Mutex::new(LruCache::new(config.cell_output_cache_size)),
        }
    }

    pub fn traverse_cell_set<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(H256, TransactionMeta) -> Result<(), Error>,
    {
        self.db
            .traverse(COLUMN_CELL_SET, |hash_slice, tx_meta_bytes| {
                let tx_hash =
                    H256::from_slice(hash_slice).expect("deserialize tx hash should be ok");
                let tx_meta: TransactionMeta = protos::TransactionMeta::from_slice(tx_meta_bytes)
                    .try_into()
                    .expect("deserialize TransactionMeta should be ok");
                callback(tx_hash, tx_meta)
            })
    }

    pub fn begin_db_transaction<'a>(&'a self) -> StoreTransaction<'a> {
        StoreTransaction {
            inner: self.db.transaction(),
        }
    }

    pub fn init(&self, consensus: &Consensus) -> Result<(), Error> {
        let genesis = consensus.genesis_block();
        let epoch = consensus.genesis_epoch_ext();
        let db_txn = self.begin_db_transaction();
        let genesis_hash = genesis.header().hash();
        let ext = BlockExt {
            received_at: genesis.header().timestamp(),
            total_difficulty: genesis.header().difficulty().clone(),
            total_uncles_count: 0,
            verified: Some(true),
            txs_fees: vec![],
        };

        let mut cells = Vec::with_capacity(genesis.transactions().len());

        for tx in genesis.transactions() {
            let tx_meta;
            let ins = if tx.is_cellbase() {
                tx_meta = TransactionMeta::new_cellbase(
                    genesis.header().number(),
                    genesis.header().epoch(),
                    tx.outputs().len(),
                    false,
                );
                Vec::new()
            } else {
                tx_meta = TransactionMeta::new(
                    genesis.header().number(),
                    genesis.header().epoch(),
                    tx.outputs().len(),
                    false,
                );
                tx.input_pts_iter().cloned().collect()
            };
            db_txn.update_cell_set(tx.hash(), &tx_meta)?;
            let outs = tx.output_pts();

            cells.push((ins, outs));
        }

        db_txn.insert_block(genesis)?;
        db_txn.insert_block_ext(&genesis_hash, &ext)?;
        db_txn.insert_tip_header(&genesis.header())?;
        db_txn.insert_current_epoch_ext(epoch)?;
        db_txn
            .insert_block_epoch_index(&genesis_hash, epoch.last_block_hash_in_previous_epoch())?;
        db_txn.insert_epoch_ext(epoch.last_block_hash_in_previous_epoch(), &epoch)?;
        db_txn.attach_block(genesis)?;
        db_txn.commit()
    }
}
