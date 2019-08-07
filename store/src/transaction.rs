use crate::store::ChainStore;
use crate::{
    COLUMN_BLOCK_BODY, COLUMN_BLOCK_EPOCH, COLUMN_BLOCK_EXT, COLUMN_BLOCK_HEADER,
    COLUMN_BLOCK_PROPOSAL_IDS, COLUMN_BLOCK_UNCLE, COLUMN_CELL_SET, COLUMN_EPOCH, COLUMN_INDEX,
    COLUMN_META, COLUMN_TRANSACTION_INFO, COLUMN_UNCLES, META_CURRENT_EPOCH_KEY,
    META_TIP_HEADER_KEY,
};
use ckb_db::{Col, DBVector, Error, RocksDBTransaction, RocksDBTransactionSnapshot};
use ckb_types::{
    core::{
        cell::{CellProvider, CellStatus, HeaderProvider, HeaderStatus},
        BlockExt, BlockView, EpochExt, HeaderView,
    },
    packed,
    prelude::*,
};

pub struct StoreTransaction {
    pub(crate) inner: RocksDBTransaction,
}

impl<'a> ChainStore<'a> for StoreTransaction {
    type Vector = DBVector;

    fn get(&self, col: Col, key: &[u8]) -> Option<Self::Vector> {
        self.inner.get(col, key).expect("db operation should be ok")
    }
}

impl<'a> ChainStore<'a> for RocksDBTransactionSnapshot<'a> {
    type Vector = DBVector;

    fn get(&self, col: Col, key: &[u8]) -> Option<Self::Vector> {
        self.get(col, key).expect("db operation should be ok")
    }
}

impl StoreTransaction {
    pub fn insert_raw(&self, col: Col, key: &[u8], value: &[u8]) -> Result<(), Error> {
        self.inner.put(col, key, value)
    }

    pub fn delete(&self, col: Col, key: &[u8]) -> Result<(), Error> {
        self.inner.delete(col, key)
    }

    pub fn commit(&self) -> Result<(), Error> {
        self.inner.commit()
    }

    pub fn get_snapshot(&self) -> RocksDBTransactionSnapshot<'_> {
        self.inner.get_snapshot()
    }

    pub fn get_update_for_tip_hash(
        &self,
        snapshot: &RocksDBTransactionSnapshot<'_>,
    ) -> Option<packed::Byte32> {
        self.inner
            .get_for_update(COLUMN_META, META_TIP_HEADER_KEY, snapshot)
            .expect("db operation should be ok")
            .map(|slice| {
                packed::Byte32Reader::from_slice(&slice.as_ref()[..])
                    .should_be_ok()
                    .to_entity()
            })
    }

    pub fn insert_tip_header(&self, h: &HeaderView) -> Result<(), Error> {
        self.insert_raw(COLUMN_META, META_TIP_HEADER_KEY, h.hash().as_slice())
    }

    pub fn insert_block(&self, block: &BlockView) -> Result<(), Error> {
        let hash = block.hash();
        let header = block.header().pack();
        let uncles = block.uncles().pack();
        let body = block.body().pack();
        let proposals = block.data().proposals();
        self.insert_raw(COLUMN_BLOCK_HEADER, hash.as_slice(), header.as_slice())?;
        self.insert_raw(COLUMN_BLOCK_UNCLE, hash.as_slice(), uncles.as_slice())?;
        self.insert_raw(
            COLUMN_BLOCK_PROPOSAL_IDS,
            hash.as_slice(),
            proposals.as_slice(),
        )?;
        self.insert_raw(COLUMN_BLOCK_BODY, hash.as_slice(), body.as_slice())?;
        Ok(())
    }

    pub fn insert_block_ext(
        &self,
        block_hash: &packed::Byte32,
        ext: &BlockExt,
    ) -> Result<(), Error> {
        self.insert_raw(
            COLUMN_BLOCK_EXT,
            block_hash.as_slice(),
            ext.pack().as_slice(),
        )
    }

    pub fn attach_block(&self, block: &BlockView) -> Result<(), Error> {
        let header = block.data().header();
        let block_hash = block.hash();
        for (index, tx_hash) in block.tx_hashes().into_iter().enumerate() {
            let info = packed::TransactionInfo::new_builder()
                .block_hash(block_hash.clone())
                .block_number(header.raw().number())
                .block_epoch(header.raw().epoch())
                .index(index.pack())
                .build();
            self.insert_raw(COLUMN_TRANSACTION_INFO, tx_hash.as_slice(), info.as_slice())?;
        }
        self.insert_raw(
            COLUMN_INDEX,
            header.raw().number().as_slice(),
            block_hash.as_slice(),
        )?;
        for uncle_hash in block.uncle_hashes().into_iter() {
            self.insert_raw(COLUMN_UNCLES, &uncle_hash.as_slice(), &[])?;
        }
        self.insert_raw(
            COLUMN_INDEX,
            block_hash.as_slice(),
            header.raw().number().as_slice(),
        )
    }

    pub fn detach_block(&self, block: &BlockView) -> Result<(), Error> {
        for tx_hash in block.tx_hashes().into_iter() {
            self.delete(COLUMN_TRANSACTION_INFO, tx_hash.as_slice())?;
        }
        for uncle_hash in block.uncle_hashes().into_iter() {
            self.delete(COLUMN_UNCLES, uncle_hash.as_slice())?;
        }
        self.delete(
            COLUMN_INDEX,
            &block.data().header().raw().number().as_slice(),
        )?;
        self.delete(COLUMN_INDEX, block.hash().as_slice())
    }

    pub fn insert_block_epoch_index(
        &self,
        block_hash: &packed::Byte32,
        epoch_hash: &packed::Byte32,
    ) -> Result<(), Error> {
        self.insert_raw(
            COLUMN_BLOCK_EPOCH,
            block_hash.as_slice(),
            epoch_hash.as_slice(),
        )
    }

    pub fn insert_epoch_ext(&self, hash: &packed::Byte32, epoch: &EpochExt) -> Result<(), Error> {
        self.insert_raw(COLUMN_EPOCH, hash.as_slice(), epoch.pack().as_slice())?;
        self.insert_raw(
            COLUMN_EPOCH,
            epoch.number().pack().as_slice(),
            hash.as_slice(),
        )
    }

    pub fn insert_current_epoch_ext(&self, epoch: &EpochExt) -> Result<(), Error> {
        self.insert_raw(COLUMN_META, META_CURRENT_EPOCH_KEY, epoch.pack().as_slice())
    }

    pub fn update_cell_set(
        &self,
        tx_hash: &packed::Byte32,
        meta: &packed::TransactionMeta,
    ) -> Result<(), Error> {
        self.insert_raw(COLUMN_CELL_SET, tx_hash.as_slice(), meta.as_slice())
    }

    pub fn delete_cell_set(&self, tx_hash: &packed::Byte32) -> Result<(), Error> {
        self.delete(COLUMN_CELL_SET, tx_hash.as_slice())
    }
}

impl CellProvider for StoreTransaction {
    fn cell(&self, out_point: &packed::OutPoint) -> CellStatus {
        if let Some(cell_out_point) = &out_point.cell().to_opt() {
            match self.get_tx_meta(&cell_out_point.tx_hash()) {
                Some(tx_meta) => match tx_meta.is_dead(cell_out_point.index().unpack()) {
                    Some(false) => {
                        let cell_meta = self
                            .get_cell_meta(
                                &cell_out_point.tx_hash(),
                                cell_out_point.index().unpack(),
                            )
                            .expect("store should be consistent with cell_set");
                        CellStatus::live_cell(cell_meta)
                    }
                    Some(true) => CellStatus::Dead,
                    None => CellStatus::Unknown,
                },
                None => CellStatus::Unknown,
            }
        } else {
            CellStatus::Unspecified
        }
    }
}

impl HeaderProvider for StoreTransaction {
    fn header(&self, out_point: &packed::OutPoint) -> HeaderStatus {
        if let Some(block_hash) = &out_point.block_hash().to_opt() {
            match self.get_block_header(&block_hash) {
                Some(header) => {
                    if let Some(cell_out_point) = &out_point.cell().to_opt() {
                        self.get_transaction_info(&cell_out_point.tx_hash()).map_or(
                            HeaderStatus::InclusionFaliure,
                            |info| {
                                if &info.block_hash() == block_hash {
                                    HeaderStatus::live_header(header)
                                } else {
                                    HeaderStatus::InclusionFaliure
                                }
                            },
                        )
                    } else {
                        HeaderStatus::live_header(header)
                    }
                }
                None => HeaderStatus::Unknown,
            }
        } else {
            HeaderStatus::Unspecified
        }
    }
}
