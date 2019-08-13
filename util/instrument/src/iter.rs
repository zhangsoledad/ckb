use ckb_shared::shared::Shared;
use ckb_store::ChainStore;
use ckb_traits::ChainProvider;
use ckb_types::{core::BlockNumber, core::BlockView};

// An iterator over the entries of a `Chain`.
pub struct ChainIterator {
    shared: Shared,
    current: Option<BlockView>,
    tip: BlockNumber,
}

impl ChainIterator {
    pub fn new(shared: Shared) -> Self {
        let current = shared
            .store()
            .get_block_hash(0)
            .and_then(|h| shared.store().get_block(&h));
        let tip = shared.lock_chain_state().tip_number();
        ChainIterator {
            shared,
            current,
            tip,
        }
    }

    pub fn len(&self) -> u64 {
        self.tip + 1
    }
}

impl Iterator for ChainIterator {
    type Item = BlockView;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.take();

        self.current = match current {
            Some(ref b) => {
                if let Some(block_hash) =
                    self.shared.store().get_block_hash(b.header().number() + 1)
                {
                    self.shared.store().get_block(&block_hash)
                } else {
                    None
                }
            }
            None => None,
        };
        current
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.current {
            Some(ref b) => (1, Some((self.tip - b.header().number() + 1) as usize)),
            None => (0, Some(0)),
        }
    }
}
