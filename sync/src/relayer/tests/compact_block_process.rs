use crate::relayer::compact_block::CompactBlock;
use crate::relayer::compact_block_process::{CompactBlockProcess, Status};
use crate::relayer::error::{Error, Ignored, Internal, Misbehavior};
use ckb_core::block::BlockBuilder;
use ckb_core::header::HeaderBuilder;
use ckb_core::transaction::{CellOutputBuilder, TransactionBuilder};
use ckb_core::{Bytes, Capacity};
use ckb_network::PeerIndex;
use ckb_protocol::{get_root, CompactBlock as FbsCompactBlock, RelayMessage, SyncMessage};
use flatbuffers::FlatBufferBuilder;
use std::collections::HashSet;

use crate::block_status::BlockStatus;
use crate::relayer::tests::helper::{build_chain, new_header_builder, MockProtocalContext};
use crate::types::InflightBlocks;
use crate::NetworkProtocol;
use crate::MAX_PEERS_PER_BLOCK;
use ckb_core::transaction::ProposalShortId;
use faketime::unix_time_as_millis;
use fnv::FnvHashMap;
use std::cell::RefCell;
use std::convert::TryInto;
use std::iter::FromIterator;
use std::sync::Arc;

#[test]
fn test_in_block_status_map() {
    let (relayer, _) = build_chain(5);

    let block = BlockBuilder::default()
        .header(
            HeaderBuilder::default()
                .number(5)
                .timestamp(unix_time_as_millis())
                .build(),
        )
        .transaction(TransactionBuilder::default().build())
        .build();
    let builder = &mut FlatBufferBuilder::new();
    let mut prefilled_transactions_indexes = HashSet::new();
    prefilled_transactions_indexes.insert(0);
    let b = FbsCompactBlock::build(builder, &block, &prefilled_transactions_indexes);
    builder.finish(b, None);

    let fbs_compact_block = get_root::<FbsCompactBlock>(builder.finished_data()).unwrap();

    let mock_protocal_context = MockProtocalContext::default();
    let nc = Arc::new(mock_protocal_context);
    let peer_index: PeerIndex = 1.into();

    let compact_block_process = CompactBlockProcess::new(
        &fbs_compact_block,
        &relayer,
        Arc::<MockProtocalContext>::clone(&nc),
        peer_index,
    );

    // BLOCK_INVALID in block_status_map
    {
        relayer
            .shared
            .insert_block_status(block.header().hash().to_owned(), BlockStatus::BLOCK_INVALID);
    }

    let r = compact_block_process.execute();
    assert_eq!(
        r.unwrap_err().downcast::<Error>().unwrap(),
        Error::Misbehavior(Misbehavior::BlockInvalid)
    );

    let compact_block_process = CompactBlockProcess::new(
        &fbs_compact_block,
        &relayer,
        Arc::<MockProtocalContext>::clone(&nc),
        peer_index,
    );

    // BLOCK_STORED in block_status_map
    {
        relayer
            .shared
            .insert_block_status(block.header().hash().clone(), BlockStatus::BLOCK_STORED);
    }

    let r = compact_block_process.execute();
    assert_eq!(
        r.unwrap_err().downcast::<Error>().unwrap(),
        Error::Ignored(Ignored::AlreadyStored)
    );
}

// send_getheaders_to_peer when UnknownParent
#[test]
fn test_unknow_parent() {
    let (relayer, _) = build_chain(5);

    // UnknownParent
    let block = BlockBuilder::default()
        .header(
            HeaderBuilder::default()
                .number(5)
                .timestamp(unix_time_as_millis())
                .build(),
        )
        .transaction(TransactionBuilder::default().build())
        .build();
    let builder = &mut FlatBufferBuilder::new();
    let mut prefilled_transactions_indexes = HashSet::new();
    prefilled_transactions_indexes.insert(0);
    let b = FbsCompactBlock::build(builder, &block, &prefilled_transactions_indexes);
    builder.finish(b, None);

    let fbs_compact_block = get_root::<FbsCompactBlock>(builder.finished_data()).unwrap();

    let mock_protocal_context = MockProtocalContext::default();
    let nc = Arc::new(mock_protocal_context);
    let peer_index: PeerIndex = 1.into();

    let compact_block_process = CompactBlockProcess::new(
        &fbs_compact_block,
        &relayer,
        Arc::<MockProtocalContext>::clone(&nc),
        peer_index,
    );

    let r = compact_block_process.execute();
    assert_eq!(r.ok(), Some(Status::UnknownParent));

    let snapshot = relayer.shared.snapshot();
    let header = snapshot.tip_header();
    let locator_hash = relayer.shared.get_locator(header);
    let fbb = &mut FlatBufferBuilder::new();
    let message = SyncMessage::build_get_headers(fbb, &locator_hash);
    fbb.finish(message, None);

    // send_getheaders_to_peer
    assert_eq!(
        nc.as_ref().sent_messages,
        RefCell::new(vec![(
            NetworkProtocol::SYNC.into(),
            peer_index,
            fbb.finished_data().into()
        )])
    );
}

#[test]
fn test_accept_not_a_better_block() {
    let (relayer, _) = build_chain(5);
    let header = {
        let snapshot = relayer.shared.snapshot();
        snapshot.tip_header().clone()
    };

    // The timestamp is random, so it may be not a better block.
    let not_sure_a_better_header = HeaderBuilder::from_header(header.clone())
        .timestamp(header.timestamp() + 1)
        .build();

    let block = BlockBuilder::default()
        .header(not_sure_a_better_header)
        .transaction(TransactionBuilder::default().build())
        .build();

    let builder = &mut FlatBufferBuilder::new();
    let mut prefilled_transactions_indexes = HashSet::new();
    prefilled_transactions_indexes.insert(0);
    let b = FbsCompactBlock::build(builder, &block, &prefilled_transactions_indexes);
    builder.finish(b, None);

    let fbs_compact_block = get_root::<FbsCompactBlock>(builder.finished_data()).unwrap();

    let mock_protocal_context = MockProtocalContext::default();
    let nc = Arc::new(mock_protocal_context);
    let peer_index: PeerIndex = 1.into();

    let compact_block_process = CompactBlockProcess::new(
        &fbs_compact_block,
        &relayer,
        Arc::<MockProtocalContext>::clone(&nc),
        peer_index,
    );

    let r = compact_block_process.execute();
    assert_eq!(r.ok(), Some(Status::AcceptBlock));
}

#[test]
fn test_already_in_flight() {
    let (relayer, _) = build_chain(5);
    let parent = {
        let snapshot = relayer.shared.snapshot();
        snapshot.tip_header().clone()
    };

    // Better block
    let header = new_header_builder(relayer.shared.shared(), &parent).build();

    // Better block
    let block = BlockBuilder::default()
        .header(header)
        .transaction(TransactionBuilder::default().build())
        .build();

    let builder = &mut FlatBufferBuilder::new();
    let mut prefilled_transactions_indexes = HashSet::new();
    prefilled_transactions_indexes.insert(0);
    let b = FbsCompactBlock::build(builder, &block, &prefilled_transactions_indexes);
    builder.finish(b, None);

    let fbs_compact_block = get_root::<FbsCompactBlock>(builder.finished_data()).unwrap();

    let mock_protocal_context = MockProtocalContext::default();
    let nc = Arc::new(mock_protocal_context);
    let peer_index: PeerIndex = 1.into();

    // Already in flight
    let mut in_flight_blocks = InflightBlocks::default();
    in_flight_blocks.insert(peer_index, block.header().hash().clone());
    *relayer.shared.write_inflight_blocks() = in_flight_blocks;

    let compact_block_process = CompactBlockProcess::new(
        &fbs_compact_block,
        &relayer,
        Arc::<MockProtocalContext>::clone(&nc),
        peer_index,
    );

    let r = compact_block_process.execute();
    assert_eq!(
        r.unwrap_err().downcast::<Error>().unwrap(),
        Error::Ignored(Ignored::AlreadyInFlight)
    );
}

#[test]
fn test_already_pending() {
    let (relayer, _) = build_chain(5);
    let parent = {
        let snapshot = relayer.shared.snapshot();
        snapshot.tip_header().clone()
    };

    // Better block
    let header = new_header_builder(relayer.shared.shared(), &parent).build();

    let block = BlockBuilder::default()
        .header(header)
        .transaction(TransactionBuilder::default().build())
        .build();

    let builder = &mut FlatBufferBuilder::new();
    let mut prefilled_transactions_indexes = HashSet::new();
    prefilled_transactions_indexes.insert(0);
    let b = FbsCompactBlock::build(builder, &block, &prefilled_transactions_indexes);
    builder.finish(b, None);

    let fbs_compact_block = get_root::<FbsCompactBlock>(builder.finished_data()).unwrap();

    let mock_protocal_context = MockProtocalContext::default();
    let nc = Arc::new(mock_protocal_context);
    let peer_index: PeerIndex = 1.into();

    // Already in pending
    {
        let compact_block: CompactBlock = fbs_compact_block.clone().try_into().unwrap();
        let mut pending_compact_blocks = relayer.shared.pending_compact_blocks();
        pending_compact_blocks.insert(
            compact_block.header.hash().clone(),
            (
                compact_block,
                FnvHashMap::from_iter(vec![(1.into(), vec![0])]),
            ),
        );
    }

    let compact_block_process = CompactBlockProcess::new(
        &fbs_compact_block,
        &relayer,
        Arc::<MockProtocalContext>::clone(&nc),
        peer_index,
    );

    let r = compact_block_process.execute();
    assert_eq!(
        r.unwrap_err().downcast::<Error>().unwrap(),
        Error::Ignored(Ignored::AlreadyPending)
    );
}

#[test]
fn test_header_invalid() {
    let (relayer, _) = build_chain(5);
    let parent = {
        let snapshot = relayer.shared.snapshot();
        snapshot.tip_header().clone()
    };

    // Better block but block number is invalid
    let header = new_header_builder(relayer.shared.shared(), &parent)
        .number(4)
        .build();

    let block = BlockBuilder::default()
        .header(header)
        .transaction(TransactionBuilder::default().build())
        .build();

    let builder = &mut FlatBufferBuilder::new();
    let mut prefilled_transactions_indexes = HashSet::new();
    prefilled_transactions_indexes.insert(0);
    let b = FbsCompactBlock::build(builder, &block, &prefilled_transactions_indexes);
    builder.finish(b, None);

    let fbs_compact_block = get_root::<FbsCompactBlock>(builder.finished_data()).unwrap();

    let mock_protocal_context = MockProtocalContext::default();
    let nc = Arc::new(mock_protocal_context);
    let peer_index: PeerIndex = 1.into();

    let compact_block_process = CompactBlockProcess::new(
        &fbs_compact_block,
        &relayer,
        Arc::<MockProtocalContext>::clone(&nc),
        peer_index,
    );

    let r = compact_block_process.execute();
    assert_eq!(
        r.unwrap_err().downcast::<Error>().unwrap(),
        Error::Misbehavior(Misbehavior::HeaderInvalid)
    );
    // Assert block_status_map update
    assert_eq!(
        relayer.shared().get_block_status(block.header().hash()),
        BlockStatus::BLOCK_INVALID
    );
}

#[test]
fn test_inflight_blocks_reach_limit() {
    let (relayer, _) = build_chain(5);
    let parent = {
        let snapshot = relayer.shared.snapshot();
        snapshot.tip_header().clone()
    };

    let header = new_header_builder(relayer.shared.shared(), &parent).build();

    // Better block including one missing transaction
    let block = BlockBuilder::default()
        .header(header.clone())
        .transaction(TransactionBuilder::default().build())
        .transaction(
            TransactionBuilder::default()
                .output(
                    CellOutputBuilder::default()
                        .capacity(Capacity::bytes(1).unwrap())
                        .build(),
                )
                .output_data(Bytes::new())
                .build(),
        )
        .build();

    let builder = &mut FlatBufferBuilder::new();
    let mut prefilled_transactions_indexes = HashSet::new();
    prefilled_transactions_indexes.insert(0);
    let b = FbsCompactBlock::build(builder, &block, &prefilled_transactions_indexes);
    builder.finish(b, None);

    let fbs_compact_block = get_root::<FbsCompactBlock>(builder.finished_data()).unwrap();

    let mock_protocal_context = MockProtocalContext::default();
    let nc = Arc::new(mock_protocal_context);
    let peer_index: PeerIndex = 100.into();

    // in_flight_blocks is full
    {
        let mut in_flight_blocks = InflightBlocks::default();
        for i in 0..=MAX_PEERS_PER_BLOCK {
            in_flight_blocks.insert(i.into(), block.header().hash().clone());
        }
        *relayer.shared.write_inflight_blocks() = in_flight_blocks;
    }

    let compact_block_process = CompactBlockProcess::new(
        &fbs_compact_block,
        &relayer,
        Arc::<MockProtocalContext>::clone(&nc),
        peer_index,
    );

    let r = compact_block_process.execute();
    assert_eq!(
        r.unwrap_err().downcast::<Error>().unwrap(),
        Error::Internal(Internal::InflightBlocksReachLimit)
    );
}

#[test]
fn test_send_missing_indexes() {
    let (relayer, _) = build_chain(5);
    let parent = {
        let snapshot = relayer.shared.snapshot();
        snapshot.tip_header().clone()
    };

    let header = new_header_builder(relayer.shared.shared(), &parent).build();

    let proposal_id = ProposalShortId::new([1u8; 10]);

    // Better block including one missing transaction
    let block = BlockBuilder::default()
        .header(header.clone())
        .transaction(TransactionBuilder::default().build())
        .transaction(
            TransactionBuilder::default()
                .output(
                    CellOutputBuilder::default()
                        .capacity(Capacity::bytes(1).unwrap())
                        .build(),
                )
                .output_data(Bytes::new())
                .build(),
        )
        .proposal(proposal_id)
        .build();

    let builder = &mut FlatBufferBuilder::new();
    let mut prefilled_transactions_indexes = HashSet::new();
    prefilled_transactions_indexes.insert(0);
    let b = FbsCompactBlock::build(builder, &block, &prefilled_transactions_indexes);
    builder.finish(b, None);

    let fbs_compact_block = get_root::<FbsCompactBlock>(builder.finished_data()).unwrap();

    let mock_protocal_context = MockProtocalContext::default();
    let nc = Arc::new(mock_protocal_context);
    let peer_index: PeerIndex = 100.into();

    let compact_block_process = CompactBlockProcess::new(
        &fbs_compact_block,
        &relayer,
        Arc::<MockProtocalContext>::clone(&nc),
        peer_index,
    );

    assert!(!relayer.shared.inflight_proposals().contains(&proposal_id));

    let r = compact_block_process.execute();
    assert_eq!(r.ok(), Some(Status::SendMissingIndexes));

    let fbb = &mut FlatBufferBuilder::new();
    let message = RelayMessage::build_get_block_transactions(fbb, &block.header().hash(), &[1u32]);
    fbb.finish(message, None);

    // send missing indexes messages
    assert!(nc
        .as_ref()
        .sent_messages_to
        .borrow()
        .contains(&(peer_index, fbb.finished_data().into())));

    // insert inflight proposal
    assert!(relayer.shared.inflight_proposals().contains(&proposal_id));

    let fbb = &mut FlatBufferBuilder::new();
    let message =
        RelayMessage::build_get_block_proposal(fbb, block.header().hash(), &[proposal_id]);
    fbb.finish(message, None);

    // send proposal request
    assert!(nc
        .as_ref()
        .sent_messages_to
        .borrow()
        .contains(&(peer_index, fbb.finished_data().into())));
}

#[test]
fn test_accept_block() {
    let (relayer, _) = build_chain(5);
    let parent = {
        let snapshot = relayer.shared.snapshot();
        snapshot.tip_header().clone()
    };

    let header = new_header_builder(relayer.shared.shared(), &parent).build();

    // Better block without missing txs
    let block = BlockBuilder::default()
        .header(header.clone())
        .transaction(TransactionBuilder::default().build())
        .build();

    let builder = &mut FlatBufferBuilder::new();
    let mut prefilled_transactions_indexes = HashSet::new();
    prefilled_transactions_indexes.insert(0);
    let b = FbsCompactBlock::build(builder, &block, &prefilled_transactions_indexes);
    builder.finish(b, None);

    let fbs_compact_block = get_root::<FbsCompactBlock>(builder.finished_data()).unwrap();

    let mock_protocal_context = MockProtocalContext::default();
    let nc = Arc::new(mock_protocal_context);
    let peer_index: PeerIndex = 100.into();

    let compact_block_process = CompactBlockProcess::new(
        &fbs_compact_block,
        &relayer,
        Arc::<MockProtocalContext>::clone(&nc),
        peer_index,
    );

    let r = compact_block_process.execute();
    assert_eq!(r.ok(), Some(Status::AcceptBlock));
}

#[test]
fn test_ignore_a_too_old_block() {
    let (relayer, _) = build_chain(1804);
    let parent = {
        let snapshot = relayer.shared.snapshot();
        snapshot.tip_header().clone()
    };
    let parent = relayer.shared.get_ancestor(parent.hash(), 2).unwrap();

    let too_old_block = new_header_builder(relayer.shared.shared(), &parent).build();

    let block = BlockBuilder::default()
        .header(too_old_block)
        .transaction(TransactionBuilder::default().build())
        .build();

    let builder = &mut FlatBufferBuilder::new();
    let mut prefilled_transactions_indexes = HashSet::new();
    prefilled_transactions_indexes.insert(0);
    let b = FbsCompactBlock::build(builder, &block, &prefilled_transactions_indexes);
    builder.finish(b, None);

    let fbs_compact_block = get_root::<FbsCompactBlock>(builder.finished_data()).unwrap();

    let mock_protocal_context = MockProtocalContext::default();
    let nc = Arc::new(mock_protocal_context);
    let peer_index: PeerIndex = 1.into();

    let compact_block_process = CompactBlockProcess::new(
        &fbs_compact_block,
        &relayer,
        Arc::<MockProtocalContext>::clone(&nc),
        peer_index,
    );

    let r = compact_block_process.execute();
    assert_eq!(
        r.unwrap_err().downcast::<Error>().unwrap(),
        Error::Ignored(Ignored::TooOldBlock)
    );
}
