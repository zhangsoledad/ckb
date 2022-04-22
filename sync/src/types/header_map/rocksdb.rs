use std::path;

use ckb_db::internal::{
    ops::{Delete as _, Get as _, GetPinned as _, Open as _, Put as _, WriteOps},
    BlockBasedOptions, Options, WriteBatch, DB,
};
use ckb_logger::{debug, error, warn};
use ckb_types::{packed::Byte32, prelude::*};
use tempfile::TempDir;

use crate::types::HeaderView;

pub(crate) struct RocksDB {
    db: DB,
    _tmpdir: TempDir,
}

impl RocksDB {
    pub fn new<P>(tmpdir: Option<P>) -> Self
    where
        P: AsRef<path::Path>,
    {
        let mut builder = tempfile::Builder::new();
        builder.prefix("ckb-tmp-");

        let tmpdir = if let Some(ref tmpdir) = tmpdir {
            builder.tempdir_in(tmpdir)
        } else {
            builder.tempdir()
        };

        let (db, tmpdir) = tmpdir
            .map(|dir| {
                // We minimize memory usage at all costs here.
                // If we want to use more memory, we should increase the limit of KeyValueMemory.
                let opts = {
                    let mut block_opts = BlockBasedOptions::default();
                    block_opts.disable_cache();
                    let mut opts = Options::default();
                    opts.create_if_missing(true);
                    opts.set_block_based_table_factory(&block_opts);
                    opts.set_write_buffer_size(4 * 1024 * 1024);
                    opts.set_max_write_buffer_number(2);
                    opts.set_min_write_buffer_number_to_merge(1);
                    opts
                };
                match DB::open(&opts, dir.path()) {
                    Ok(db) => {
                        debug!(
                            "open a key-value database({}) to save header map into disk",
                            dir.path().to_str().unwrap_or("")
                        );
                        (db, dir)
                    }
                    Err(e) => panic!(
                        "failed to open a key-value database to save header map into disk: {}",
                        e
                    ),
                }
            })
            .expect("failed to create a tempdir to save header map into disk");

        Self {
            db,
            _tmpdir: tmpdir,
        }
    }

    pub fn contains_key(&self, key: &Byte32) -> bool {
        // use get avoid pinned overhead
        self.db
            .get(key.as_slice())
            .unwrap_or_else(|err| panic!("read header map from disk should be ok, but {}", err))
            .is_some()
    }

    pub fn get(&self, key: &Byte32) -> Option<HeaderView> {
        self.db
            .get_pinned(key.as_slice())
            .unwrap_or_else(|err| panic!("read header map from disk should be ok, but {}", err))
            .map(|slice| HeaderView::from_slice_should_be_ok(&slice))
    }

    pub fn remove(&self, key: &Byte32) {
        if self.db.delete(key.as_slice()).is_err() {
            error!("failed to delete a value from database {}", key);
        }
    }

    pub fn insert_batch(&self, pairs: &[(Byte32, HeaderView)]) {
        let mut wb = WriteBatch::default();
        for (key, value) in pairs {
            wb.put(key.as_slice(), &value.to_vec());
        }
        if self.db.write(&wb).is_err() {
            error!("failed to insert items into header map store");
        }
    }
}
