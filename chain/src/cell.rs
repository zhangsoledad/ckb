use ckb_core::block::Block;
use ckb_core::transaction_meta::TransactionMeta;
use ckb_store::{ChainStore, StoreTransaction};
use failure::Error as FailureError;
use im::hashmap as hamt;
use im::hashmap::HashMap as HamtMap;
use numext_fixed_hash::H256;

pub fn attach_block_cell(
    txn: &StoreTransaction,
    block: &Block,
    cell_set: &mut HamtMap<H256, TransactionMeta>,
) -> Result<(), FailureError> {
    for tx in block.transactions() {
        for cell in tx.input_pts_iter() {
            if let hamt::Entry::Occupied(mut o) = cell_set.entry(cell.tx_hash.clone()) {
                o.get_mut().set_dead(cell.index as usize);
                if o.get().all_dead() {
                    txn.delete_cell_set(&cell.tx_hash)?;
                    o.remove_entry();
                } else {
                    txn.update_cell_set(&cell.tx_hash, o.get())?;
                }
            }
        }
        let tx_hash = tx.hash();
        let outputs_len = tx.outputs().len();
        let meta = if tx.is_cellbase() {
            TransactionMeta::new_cellbase(
                block.header().number(),
                block.header().epoch(),
                block.header().hash().to_owned(),
                outputs_len,
                false,
            )
        } else {
            TransactionMeta::new(
                block.header().number(),
                block.header().epoch(),
                block.header().hash().to_owned(),
                outputs_len,
                false,
            )
        };
        txn.update_cell_set(tx_hash, &meta)?;
        cell_set.insert(tx_hash.to_owned(), meta);
    }
    Ok(())
}

pub fn detach_block_cell(
    txn: &StoreTransaction,
    block: &Block,
    cell_set: &mut HamtMap<H256, TransactionMeta>,
) -> Result<(), FailureError> {
    for tx in block.transactions().iter().rev() {
        txn.delete_cell_set(tx.hash())?;
        cell_set.remove(tx.hash());

        for cell in tx.input_pts_iter() {
            if let Some(tx_meta) = cell_set.get_mut(&cell.tx_hash) {
                tx_meta.unset_dead(cell.index as usize);
                txn.update_cell_set(&cell.tx_hash, &tx_meta)?;
            } else {
                // the tx is full dead, deleted from cellset, we need recover it when fork
                if let Some((tx, header)) =
                    txn.get_transaction(&cell.tx_hash)
                        .and_then(|(tx, block_hash)| {
                            txn.get_block_header(&block_hash).map(|header| (tx, header))
                        })
                {
                    let mut meta = if tx.is_cellbase() {
                        TransactionMeta::new_cellbase(
                            header.number(),
                            header.epoch(),
                            header.hash().to_owned(),
                            tx.outputs().len(),
                            true, // init with all dead
                        )
                    } else {
                        TransactionMeta::new(
                            header.number(),
                            header.epoch(),
                            header.hash().to_owned(),
                            tx.outputs().len(),
                            true, // init with all dead
                        )
                    };
                    meta.unset_dead(cell.index as usize); // recover
                    txn.update_cell_set(&cell.tx_hash, &meta)?;
                    cell_set.insert(cell.tx_hash.to_owned(), meta);
                }
            }
        }
    }
    Ok(())
}
