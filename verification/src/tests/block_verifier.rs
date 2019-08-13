use super::super::block_verifier::{
    BlockBytesVerifier, BlockProposalsLimitVerifier, CellbaseVerifier, DuplicateVerifier,
    MerkleRootVerifier,
};
use super::super::error::{CellbaseError, Error as VerifyError};
use ckb_types::{
    bytes::Bytes,
    core::{
        capacity_bytes, BlockBuilder, BlockNumber, Capacity, HeaderBuilder, TransactionBuilder,
        TransactionView,
    },
    h256,
    packed::{CellInput, CellOutputBuilder, OutPoint, ProposalShortId, Script},
    prelude::*,
    H256,
};

fn create_cellbase_transaction_with_block_number(number: BlockNumber) -> TransactionView {
    TransactionBuilder::default()
        .input(CellInput::new_cellbase_input(number))
        .output(
            CellOutputBuilder::default()
                .capacity(capacity_bytes!(100).pack())
                .build(),
        )
        .output_data(Bytes::new().pack())
        .witness(Script::default().into_witness())
        .build()
}

fn create_cellbase_transaction_with_capacity(capacity: Capacity) -> TransactionView {
    TransactionBuilder::default()
        .input(CellInput::new_cellbase_input(0))
        .output(
            CellOutputBuilder::default()
                .capacity(capacity.pack())
                .build(),
        )
        .output_data(Bytes::new().pack())
        .witness(Script::default().into_witness())
        .build()
}

fn create_cellbase_transaction() -> TransactionView {
    create_cellbase_transaction_with_capacity(capacity_bytes!(100))
}

fn create_normal_transaction() -> TransactionView {
    TransactionBuilder::default()
        .input(CellInput::new(OutPoint::new_cell(h256!("0x1"), 0), 0))
        .output(
            CellOutputBuilder::default()
                .capacity(capacity_bytes!(100).pack())
                .build(),
        )
        .output_data(Bytes::new().pack())
        .build()
}

#[test]
pub fn test_block_without_cellbase() {
    let block = BlockBuilder::new()
        .header(HeaderBuilder::default().number(1u64.pack()).build())
        .transaction(TransactionBuilder::default().build())
        .build();
    let verifier = CellbaseVerifier::new();
    assert_eq!(
        verifier.verify(&block),
        Err(VerifyError::Cellbase(CellbaseError::InvalidQuantity))
    );
}

#[test]
pub fn test_block_with_one_cellbase_at_first() {
    let transaction = create_normal_transaction();

    let block = BlockBuilder::new()
        .header(HeaderBuilder::default().number(1u64.pack()).build())
        .transaction(create_cellbase_transaction_with_block_number(1))
        .transaction(transaction)
        .build();

    let verifier = CellbaseVerifier::new();
    assert!(verifier.verify(&block).is_ok());
}

#[test]
pub fn test_block_with_correct_cellbase_number() {
    let block = BlockBuilder::new()
        .header(HeaderBuilder::default().number(2u64.pack()).build())
        .transaction(create_cellbase_transaction_with_block_number(2))
        .build();

    let verifier = CellbaseVerifier::new();
    assert!(verifier.verify(&block).is_ok());
}

#[test]
pub fn test_block_with_incorrect_cellbase_number() {
    let block = BlockBuilder::new()
        .header(HeaderBuilder::default().number(2u64.pack()).build())
        .transaction(create_cellbase_transaction_with_block_number(3))
        .build();

    let verifier = CellbaseVerifier::new();
    assert_eq!(
        verifier.verify(&block),
        Err(VerifyError::Cellbase(CellbaseError::InvalidInput))
    );
}

#[test]
pub fn test_block_with_one_cellbase_at_last() {
    let block = BlockBuilder::new()
        .header(HeaderBuilder::default().number(2u64.pack()).build())
        .transaction(create_normal_transaction())
        .transaction(create_cellbase_transaction())
        .build();

    let verifier = CellbaseVerifier::new();
    assert_eq!(
        verifier.verify(&block),
        Err(VerifyError::Cellbase(CellbaseError::InvalidPosition))
    );
}

#[test]
pub fn test_block_with_duplicated_txs() {
    let tx = create_normal_transaction();
    let block = BlockBuilder::new()
        .header(HeaderBuilder::default().number(2u64.pack()).build())
        .transaction(tx.clone())
        .transaction(tx)
        .build();

    let verifier = DuplicateVerifier::new();
    assert_eq!(
        verifier.verify(&block),
        Err(VerifyError::CommitTransactionDuplicate)
    );
}

#[test]
pub fn test_block_with_duplicated_proposals() {
    let block = BlockBuilder::new()
        .header(HeaderBuilder::default().number(2u64.pack()).build())
        .proposal(ProposalShortId::zero())
        .proposal(ProposalShortId::zero())
        .build();

    let verifier = DuplicateVerifier::new();
    assert_eq!(
        verifier.verify(&block),
        Err(VerifyError::ProposalTransactionDuplicate)
    );
}

#[test]
pub fn test_transaction_root() {
    let header = HeaderBuilder::default()
        .number(2u64.pack())
        .transactions_root(H256::zero().pack())
        .build();
    let block = BlockBuilder::new()
        .header(header)
        .transaction(create_normal_transaction())
        .build_unchecked();

    let verifier = MerkleRootVerifier::new();
    assert_eq!(
        verifier.verify(&block),
        Err(VerifyError::CommitTransactionsRoot)
    );
}

#[test]
pub fn test_proposals_root() {
    let header = HeaderBuilder::default()
        .number(2u64.pack())
        .proposals_hash(h256!("0x1").pack())
        .build();
    let block = BlockBuilder::new()
        .header(header)
        .transaction(create_normal_transaction())
        .build_unchecked();

    let verifier = MerkleRootVerifier::new();
    assert_eq!(
        verifier.verify(&block),
        Err(VerifyError::CommitTransactionsRoot)
    );
}

#[test]
pub fn test_witnesses_root() {
    let header = HeaderBuilder::default()
        .number(2u64.pack())
        .witnesses_root(h256!("0x1").pack())
        .build();
    let block = BlockBuilder::new()
        .header(header)
        .proposal(ProposalShortId::zero())
        .build_unchecked();

    let verifier = MerkleRootVerifier::new();
    assert_eq!(
        verifier.verify(&block),
        Err(VerifyError::WitnessesMerkleRoot)
    );
}

#[test]
pub fn test_block_with_two_cellbases() {
    let block = BlockBuilder::new()
        .header(HeaderBuilder::default().number(2u64.pack()).build())
        .transaction(create_cellbase_transaction())
        .transaction(create_cellbase_transaction())
        .build();

    let verifier = CellbaseVerifier::new();
    assert_eq!(
        verifier.verify(&block),
        Err(VerifyError::Cellbase(CellbaseError::InvalidQuantity))
    );
}

#[test]
pub fn test_cellbase_with_less_reward() {
    let transaction = create_normal_transaction();

    let block = BlockBuilder::default()
        .transaction(create_cellbase_transaction_with_capacity(capacity_bytes!(
            50
        )))
        .transaction(transaction)
        .build();

    let verifier = CellbaseVerifier::new();
    assert!(verifier.verify(&block).is_ok());
}

#[test]
pub fn test_cellbase_with_fee() {
    let transaction = create_normal_transaction();

    let block = BlockBuilder::default()
        .transaction(create_cellbase_transaction_with_capacity(capacity_bytes!(
            110
        )))
        .transaction(transaction)
        .build();

    let verifier = CellbaseVerifier::new();
    assert!(verifier.verify(&block).is_ok());
}

#[test]
pub fn test_max_block_bytes_verifier_skip_genesis() {
    let block = BlockBuilder::default().build();

    {
        let verifier = BlockBytesVerifier::new(block.serialized_size() as u64);
        assert_eq!(verifier.verify(&block), Ok(()));
    }

    {
        let verifier = BlockBytesVerifier::new(block.serialized_size() as u64 - 1);
        assert_eq!(verifier.verify(&block), Ok(()));
    }
}

#[test]
pub fn test_max_block_bytes_verifier() {
    let block = BlockBuilder::new()
        .header(HeaderBuilder::default().number(2u64.pack()).build())
        .build();

    {
        let verifier = BlockBytesVerifier::new(block.serialized_size() as u64);
        assert_eq!(verifier.verify(&block), Ok(()));
    }

    {
        let verifier = BlockBytesVerifier::new(block.serialized_size() as u64 - 1);
        assert_eq!(
            verifier.verify(&block),
            Err(VerifyError::ExceededMaximumBlockBytes)
        );
    }
}

#[test]
pub fn test_max_proposals_limit_verifier() {
    let block = BlockBuilder::default()
        .proposal(ProposalShortId::zero())
        .build();

    {
        let verifier = BlockProposalsLimitVerifier::new(1);
        assert_eq!(verifier.verify(&block), Ok(()));
    }

    {
        let verifier = BlockProposalsLimitVerifier::new(0);
        assert_eq!(
            verifier.verify(&block),
            Err(VerifyError::ExceededMaximumProposalsLimit)
        );
    }
}
