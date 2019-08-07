use crate::{CELL_DATA_CACHE, HEADER_CACHE};
use crate::{
    COLUMN_BLOCK_BODY, COLUMN_BLOCK_EPOCH, COLUMN_BLOCK_EXT, COLUMN_BLOCK_HEADER,
    COLUMN_BLOCK_PROPOSAL_IDS, COLUMN_BLOCK_UNCLE, COLUMN_CELL_SET, COLUMN_EPOCH, COLUMN_INDEX,
    COLUMN_META, COLUMN_TRANSACTION_INFO, COLUMN_UNCLES, META_CURRENT_EPOCH_KEY,
    META_TIP_HEADER_KEY,
};
use ckb_chain_spec::consensus::Consensus;
use ckb_db::Col;
use ckb_types::{
    bytes::Bytes,
    core::{
        cell::{BlockInfo, CellMeta, TransactionMeta},
        BlockBodyView, BlockExt, BlockNumber, BlockView, EpochExt, EpochNumber, HeaderView,
        TransactionView, UncleBlockVecView,
    },
    packed,
    prelude::*,
};

pub trait ChainStore<'a>: Send + Sync {
    type Vector: AsRef<[u8]>;
    fn get(&'a self, col: Col, key: &[u8]) -> Option<Self::Vector>;

    /// Get block by block header hash
    fn get_block(&'a self, h: &packed::Byte32) -> Option<BlockView> {
        self.get_block_header(h).map(|header| {
            let transactions = self
                .get_block_body(h)
                .expect("block transactions must be stored");
            let uncles = self
                .get_block_uncles(h)
                .expect("block uncles must be stored");
            let proposals = self
                .get_block_proposal_txs_ids(h)
                .expect("block proposal_ids must be stored");
            BlockView::new_unchecked(header, uncles, transactions, proposals)
        })
    }

    /// Get header by block header hash
    fn get_block_header(&'a self, hash: &packed::Byte32) -> Option<HeaderView> {
        {
            if let Some(header) = HEADER_CACHE.lock().get_refresh(hash) {
                return Some(header.clone());
            }
        }
        self.get(COLUMN_BLOCK_HEADER, hash.as_slice())
            .map(|slice| {
                packed::HeaderViewReader::from_slice(&slice.as_ref())
                    .should_be_ok()
                    .unpack()
            })
            .map(|header: HeaderView| {
                HEADER_CACHE.lock().insert(hash.clone(), header.clone());
                header
            })
    }

    /// Get block body by block header hash
    fn get_block_body(&'a self, hash: &packed::Byte32) -> Option<BlockBodyView> {
        self.get(COLUMN_BLOCK_BODY, hash.as_slice()).map(|slice| {
            packed::BlockBodyViewReader::from_slice(&slice.as_ref())
                .should_be_ok()
                .unpack()
        })
    }

    /// Get all transaction-hashes in block body by block header hash
    fn get_block_txs_hashes(&'a self, hash: &packed::Byte32) -> Option<packed::Byte32Vec> {
        self.get(COLUMN_BLOCK_BODY, hash.as_slice()).map(|slice| {
            packed::BlockBodyViewReader::from_slice(&slice.as_ref())
                .should_be_ok()
                .tx_hashes()
                .to_entity()
        })
    }

    /// Get proposal short id by block header hash
    fn get_block_proposal_txs_ids(
        &'a self,
        hash: &packed::Byte32,
    ) -> Option<packed::ProposalShortIdVec> {
        self.get(COLUMN_BLOCK_PROPOSAL_IDS, hash.as_slice())
            .map(|slice| {
                packed::ProposalShortIdVecReader::from_slice(&slice.as_ref())
                    .should_be_ok()
                    .to_entity()
            })
    }

    /// Get block uncles by block header hash
    fn get_block_uncles(&'a self, hash: &packed::Byte32) -> Option<UncleBlockVecView> {
        self.get(COLUMN_BLOCK_UNCLE, hash.as_slice()).map(|slice| {
            packed::UncleBlockVecViewReader::from_slice(&slice.as_ref())
                .should_be_ok()
                .unpack()
        })
    }

    /// Get block ext by block header hash
    fn get_block_ext(&'a self, block_hash: &packed::Byte32) -> Option<BlockExt> {
        self.get(COLUMN_BLOCK_EXT, block_hash.as_slice())
            .map(|slice| {
                packed::BlockExtReader::from_slice(&slice.as_ref()[..])
                    .should_be_ok()
                    .unpack()
            })
    }

    /// Get block header hash by block number
    fn get_block_hash(&'a self, number: BlockNumber) -> Option<packed::Byte32> {
        self.get(COLUMN_INDEX, number.pack().as_slice()).map(|raw| {
            packed::Byte32Reader::from_slice(&raw.as_ref()[..])
                .should_be_ok()
                .to_entity()
        })
    }

    /// Get block number by block header hash
    fn get_block_number(&'a self, hash: &packed::Byte32) -> Option<BlockNumber> {
        self.get(COLUMN_INDEX, hash.as_slice()).map(|raw| {
            packed::Uint64Reader::from_slice(&raw.as_ref()[..])
                .should_be_ok()
                .unpack()
        })
    }

    fn get_tip_header(&'a self) -> Option<HeaderView> {
        self.get(COLUMN_META, META_TIP_HEADER_KEY)
            .and_then(|raw| {
                self.get_block_header(
                    &packed::Byte32Reader::from_slice(&raw.as_ref()[..])
                        .should_be_ok()
                        .to_entity(),
                )
            })
            .map(Into::into)
    }

    /// Get commit transaction and block hash by it's hash
    fn get_transaction(
        &'a self,
        hash: &packed::Byte32,
    ) -> Option<(TransactionView, packed::Byte32)> {
        self.get_transaction_info(hash).and_then(|info| {
            self.get(COLUMN_BLOCK_BODY, info.block_hash().as_slice())
                .and_then(|slice| {
                    packed::BlockBodyViewReader::from_slice(&slice.as_ref())
                        .should_be_ok()
                        .transaction(info.index().unpack())
                        .map(|tx| (tx, info.block_hash()))
                })
        })
    }

    fn get_transaction_info(&'a self, hash: &packed::Byte32) -> Option<packed::TransactionInfo> {
        self.get(COLUMN_TRANSACTION_INFO, hash.as_slice())
            .map(|slice| {
                packed::TransactionInfoReader::from_slice(&slice.as_ref())
                    .should_be_ok()
                    .to_entity()
            })
    }

    fn get_tx_meta(&'a self, tx_hash: &packed::Byte32) -> Option<TransactionMeta> {
        self.get(COLUMN_CELL_SET, tx_hash.as_slice()).map(|slice| {
            packed::TransactionMetaReader::from_slice(&slice.as_ref())
                .should_be_ok()
                .unpack()
        })
    }

    fn get_cell_meta(&'a self, tx_hash: &packed::Byte32, index: u32) -> Option<CellMeta> {
        self.get_transaction_info(tx_hash)
            .and_then(|tx_info| {
                let tx_index: usize = tx_info.index().unpack();
                self.get(COLUMN_BLOCK_BODY, tx_info.block_hash().as_slice())
                    .and_then(|slice| {
                        packed::BlockBodyViewReader::from_slice(&slice.as_ref())
                            .should_be_ok()
                            .data()
                            .get(tx_index)
                            .and_then(|tx| {
                                tx.slim()
                                    .raw()
                                    .outputs()
                                    .get(index as usize)
                                    .and_then(|output| {
                                        let output = output.to_entity();
                                        tx.outputs_data().get(index as usize).map(|data| {
                                            let data_len = data.raw_data().len();
                                            (tx_info, output, data_len as u64)
                                        })
                                    })
                            })
                    })
            })
            .map(|(tx_info, cell_output, data_bytes)| {
                let out_point = packed::CellOutPoint::new_builder()
                    .tx_hash(tx_hash.clone())
                    .index(index.pack())
                    .build();
                let tx_index: u32 = tx_info.index().unpack();
                let cellbase = tx_index == 0;
                let block_info = BlockInfo {
                    number: tx_info.block_number().unpack(),
                    epoch: tx_info.block_epoch().unpack(),
                    hash: tx_info.block_hash().unpack(),
                };
                // notice mem_cell_data is set to None, the cell data should be load in need
                CellMeta {
                    cell_output,
                    out_point,
                    block_info: Some(block_info),
                    cellbase,
                    data_bytes,
                    mem_cell_data: None,
                }
            })
    }

    fn get_cell_data(&'a self, tx_hash: &packed::Byte32, index: u32) -> Option<Bytes> {
        {
            if let Some(data) = CELL_DATA_CACHE
                .lock()
                .get_refresh(&(tx_hash.clone(), index))
            {
                return Some(data.clone());
            }
        }

        self.get_transaction_info(tx_hash)
            .and_then(|info| {
                let tx_index: usize = info.index().unpack();
                self.get(COLUMN_BLOCK_BODY, info.block_hash().as_slice())
                    .and_then(|slice| {
                        packed::BlockBodyViewReader::from_slice(&slice.as_ref())
                            .should_be_ok()
                            .data()
                            .get(tx_index)
                            .and_then(|tx| {
                                tx.outputs_data()
                                    .get(index as usize)
                                    .map(|data| data.raw_data().into())
                            })
                    })
            })
            .map(|data: Bytes| {
                CELL_DATA_CACHE
                    .lock()
                    .insert((tx_hash.clone(), index), data.clone());
                data
            })
    }

    // Get current epoch ext
    fn get_current_epoch_ext(&'a self) -> Option<EpochExt> {
        self.get(COLUMN_META, META_CURRENT_EPOCH_KEY).map(|slice| {
            packed::EpochExtReader::from_slice(&slice.as_ref())
                .should_be_ok()
                .unpack()
        })
    }

    // Get epoch ext by epoch index
    fn get_epoch_ext(&'a self, hash: &packed::Byte32) -> Option<EpochExt> {
        self.get(COLUMN_EPOCH, hash.as_slice()).map(|slice| {
            packed::EpochExtReader::from_slice(&slice.as_ref())
                .should_be_ok()
                .unpack()
        })
    }

    // Get epoch index by epoch number
    fn get_epoch_index(&'a self, number: EpochNumber) -> Option<packed::Byte32> {
        self.get(COLUMN_EPOCH, number.pack().as_slice()).map(|raw| {
            packed::Byte32Reader::from_slice(&raw.as_ref())
                .should_be_ok()
                .to_entity()
        })
    }

    // Get epoch index by block hash
    fn get_block_epoch_index(&'a self, block_hash: &packed::Byte32) -> Option<packed::Byte32> {
        self.get(COLUMN_BLOCK_EPOCH, block_hash.as_slice())
            .map(|raw| {
                packed::Byte32Reader::from_slice(&raw.as_ref())
                    .should_be_ok()
                    .to_entity()
            })
    }

    fn get_block_epoch(&'a self, hash: &packed::Byte32) -> Option<EpochExt> {
        self.get_block_epoch_index(hash)
            .and_then(|index| self.get_epoch_ext(&index))
    }

    fn is_uncle(&'a self, hash: &packed::Byte32) -> bool {
        self.get(COLUMN_UNCLES, hash.as_slice()).is_some()
    }

    fn block_exists(&'a self, hash: &packed::Byte32) -> bool {
        self.get(COLUMN_BLOCK_HEADER, hash.as_slice()).is_some()
    }

    // Get cellbase by block hash
    fn get_cellbase(&'a self, hash: &packed::Byte32) -> Option<TransactionView> {
        self.get(COLUMN_BLOCK_BODY, hash.as_slice())
            .and_then(|slice| {
                packed::BlockBodyViewReader::from_slice(&slice.as_ref())
                    .should_be_ok()
                    .transaction(0)
            })
    }

    fn next_epoch_ext(
        &'a self,
        consensus: &Consensus,
        last_epoch: &EpochExt,
        header: &HeaderView,
    ) -> Option<EpochExt> {
        consensus.next_epoch_ext(
            last_epoch,
            header,
            |hash| self.get_block_header(&hash),
            |hash| self.get_block_ext(&hash).map(|ext| ext.total_uncles_count),
        )
    }

    fn get_ancestor(&'a self, base: &packed::Byte32, number: BlockNumber) -> Option<HeaderView> {
        if let Some(header) = self.get_block_header(base) {
            let mut n_number: BlockNumber = header.data().raw().number().unpack();
            let mut index_walk = header;
            if number > n_number {
                return None;
            }

            while n_number > number {
                if let Some(header) = self.get_block_header(&index_walk.data().raw().parent_hash())
                {
                    index_walk = header;
                    n_number -= 1;
                } else {
                    return None;
                }
            }
            return Some(index_walk);
        }
        None
    }

    // Only for test
    fn clear_header_cache(&'a self) {
        HEADER_CACHE.lock().clear();
    }

    // Only for test
    fn clear_cell_data_cache(&'a self) {
        CELL_DATA_CACHE.lock().clear();
    }
}
