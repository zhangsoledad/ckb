use crate::snapshot::Snapshot;
use crate::tx_pool::{DefectEntry, PoolError, ProposedEntry, TxPool};
use ckb_dao::DaoCalculator;
use ckb_logger::{debug_target, error_target, trace_target};
use ckb_store::ChainStore;
use ckb_types::{
    core::{
        cell::{
            get_related_dep_out_points, resolve_transaction, OverlayCellProvider,
            ResolvedTransaction, UnresolvableError,
        },
        BlockView, Cycle, TransactionView,
    },
    packed::{Byte32, OutPoint, ProposalShortId},
    prelude::*,
};
use ckb_util::LinkedFnvHashSet;
use ckb_verification::{ContextualTransactionVerifier, TransactionVerifier};
use lru_cache::LruCache;
use std::collections::HashSet;
use std::sync::Arc;

impl TxPool {
    pub fn add_txs_to_pool(&mut self, txs: Vec<TransactionView>) -> Result<Vec<Cycle>, PoolError> {
        txs.into_iter()
            .map(|tx| self.add_tx_to_pool(tx, None))
            .collect()
    }

    // Add a verified tx into pool
    // this method will handle fork related verifications to make sure we are safe during a fork
    pub fn add_tx_to_pool(
        &mut self,
        tx: TransactionView,
        cycles: Option<Cycle>,
    ) -> Result<Cycle, PoolError> {
        let tx_size = tx.serialized_size();
        if self.reach_size_limit(tx_size) {
            return Err(PoolError::LimitReached);
        }
        let short_id = tx.proposal_short_id();
        match self.resolve_tx_from_pending_and_proposed(&tx) {
            Ok(rtx) => self.verify_rtx(&rtx, cycles).and_then(|cycles| {
                if self.reach_cycles_limit(cycles) {
                    return Err(PoolError::LimitReached);
                }
                if self.contains_proposed(&short_id) {
                    if let Err(e) = self.proposed_tx_and_descendants(Some(cycles), tx_size, tx) {
                        debug_target!(
                            crate::LOG_TARGET_TX_POOL,
                            "Failed to add proposed tx {:?}, reason: {:?}",
                            short_id,
                            e
                        );
                        return Err(e);
                    }
                    self.update_statics_for_add_tx(tx_size, cycles);
                } else if self.enqueue_tx(Some(cycles), tx_size, tx) {
                    self.update_statics_for_add_tx(tx_size, cycles);
                }
                Ok(cycles)
            }),
            Err(err) => Err(PoolError::UnresolvableTransaction(err)),
        }
    }

    fn contains_proposed(&self, short_id: &ProposalShortId) -> bool {
        self.snapshot().proposals().contains_proposed(short_id)
    }

    // fn contains_gap(&self, short_id: &ProposalShortId) -> bool {
    //     self.snapshot().proposals().contains_gap(short_id)
    // }

    pub fn resolve_tx_from_pending_and_proposed<'a>(
        &self,
        tx: &'a TransactionView,
    ) -> Result<ResolvedTransaction<'a>, UnresolvableError> {
        let snapshot = self.snapshot();
        let proposed_provider = OverlayCellProvider::new(&self.proposed, snapshot);
        let gap_and_proposed_provider = OverlayCellProvider::new(&self.gap, &proposed_provider);
        let pending_and_proposed_provider =
            OverlayCellProvider::new(&self.pending, &gap_and_proposed_provider);
        let mut seen_inputs = HashSet::new();
        resolve_transaction(
            tx,
            &mut seen_inputs,
            &pending_and_proposed_provider,
            snapshot,
        )
    }

    pub fn resolve_tx_from_proposed<'a>(
        &self,
        tx: &'a TransactionView,
    ) -> Result<ResolvedTransaction<'a>, UnresolvableError> {
        let snapshot = self.snapshot();
        let cell_provider = OverlayCellProvider::new(&self.proposed, snapshot);
        let mut seen_inputs = HashSet::new();
        resolve_transaction(tx, &mut seen_inputs, &cell_provider, snapshot)
    }

    pub(crate) fn verify_rtx(
        &self,
        rtx: &ResolvedTransaction,
        cycles: Option<Cycle>,
    ) -> Result<Cycle, PoolError> {
        let snapshot = self.snapshot();
        let tip_header = snapshot.tip_header();
        let tip_number = tip_header.number();
        let epoch_number = tip_header.epoch();
        let consensus = snapshot.consensus();

        match cycles {
            Some(cycles) => {
                ContextualTransactionVerifier::new(
                    &rtx,
                    snapshot,
                    tip_number + 1,
                    epoch_number,
                    tip_header.hash(),
                    consensus,
                )
                .verify()
                .map_err(PoolError::InvalidTx)?;
                Ok(cycles)
            }
            None => {
                let max_cycles = consensus.max_block_cycles();
                let cycles = TransactionVerifier::new(
                    &rtx,
                    snapshot,
                    tip_number + 1,
                    epoch_number,
                    tip_header.hash(),
                    consensus,
                    &self.script_config,
                    snapshot,
                )
                .verify(max_cycles)
                .map_err(PoolError::InvalidTx)?;
                Ok(cycles)
            }
        }
    }

    // remove resolved tx from orphan pool
    pub(crate) fn try_proposed_orphan_by_ancestor(&mut self, tx: &TransactionView) {
        let entries = self.orphan.remove_by_ancestor(tx);
        for entry in entries {
            if self.contains_proposed(&tx.proposal_short_id()) {
                let tx_hash = entry.transaction.hash().to_owned();
                let ret = self.proposed_tx(entry.cycles, entry.size, entry.transaction);
                if ret.is_err() {
                    self.update_statics_for_remove_tx(entry.size, entry.cycles.unwrap_or(0));
                    trace_target!(
                        crate::LOG_TARGET_TX_POOL,
                        "proposed tx {} failed {:?}",
                        tx_hash,
                        ret
                    );
                }
            } else {
                self.enqueue_tx(entry.cycles, entry.size, entry.transaction);
            }
        }
    }

    pub(crate) fn proposed_tx(
        &mut self,
        cycles: Option<Cycle>,
        size: usize,
        tx: TransactionView,
    ) -> Result<Cycle, PoolError> {
        let short_id = tx.proposal_short_id();
        let tx_hash = tx.hash();

        match self.resolve_tx_from_proposed(&tx) {
            Ok(rtx) => match self.verify_rtx(&rtx, cycles) {
                Ok(cycles) => {
                    let fee = DaoCalculator::new(self.snapshot().consensus(), self.snapshot())
                        .transaction_fee(&rtx)
                        .map_err(|e| {
                            error_target!(
                                crate::LOG_TARGET_TX_POOL,
                                "Failed to generate tx fee for {}, reason: {:?}",
                                tx_hash,
                                e
                            );
                            self.update_statics_for_remove_tx(size, cycles);
                            PoolError::TxFee
                        })?;
                    // Resolved out points and dep group out point
                    let related_out_points: Vec<OutPoint> = rtx
                        .resolved_cell_deps
                        .iter()
                        .map(|cell_meta| cell_meta.out_point.clone())
                        .chain({
                            tx.cell_deps_iter()
                                .filter(|dep| dep.is_dep_group().unpack())
                                .map(|dep| dep.out_point().clone())
                        })
                        .collect();
                    self.add_proposed(cycles, fee, size, tx, related_out_points);
                    Ok(cycles)
                }
                Err(e) => {
                    self.update_statics_for_remove_tx(size, cycles.unwrap_or(0));
                    debug_target!(
                        crate::LOG_TARGET_TX_POOL,
                        "Failed to add proposed tx {}, reason: {:?}",
                        tx_hash,
                        e
                    );
                    Err(e)
                }
            },
            Err(err) => {
                match &err {
                    UnresolvableError::Dead(_) => {
                        if self
                            .conflict
                            .insert(short_id, DefectEntry::new(tx, 0, cycles, size))
                            .is_some()
                        {
                            self.update_statics_for_remove_tx(size, cycles.unwrap_or(0));
                        }
                    }
                    UnresolvableError::Unknown(out_points) => {
                        if self
                            .add_orphan(cycles, size, tx, out_points.to_owned())
                            .is_some()
                        {
                            self.update_statics_for_remove_tx(size, cycles.unwrap_or(0));
                        }
                    }
                    // The remaining errors are InvalidHeader/InvalidDepGroup.
                    // They all represent invalid transactions
                    // that should just be discarded.
                    // OutOfOrder should only appear in BlockCellProvider
                    UnresolvableError::InvalidDepGroup(_)
                    | UnresolvableError::InvalidHeader(_)
                    | UnresolvableError::OutOfOrder(_) => {
                        self.update_statics_for_remove_tx(size, cycles.unwrap_or(0));
                    }
                }
                Err(PoolError::UnresolvableTransaction(err))
            }
        }
    }

    pub(crate) fn proposed_tx_and_descendants(
        &mut self,
        cycles: Option<Cycle>,
        size: usize,
        tx: TransactionView,
    ) -> Result<Cycle, PoolError> {
        self.proposed_tx(cycles, size, tx.clone()).map(|cycles| {
            self.try_proposed_orphan_by_ancestor(&tx);
            cycles
        })
    }

    pub fn update_tx_pool_for_reorg<'a>(
        &mut self,
        detached_blocks: impl Iterator<Item = &'a BlockView>,
        attached_blocks: impl Iterator<Item = &'a BlockView>,
        detached_proposal_id: impl Iterator<Item = &'a ProposalShortId>,
        txs_verify_cache: &mut LruCache<Byte32, Cycle>,
        snapshot: Arc<Snapshot>,
    ) {
        self.snapshot = Arc::clone(&snapshot);
        let mut detached = LinkedFnvHashSet::default();
        let mut attached = LinkedFnvHashSet::default();

        for blk in detached_blocks {
            detached.extend(blk.transactions().iter().skip(1).cloned())
        }

        for blk in attached_blocks {
            attached.extend(blk.transactions().iter().skip(1).cloned())
        }

        let retain: Vec<TransactionView> = detached.difference(&attached).cloned().collect();

        let txs_iter = attached.iter().map(|tx| {
            let get_cell_data = |out_point: &OutPoint| {
                snapshot.get_cell_data(&out_point.tx_hash(), out_point.index().unpack())
            };
            let related_out_points =
                get_related_dep_out_points(tx, get_cell_data).expect("Get dep out points failed");
            (tx, related_out_points)
        });
        self.remove_expired(detached_proposal_id);
        self.remove_committed_txs_from_proposed(txs_iter);

        for tx in retain {
            let tx_hash = tx.hash().to_owned();
            let cached_cycles = txs_verify_cache.get(&tx_hash).cloned();
            let tx_short_id = tx.proposal_short_id();
            let tx_size = tx.serialized_size();
            if snapshot.proposals().contains_proposed(&tx_short_id) {
                if let Ok(cycles) = self.proposed_tx_and_descendants(cached_cycles, tx_size, tx) {
                    if cached_cycles.is_none() {
                        txs_verify_cache.insert(tx_hash, cycles);
                    }
                    self.update_statics_for_add_tx(tx_size, cycles);
                }
            } else if snapshot.proposals().contains_gap(&tx_short_id) {
                if self.add_gap(cached_cycles, tx_size, tx) {
                    self.update_statics_for_add_tx(tx_size, cached_cycles.unwrap_or(0));
                }
            } else if self.enqueue_tx(cached_cycles, tx_size, tx) {
                self.update_statics_for_add_tx(tx_size, cached_cycles.unwrap_or(0));
            }
        }

        for tx in &attached {
            self.try_proposed_orphan_by_ancestor(tx);
        }

        let mut entries = Vec::new();
        let mut gaps = Vec::new();

        // pending ---> gap ----> proposed
        // try move gap to proposed

        for entry in self.gap.entries() {
            if snapshot.proposals().contains_proposed(entry.key()) {
                let entry = entry.remove();
                entries.push((entry.cycles, entry.size, entry.transaction));
            }
        }

        // try move pending to proposed
        for entry in self.pending.entries() {
            if snapshot.proposals().contains_proposed(entry.key()) {
                let entry = entry.remove();
                entries.push((entry.cycles, entry.size, entry.transaction));
            } else if snapshot.proposals().contains_gap(entry.key()) {
                let entry = entry.remove();
                gaps.push((entry.cycles, entry.size, entry.transaction));
            }
        }

        // try move conflict to proposed
        for entry in self.conflict.entries() {
            if snapshot.proposals().contains_proposed(entry.key()) {
                let entry = entry.remove();
                entries.push((entry.cycles, entry.size, entry.transaction));
            } else if snapshot.proposals().contains_gap(entry.key()) {
                let entry = entry.remove();
                gaps.push((entry.cycles, entry.size, entry.transaction));
            }
        }

        for (cycles, size, tx) in entries {
            let tx_hash = tx.hash().to_owned();
            if let Err(e) = self.proposed_tx_and_descendants(cycles, size, tx) {
                debug_target!(
                    crate::LOG_TARGET_TX_POOL,
                    "Failed to add proposed tx {}, reason: {:?}",
                    tx_hash,
                    e
                );
            }
        }

        for (cycles, size, tx) in gaps {
            debug_target!(
                crate::LOG_TARGET_TX_POOL,
                "tx proposed, add to gap {}",
                tx.hash()
            );
            self.add_gap(cycles, size, tx);
        }
    }

    pub fn get_proposals(&self, proposals_limit: usize) -> HashSet<ProposalShortId> {
        self.pending
            .keys()
            .chain(self.gap.keys())
            .take(proposals_limit)
            .cloned()
            .collect()
    }

    pub fn get_proposed_txs(
        &self,
        txs_size_limit: usize,
        cycles_limit: Cycle,
    ) -> (Vec<ProposedEntry>, usize, Cycle) {
        let mut size = 0;
        let mut cycles = 0;
        let entries = self
            .proposed
            .txs_iter()
            .take_while(|tx| {
                cycles += tx.cycles;
                size += tx.size;
                (size < txs_size_limit) && (cycles < cycles_limit)
            })
            .cloned()
            .collect();
        (entries, size, cycles)
    }

    pub fn get_tx_from_pool_or_store(
        &self,
        proposal_id: &ProposalShortId,
    ) -> Option<TransactionView> {
        self.get_tx_from_proposed_and_others(proposal_id)
            .or_else(|| {
                self.committed_txs_hash_cache
                    .get(proposal_id)
                    .and_then(|tx_hash| self.snapshot().get_transaction(tx_hash).map(|(tx, _)| tx))
            })
    }
}
