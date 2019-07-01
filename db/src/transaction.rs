use crate::rocksdb::cf_handle;
use crate::{Col, Result};
use rocksdb::ops::{DeleteCF, GetCF, PutCF};
pub use rocksdb::{DBPinnableSlice, DBVector};
use rocksdb::{
    OptimisticTransactionDB, ReadOptions, Transaction as RocksRawTransaction, TransactionSnapshot,
};

pub struct RocksDBTransaction<'a> {
    pub(crate) db: &'a OptimisticTransactionDB,
    pub(crate) txn: RocksRawTransaction<'a, OptimisticTransactionDB>,
}

pub struct RocksDBTransactionSnapshot<'a> {
    inner: TransactionSnapshot<'a, OptimisticTransactionDB>,
}

impl<'a> RocksDBTransactionSnapshot<'a> {
    fn inner(&self) -> &TransactionSnapshot<'a, OptimisticTransactionDB> {
        &self.inner
    }
}

impl<'a> RocksDBTransaction<'a> {
    pub fn get(&self, col: Col, key: &[u8]) -> Result<Option<DBVector>> {
        let cf = cf_handle(&self.db, col)?;
        self.txn.get_cf(cf, key).map_err(Into::into)
    }

    pub fn put(&self, col: Col, key: &[u8], value: &[u8]) -> Result<()> {
        let cf = cf_handle(&self.db, col)?;
        self.txn.put_cf(cf, key, value).map_err(Into::into)
    }

    pub fn delete(&self, col: Col, key: &[u8]) -> Result<()> {
        let cf = cf_handle(&self.db, col)?;
        self.txn.delete_cf(cf, key).map_err(Into::into)
    }

    pub fn get_for_update(
        &self,
        col: Col,
        key: &[u8],
        snapshot: &RocksDBTransactionSnapshot<'a>,
    ) -> Result<Option<DBVector>> {
        let mut opts = ReadOptions::default();
        opts.set_snapshot(snapshot.inner());
        let cf = cf_handle(&self.db, col)?;
        self.txn
            .get_for_update_cf_opt(cf, key, &opts, true)
            .map_err(Into::into)
    }

    pub fn commit(&self) -> Result<()> {
        self.txn.commit().map_err(Into::into)
    }

    pub fn rollback(&self) -> Result<()> {
        self.txn.rollback().map_err(Into::into)
    }

    pub fn get_snapshot(&'a self) -> RocksDBTransactionSnapshot<'a> {
        RocksDBTransactionSnapshot {
            inner: self.txn.snapshot(),
        }
    }

    pub fn set_savepoint(&self) {
        self.txn.set_savepoint()
    }

    pub fn rollback_to_savepoint(&self) -> Result<()> {
        self.txn.rollback_to_savepoint().map_err(Into::into)
    }
}
