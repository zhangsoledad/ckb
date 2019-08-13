use ckb_types::{core, packed};
use ckb_util::RwLock;
use fnv::FnvHashMap;
use std::collections::VecDeque;

pub type ParentHash = packed::Byte32;

// NOTE: Never use `LruCache` as container. We have to ensure synchronizing between
// orphan_block_pool and block_status_map, but `LruCache` would prune old items implicitly.
#[derive(Default)]
pub struct OrphanBlockPool {
    blocks: RwLock<FnvHashMap<ParentHash, FnvHashMap<packed::Byte32, core::BlockView>>>,
}

impl OrphanBlockPool {
    pub fn with_capacity(capacity: usize) -> Self {
        OrphanBlockPool {
            blocks: RwLock::new(FnvHashMap::with_capacity_and_hasher(
                capacity,
                Default::default(),
            )),
        }
    }

    /// Insert orphaned block, for which we have already requested its parent block
    pub fn insert(&self, block: core::BlockView) {
        self.blocks
            .write()
            .entry(block.data().header().raw().parent_hash())
            .or_insert_with(FnvHashMap::default)
            .insert(block.header().hash(), block);
    }

    pub fn remove_blocks_by_parent(&self, hash: &packed::Byte32) -> Vec<core::BlockView> {
        let mut guard = self.blocks.write();
        let mut queue: VecDeque<packed::Byte32> = VecDeque::new();
        queue.push_back(hash.to_owned());

        let mut removed: Vec<core::BlockView> = Vec::new();
        while let Some(parent_hash) = queue.pop_front() {
            if let Some(orphaned) = guard.remove(&parent_hash) {
                let (hashes, blocks): (Vec<_>, Vec<_>) = orphaned.into_iter().unzip();
                queue.extend(hashes);
                removed.extend(blocks);
            }
        }
        removed
    }
}

/* TODO apply-serialization fix tests
#[cfg(test)]
mod tests {
    use super::*;
    use ckb_chain_spec::consensus::Consensus;
    use ckb_core::block::BlockBuilder;
    use ckb_core::header::{Header, HeaderBuilder};
    use faketime::unix_time_as_millis;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    fn gen_block(parent_header: &Header) -> Block {
        let header = HeaderBuilder::default()
            .parent_hash(parent_header.hash().to_owned())
            .timestamp(unix_time_as_millis())
            .number(parent_header.number() + 1)
            .nonce(parent_header.nonce() + 1)
            .build();

        BlockBuilder::default().header(header).build()
    }

    #[test]
    fn test_remove_blocks_by_parent() {
        let consensus = Consensus::default();
        let block_number = 200;
        let mut blocks: Vec<Block> = Vec::new();
        let mut parent = consensus.genesis_block().header().to_owned();
        let pool = OrphanBlockPool::with_capacity(200);
        for _ in 1..block_number {
            let new_block = gen_block(&parent);
            blocks.push(new_block.clone());
            pool.insert(new_block.clone());
            parent = new_block.header().to_owned();
        }

        let orphan = pool.remove_blocks_by_parent(&consensus.genesis_block().header().hash());
        let orphan: HashSet<Block> = HashSet::from_iter(orphan.into_iter());
        let block: HashSet<Block> = HashSet::from_iter(blocks.into_iter());
        assert_eq!(orphan, block)
    }
}
*/
