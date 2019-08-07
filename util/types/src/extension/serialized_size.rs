use std::mem;

use crate::{core::Version, packed, H256, U256};

// TODO apply-serialization check

impl packed::ProposalShortId {
    pub const fn serialized_size() -> usize {
        mem::size_of::<[u8; 10]>()
    }
}

impl packed::UncleBlock {
    pub fn serialized_size(&self, proof_size: usize) -> usize {
        packed::Header::serialized_size(proof_size)
    }
}

impl packed::Block {
    pub fn serialized_size(&self, proof_size: usize) -> usize {
        packed::Header::serialized_size(proof_size)
            + self
                .uncles()
                .into_iter()
                .map(|u| u.serialized_size(proof_size))
                .sum::<usize>()
            + 4
            + self.proposals().len() * packed::ProposalShortId::serialized_size()
            + 4
            + self
                .transactions()
                .into_iter()
                .map(|x| x.serialized_size())
                .sum::<usize>()
            + 4
    }
}

impl packed::Script {
    pub fn serialized_size(&self) -> usize {
        self.args()
            .into_iter()
            .map(|b| b.raw_data().len() + 4)
            .sum::<usize>()
            + 4
            + H256::size_of()
            + 1
    }
}

impl packed::RawHeader {
    pub const fn serialized_size() -> usize {
        mem::size_of::<Version>()
            + H256::size_of() * 5
            + U256::size_of()
            + mem::size_of::<u64>() * 3
            + mem::size_of::<u32>()
    }
}

impl packed::Header {
    pub fn serialized_size(proof_size: usize) -> usize {
        packed::RawHeader::serialized_size() + proof_size + mem::size_of::<u64>()
    }
}

impl packed::CellOutPoint {
    pub const fn serialized_size() -> usize {
        H256::size_of() + mem::size_of::<u32>()
    }
}

impl packed::OutPoint {
    pub fn serialized_size(&self) -> usize {
        self.cell()
            .to_opt()
            .map(|_| packed::CellOutPoint::serialized_size())
            .unwrap_or(0)
            + self
                .block_hash()
                .to_opt()
                .map(|_| H256::size_of())
                .unwrap_or(0)
    }
}

impl packed::CellInput {
    pub fn serialized_size(&self) -> usize {
        self.previous_output().serialized_size() + mem::size_of::<u64>()
    }
}

impl packed::CellOutput {
    pub fn serialized_size(&self) -> usize {
        mem::size_of::<u64>()
            + 32
            + self.lock().serialized_size()
            + self
                .type_()
                .to_opt()
                .map(|x| x.serialized_size())
                .unwrap_or(0)
    }
}

impl packed::Transaction {
    pub fn serialized_size(&self) -> usize {
        mem::size_of::<Version>()
            + self
                .slim()
                .raw()
                .deps()
                .into_iter()
                .map(|x| x.serialized_size())
                .sum::<usize>()
            + 4
            + self
                .slim()
                .raw()
                .inputs()
                .into_iter()
                .map(|x| x.serialized_size())
                .sum::<usize>()
            + 4
            + self
                .slim()
                .raw()
                .outputs()
                .into_iter()
                .map(|x| x.serialized_size())
                .sum::<usize>()
            + 4
            + self
                .slim()
                .witnesses()
                .into_iter()
                .flat_map(|witness| witness.into_iter().map(|w| w.raw_data().len()))
                .sum::<usize>()
            + 4
            + self
                .outputs_data()
                .into_iter()
                .map(|d| d.raw_data().len())
                .sum::<usize>()
            + 4
    }
}
