use crate::consensus::Consensus;
use ckb_types::{
    core::{
        EpochExt, EpochNumber, EpochNumberWithFraction, HeaderView, Ratio, TransactionView, Version,
    },
    packed::{Byte32, CellbaseWitnessReader},
    prelude::*,
};
use ckb_util::Mutex;
use std::collections::HashMap;

/// What bits to set in version for versionbits blocks
pub const VERSIONBITS_TOP_BITS: Version = 0x20000000;
/// What bitmask determines whether versionbits is in use
pub const VERSIONBITS_TOP_MASK: Version = 0xE0000000;
/// Total bits available for versionbits
pub const VERSIONBITS_NUM_BITS: u32 = 29;

/// RFC0000 defines a finite-state-machine to deploy a soft fork in multiple stages.
/// State transitions happen during epoch if conditions are met
/// In case of reorg, transitions can go backward. Without transition, state is
/// inherited between epochs. All blocks of a epoch share the same state.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ThresholdState {
    DEFINED,
    STARTED,
    LOCKED_IN,
    ACTIVE,
    FAILED,
}

/// This is useful for testing, as it means tests don't need to deal with the activation
/// process. Only tests that specifically test the behaviour during activation cannot use this.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ActiveMode {
    NORMAL,
    ALWAYS,
    NEVER,
}

// Soft fork deployment
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum DeploymentPos {
    TESTDUMMY,
}

/// EpochIndexer
pub trait EpochIndexer {
    fn get_block_epoch_index(&self, block_hash: &Byte32) -> Option<Byte32>;
    fn get_epoch_ext(&self, index: &Byte32) -> Option<EpochExt>;
    fn get_block_header(&self, block_hash: &Byte32) -> Option<HeaderView>;
    fn get_cellbase(&self, block_hash: &Byte32) -> Option<TransactionView>;
}

///Struct for each individual consensus rule change using soft fork.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Deployment {
    bit: u8,
    start: EpochNumber,
    timeout: EpochNumber,
    activation_epoch: EpochNumber,
    active_mode: ActiveMode,
}

type Cache = Mutex<HashMap<Byte32, ThresholdState>>;

/// RFC0000 allows multiple soft forks to be deployed in parallel. We cache
/// per-epoch state for every one of them. */
pub struct VersionBitsCache {
    caches: HashMap<DeploymentPos, Cache>,
}

/// implements RFC0000 threshold logic, and caches results.
pub struct VersionBits<'a> {
    id: DeploymentPos,
    consensus: &'a Consensus,
}

pub trait VersionBitsConditionChecker {
    fn start(&self) -> EpochNumber;
    fn timeout(&self) -> EpochNumber;
    fn active_mode(&self) -> ActiveMode;
    // fn condition(&self, header: &HeaderView) -> bool;
    fn condition<I: EpochIndexer>(&self, header: &HeaderView, indexer: &I) -> bool;
    fn activation_epoch(&self) -> EpochNumber;
    fn threshold(&self) -> Ratio;

    fn get_state<I: EpochIndexer>(
        &self,
        header: &HeaderView,
        cache: &Cache,
        indexer: &I,
    ) -> Option<ThresholdState> {
        let active_mode = self.active_mode();
        let start = self.start();
        let timeout = self.timeout();
        let activation_epoch = self.activation_epoch();

        if active_mode == ActiveMode::ALWAYS {
            return Some(ThresholdState::ACTIVE);
        }

        if active_mode == ActiveMode::NEVER {
            return Some(ThresholdState::FAILED);
        }

        if header.epoch().number() < start {
            return Some(ThresholdState::DEFINED);
        }
        let block_hash = header.hash();
        let mut g_cache = cache.lock();

        let mut to_compute = Vec::new();
        let mut epoch_index = indexer.get_block_epoch_index(&block_hash)?;
        while g_cache.get(&epoch_index).is_none() {
            let epoch_ext = indexer.get_epoch_ext(&epoch_index)?;
            if epoch_ext.is_genesis() {
                // The genesis is by definition defined.
                g_cache.insert(epoch_index.clone(), ThresholdState::DEFINED);
                break;
            }
            if epoch_ext.number() < start {
                // The genesis is by definition defined.
                g_cache.insert(epoch_index.clone(), ThresholdState::DEFINED);
                break;
            }
            to_compute.push(epoch_ext.clone());
            let last_block_header_in_previous_epoch =
                indexer.get_block_header(&epoch_ext.last_block_hash_in_previous_epoch())?;
            let previous_epoch_index =
                indexer.get_block_epoch_index(&last_block_header_in_previous_epoch.hash())?;
            epoch_index = previous_epoch_index;
        }

        let mut state = *g_cache
            .get(&epoch_index)
            .expect("cache[epoch_index] is known");
        while let Some(epoch_ext) = to_compute.pop() {
            let mut next_state = state;

            match state {
                ThresholdState::DEFINED => {
                    if epoch_ext.number() >= start {
                        next_state = ThresholdState::STARTED;
                    }
                }
                ThresholdState::STARTED => {
                    // We need to count
                    let mut count = 0;
                    let mut header =
                        indexer.get_block_header(&epoch_ext.last_block_hash_in_previous_epoch())?;
                    let epoch_length = epoch_ext.length();

                    for _ in 0..epoch_length - 1 {
                        if self.condition(&header, indexer) {
                            count += 1;
                        }
                        header = indexer.get_block_header(&header.parent_hash())?;
                    }
                    let threshold_number = threshold_number(epoch_length, self.threshold())?;
                    if count >= threshold_number {
                        next_state = ThresholdState::LOCKED_IN;
                    } else if epoch_ext.number() >= timeout {
                        next_state = ThresholdState::FAILED;
                    }
                }
                ThresholdState::LOCKED_IN => {
                    if epoch_ext.number() >= activation_epoch {
                        next_state = ThresholdState::ACTIVE;
                    }
                }
                ThresholdState::FAILED | ThresholdState::ACTIVE => {
                    // Nothing happens, these are terminal states.
                }
            }
            state = next_state;
            g_cache.insert(epoch_ext.last_block_hash_in_previous_epoch(), state);
        }

        Some(state)
    }
}

impl<'a> VersionBits<'a> {
    fn deployment(&self) -> &Deployment {
        &self.consensus.deployments[&self.id]
    }

    fn mask(&self) -> u32 {
        1u32 << self.deployment().bit as u32
    }
}

impl<'a> VersionBitsConditionChecker for VersionBits<'a> {
    fn start(&self) -> EpochNumber {
        self.deployment().start
    }

    fn timeout(&self) -> EpochNumber {
        self.deployment().timeout
    }

    fn condition<I: EpochIndexer>(&self, header: &HeaderView, indexer: &I) -> bool {
        if let Some(cellbase) = indexer.get_cellbase(&header.hash()) {
            if let Some(witness) = cellbase.witnesses().get(0) {
                if let Some(reader) = CellbaseWitnessReader::from_slice(&witness.raw_data()).ok() {
                    let message = reader.message().to_entity();
                    if message.len() >= 4 {
                        if let Ok(raw) = message.as_slice()[..4].try_into() {
                            let version = u32::from_le_bytes(raw);
                            return ((version & VERSIONBITS_TOP_MASK) == VERSIONBITS_TOP_BITS)
                                && (version & self.mask()) != 0;
                        }
                    }
                }
            }
        }
        false
    }

    // fn condition(&self, header: &HeaderView) -> bool {
    //     let version = header.version();
    //     (((version & VERSIONBITS_TOP_MASK) == VERSIONBITS_TOP_BITS) && (version & self.mask()) != 0)
    // }

    fn activation_epoch(&self) -> EpochNumber {
        self.deployment().activation_epoch
    }

    fn active_mode(&self) -> ActiveMode {
        self.deployment().active_mode
    }

    fn threshold(&self) -> Ratio {
        self.consensus.soft_fork_activation_threshold
    }
}

fn threshold_number(length: u64, threshold: Ratio) -> Option<u64> {
    length
        .checked_mul(threshold.numer())
        .and_then(|ret| ret.checked_div(threshold.denom()))
}
