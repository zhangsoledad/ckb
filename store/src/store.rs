use crate::cache::StoreCache;
use crate::{
    COLUMN_BLOCK_BODY, COLUMN_BLOCK_EPOCH, COLUMN_BLOCK_EXT, COLUMN_BLOCK_HEADER,
    COLUMN_BLOCK_PROPOSAL_IDS, COLUMN_BLOCK_UNCLE, COLUMN_CELL, COLUMN_CELL_DATA, COLUMN_EPOCH,
    COLUMN_INDEX, COLUMN_META, COLUMN_TRANSACTION_INFO, COLUMN_UNCLES, META_CURRENT_EPOCH_KEY,
    META_TIP_HEADER_KEY,
};
use ckb_chain_spec::consensus::Consensus;
use ckb_db::{
    iter::{DBIter, Direction, IteratorMode},
    Col,
};
use ckb_freezer::Freezer;
use ckb_types::{
    bytes::Bytes,
    core::{
        cell::{CellMeta, CellProvider, CellStatus},
        BlockExt, BlockNumber, BlockView, EpochExt, EpochNumber, HeaderView, TransactionInfo,
        TransactionView, UncleBlockVecView,
    },
    packed::{self, OutPoint},
    prelude::*,
};

pub struct CellProviderWrapper<'a, S>(&'a S);

pub trait ChainStore<'a>: Send + Sync + Sized {
    type Vector: AsRef<[u8]>;
    fn cache(&'a self) -> Option<&'a StoreCache>;
    fn freezer(&'a self) -> Option<&'a Freezer>;
    fn get(&'a self, col: Col, key: &[u8]) -> Option<Self::Vector>;
    fn get_iter(&self, col: Col, mode: IteratorMode) -> DBIter;
    fn cell_provider(&self) -> CellProviderWrapper<Self> {
        CellProviderWrapper(self)
    }

    /// Get block by block header hash
    fn get_block(&'a self, h: &packed::Byte32) -> Option<BlockView> {
        self.get_block_header(h).map(|header| {
            if let Some(freezer) = self.freezer() {
                if header.number() > 0 && header.number() < freezer.number() {
                    let raw_block = freezer.retrieve(header.number()).expect("block frozen");
                    let raw_block =
                        packed::BlockReader::from_slice_should_be_ok(&raw_block).to_entity();
                    return raw_block.into_view();
                }
            }
            let body = self.get_block_body(h);
            let uncles = self
                .get_block_uncles(h)
                .expect("block uncles must be stored");
            let proposals = self
                .get_block_proposal_txs_ids(h)
                .expect("block proposal_ids must be stored");
            BlockView::new_unchecked(header, uncles, body, proposals)
        })
    }

    /// Get header by block header hash
    fn get_block_header(&'a self, hash: &packed::Byte32) -> Option<HeaderView> {
        if let Some(cache) = self.cache() {
            if let Some(header) = cache.headers.lock().get_refresh(hash) {
                return Some(header.clone());
            }
        };
        let ret = self.get(COLUMN_BLOCK_HEADER, hash.as_slice()).map(|slice| {
            let reader = packed::HeaderViewReader::from_slice_should_be_ok(&slice.as_ref());
            Unpack::<HeaderView>::unpack(&reader)
        });

        if let Some(cache) = self.cache() {
            ret.map(|header| {
                cache.headers.lock().insert(hash.clone(), header.clone());
                header
            })
        } else {
            ret
        }
    }

    /// Get block body by block header hash
    fn get_block_body(&'a self, hash: &packed::Byte32) -> Vec<TransactionView> {
        let prefix = hash.as_slice();
        self.get_iter(
            COLUMN_BLOCK_BODY,
            IteratorMode::From(prefix, Direction::Forward),
        )
        .take_while(|(key, _)| key.starts_with(prefix))
        .map(|(_key, value)| {
            let reader = packed::TransactionViewReader::from_slice_should_be_ok(&value.as_ref());
            Unpack::<TransactionView>::unpack(&reader)
        })
        .collect()
    }

    fn get_packed_block(&'a self, hash: &packed::Byte32) -> Option<packed::Block> {
        let header = self
            .get(COLUMN_BLOCK_HEADER, hash.as_slice())
            .map(|slice| {
                let reader = packed::HeaderViewReader::from_slice_should_be_ok(&slice.as_ref());
                reader.data().to_entity()
            })?;

        let prefix = hash.as_slice();
        let transactions: packed::TransactionVec = self
            .get_iter(
                COLUMN_BLOCK_BODY,
                IteratorMode::From(prefix, Direction::Forward),
            )
            .take_while(|(key, _)| key.starts_with(prefix))
            .map(|(_key, value)| {
                let reader =
                    packed::TransactionViewReader::from_slice_should_be_ok(&value.as_ref());
                reader.data().to_entity()
            })
            .pack();

        let uncles = self.get(COLUMN_BLOCK_UNCLE, hash.as_slice()).map(|slice| {
            let reader = packed::UncleBlockVecViewReader::from_slice_should_be_ok(&slice.as_ref());
            reader.data().to_entity()
        })?;

        let proposals = self.get_block_proposal_txs_ids(hash)?;
        Some(
            packed::Block::new_builder()
                .header(header)
                .uncles(uncles)
                .transactions(transactions)
                .proposals(proposals)
                .build(),
        )
    }

    /// Get all transaction-hashes in block body by block header hash
    fn get_block_txs_hashes(&'a self, hash: &packed::Byte32) -> Vec<packed::Byte32> {
        if let Some(cache) = self.cache() {
            if let Some(hashes) = cache.block_tx_hashes.lock().get_refresh(hash) {
                return hashes.clone();
            }
        };

        let prefix = hash.as_slice();
        let ret: Vec<_> = self
            .get_iter(
                COLUMN_BLOCK_BODY,
                IteratorMode::From(prefix, Direction::Forward),
            )
            .take_while(|(key, _)| key.starts_with(prefix))
            .map(|(_key, value)| {
                let reader =
                    packed::TransactionViewReader::from_slice_should_be_ok(&value.as_ref());
                reader.hash().to_entity()
            })
            .collect();

        if let Some(cache) = self.cache() {
            cache
                .block_tx_hashes
                .lock()
                .insert(hash.clone(), ret.clone());
        }

        ret
    }

    /// Get proposal short id by block header hash
    fn get_block_proposal_txs_ids(
        &'a self,
        hash: &packed::Byte32,
    ) -> Option<packed::ProposalShortIdVec> {
        if let Some(cache) = self.cache() {
            if let Some(data) = cache.block_proposals.lock().get_refresh(hash) {
                return Some(data.clone());
            }
        };

        let ret = self
            .get(COLUMN_BLOCK_PROPOSAL_IDS, hash.as_slice())
            .map(|slice| {
                packed::ProposalShortIdVecReader::from_slice_should_be_ok(&slice.as_ref())
                    .to_entity()
            });

        if let Some(cache) = self.cache() {
            ret.map(|data| {
                cache
                    .block_proposals
                    .lock()
                    .insert(hash.clone(), data.clone());
                data
            })
        } else {
            ret
        }
    }

    /// Get block uncles by block header hash
    fn get_block_uncles(&'a self, hash: &packed::Byte32) -> Option<UncleBlockVecView> {
        if let Some(cache) = self.cache() {
            if let Some(data) = cache.block_uncles.lock().get_refresh(hash) {
                return Some(data.clone());
            }
        };

        let ret = self.get(COLUMN_BLOCK_UNCLE, hash.as_slice()).map(|slice| {
            let reader = packed::UncleBlockVecViewReader::from_slice_should_be_ok(&slice.as_ref());
            Unpack::<UncleBlockVecView>::unpack(&reader)
        });

        if let Some(cache) = self.cache() {
            ret.map(|uncles| {
                cache
                    .block_uncles
                    .lock()
                    .insert(hash.clone(), uncles.clone());
                uncles
            })
        } else {
            ret
        }
    }

    /// Get block ext by block header hash
    fn get_block_ext(&'a self, block_hash: &packed::Byte32) -> Option<BlockExt> {
        self.get(COLUMN_BLOCK_EXT, block_hash.as_slice())
            .map(|slice| {
                packed::BlockExtReader::from_slice_should_be_ok(&slice.as_ref()[..]).unpack()
            })
    }

    /// Get block header hash by block number
    fn get_block_hash(&'a self, number: BlockNumber) -> Option<packed::Byte32> {
        let block_number: packed::Uint64 = number.pack();
        self.get(COLUMN_INDEX, block_number.as_slice())
            .map(|raw| packed::Byte32Reader::from_slice_should_be_ok(&raw.as_ref()[..]).to_entity())
    }

    /// Get block number by block header hash
    fn get_block_number(&'a self, hash: &packed::Byte32) -> Option<BlockNumber> {
        self.get(COLUMN_INDEX, hash.as_slice())
            .map(|raw| packed::Uint64Reader::from_slice_should_be_ok(&raw.as_ref()[..]).unpack())
    }

    fn is_main_chain(&'a self, hash: &packed::Byte32) -> bool {
        self.get(COLUMN_INDEX, hash.as_slice()).is_some()
    }

    fn get_tip_header(&'a self) -> Option<HeaderView> {
        self.get(COLUMN_META, META_TIP_HEADER_KEY)
            .and_then(|raw| {
                self.get_block_header(
                    &packed::Byte32Reader::from_slice_should_be_ok(&raw.as_ref()[..]).to_entity(),
                )
            })
            .map(Into::into)
    }

    /// Get commit transaction and block hash by its hash
    fn get_transaction(
        &'a self,
        hash: &packed::Byte32,
    ) -> Option<(TransactionView, packed::Byte32)> {
        self.get_transaction_info_packed(hash).map(|info| {
            self.get(COLUMN_BLOCK_BODY, info.key().as_slice())
                .map(|slice| {
                    let reader =
                        packed::TransactionViewReader::from_slice_should_be_ok(&slice.as_ref());
                    let hash = info.as_reader().key().block_hash().to_entity();
                    (reader.unpack(), hash)
                })
                .expect("since tx info is existed, so tx data should be existed")
        })
    }

    fn get_transaction_with_info(
        &'a self,
        hash: &packed::Byte32,
    ) -> Option<(TransactionView, TransactionInfo)> {
        self.get_transaction_info(hash).map(|info| {
            self.get(COLUMN_BLOCK_BODY, info.key().as_slice())
                .map(|slice| {
                    let reader =
                        packed::TransactionViewReader::from_slice_should_be_ok(&slice.as_ref());
                    (reader.unpack(), info)
                })
                .expect("since tx info is existed, so tx data should be existed")
        })
    }

    fn get_cell(&'a self, out_point: &OutPoint) -> Option<CellMeta> {
        let key = cell_key_from_out_point(&out_point);
        self.get(COLUMN_CELL, &key[..]).map(|slice| {
            let reader = packed::CellEntryReader::from_slice_should_be_ok(&slice.as_ref()[..]);
            build_cell_meta_from_reader(out_point.clone(), reader)
        })
    }

    fn have_cell(&'a self, out_point: &OutPoint) -> bool {
        let key = cell_key_from_out_point(out_point);
        self.get(COLUMN_CELL, &key[..]).is_some()
    }

    fn get_cell_data(&'a self, out_point: &OutPoint) -> Option<(Bytes, packed::Byte32)> {
        let key = cell_key_from_out_point(out_point);
        if let Some(cache) = self.cache() {
            if let Some(cached) = cache.cell_data.lock().get_refresh(&key) {
                return Some(cached.clone());
            }
        };

        let ret = self.get(COLUMN_CELL_DATA, &key[..]).map(|slice| {
            let reader = packed::CellDataEntryReader::from_slice_should_be_ok(&slice.as_ref()[..]);
            let data = reader.output_data().unpack();
            let data_hash = reader.output_data_hash().to_entity();
            (data, data_hash)
        });

        if let Some(cache) = self.cache() {
            ret.map(|cached| {
                cache.cell_data.lock().insert(key, cached.clone());
                cached
            })
        } else {
            ret
        }
    }

    fn get_transaction_info_packed(
        &'a self,
        hash: &packed::Byte32,
    ) -> Option<packed::TransactionInfo> {
        self.get(COLUMN_TRANSACTION_INFO, hash.as_slice())
            .map(|slice| {
                let reader =
                    packed::TransactionInfoReader::from_slice_should_be_ok(&slice.as_ref());
                reader.to_entity()
            })
    }

    fn get_transaction_info(&'a self, hash: &packed::Byte32) -> Option<TransactionInfo> {
        self.get(COLUMN_TRANSACTION_INFO, hash.as_slice())
            .map(|slice| {
                let reader =
                    packed::TransactionInfoReader::from_slice_should_be_ok(&slice.as_ref());
                Unpack::<TransactionInfo>::unpack(&reader)
            })
    }

    // fn get_tx_meta(&'a self, tx_hash: &packed::Byte32) -> Option<TransactionMeta> {
    //     self.get(COLUMN_TX_META, tx_hash.as_slice()).map(|slice| {
    //         packed::TransactionMetaReader::from_slice_should_be_ok(&slice.as_ref()).unpack()
    //     })
    // }

    // fn get_cell_meta(&'a self, tx_hash: &packed::Byte32, index: u32) -> Option<CellMeta> {
    //     self.get_transaction_info_packed(&tx_hash)
    //         .and_then(|tx_info| {
    //             self.get(COLUMN_BLOCK_BODY, tx_info.key().as_slice())
    //                 .and_then(|slice| {
    //                     let reader =
    //                         packed::TransactionViewReader::from_slice_should_be_ok(&slice.as_ref());
    //                     reader
    //                         .data()
    //                         .raw()
    //                         .outputs()
    //                         .get(index as usize)
    //                         .map(|cell_output| {
    //                             let cell_output = cell_output.to_entity();
    //                             let data_bytes = reader
    //                                 .data()
    //                                 .raw()
    //                                 .outputs_data()
    //                                 .get(index as usize)
    //                                 .expect("inconsistent index")
    //                                 .raw_data()
    //                                 .len() as u64;
    //                             let out_point = packed::OutPoint::new_builder()
    //                                 .tx_hash(tx_hash.to_owned())
    //                                 .index(index.pack())
    //                                 .build();
    //                             // notice mem_cell_data is set to None, the cell data should be load in need
    //                             CellMeta {
    //                                 cell_output,
    //                                 out_point,
    //                                 transaction_info: Some(tx_info.unpack()),
    //                                 data_bytes,
    //                                 mem_cell_data: None,
    //                             }
    //                         })
    //                 })
    //         })
    // }

    // Get current epoch ext
    fn get_current_epoch_ext(&'a self) -> Option<EpochExt> {
        self.get(COLUMN_META, META_CURRENT_EPOCH_KEY)
            .map(|slice| packed::EpochExtReader::from_slice_should_be_ok(&slice.as_ref()).unpack())
    }

    // Get epoch ext by epoch index
    fn get_epoch_ext(&'a self, hash: &packed::Byte32) -> Option<EpochExt> {
        self.get(COLUMN_EPOCH, hash.as_slice())
            .map(|slice| packed::EpochExtReader::from_slice_should_be_ok(&slice.as_ref()).unpack())
    }

    // Get epoch index by epoch number
    fn get_epoch_index(&'a self, number: EpochNumber) -> Option<packed::Byte32> {
        let epoch_number: packed::Uint64 = number.pack();
        self.get(COLUMN_EPOCH, epoch_number.as_slice())
            .map(|raw| packed::Byte32Reader::from_slice_should_be_ok(&raw.as_ref()).to_entity())
    }

    // Get epoch index by block hash
    fn get_block_epoch_index(&'a self, block_hash: &packed::Byte32) -> Option<packed::Byte32> {
        self.get(COLUMN_BLOCK_EPOCH, block_hash.as_slice())
            .map(|raw| packed::Byte32Reader::from_slice_should_be_ok(&raw.as_ref()).to_entity())
    }

    fn get_block_epoch(&'a self, hash: &packed::Byte32) -> Option<EpochExt> {
        self.get_block_epoch_index(hash)
            .and_then(|index| self.get_epoch_ext(&index))
    }

    fn is_uncle(&'a self, hash: &packed::Byte32) -> bool {
        self.get(COLUMN_UNCLES, hash.as_slice()).is_some()
    }

    /// Get header by uncle header hash
    fn get_uncle_header(&'a self, hash: &packed::Byte32) -> Option<HeaderView> {
        self.get(COLUMN_UNCLES, hash.as_slice()).map(|slice| {
            let reader = packed::HeaderViewReader::from_slice_should_be_ok(&slice.as_ref());
            Unpack::<HeaderView>::unpack(&reader)
        })
    }

    fn block_exists(&'a self, hash: &packed::Byte32) -> bool {
        if let Some(cache) = self.cache() {
            if cache.headers.lock().get_refresh(hash).is_some() {
                return true;
            }
        };
        self.get(COLUMN_BLOCK_HEADER, hash.as_slice()).is_some()
    }

    // Get cellbase by block hash
    fn get_cellbase(&'a self, hash: &packed::Byte32) -> Option<TransactionView> {
        if let Some(cache) = self.cache() {
            if let Some(data) = cache.cellbase.lock().get_refresh(hash) {
                return Some(data.clone());
            }
        };
        let key = packed::TransactionKey::new_builder()
            .block_hash(hash.to_owned())
            .build();
        let ret = self.get(COLUMN_BLOCK_BODY, key.as_slice()).map(|slice| {
            let reader = packed::TransactionViewReader::from_slice_should_be_ok(&slice.as_ref());
            Unpack::<TransactionView>::unpack(&reader)
        });
        if let Some(cache) = self.cache() {
            ret.map(|data| {
                cache.cellbase.lock().insert(hash.clone(), data.clone());
                data
            })
        } else {
            ret
        }
    }

    fn next_epoch_ext(
        &'a self,
        consensus: &Consensus,
        last_epoch: &EpochExt,
        header: &HeaderView,
    ) -> Option<EpochExt> {
        consensus.next_epoch_ext(
            last_epoch,
            header,
            |hash| self.get_block_header(&hash),
            |hash| self.get_block_ext(&hash).map(|ext| ext.total_uncles_count),
        )
    }
}

#[inline]
pub(crate) fn cell_key_from_out_point(out_point: &OutPoint) -> Vec<u8> {
    let mut key = Vec::with_capacity(36);
    let index: u32 = out_point.index().unpack();
    key.extend_from_slice(out_point.tx_hash().as_slice());
    key.extend_from_slice(&index.to_be_bytes()[..]);
    key
}

fn build_cell_meta_from_reader(out_point: OutPoint, reader: packed::CellEntryReader) -> CellMeta {
    CellMeta {
        out_point,
        cell_output: reader.output().to_entity(),
        transaction_info: Some(TransactionInfo {
            block_number: reader.block_number().unpack(),
            block_hash: reader.block_hash().to_entity(),
            block_epoch: reader.block_epoch().unpack(),
            index: reader.index().unpack(),
        }),
        data_bytes: reader.data_size().unpack(),
        mem_cell_data: None,
    }
}

impl<'a, S> CellProvider for CellProviderWrapper<'a, S>
where
    S: ChainStore<'a>,
{
    fn cell(&self, out_point: &OutPoint, with_data: bool) -> CellStatus {
        match self.0.get_cell(out_point) {
            Some(mut cell_meta) => {
                if with_data {
                    cell_meta.mem_cell_data = self.0.get_cell_data(out_point);
                }
                CellStatus::live_cell(cell_meta)
            }
            None => {
                // TODO: it is necessary ?
                // let tx_hash = out_point.tx_hash();
                // let index: u32 = out_point.index().unpack();

                // if Some(true)
                //     == self
                //         .0
                //         .get_tx_meta(&tx_hash)
                //         .and_then(|tx_meta| tx_meta.is_dead(index as usize))
                // {
                CellStatus::Unknown
                // } else {
                //     CellStatus::Unknown
                // }
            }
        }
    }
}
