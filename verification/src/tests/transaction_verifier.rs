use super::super::transaction_verifier::{
    CapacityVerifier, DuplicateDepsVerifier, EmptyVerifier, MaturityVerifier, OutputsDataVerifier,
    Since, SinceVerifier, SizeVerifier, VersionVerifier,
};
use crate::error::TransactionError;
use ckb_resource::CODE_HASH_DAO;
use ckb_test_chain_utils::MockMedianTime;
use ckb_traits::BlockMedianTimeContext;
use ckb_types::{
    bytes::Bytes,
    core::{
        capacity_bytes,
        cell::{BlockInfo, CellMetaBuilder, ResolvedOutPoint, ResolvedTransaction},
        BlockNumber, Capacity, ScriptHashType, TransactionBuilder, TransactionView, Version,
        TX_VERSION,
    },
    h256,
    packed::{CellInput, CellOutput, OutPoint, Script},
    prelude::*,
    H256,
};
use std::sync::Arc;

#[test]
pub fn test_empty() {
    let transaction = TransactionBuilder::default().build();
    let verifier = EmptyVerifier::new(&transaction);

    assert_eq!(verifier.verify().err(), Some(TransactionError::Empty));
}

#[test]
pub fn test_version() {
    let transaction = TransactionBuilder::default()
        .version((TX_VERSION + 1).pack())
        .build();
    let verifier = VersionVerifier::new(&transaction);

    assert_eq!(verifier.verify().err(), Some(TransactionError::Version));
}

#[test]
pub fn test_exceeded_maximum_block_bytes() {
    let data: Bytes = vec![1; 500].into();
    let transaction = TransactionBuilder::default()
        .version((Version::default() + 1).pack())
        .output(
            CellOutput::new_builder()
                .data_hash(CellOutput::calc_data_hash(&data).pack())
                .capacity(capacity_bytes!(50).pack())
                .build(),
        )
        .output_data(data.pack())
        .build();
    let verifier = SizeVerifier::new(&transaction, 100);

    assert_eq!(
        verifier.verify().err(),
        Some(TransactionError::ExceededMaximumBlockBytes)
    );
}

#[test]
pub fn test_capacity_outofbound() {
    let data = Bytes::from(vec![1; 51]);
    let transaction = TransactionBuilder::default()
        .output(
            CellOutput::new_builder()
                .data_hash(CellOutput::calc_data_hash(&data).pack())
                .capacity(capacity_bytes!(50).pack())
                .build(),
        )
        .output_data(data.pack())
        .build();

    let rtx = ResolvedTransaction {
        transaction: &transaction,
        resolved_deps: Vec::new(),
        resolved_inputs: vec![ResolvedOutPoint::cell_only(
            CellMetaBuilder::from_cell_output(
                CellOutput::new_builder()
                    .capacity(capacity_bytes!(50).pack())
                    .build(),
                Bytes::new(),
            )
            .build(),
        )],
    };
    let verifier = CapacityVerifier::new(&rtx);

    assert_eq!(
        verifier.verify().err(),
        Some(TransactionError::InsufficientCellCapacity)
    );
}

#[test]
pub fn test_skip_dao_capacity_check() {
    let data = Bytes::from(vec![1; 10]);
    let transaction = TransactionBuilder::default()
        .output(
            CellOutput::new_builder()
                .data_hash(CellOutput::calc_data_hash(&data).pack())
                .capacity(capacity_bytes!(500).pack())
                .type_(
                    Some(
                        Script::new_builder()
                            .code_hash(CODE_HASH_DAO.pack())
                            .hash_type(ScriptHashType::Data.pack())
                            .build(),
                    )
                    .pack(),
                )
                .build(),
        )
        .output_data(Bytes::new().pack())
        .build();

    let rtx = ResolvedTransaction {
        transaction: &transaction,
        resolved_deps: Vec::new(),
        resolved_inputs: vec![],
    };
    let verifier = CapacityVerifier::new(&rtx);

    assert!(verifier.verify().is_ok());
}

// inputs immature verify
#[test]
pub fn test_inputs_cellbase_maturity() {
    let transaction = TransactionBuilder::default().build();
    let output = CellOutput::new_builder()
        .capacity(capacity_bytes!(50).pack())
        .build();

    let rtx = ResolvedTransaction {
        transaction: &transaction,
        resolved_deps: Vec::new(),
        resolved_inputs: vec![ResolvedOutPoint::cell_only(
            CellMetaBuilder::from_cell_output(output.clone(), Bytes::new())
                .block_info(MockMedianTime::get_block_info(30, 0))
                .cellbase(true)
                .build(),
        )],
    };

    let tip_number = 70;
    let cellbase_maturity = 100;
    let verifier = MaturityVerifier::new(&rtx, tip_number, cellbase_maturity);

    assert_eq!(
        verifier.verify().err(),
        Some(TransactionError::CellbaseImmaturity)
    );

    let tip_number = 130;
    let verifier = MaturityVerifier::new(&rtx, tip_number, cellbase_maturity);
    assert!(verifier.verify().is_ok());
}

// deps immature verify
#[test]
pub fn test_deps_cellbase_maturity() {
    let transaction = TransactionBuilder::default().build();
    let output = CellOutput::new_builder()
        .capacity(capacity_bytes!(50).pack())
        .build();

    // The 1st dep is cellbase, the 2nd one is not.
    let rtx = ResolvedTransaction {
        transaction: &transaction,
        resolved_deps: vec![
            ResolvedOutPoint::cell_only(
                CellMetaBuilder::from_cell_output(output.clone(), Bytes::new())
                    .block_info(MockMedianTime::get_block_info(30, 0))
                    .cellbase(true)
                    .build(),
            ),
            ResolvedOutPoint::cell_only(
                CellMetaBuilder::from_cell_output(output.clone(), Bytes::new())
                    .block_info(MockMedianTime::get_block_info(40, 0))
                    .cellbase(false)
                    .build(),
            ),
        ],
        resolved_inputs: Vec::new(),
    };

    let tip_number = 70;
    let cellbase_maturity = 100;
    let verifier = MaturityVerifier::new(&rtx, tip_number, cellbase_maturity);

    assert_eq!(
        verifier.verify().err(),
        Some(TransactionError::CellbaseImmaturity)
    );

    let tip_number = 130;
    let verifier = MaturityVerifier::new(&rtx, tip_number, cellbase_maturity);
    assert!(verifier.verify().is_ok());
}

#[test]
pub fn test_capacity_invalid() {
    // The outputs capacity is 50 + 100 = 150
    let transaction = TransactionBuilder::default()
        .outputs(vec![
            CellOutput::new_builder()
                .capacity(capacity_bytes!(50).pack())
                .build(),
            CellOutput::new_builder()
                .capacity(capacity_bytes!(100).pack())
                .build(),
        ])
        .outputs_data(vec![Bytes::new().pack(); 2])
        .build();

    // The inputs capacity is 49 + 100 = 149,
    // is less than outputs capacity
    let rtx = ResolvedTransaction {
        transaction: &transaction,
        resolved_deps: Vec::new(),
        resolved_inputs: vec![
            ResolvedOutPoint::cell_only(
                CellMetaBuilder::from_cell_output(
                    CellOutput::new_builder()
                        .capacity(capacity_bytes!(49).pack())
                        .build(),
                    Bytes::new(),
                )
                .build(),
            ),
            ResolvedOutPoint::cell_only(
                CellMetaBuilder::from_cell_output(
                    CellOutput::new_builder()
                        .capacity(capacity_bytes!(100).pack())
                        .build(),
                    Bytes::new(),
                )
                .build(),
            ),
        ],
    };
    let verifier = CapacityVerifier::new(&rtx);

    assert_eq!(
        verifier.verify().err(),
        Some(TransactionError::OutputsSumOverflow)
    );
}

#[test]
pub fn test_duplicate_deps() {
    let out_point = OutPoint::new_cell(h256!("0x1"), 0);
    let transaction = TransactionBuilder::default()
        .deps(vec![out_point.clone(), out_point])
        .build();

    let verifier = DuplicateDepsVerifier::new(&transaction);

    assert_eq!(
        verifier.verify().err(),
        Some(TransactionError::DuplicateDeps)
    );
}

fn verify_since<'a, M>(
    rtx: &'a ResolvedTransaction,
    block_median_time_context: &'a M,
    block_number: BlockNumber,
    epoch_number: BlockNumber,
) -> Result<(), TransactionError>
where
    M: BlockMedianTimeContext,
{
    let parent_hash = Arc::new(MockMedianTime::get_block_hash(block_number - 1));
    SinceVerifier::new(
        rtx,
        block_median_time_context,
        block_number,
        epoch_number,
        parent_hash.as_ref().pack(),
    )
    .verify()
}

#[test]
fn test_since() {
    let valids = vec![
        0x0000_0000_0000_0001,
        0x2000_0000_0000_0001,
        0x4000_0000_0000_0001,
        0x8000_0000_0000_0001,
        0xa000_0000_0000_0001,
        0xc000_0000_0000_0001,
    ];

    for v in valids.into_iter() {
        let since = Since(v);
        assert_eq!(since.flags_is_valid(), true);
    }

    let invalids = vec![
        0x0100_0000_0000_0001,
        0x1000_0000_0000_0001,
        0xd000_0000_0000_0001,
    ];

    for v in invalids.into_iter() {
        let since = Since(v);
        assert_eq!(since.flags_is_valid(), false);
    }
}

fn create_tx_with_lock(since: u64) -> TransactionView {
    TransactionBuilder::default()
        .inputs(vec![CellInput::new(
            OutPoint::new_cell(h256!("0x1"), 0),
            since,
        )])
        .build()
}

fn create_resolve_tx_with_block_info(
    tx: &TransactionView,
    block_info: BlockInfo,
) -> ResolvedTransaction<'_> {
    ResolvedTransaction {
        transaction: &tx,
        resolved_deps: Vec::new(),
        resolved_inputs: vec![ResolvedOutPoint::cell_only(
            CellMetaBuilder::from_cell_output(
                CellOutput::new_builder()
                    .capacity(capacity_bytes!(50).pack())
                    .build(),
                Bytes::new(),
            )
            .block_info(block_info)
            .build(),
        )],
    }
}

#[test]
fn test_invalid_since_verify() {
    // use remain flags
    let tx = create_tx_with_lock(0x0100_0000_0000_0001);
    let rtx = create_resolve_tx_with_block_info(&tx, MockMedianTime::get_block_info(1, 0));

    let median_time_context = MockMedianTime::new(vec![0; 11]);
    assert_eq!(
        verify_since(&rtx, &median_time_context, 5, 1).err(),
        Some(TransactionError::InvalidSince)
    );
}

#[test]
pub fn test_absolute_block_number_lock() {
    // absolute lock until block number 0xa
    let tx = create_tx_with_lock(0x0000_0000_0000_000a);
    let rtx = create_resolve_tx_with_block_info(&tx, MockMedianTime::get_block_info(1, 0));
    let median_time_context = MockMedianTime::new(vec![0; 11]);

    assert_eq!(
        verify_since(&rtx, &median_time_context, 5, 1).err(),
        Some(TransactionError::Immature)
    );
    // spent after 10 height
    assert!(verify_since(&rtx, &median_time_context, 10, 1).is_ok());
}

#[test]
pub fn test_absolute_epoch_number_lock() {
    // absolute lock until epoch number 0xa
    let tx = create_tx_with_lock(0x2000_0000_0000_000a);
    let rtx = create_resolve_tx_with_block_info(&tx, MockMedianTime::get_block_info(1, 0));

    let median_time_context = MockMedianTime::new(vec![0; 11]);
    assert_eq!(
        verify_since(&rtx, &median_time_context, 5, 1).err(),
        Some(TransactionError::Immature)
    );
    // spent after 10 epoch
    assert!(verify_since(&rtx, &median_time_context, 100, 10).is_ok());
}

#[test]
pub fn test_relative_timestamp_lock() {
    // relative lock timestamp lock
    let tx = create_tx_with_lock(0xc000_0000_0000_0002);
    let rtx = create_resolve_tx_with_block_info(&tx, MockMedianTime::get_block_info(1, 0));

    let median_time_context = MockMedianTime::new(vec![0; 11]);
    assert_eq!(
        verify_since(&rtx, &median_time_context, 4, 1).err(),
        Some(TransactionError::Immature)
    );

    // spent after 1024 seconds
    // fake median time: 1124
    let median_time_context =
        MockMedianTime::new(vec![0, 100_000, 1_124_000, 2_000_000, 3_000_000]);
    assert!(verify_since(&rtx, &median_time_context, 4, 1).is_ok());
}

#[test]
pub fn test_relative_epoch() {
    // next epoch
    let tx = create_tx_with_lock(0xa000_0000_0000_0001);
    let rtx = create_resolve_tx_with_block_info(&tx, MockMedianTime::get_block_info(1, 1));

    let median_time_context = MockMedianTime::new(vec![0; 11]);

    assert_eq!(
        verify_since(&rtx, &median_time_context, 4, 1).err(),
        Some(TransactionError::Immature)
    );

    assert!(verify_since(&rtx, &median_time_context, 4, 2).is_ok());
}

#[test]
pub fn test_since_both() {
    // both
    let tx = TransactionBuilder::default()
        .inputs(vec![
            // absolute lock until epoch number 0xa
            CellInput::new(OutPoint::new_cell(h256!("0x1"), 0), 0x0000_0000_0000_000a),
            // relative lock until after 2 blocks
            CellInput::new(OutPoint::new_cell(h256!("0x1"), 0), 0xc000_0000_0000_0002),
        ])
        .build();

    let rtx = create_resolve_tx_with_block_info(&tx, MockMedianTime::get_block_info(1, 0));
    // spent after 1024 seconds and 4 blocks (less than 10 blocks)
    // fake median time: 1124
    let median_time_context =
        MockMedianTime::new(vec![0, 100_000, 1_124_000, 2_000_000, 3_000_000]);

    assert_eq!(
        verify_since(&rtx, &median_time_context, 4, 1).err(),
        Some(TransactionError::Immature)
    );
    // spent after 1024 seconds and 10 blocks
    // fake median time: 1124
    let median_time_context = MockMedianTime::new(vec![
        0, 1, 2, 3, 4, 100_000, 1_124_000, 2_000_000, 3_000_000, 4_000_000, 5_000_000, 6_000_000,
    ]);
    assert!(verify_since(&rtx, &median_time_context, 10, 1).is_ok());
}

#[test]
pub fn test_outputs_data_length_mismatch() {
    let transaction = TransactionBuilder::default()
        .output(Default::default())
        .build();
    let verifier = OutputsDataVerifier::new(&transaction);

    assert_eq!(
        verifier.verify().err(),
        Some(TransactionError::OutputsDataLengthMismatch)
    );

    let transaction = TransactionBuilder::default()
        .output(Default::default())
        .output_data(Default::default())
        .build();
    let verifier = OutputsDataVerifier::new(&transaction);

    assert!(verifier.verify().is_ok());
}

#[test]
pub fn test_outputs_data_hash_mismatch() {
    let data: Bytes = Bytes::from(&b"Hello Wrold"[..]);
    let transaction = TransactionBuilder::default()
        .output(Default::default())
        .output_data(data.pack())
        .build();
    let verifier = OutputsDataVerifier::new(&transaction);

    assert_eq!(
        verifier.verify().err(),
        Some(TransactionError::OutputDataHashMismatch)
    );

    let transaction = TransactionBuilder::default()
        .output(
            CellOutput::new_builder()
                .data_hash(CellOutput::calc_data_hash(&data).pack())
                .build(),
        )
        .output_data(data.pack())
        .build();
    let verifier = OutputsDataVerifier::new(&transaction);

    assert!(verifier.verify().is_ok());
}
