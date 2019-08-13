use crate::tx_pool::types::PendingEntry;
use ckb_types::{
    core::{
        cell::{CellMetaBuilder, CellProvider, CellStatus},
        Cycle, TransactionView,
    },
    packed::{OutPoint, ProposalShortId},
    prelude::*,
    H256,
};
use ckb_util::{LinkedFnvHashMap, LinkedFnvHashMapEntries};

#[derive(Default, Debug, Clone)]
pub(crate) struct PendingQueue {
    pub(crate) inner: LinkedFnvHashMap<ProposalShortId, PendingEntry>,
}

impl PendingQueue {
    pub(crate) fn new() -> Self {
        PendingQueue {
            inner: LinkedFnvHashMap::default(),
        }
    }

    pub(crate) fn size(&self) -> usize {
        self.inner.len()
    }

    pub(crate) fn add_tx(
        &mut self,
        cycles: Option<Cycle>,
        size: usize,
        tx: TransactionView,
    ) -> Option<PendingEntry> {
        let short_id = tx.proposal_short_id();
        self.inner
            .insert(short_id, PendingEntry::new(tx, cycles, size))
    }

    pub(crate) fn contains_key(&self, id: &ProposalShortId) -> bool {
        self.inner.contains_key(id)
    }

    pub(crate) fn get(&self, id: &ProposalShortId) -> Option<&PendingEntry> {
        self.inner.get(id)
    }

    pub(crate) fn get_tx(&self, id: &ProposalShortId) -> Option<&TransactionView> {
        self.get(id).map(|x| &x.transaction)
    }

    pub(crate) fn remove(&mut self, id: &ProposalShortId) -> Option<PendingEntry> {
        self.inner.remove(id)
    }

    pub(crate) fn keys(&self) -> impl Iterator<Item = &ProposalShortId> {
        self.inner.keys()
    }

    pub(crate) fn entries(&mut self) -> LinkedFnvHashMapEntries<ProposalShortId, PendingEntry> {
        self.inner.entries()
    }
}

impl CellProvider for PendingQueue {
    fn cell(&self, o: &OutPoint) -> CellStatus {
        if let Some(cell_out_point) = &o.cell().to_opt() {
            let hash: H256 = cell_out_point.tx_hash().unpack();
            if let Some(x) = self.inner.get(&ProposalShortId::from_tx_hash(&hash)) {
                match x
                    .transaction
                    .output_with_data(cell_out_point.index().unpack())
                {
                    Some((output, data)) => CellStatus::live_cell(
                        CellMetaBuilder::from_cell_output(output.to_owned(), data)
                            .out_point(cell_out_point.to_owned())
                            .build(),
                    ),
                    None => CellStatus::Unknown,
                }
            } else {
                CellStatus::Unknown
            }
        } else {
            CellStatus::Unspecified
        }
    }
}
