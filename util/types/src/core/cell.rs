use crate::{
    bytes::Bytes,
    core::{BlockNumber, BlockView, Capacity, EpochNumber, HeaderView, TransactionView},
    packed::{Byte32, CellOutPoint, CellOutput, OutPoint},
    prelude::*,
    H256,
};
use bit_vec::BitVec;
use ckb_occupied_capacity::Result as CapacityResult;
use std::collections::{HashMap, HashSet};
use std::convert::AsRef;
use std::convert::TryInto;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct BlockInfo {
    pub number: BlockNumber,
    pub epoch: EpochNumber,
    pub hash: H256,
}

impl BlockInfo {
    pub fn new(number: BlockNumber, epoch: EpochNumber, hash: H256) -> Self {
        BlockInfo {
            number,
            epoch,
            hash,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Default)]
pub struct CellMeta {
    pub cell_output: CellOutput,
    pub out_point: CellOutPoint,
    pub block_info: Option<BlockInfo>,
    pub cellbase: bool,
    pub data_bytes: u64,
    /// In memory cell data
    /// A live cell either exists in memory or DB
    /// must check DB if this field is None
    pub mem_cell_data: Option<Bytes>,
}

#[derive(Default)]
pub struct CellMetaBuilder {
    cell_output: CellOutput,
    out_point: CellOutPoint,
    block_info: Option<BlockInfo>,
    cellbase: bool,
    data_bytes: u64,
    mem_cell_data: Option<Bytes>,
}

impl CellMetaBuilder {
    pub fn from_cell_meta(cell_meta: CellMeta) -> Self {
        let CellMeta {
            cell_output,
            out_point,
            block_info,
            cellbase,
            data_bytes,
            mem_cell_data,
        } = cell_meta;
        Self {
            cell_output,
            out_point,
            block_info,
            cellbase,
            data_bytes,
            mem_cell_data,
        }
    }

    pub fn from_cell_output(cell_output: CellOutput, data: Bytes) -> Self {
        let mut builder = CellMetaBuilder::default();
        builder.cell_output = cell_output;
        builder.data_bytes = data.len().try_into().expect("u32");
        builder.mem_cell_data = Some(data);
        builder
    }

    pub fn out_point(mut self, out_point: CellOutPoint) -> Self {
        self.out_point = out_point;
        self
    }

    pub fn block_info(mut self, block_info: BlockInfo) -> Self {
        self.block_info = Some(block_info);
        self
    }

    pub fn cellbase(mut self, cellbase: bool) -> Self {
        self.cellbase = cellbase;
        self
    }

    pub fn build(self) -> CellMeta {
        let Self {
            cell_output,
            out_point,
            block_info,
            cellbase,
            data_bytes,
            mem_cell_data,
        } = self;
        CellMeta {
            cell_output,
            out_point,
            block_info,
            cellbase,
            data_bytes,
            mem_cell_data,
        }
    }
}

impl fmt::Debug for CellMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CellMeta")
            .field("cell_output", &self.cell_output)
            .field("out_point", &self.out_point)
            .field("block_info", &self.block_info)
            .field("cellbase", &self.cellbase)
            .field("data_bytes", &self.data_bytes)
            .finish()
    }
}

impl CellMeta {
    pub fn is_cellbase(&self) -> bool {
        self.cellbase
    }

    pub fn capacity(&self) -> Capacity {
        self.cell_output.capacity().unpack()
    }

    pub fn data_hash(&self) -> H256 {
        self.cell_output.data_hash().unpack()
    }

    pub fn occupied_capacity(&self) -> CapacityResult<Capacity> {
        self.cell_output
            .occupied_capacity(Capacity::bytes(self.data_bytes as usize)?)
    }

    pub fn is_lack_of_capacity(&self) -> CapacityResult<bool> {
        self.cell_output
            .is_lack_of_capacity(Capacity::bytes(self.data_bytes as usize)?)
    }
}

#[derive(PartialEq, Debug)]
pub enum CellStatus {
    /// Cell exists and has not been spent.
    Live(Box<CellMeta>),
    /// Cell exists and has been spent.
    Dead,
    /// Cell does not exist.
    Unknown,
    /// OutPoint doesn't contain reference to a cell.
    Unspecified,
}

impl CellStatus {
    pub fn live_cell(cell_meta: CellMeta) -> CellStatus {
        CellStatus::Live(Box::new(cell_meta))
    }

    pub fn is_live(&self) -> bool {
        match *self {
            CellStatus::Live(_) => true,
            _ => false,
        }
    }

    pub fn is_dead(&self) -> bool {
        self == &CellStatus::Dead
    }

    pub fn is_unknown(&self) -> bool {
        self == &CellStatus::Unknown
    }

    pub fn is_unspecified(&self) -> bool {
        self == &CellStatus::Unspecified
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum HeaderStatus {
    /// Header exists on current chain
    Live(Box<HeaderView>),
    /// Header exists, but the specified block doesn't contain referenced transaction.
    InclusionFaliure,
    /// Header does not exist on current chain
    Unknown,
    /// OutPoint doesn't contain reference to a header.
    Unspecified,
}

impl HeaderStatus {
    pub fn live_header(header: HeaderView) -> HeaderStatus {
        HeaderStatus::Live(Box::new(header))
    }

    pub fn is_live(&self) -> bool {
        match *self {
            HeaderStatus::Live(_) => true,
            _ => false,
        }
    }

    pub fn is_inclusion_failure(&self) -> bool {
        self == &HeaderStatus::InclusionFaliure
    }

    pub fn is_unknown(&self) -> bool {
        self == &HeaderStatus::Unknown
    }

    pub fn is_unspecified(&self) -> bool {
        self == &HeaderStatus::Unspecified
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct TransactionMeta {
    pub(crate) block_number: u64,
    pub(crate) epoch_number: u64,
    pub(crate) block_hash: H256,
    pub(crate) cellbase: bool,
    /// each bits indicate if transaction has dead cells
    pub(crate) dead_cell: BitVec,
}

impl TransactionMeta {
    pub fn new(
        block_number: u64,
        epoch_number: u64,
        block_hash: H256,
        outputs_count: usize,
        all_dead: bool,
    ) -> TransactionMeta {
        TransactionMeta {
            block_number,
            epoch_number,
            block_hash,
            cellbase: false,
            dead_cell: BitVec::from_elem(outputs_count, all_dead),
        }
    }

    /// New cellbase transaction
    pub fn new_cellbase(
        block_number: u64,
        epoch_number: u64,
        block_hash: H256,
        outputs_count: usize,
        all_dead: bool,
    ) -> Self {
        let mut result = Self::new(
            block_number,
            epoch_number,
            block_hash,
            outputs_count,
            all_dead,
        );
        result.cellbase = true;
        result
    }

    /// Returns true if it is a cellbase transaction
    pub fn is_cellbase(&self) -> bool {
        self.cellbase
    }

    /// Returns transaction outputs count
    pub fn len(&self) -> usize {
        self.dead_cell.len()
    }

    pub fn block_number(&self) -> u64 {
        self.block_number
    }

    pub fn epoch_number(&self) -> u64 {
        self.epoch_number
    }

    pub fn block_hash(&self) -> &H256 {
        &self.block_hash
    }

    pub fn is_empty(&self) -> bool {
        self.dead_cell.is_empty()
    }

    pub fn is_dead(&self, index: usize) -> Option<bool> {
        self.dead_cell.get(index)
    }

    pub fn all_dead(&self) -> bool {
        self.dead_cell.all()
    }

    pub fn set_dead(&mut self, index: usize) {
        if index < self.len() {
            self.dead_cell.set(index, true);
        }
    }

    pub fn unset_dead(&mut self, index: usize) {
        if index < self.len() {
            self.dead_cell.set(index, false);
        }
    }
}

#[derive(Default)]
pub struct TransactionMetaBuilder {
    block_number: u64,
    epoch_number: u64,
    block_hash: H256,
    cellbase: bool,
    bits: Vec<u8>,
    len: usize,
}

impl TransactionMetaBuilder {
    pub fn block_number(mut self, block_number: u64) -> Self {
        self.block_number = block_number;
        self
    }

    pub fn epoch_number(mut self, epoch_number: u64) -> Self {
        self.epoch_number = epoch_number;
        self
    }

    pub fn block_hash(mut self, block_hash: H256) -> Self {
        self.block_hash = block_hash;
        self
    }

    pub fn cellbase(mut self, cellbase: bool) -> Self {
        self.cellbase = cellbase;
        self
    }

    pub fn bits(mut self, bits: Vec<u8>) -> Self {
        self.bits = bits;
        self
    }

    pub fn len(mut self, len: usize) -> Self {
        self.len = len;
        self
    }

    pub fn build(self) -> TransactionMeta {
        let TransactionMetaBuilder {
            block_number,
            epoch_number,
            block_hash,
            cellbase,
            bits,
            len,
        } = self;
        let mut dead_cell = BitVec::from_bytes(&bits);
        dead_cell.truncate(len);
        TransactionMeta {
            block_number,
            epoch_number,
            block_hash,
            cellbase,
            dead_cell,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedOutPoint {
    pub cell: Option<Box<CellMeta>>,
    pub header: Option<Box<HeaderView>>,
}

impl ResolvedOutPoint {
    pub fn cell_only(cell: CellMeta) -> ResolvedOutPoint {
        ResolvedOutPoint {
            cell: Some(Box::new(cell)),
            header: None,
        }
    }

    pub fn header_only(header: HeaderView) -> ResolvedOutPoint {
        ResolvedOutPoint {
            cell: None,
            header: Some(Box::new(header)),
        }
    }

    pub fn cell_and_header(cell: CellMeta, header: HeaderView) -> ResolvedOutPoint {
        ResolvedOutPoint {
            cell: Some(Box::new(cell)),
            header: Some(Box::new(header)),
        }
    }

    pub fn cell(&self) -> Option<&CellMeta> {
        self.cell.as_ref().map(AsRef::as_ref)
    }

    pub fn header(&self) -> Option<&HeaderView> {
        self.header.as_ref().map(AsRef::as_ref)
    }
}

/// Transaction with resolved input cells.
#[derive(Debug)]
pub struct ResolvedTransaction<'a> {
    pub transaction: &'a TransactionView,
    pub resolved_deps: Vec<ResolvedOutPoint>,
    pub resolved_inputs: Vec<ResolvedOutPoint>,
}

pub trait CellProvider {
    fn cell(&self, out_point: &OutPoint) -> CellStatus;
}

pub struct OverlayCellProvider<'a, A, B> {
    overlay: &'a A,
    cell_provider: &'a B,
}

impl<'a, A, B> OverlayCellProvider<'a, A, B>
where
    A: CellProvider,
    B: CellProvider,
{
    pub fn new(overlay: &'a A, cell_provider: &'a B) -> Self {
        Self {
            overlay,
            cell_provider,
        }
    }
}

impl<'a, A, B> CellProvider for OverlayCellProvider<'a, A, B>
where
    A: CellProvider,
    B: CellProvider,
{
    fn cell(&self, out_point: &OutPoint) -> CellStatus {
        match self.overlay.cell(out_point) {
            CellStatus::Live(cell_meta) => CellStatus::Live(cell_meta),
            CellStatus::Dead => CellStatus::Dead,
            CellStatus::Unknown => self.cell_provider.cell(out_point),
            CellStatus::Unspecified => CellStatus::Unspecified,
        }
    }
}

pub struct BlockCellProvider<'a> {
    output_indices: HashMap<Byte32, usize>,
    block: &'a BlockView,
}

// Transactions are expected to be sorted within a block,
// Transactions have to appear after any transactions upon which they depend
impl<'a> BlockCellProvider<'a> {
    pub fn new(block: &'a BlockView) -> Result<Self, UnresolvableError> {
        let output_indices: HashMap<Byte32, usize> = block
            .transactions()
            .iter()
            .enumerate()
            .map(|(idx, tx)| (tx.hash(), idx))
            .collect();

        for (idx, tx) in block.transactions().iter().enumerate() {
            for dep in tx.deps_iter() {
                if let Some(output_idx) = dep
                    .cell()
                    .to_opt()
                    .and_then(|cell| output_indices.get(&cell.tx_hash()))
                {
                    if *output_idx >= idx {
                        return Err(UnresolvableError::OutOfOrder(dep.clone()));
                    }
                }
            }
            for input_pt in tx.input_pts_iter() {
                if let Some(output_idx) = input_pt
                    .cell()
                    .to_opt()
                    .and_then(|cell| output_indices.get(&cell.tx_hash()))
                {
                    if *output_idx >= idx {
                        return Err(UnresolvableError::OutOfOrder(input_pt.clone()));
                    }
                }
            }
        }

        Ok(Self {
            output_indices,
            block,
        })
    }
}

impl<'a> CellProvider for BlockCellProvider<'a> {
    fn cell(&self, out_point: &OutPoint) -> CellStatus {
        if out_point.cell().is_none() {
            return CellStatus::Unspecified;
        }
        let out_point = out_point.cell().to_opt().should_be_ok();

        self.output_indices
            .get(&out_point.tx_hash())
            .and_then(|i| {
                let transaction = self.block.transaction(*i).should_be_ok();
                let j: usize = out_point.index().unpack();
                self.block.output(*i, j).map(|output| {
                    let data = transaction
                        .data()
                        .outputs_data()
                        .get(j)
                        .expect("must exists")
                        .raw_data();
                    let header = self.block.data().header().raw();
                    CellStatus::live_cell(CellMeta {
                        cell_output: output,
                        out_point: out_point,
                        block_info: Some(BlockInfo {
                            number: header.number().unpack(),
                            epoch: header.epoch().unpack(),
                            hash: self.block.hash().unpack(),
                        }),
                        cellbase: *i == 0,
                        data_bytes: data.len() as u64,
                        mem_cell_data: Some(data),
                    })
                })
            })
            .unwrap_or_else(|| CellStatus::Unknown)
    }
}

pub struct TransactionsProvider {
    transactions: HashMap<Byte32, TransactionView>,
}

impl TransactionsProvider {
    pub fn new(transactions: &[TransactionView]) -> Self {
        let transactions = transactions
            .iter()
            .map(|tx| (tx.hash(), tx.to_owned()))
            .collect();
        Self { transactions }
    }
}

impl CellProvider for TransactionsProvider {
    fn cell(&self, out_point: &OutPoint) -> CellStatus {
        if let Some(cell_out_point) = &out_point.cell().to_opt() {
            match self.transactions.get(&cell_out_point.tx_hash()) {
                Some(tx) => tx
                    .outputs()
                    .get(cell_out_point.index().unpack())
                    .map(|cell| {
                        let data = tx
                            .outputs_data()
                            .get(cell_out_point.index().unpack())
                            .expect("output data")
                            .raw_data();

                        CellStatus::live_cell(CellMetaBuilder::from_cell_output(cell, data).build())
                    })
                    .unwrap_or(CellStatus::Unknown),
                None => CellStatus::Unknown,
            }
        } else {
            CellStatus::Unspecified
        }
    }
}

pub trait HeaderProvider {
    fn header(&self, out_point: &OutPoint) -> HeaderStatus;
}

#[derive(Default)]
pub struct BlockHeadersProvider {
    attached_indices: HashMap<Byte32, HeaderView>,
    attached_transaction_blocks: HashMap<Byte32, Byte32>,
    detached_indices: HashMap<Byte32, HeaderView>,
}

impl BlockHeadersProvider {
    pub fn push_attached(&mut self, block: &BlockView) {
        self.attached_indices.insert(block.hash(), block.header());
        for tx_hash in block.tx_hashes().into_iter() {
            self.attached_transaction_blocks
                .insert(tx_hash, block.hash());
        }
    }

    pub fn push_detached(&mut self, block: &BlockView) {
        self.detached_indices.insert(block.hash(), block.header());
    }

    #[cfg(test)]
    pub fn insert_attached_transaction_block(&mut self, tx_hash: H256, header_hash: H256) {
        self.attached_transaction_blocks
            .insert(tx_hash.pack(), header_hash.pack());
    }
}

impl HeaderProvider for BlockHeadersProvider {
    fn header(&self, out_point: &OutPoint) -> HeaderStatus {
        if let Some(block_hash) = &out_point.block_hash().to_opt() {
            if self.detached_indices.contains_key(&block_hash) {
                return HeaderStatus::Unknown;
            }
            match self.attached_indices.get(&block_hash) {
                Some(header) => {
                    if let Some(cell_out_point) = &out_point.cell().to_opt() {
                        self.attached_transaction_blocks
                            .get(&cell_out_point.tx_hash())
                            .map_or(HeaderStatus::InclusionFaliure, |tx_block_hash| {
                                if *tx_block_hash == *block_hash {
                                    HeaderStatus::live_header((*header).clone())
                                } else {
                                    HeaderStatus::InclusionFaliure
                                }
                            })
                    } else {
                        HeaderStatus::live_header((*header).clone())
                    }
                }
                None => HeaderStatus::Unknown,
            }
        } else {
            HeaderStatus::Unspecified
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UnresolvableError {
    // OutPoint is empty
    Empty,
    // OutPoint is used as input, but a cell is not specified
    UnspecifiedInputCell(OutPoint),
    // OutPoint specifies an invalid header, this could be due to either
    // of the following 2 reasons:
    // 1. Specified header doesn't exist on chain.
    // 2. OutPoint specifies both header and cell, but the specified cell
    // is not included in the specified block header.
    InvalidHeader(OutPoint),
    Dead(OutPoint),
    Unknown(Vec<OutPoint>),
    OutOfOrder(OutPoint),
}

pub fn resolve_transaction<'a, CP: CellProvider, HP: HeaderProvider>(
    transaction: &'a TransactionView,
    seen_inputs: &mut HashSet<OutPoint>,
    cell_provider: &CP,
    header_provider: &HP,
) -> Result<ResolvedTransaction<'a>, UnresolvableError> {
    let (mut unknown_out_points, mut resolved_inputs, mut resolved_deps) = (
        Vec::new(),
        Vec::with_capacity(transaction.inputs().len()),
        Vec::with_capacity(transaction.deps().len()),
    );
    let mut current_inputs: HashSet<OutPoint> = HashSet::default();

    // skip resolve input of cellbase
    if !transaction.is_cellbase() {
        for out_point in transaction.input_pts_iter() {
            if seen_inputs.contains(&out_point) {
                return Err(UnresolvableError::Dead(out_point.to_owned()));
            }

            let (cell_status, header_status) = if current_inputs.insert(out_point.to_owned()) {
                (
                    cell_provider.cell(&out_point),
                    header_provider.header(&out_point),
                )
            } else {
                (CellStatus::Dead, HeaderStatus::Unknown)
            };

            match (cell_status, header_status) {
                (CellStatus::Dead, _) => {
                    return Err(UnresolvableError::Dead(out_point.clone()));
                }
                (CellStatus::Unknown, _) => {
                    unknown_out_points.push(out_point.clone());
                }
                // Input cell must exist
                (CellStatus::Unspecified, _) => {
                    return Err(UnresolvableError::UnspecifiedInputCell(out_point.clone()));
                }
                (_, HeaderStatus::Unknown) => {
                    // TODO: should we change transaction pool so transactions
                    // with unknown header can be included as orphans, waiting
                    // for the correct block header to enable it?
                    return Err(UnresolvableError::InvalidHeader(out_point.clone()));
                }
                (_, HeaderStatus::InclusionFaliure) => {
                    return Err(UnresolvableError::InvalidHeader(out_point.clone()));
                }

                (CellStatus::Live(cell_meta), HeaderStatus::Live(header)) => {
                    resolved_inputs.push(ResolvedOutPoint::cell_and_header(*cell_meta, *header));
                }
                (CellStatus::Live(cell_meta), HeaderStatus::Unspecified) => {
                    resolved_inputs.push(ResolvedOutPoint::cell_only(*cell_meta));
                }
            }
        }
    }

    for out_point in transaction.deps_iter() {
        let cell_status = cell_provider.cell(&out_point);
        let header_status = header_provider.header(&out_point);

        match (cell_status, header_status) {
            (CellStatus::Dead, _) => {
                return Err(UnresolvableError::Dead(out_point.clone()));
            }
            (CellStatus::Unknown, _) => {
                unknown_out_points.push(out_point.clone());
            }
            (_, HeaderStatus::Unknown) => {
                // TODO: should we change transaction pool so transactions
                // with unknown header can be included as orphans, waiting
                // for the correct block header to enable it?
                return Err(UnresolvableError::InvalidHeader(out_point.clone()));
            }
            (_, HeaderStatus::InclusionFaliure) => {
                return Err(UnresolvableError::InvalidHeader(out_point.clone()));
            }
            (CellStatus::Live(_), _) if seen_inputs.contains(&out_point) => {
                return Err(UnresolvableError::Dead(out_point.clone()));
            }
            (CellStatus::Live(cell_meta), HeaderStatus::Live(header)) => {
                resolved_deps.push(ResolvedOutPoint::cell_and_header(*cell_meta, *header));
            }
            (CellStatus::Live(cell_meta), HeaderStatus::Unspecified) => {
                resolved_deps.push(ResolvedOutPoint::cell_only(*cell_meta));
            }
            (CellStatus::Unspecified, HeaderStatus::Live(header)) => {
                resolved_deps.push(ResolvedOutPoint::header_only(*header));
            }
            (CellStatus::Unspecified, HeaderStatus::Unspecified) => {
                return Err(UnresolvableError::Empty);
            }
        }
    }

    if !unknown_out_points.is_empty() {
        Err(UnresolvableError::Unknown(unknown_out_points))
    } else {
        seen_inputs.extend(current_inputs);
        Ok(ResolvedTransaction {
            transaction,
            resolved_inputs,
            resolved_deps,
        })
    }
}

impl<'a> ResolvedTransaction<'a> {
    // cellbase will be resolved with empty input cells, we can use low cost check here:
    pub fn is_cellbase(&self) -> bool {
        self.resolved_inputs.is_empty()
    }

    pub fn inputs_capacity(&self) -> CapacityResult<Capacity> {
        self.resolved_inputs
            .iter()
            .map(|o| o.cell().map_or_else(Capacity::zero, CellMeta::capacity))
            .try_fold(Capacity::zero(), Capacity::safe_add)
    }

    pub fn outputs_capacity(&self) -> CapacityResult<Capacity> {
        self.transaction.outputs_capacity()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::{capacity_bytes, BlockBuilder, BlockView, Capacity, TransactionBuilder},
        h256,
        packed::CellInput,
        H256,
    };
    use std::collections::HashMap;

    #[test]
    fn set_unset_dead_out_of_bounds() {
        let mut meta = TransactionMeta::new(0, 0, H256::zero(), 4, false);
        meta.set_dead(3);
        assert!(meta.is_dead(3) == Some(true));
        meta.unset_dead(3);
        assert!(meta.is_dead(3) == Some(false));
        // none-op
        meta.set_dead(4);
        assert!(meta.is_dead(4) == None);
        meta.unset_dead(4);
        assert!(meta.is_dead(4) == None);
    }

    #[derive(Default)]
    struct CellMemoryDb {
        cells: HashMap<CellOutPoint, Option<CellMeta>>,
    }
    impl CellProvider for CellMemoryDb {
        fn cell(&self, o: &OutPoint) -> CellStatus {
            if o.cell().is_none() {
                return CellStatus::Unspecified;
            }

            match self.cells.get(&o.cell().to_opt().should_be_ok()) {
                Some(&Some(ref cell_meta)) => CellStatus::live_cell(cell_meta.clone()),
                Some(&None) => CellStatus::Dead,
                None => CellStatus::Unknown,
            }
        }
    }

    fn generate_dummy_cell_meta() -> CellMeta {
        let data = Bytes::default();
        let cell_output = CellOutput::new_builder()
            .capacity(capacity_bytes!(2).pack())
            .data_hash(CellOutput::calc_data_hash(&data).pack())
            .build();
        let out_point = CellOutPoint::default();
        CellMeta {
            block_info: Some(BlockInfo {
                number: 1,
                epoch: 1,
                hash: H256::zero(),
            }),
            cell_output,
            out_point,
            cellbase: false,
            data_bytes: data.len() as u64,
            mem_cell_data: Some(data),
        }
    }

    fn generate_block(txs: Vec<TransactionView>) -> BlockView {
        BlockBuilder::new().transactions(txs).build()
    }

    fn generate_transaction_from_dep(out_point: OutPoint) -> TransactionView {
        TransactionBuilder::new().dep(out_point).build()
    }

    fn generate_transaction_from_input(input: CellInput) -> TransactionView {
        TransactionBuilder::new().input(input).build()
    }

    #[test]
    fn cell_provider_trait_works() {
        let mut db = CellMemoryDb::default();

        let p1 = OutPoint::new_cell(H256::zero(), 1);
        let p2 = OutPoint::new_cell(H256::zero(), 2);
        let p3 = OutPoint::new_cell(H256::zero(), 3);
        let o = generate_dummy_cell_meta();

        db.cells
            .insert(p1.cell().to_opt().should_be_ok(), Some(o.clone()));
        db.cells.insert(p2.cell().to_opt().should_be_ok(), None);

        assert_eq!(CellStatus::Live(Box::new(o)), db.cell(&p1));
        assert_eq!(CellStatus::Dead, db.cell(&p2));
        assert_eq!(CellStatus::Unknown, db.cell(&p3));
    }

    #[test]
    fn resolve_transaction_should_resolve_header_only_out_point() {
        let cell_provider = CellMemoryDb::default();
        let mut header_provider = BlockHeadersProvider::default();

        let block = generate_block(vec![]);
        let header_hash = block.hash();

        header_provider.push_attached(&block);

        let out_point = OutPoint::new_block_hash(header_hash.unpack());
        let transaction = generate_transaction_from_dep(out_point);

        let mut seen_inputs = HashSet::default();
        let result = resolve_transaction(
            &transaction,
            &mut seen_inputs,
            &cell_provider,
            &header_provider,
        )
        .unwrap();

        assert!(result.resolved_deps[0].cell().is_none());
        assert_eq!(
            result.resolved_deps[0].header,
            Some(Box::new(block.header().clone()))
        );
    }

    #[test]
    fn resolve_transaction_should_reject_input_without_cells() {
        let cell_provider = CellMemoryDb::default();
        let mut header_provider = BlockHeadersProvider::default();

        let block = generate_block(vec![]);
        let header_hash = block.hash();

        header_provider.push_attached(&block);

        let out_point = OutPoint::new_block_hash(header_hash.unpack());
        let input = CellInput::new(out_point.clone(), 0);
        let transaction = generate_transaction_from_input(input);

        let mut seen_inputs = HashSet::default();
        let result = resolve_transaction(
            &transaction,
            &mut seen_inputs,
            &cell_provider,
            &header_provider,
        );

        assert_eq!(
            result.err(),
            Some(UnresolvableError::UnspecifiedInputCell(out_point))
        );
    }

    #[test]
    fn resolve_transaction_should_resolve_both_header_and_cell() {
        let mut cell_provider = CellMemoryDb::default();
        let mut header_provider = BlockHeadersProvider::default();

        let block = generate_block(vec![]);
        let header_hash = block.hash();
        let out_point = OutPoint::new(header_hash.unpack(), h256!("0x2"), 3);

        cell_provider.cells.insert(
            out_point.cell().to_opt().should_be_ok(),
            Some(generate_dummy_cell_meta()),
        );
        header_provider.push_attached(&block);
        header_provider.insert_attached_transaction_block(
            out_point.cell().to_opt().should_be_ok().tx_hash().unpack(),
            header_hash.unpack(),
        );

        let transaction = generate_transaction_from_dep(out_point);

        let mut seen_inputs = HashSet::default();
        let result = resolve_transaction(
            &transaction,
            &mut seen_inputs,
            &cell_provider,
            &header_provider,
        )
        .unwrap();

        assert!(result.resolved_deps[0].cell().is_some());
        assert_eq!(
            result.resolved_deps[0].header,
            Some(Box::new(block.header()))
        );
    }

    #[test]
    fn resolve_transaction_should_test_header_includes_cell() {
        let mut cell_provider = CellMemoryDb::default();
        let mut header_provider = BlockHeadersProvider::default();

        let block = generate_block(vec![]);
        let header_hash = block.hash();
        let out_point = OutPoint::new(header_hash.unpack(), h256!("0x2"), 3);

        cell_provider.cells.insert(
            out_point.cell().to_opt().should_be_ok(),
            Some(generate_dummy_cell_meta()),
        );
        header_provider.push_attached(&block);

        let transaction = generate_transaction_from_dep(out_point.clone());

        let mut seen_inputs = HashSet::default();
        let result = resolve_transaction(
            &transaction,
            &mut seen_inputs,
            &cell_provider,
            &header_provider,
        );

        assert_eq!(
            result.err(),
            Some(UnresolvableError::InvalidHeader(out_point))
        );
    }

    #[test]
    fn resolve_transaction_should_reject_empty_out_point() {
        let mut cell_provider = CellMemoryDb::default();
        let mut header_provider = BlockHeadersProvider::default();

        let block = generate_block(vec![]);
        let header_hash = block.hash();
        let out_point = OutPoint::new(header_hash.unpack(), h256!("0x2"), 3);

        cell_provider.cells.insert(
            out_point.cell().to_opt().should_be_ok(),
            Some(generate_dummy_cell_meta()),
        );
        header_provider.push_attached(&block);
        header_provider.insert_attached_transaction_block(
            out_point.cell().to_opt().should_be_ok().tx_hash().unpack(),
            header_hash.unpack(),
        );

        let transaction = generate_transaction_from_dep(OutPoint::default());

        let mut seen_inputs = HashSet::default();
        let result = resolve_transaction(
            &transaction,
            &mut seen_inputs,
            &cell_provider,
            &header_provider,
        );

        assert_eq!(result.err(), Some(UnresolvableError::Empty));
    }

    #[test]
    fn resolve_transaction_should_reject_incorrect_order_txs() {
        let out_point = OutPoint::new_cell(h256!("0x2"), 3);
        let cell_output = CellOutput::new_builder()
            .capacity(capacity_bytes!(2).pack())
            .build();

        let tx1 = TransactionBuilder::new()
            .input(CellInput::new(out_point, 0))
            .output(cell_output)
            .build();
        let dep = OutPoint::new_cell(tx1.hash().unpack(), 0);
        let input = CellInput::new(dep.clone(), 0);
        let tx2 = generate_transaction_from_input(input);
        let tx3 = generate_transaction_from_dep(dep.clone());

        // tx1 <- tx2
        // ok
        {
            let block = generate_block(vec![tx1.clone(), tx2.clone()]);
            let provider = BlockCellProvider::new(&block);
            assert!(provider.is_ok());
        }

        // tx1 -> tx2
        // resolve err
        {
            let block = generate_block(vec![tx2.clone(), tx1.clone()]);
            let provider = BlockCellProvider::new(&block);

            assert_eq!(
                provider.err(),
                Some(UnresolvableError::OutOfOrder(dep.clone()))
            );
        }

        // tx1 <- tx3
        // ok
        {
            let block = generate_block(vec![tx1.clone(), tx3.clone()]);
            let provider = BlockCellProvider::new(&block);

            assert!(provider.is_ok());
        }

        // tx1 -> tx3
        // resolve err
        {
            let block = generate_block(vec![tx3.clone(), tx1.clone()]);
            let provider = BlockCellProvider::new(&block);

            assert_eq!(
                provider.err(),
                Some(UnresolvableError::OutOfOrder(dep.clone()))
            );
        }
    }

    #[test]
    fn resolve_transaction_should_allow_dep_cell_in_current_tx_input() {
        let mut cell_provider = CellMemoryDb::default();
        let header_provider = BlockHeadersProvider::default();

        let out_point = OutPoint::new_cell(h256!("0x2"), 3);

        let dummy_cell_meta = generate_dummy_cell_meta();
        cell_provider.cells.insert(
            out_point.cell().to_opt().should_be_ok(),
            Some(dummy_cell_meta.clone()),
        );

        let tx = TransactionBuilder::new()
            .input(CellInput::new(out_point.clone(), 0))
            .dep(out_point)
            .build();

        let mut seen_inputs = HashSet::default();
        let rtx =
            resolve_transaction(&tx, &mut seen_inputs, &cell_provider, &header_provider).unwrap();

        assert_eq!(
            rtx.resolved_deps[0],
            ResolvedOutPoint::cell_only(dummy_cell_meta),
        );
    }

    #[test]
    fn resolve_transaction_should_reject_dep_cell_consumed_by_previous_input() {
        let mut cell_provider = CellMemoryDb::default();
        let header_provider = BlockHeadersProvider::default();

        let out_point = OutPoint::new_cell(h256!("0x2"), 3);

        cell_provider.cells.insert(
            out_point.cell().to_opt().should_be_ok(),
            Some(generate_dummy_cell_meta()),
        );

        // tx1 dep
        // tx2 input consumed
        // ok
        {
            let tx1 = generate_transaction_from_dep(out_point.clone());
            let tx2 = generate_transaction_from_input(CellInput::new(out_point.clone(), 0));

            let mut seen_inputs = HashSet::default();
            let result1 =
                resolve_transaction(&tx1, &mut seen_inputs, &cell_provider, &header_provider);
            assert!(result1.is_ok());

            let result2 =
                resolve_transaction(&tx2, &mut seen_inputs, &cell_provider, &header_provider);
            assert!(result2.is_ok());
        }

        // tx1 input consumed
        // tx2 dep
        // tx2 resolve err
        {
            let tx1 = generate_transaction_from_input(CellInput::new(out_point.clone(), 0));
            let tx2 = generate_transaction_from_dep(out_point.clone());

            let mut seen_inputs = HashSet::default();
            let result1 =
                resolve_transaction(&tx1, &mut seen_inputs, &cell_provider, &header_provider);

            assert!(result1.is_ok());

            let result2 =
                resolve_transaction(&tx2, &mut seen_inputs, &cell_provider, &header_provider);

            assert_eq!(
                result2.err(),
                Some(UnresolvableError::Dead(out_point.clone()))
            );
        }
    }
}
