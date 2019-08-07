use crate::store::ChainStore;
use ckb_core::cell::{CellProvider, CellStatus, HeaderChecker};
use ckb_core::transaction::OutPoint;
use ckb_db::{
    iter::{DBIterator, DBIteratorItem, Direction},
    Col, DBPinnableSlice, RocksDBSnapshot,
};
use numext_fixed_hash::H256;

pub struct StoreSnapshot {
    pub inner: RocksDBSnapshot,
}

impl<'a> ChainStore<'a> for StoreSnapshot {
    type Vector = DBPinnableSlice<'a>;

    fn get(&'a self, col: Col, key: &[u8]) -> Option<Self::Vector> {
        self.inner
            .get_pinned(col, key)
            .expect("db operation should be ok")
    }

    fn get_iter<'i>(
        &'i self,
        col: Col,
        from_key: &'i [u8],
        direction: Direction,
    ) -> Box<Iterator<Item = DBIteratorItem> + 'i> {
        self.inner
            .iter(col, from_key, direction)
            .expect("db operation should be ok")
    }
}

impl CellProvider for StoreSnapshot {
    fn cell(&self, out_point: &OutPoint, with_data: bool) -> CellStatus {
        match self.get_tx_meta(&out_point.tx_hash) {
            Some(tx_meta) => match tx_meta.is_dead(out_point.index as usize) {
                Some(false) => {
                    let mut cell_meta = self
                        .get_cell_meta(&out_point.tx_hash, out_point.index)
                        .expect("store should be consistent with cell_set");
                    if with_data {
                        cell_meta.mem_cell_data =
                            self.get_cell_data(&out_point.tx_hash, out_point.index);
                    }
                    CellStatus::live_cell(cell_meta)
                }
                Some(true) => CellStatus::Dead,
                None => CellStatus::Unknown,
            },
            None => CellStatus::Unknown,
        }
    }
}

impl HeaderChecker for StoreSnapshot {
    fn is_valid(&self, block_hash: &H256) -> bool {
        self.get_block_number(block_hash).is_some()
    }
}
