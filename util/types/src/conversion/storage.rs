use crate::{core, packed, prelude::*};

impl Pack<packed::HeaderView> for core::HeaderView {
    fn pack(&self) -> packed::HeaderView {
        packed::HeaderView::new_builder()
            .data(self.data())
            .hash(self.hash())
            .build()
    }
}

impl<'r> Unpack<core::HeaderView> for packed::HeaderViewReader<'r> {
    fn unpack(&self) -> core::HeaderView {
        core::HeaderView {
            data: self.data().to_entity(),
            hash: self.hash().to_entity(),
        }
    }
}
impl_conversion_for_entity_unpack!(core::HeaderView, HeaderView);

impl Pack<packed::UncleBlockVecView> for core::UncleBlockVecView {
    fn pack(&self) -> packed::UncleBlockVecView {
        packed::UncleBlockVecView::new_builder()
            .data(self.data())
            .hashes(self.hashes())
            .build()
    }
}

impl<'r> Unpack<core::UncleBlockVecView> for packed::UncleBlockVecViewReader<'r> {
    fn unpack(&self) -> core::UncleBlockVecView {
        core::UncleBlockVecView {
            data: self.data().to_entity(),
            hashes: self.hashes().to_entity(),
        }
    }
}
impl_conversion_for_entity_unpack!(core::UncleBlockVecView, UncleBlockVecView);

impl Pack<packed::BlockBodyView> for core::BlockBodyView {
    fn pack(&self) -> packed::BlockBodyView {
        packed::BlockBodyView::new_builder()
            .data(self.data())
            .tx_hashes(self.tx_hashes())
            .tx_witness_hashes(self.tx_witness_hashes())
            .build()
    }
}

impl<'r> Unpack<core::BlockBodyView> for packed::BlockBodyViewReader<'r> {
    fn unpack(&self) -> core::BlockBodyView {
        core::BlockBodyView {
            data: self.data().to_entity(),
            tx_hashes: self.tx_hashes().to_entity(),
            tx_witness_hashes: self.tx_witness_hashes().to_entity(),
        }
    }
}
impl_conversion_for_entity_unpack!(core::BlockBodyView, BlockBodyView);

impl<'r> packed::BlockBodyViewReader<'r> {
    pub fn transaction(&self, index: usize) -> Option<core::TransactionView> {
        self.data().get(index).map(|data| {
            let hash = self.tx_hashes().get(index).should_be_ok().to_entity();
            let witness_hash = self
                .tx_witness_hashes()
                .get(index)
                .should_be_ok()
                .to_entity();
            core::TransactionView {
                data: data.to_entity(),
                hash,
                witness_hash,
            }
        })
    }

    pub fn output(&self, tx_index: usize, index: usize) -> Option<packed::CellOutput> {
        self.data().get(tx_index).and_then(|tx| {
            tx.slim()
                .raw()
                .outputs()
                .get(index)
                .map(|output| output.to_entity())
        })
    }
}

impl Pack<packed::BlockExt> for core::BlockExt {
    fn pack(&self) -> packed::BlockExt {
        packed::BlockExt::new_builder()
            .received_at(self.received_at.pack())
            .total_difficulty(self.total_difficulty.pack())
            .total_uncles_count(self.total_uncles_count.pack())
            .verified(self.verified.pack())
            .txs_fees((&self.txs_fees[..]).pack())
            .build()
    }
}

impl<'r> Unpack<core::BlockExt> for packed::BlockExtReader<'r> {
    fn unpack(&self) -> core::BlockExt {
        core::BlockExt {
            received_at: self.received_at().unpack(),
            total_difficulty: self.total_difficulty().unpack(),
            total_uncles_count: self.total_uncles_count().unpack(),
            verified: self.verified().unpack(),
            txs_fees: self.txs_fees().unpack(),
        }
    }
}
impl_conversion_for_entity_unpack!(core::BlockExt, BlockExt);

impl Pack<packed::EpochExt> for core::EpochExt {
    fn pack(&self) -> packed::EpochExt {
        packed::EpochExt::new_builder()
            .number(self.number().pack())
            .block_reward(self.base_block_reward().pack())
            .remainder_reward(self.remainder_reward().pack())
            .start_number(self.start_number().pack())
            .length(self.length().pack())
            .difficulty(self.difficulty().pack())
            .last_block_hash_in_previous_epoch(self.last_block_hash_in_previous_epoch().pack())
            .build()
    }
}

impl<'r> Unpack<core::EpochExt> for packed::EpochExtReader<'r> {
    fn unpack(&self) -> core::EpochExt {
        core::EpochExt {
            number: self.number().unpack(),
            block_reward: self.block_reward().unpack(),
            remainder_reward: self.remainder_reward().unpack(),
            last_block_hash_in_previous_epoch: self.last_block_hash_in_previous_epoch().unpack(),
            start_number: self.start_number().unpack(),
            length: self.length().unpack(),
            difficulty: self.difficulty().unpack(),
        }
    }
}
impl_conversion_for_entity_unpack!(core::EpochExt, EpochExt);

impl Pack<packed::TransactionMeta> for core::cell::TransactionMeta {
    fn pack(&self) -> packed::TransactionMeta {
        let len = self.dead_cell.len();
        let bits = self.dead_cell.to_bytes();
        packed::TransactionMeta::new_builder()
            .block_number(self.block_number.pack())
            .epoch_number(self.epoch_number.pack())
            .block_hash(self.block_hash.pack())
            .cellbase(self.cellbase.pack())
            .bits(bits.pack())
            .len(len.pack())
            .build()
    }
}

impl<'r> Unpack<core::cell::TransactionMeta> for packed::TransactionMetaReader<'r> {
    fn unpack(&self) -> core::cell::TransactionMeta {
        core::cell::TransactionMetaBuilder::default()
            .block_number(self.block_number().unpack())
            .epoch_number(self.epoch_number().unpack())
            .block_hash(self.block_hash().unpack())
            .cellbase(self.cellbase().unpack())
            .bits(self.bits().unpack())
            .len(self.len().unpack())
            .build()
    }
}
impl_conversion_for_entity_unpack!(core::cell::TransactionMeta, TransactionMeta);
