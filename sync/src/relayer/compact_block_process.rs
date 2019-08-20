use crate::block_status::BlockStatus;
use crate::relayer::compact_block_verifier::CompactBlockVerifier;
use crate::relayer::error::{Error, Ignored, Internal, Misbehavior};
use crate::relayer::Relayer;
use ckb_logger::debug_target;
use ckb_network::{CKBProtocolContext, PeerIndex};
use ckb_shared::Snapshot;
use ckb_store::ChainStore;
use ckb_traits::BlockMedianTimeContext;
use ckb_types::{
    core::{self, BlockNumber},
    packed,
    prelude::*,
};
use ckb_verification::{HeaderResolver, HeaderVerifier, Verifier};
use failure::Error as FailureError;
use fnv::FnvHashMap;
use std::sync::Arc;

pub struct CompactBlockProcess<'a> {
    message: packed::CompactBlockReader<'a>,
    relayer: &'a Relayer,
    nc: Arc<dyn CKBProtocolContext>,
    peer: PeerIndex,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Status {
    // Accept block
    AcceptBlock,
    // Send get_headers
    UnknownParent,
    // Send missing_indexes by get_block_transactions
    SendMissingIndexes,
}

impl<'a> CompactBlockProcess<'a> {
    pub fn new(
        message: packed::CompactBlockReader<'a>,
        relayer: &'a Relayer,
        nc: Arc<dyn CKBProtocolContext>,
        peer: PeerIndex,
    ) -> Self {
        CompactBlockProcess {
            message,
            nc,
            relayer,
            peer,
        }
    }

    pub fn execute(self) -> Result<Status, FailureError> {
        let compact_block = self.message.to_entity();
        let header = compact_block.header().into_view();
        let block_hash = header.hash();

        // Only accept blocks with a height greater than tip - N
        // where N is the current epoch length
        let snapshot: &Snapshot = &self.relayer.shared.snapshot();
        let tip = snapshot.tip_header().clone();
        let epoch_length = snapshot.epoch_ext().length();
        let lowest_number = tip.number().saturating_sub(epoch_length);

        if lowest_number > header.number() {
            return Err(Error::Ignored(Ignored::TooOldBlock).into());
        }

        let status = self.relayer.shared().get_block_status(&block_hash);
        if status.contains(BlockStatus::BLOCK_STORED) {
            return Err(Error::Ignored(Ignored::AlreadyStored).into());
        } else if status.contains(BlockStatus::BLOCK_INVALID) {
            debug_target!(
                crate::LOG_TARGET_RELAY,
                "receive a compact block with invalid status, {}, peer: {}",
                block_hash,
                self.peer
            );
            return Err(Error::Misbehavior(Misbehavior::BlockInvalid).into());
        }

        let parent = self
            .relayer
            .shared
            .get_header_view(&header.data().raw().parent_hash());
        if parent.is_none() {
            debug_target!(
                crate::LOG_TARGET_RELAY,
                "UnknownParent: {}, send_getheaders_to_peer({})",
                block_hash,
                self.peer
            );
            self.relayer
                .shared
                .send_getheaders_to_peer(self.nc.as_ref(), self.peer, &tip);
            return Ok(Status::UnknownParent);
        }

        let parent = parent.unwrap();

        if let Some(flight) = self
            .relayer
            .shared()
            .read_inflight_blocks()
            .inflight_state_by_block(&block_hash)
        {
            if flight.peers.contains(&self.peer) {
                debug_target!(
                    crate::LOG_TARGET_RELAY,
                    "discard already in-flight compact block {}",
                    block_hash,
                );
                return Err(Error::Ignored(Ignored::AlreadyInFlight).into());
            }
        }

        // The new arrived has greater difficulty than local best known chain
        let missing_indexes: Vec<u32>;
        {
            // Verify compact block
            let mut pending_compact_blocks = self.relayer.shared().pending_compact_blocks();
            if pending_compact_blocks
                .get(&block_hash)
                .map(|(_, peers_map)| peers_map.contains_key(&self.peer))
                .unwrap_or(false)
            {
                debug_target!(
                    crate::LOG_TARGET_RELAY,
                    "discard already pending compact block {}",
                    block_hash
                );
                return Err(Error::Ignored(Ignored::AlreadyPending).into());
            } else {
                let fn_get_pending_header = {
                    |block_hash| {
                        pending_compact_blocks
                            .get(&block_hash)
                            .map(|(compact_block, _)| compact_block.header().into_view())
                            .or_else(|| {
                                self.relayer
                                    .shared
                                    .get_header_view(&block_hash)
                                    .map(|header_view| header_view.into_inner())
                            })
                    }
                };
                let resolver = self
                    .relayer
                    .shared
                    .new_header_resolver(&header, parent.into_inner());
                let median_time_context = CompactBlockMedianTimeView {
                    fn_get_pending_header: Box::new(fn_get_pending_header),
                    snapshot,
                };
                let header_verifier = HeaderVerifier::new(
                    &median_time_context,
                    Arc::clone(&self.relayer.shared.consensus().pow_engine()),
                );
                if let Err(err) = header_verifier.verify(&resolver) {
                    debug_target!(crate::LOG_TARGET_RELAY, "invalid header: {}", err);
                    self.relayer
                        .shared()
                        .insert_block_status(block_hash, BlockStatus::BLOCK_INVALID);
                    return Err(Error::Misbehavior(Misbehavior::HeaderInvalid).into());
                }
                CompactBlockVerifier::verify(&compact_block)?;

                // Header has been verified ok, update state
                let epoch = resolver.epoch().expect("epoch verified").clone();
                self.relayer
                    .shared()
                    .insert_valid_header(self.peer, &header, epoch);
            }

            // Reconstruct block
            let ret = {
                self.relayer
                    .request_proposal_txs(self.nc.as_ref(), self.peer, &compact_block);
                self.relayer.reconstruct_block(&compact_block, Vec::new())
            };

            // Accept block
            // `relayer.accept_block` will make sure the validity of block before persisting
            // into database
            match ret {
                Ok(block) => {
                    pending_compact_blocks.remove(&block_hash);
                    self.relayer
                        .accept_block(self.nc.as_ref(), self.peer, block);
                    return Ok(Status::AcceptBlock);
                }
                Err(missing) => {
                    missing_indexes = missing.into_iter().map(|i| i as u32).collect::<Vec<_>>();

                    assert!(!missing_indexes.is_empty());

                    pending_compact_blocks
                        .entry(block_hash.clone())
                        .or_insert_with(|| (compact_block, FnvHashMap::default()))
                        .1
                        .insert(self.peer, missing_indexes.clone());
                }
            }
        }

        if !self
            .relayer
            .shared()
            .write_inflight_blocks()
            .insert(self.peer, block_hash.clone())
        {
            debug_target!(
                crate::LOG_TARGET_RELAY,
                "BlockInFlight reach limit or had requested, peer: {}, block: {}",
                self.peer,
                block_hash,
            );
            return Err(Error::Internal(Internal::InflightBlocksReachLimit).into());
        }

        let content = packed::GetBlockTransactions::new_builder()
            .block_hash(block_hash)
            .indexes(missing_indexes.pack())
            .build();
        let message = packed::RelayMessage::new_builder().set(content).build();
        let data = message.as_slice().into();
        if let Err(err) = self.nc.send_message_to(self.peer, data) {
            ckb_logger::debug!("relayer send get_block_transactions error: {:?}", err);
        }

        Ok(Status::SendMissingIndexes)
    }
}

struct CompactBlockMedianTimeView<'a> {
    fn_get_pending_header: Box<Fn(packed::Byte32) -> Option<core::HeaderView> + 'a>,
    snapshot: &'a Snapshot,
}

impl<'a> CompactBlockMedianTimeView<'a> {
    fn get_header(&self, hash: &packed::Byte32) -> Option<core::HeaderView> {
        (self.fn_get_pending_header)(hash.to_owned())
            .or_else(|| self.snapshot.get_block_header(hash))
    }
}

impl<'a> BlockMedianTimeContext for CompactBlockMedianTimeView<'a> {
    fn median_block_count(&self) -> u64 {
        self.snapshot.consensus().median_time_block_count() as u64
    }

    fn timestamp_and_parent(
        &self,
        block_hash: &packed::Byte32,
    ) -> (u64, BlockNumber, packed::Byte32) {
        let header = self
            .get_header(&block_hash)
            .expect("[CompactBlockMedianTimeView] blocks used for median time exist");
        (
            header.timestamp(),
            header.number(),
            header.data().raw().parent_hash(),
        )
    }
}
