use crate::SyncSharedState;
use ckb_chain::chain::{ChainController, ChainService};

use ckb_dao::DaoCalculator;
use ckb_merkle_mountain_range::leaf_index_to_mmr_size;
use ckb_notify::NotifyService;
use ckb_shared::{
    shared::{Shared, SharedBuilder},
    Snapshot,
};
use ckb_store::ChainStore;
use ckb_test_chain_utils::{always_success_cellbase, always_success_consensus};
use ckb_types::prelude::*;
use ckb_types::{
    core::{cell::resolve_transaction, BlockBuilder, BlockNumber, TransactionView},
    packed::Byte32,
    utilities::ChainRootMMR,
};
use std::collections::HashSet;
use std::sync::Arc;

pub fn build_chain(tip: BlockNumber) -> (SyncSharedState, ChainController) {
    let (shared, table) = SharedBuilder::default()
        .consensus(always_success_consensus())
        .build()
        .unwrap();
    let chain_controller = {
        let notify_controller = NotifyService::default().start::<&str>(None);
        let chain_service = ChainService::new(shared.clone(), table, notify_controller);
        chain_service.start::<&str>(None)
    };
    generate_blocks(&shared, &chain_controller, tip);
    let sync_shared_state = SyncSharedState::new(shared);
    (sync_shared_state, chain_controller)
}

pub fn generate_blocks(
    shared: &Shared,
    chain_controller: &ChainController,
    target_tip: BlockNumber,
) {
    let snapshot = shared.snapshot();
    let parent_number = snapshot.tip_number();
    let mut parent_hash = snapshot.tip_header().hash().clone();
    for number in parent_number..target_tip {
        println!("generate_blocks number {}", number);
        let block = inherit_block(shared, &parent_hash).build();
        parent_hash = block.header().hash().to_owned();
        chain_controller
            .process_block(Arc::new(block), false)
            .expect("processing block should be ok");
    }

}

pub fn inherit_block(shared: &Shared, parent_hash: &Byte32) -> BlockBuilder {
    let snapshot = shared.snapshot();
    println!("get_block snapshot tip_header hash {}", snapshot.get_tip_header().map(|header| header.hash()).unwrap());
    println!("get_block snapshot parent_hash {:?} block {:?}", parent_hash, snapshot.get_block(parent_hash).map(|block| block.header().hash()));
    // if snapshot.get_block(parent_hash).is_none() {
    //     println!("get_block snapshot tip_header {}", snapshot.get_tip_header().map(|header| header.hash()).unwrap());
    //     // println!("get_block store tip_header {:?}", shared.store().get_tip_header().map(|header| header.hash()).unwrap());

    //     // println!("get_block snapshot block {:?}", snapshot.get_block(parent_hash).map(|block| block.number()));
    //     // println!("get_block store block {:?}", shared.store().get_block(parent_hash).map(|block| block.number()));
    //     // println!("get_block snapshot block {:?}", snapshot.get_block(parent_hash).map(|block| block.number()));
    // }
    let parent = snapshot.get_block(parent_hash).expect("parent block");
    // if snapshot.get_block_epoch(parent_hash).is_none() {
    //     println!("get_block_epoch snapshot tip_header {}", snapshot.get_tip_header().map(|header| header.hash()).unwrap());
    //     println!("get_block_epoch store tip_header {}", shared.store().get_tip_header().map(|header| header.hash()).unwrap());

    //     println!("get_block_epoch snapshot {:?}", snapshot.get_block_epoch(parent_hash));
    //     println!("get_block_epoch store {:?}", shared.store().get_block_epoch(parent_hash));
    // }

    let parent_epoch = snapshot.get_block_epoch(parent_hash).expect("parent epoch");
    let parent_number = parent.header().number();
    let epoch = snapshot
        .next_epoch_ext(shared.consensus(), &parent_epoch, &parent.header())
        .unwrap_or(parent_epoch);
    let cellbase = {
        let (_, reward) = snapshot.finalize_block_reward(&parent.header()).unwrap();
        always_success_cellbase(parent_number + 1, reward.total)
    };
    let chain_root = ChainRootMMR::new(
        leaf_index_to_mmr_size(parent_number),
        snapshot.as_ref(),
    )
    .get_root()
    .unwrap()
    .hash();
    let dao = {
        let resolved_cellbase =
            resolve_transaction(&cellbase, &mut HashSet::new(), snapshot.as_ref(), snapshot.as_ref()).unwrap();
        DaoCalculator::new(shared.consensus(), snapshot.as_ref())
            .dao_field(&[resolved_cellbase], &parent.header())
            .unwrap()
    };

    BlockBuilder::default()
        .parent_hash(parent_hash.to_owned())
        .number((parent.header().number() + 1).pack())
        .timestamp((parent.header().timestamp() + 1).pack())
        .epoch(epoch.number().pack())
        .difficulty(epoch.difficulty().pack())
        .dao(dao)
        .chain_root(chain_root)
        .transaction(inherit_cellbase(snapshot.as_ref(), parent_number))
}

pub fn inherit_cellbase(snapshot: &Snapshot, parent_number: BlockNumber) -> TransactionView {
    let parent_header = {
        let parent_hash = snapshot
            .get_block_hash(parent_number)
            .expect("parent exist");
        snapshot
            .get_block_header(&parent_hash)
            .expect("parent exist")
    };
    let (_, reward) = snapshot.finalize_block_reward(&parent_header).unwrap();
    always_success_cellbase(parent_number + 1, reward.total)
}
