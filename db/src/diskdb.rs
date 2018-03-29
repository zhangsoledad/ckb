use batch::{Batch, Key, KeyValue, Operation, Value};
use bincode::{deserialize, serialize};
use kvdb::{KeyValueDB, Result};
use rocksdb::{Options, WriteBatch, DB};
use std::path::Path;

const COLUMN_FAMILIES: &[&str] = &[
    "block_hash",
    "block_header",
    "block_transactions",
    "meta",
    "transaction",
    "transaction_meta",
    "block_height",
];

pub struct RocksKeyValueDB {
    db: DB,
}

impl RocksKeyValueDB {
    pub fn open<P: AsRef<Path>>(path: P) -> Self {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);

        let db = DB::open_cf(&opts, path, COLUMN_FAMILIES).unwrap();

        RocksKeyValueDB { db }
    }
}

impl KeyValueDB for RocksKeyValueDB {
    fn write(&self, batch: Batch) -> Result<()> {
        let mut wb = WriteBatch::default();
        for op in batch.operations {
            match op {
                Operation::Insert(insert) => match insert {
                    KeyValue::BlockHash(key, value) => {
                        wb.put_cf(
                            self.db.cf_handle(COLUMN_FAMILIES[0]).unwrap(),
                            &serialize(&key).unwrap().to_vec(),
                            &serialize(&value).unwrap().to_vec(),
                        )?;
                    }
                    KeyValue::BlockHeader(key, value) => {
                        wb.put_cf(
                            self.db.cf_handle(COLUMN_FAMILIES[1]).unwrap(),
                            &serialize(&key).unwrap().to_vec(),
                            &serialize(&value).unwrap().to_vec(),
                        )?;
                    }
                    KeyValue::BlockTransactions(key, value) => {
                        wb.put_cf(
                            self.db.cf_handle(COLUMN_FAMILIES[2]).unwrap(),
                            &serialize(&key).unwrap().to_vec(),
                            &serialize(&value).unwrap().to_vec(),
                        )?;
                    }
                    KeyValue::Meta(key, value) => {
                        wb.put_cf(
                            self.db.cf_handle(COLUMN_FAMILIES[3]).unwrap(),
                            &serialize(&key).unwrap().to_vec(),
                            &value,
                        )?;
                    }
                    KeyValue::Transaction(key, value) => {
                        wb.put_cf(
                            self.db.cf_handle(COLUMN_FAMILIES[4]).unwrap(),
                            &serialize(&key).unwrap().to_vec(),
                            &serialize(&value).unwrap().to_vec(),
                        )?;
                    }
                    KeyValue::TransactionMeta(key, value) => {
                        wb.put_cf(
                            self.db.cf_handle(COLUMN_FAMILIES[5]).unwrap(),
                            &serialize(&key).unwrap().to_vec(),
                            &serialize(&value).unwrap().to_vec(),
                        )?;
                    }
                    KeyValue::BlockHeight(key, value) => {
                        wb.put_cf(
                            self.db.cf_handle(COLUMN_FAMILIES[6]).unwrap(),
                            &serialize(&key).unwrap().to_vec(),
                            &serialize(&value).unwrap().to_vec(),
                        )?;
                    }
                },
                Operation::Delete(delete) => match delete {
                    Key::BlockHash(key) => {
                        wb.delete_cf(
                            self.db.cf_handle(COLUMN_FAMILIES[0]).unwrap(),
                            &serialize(&key).unwrap().to_vec(),
                        )?;
                    }
                    Key::BlockHeader(key) => {
                        wb.delete_cf(
                            self.db.cf_handle(COLUMN_FAMILIES[1]).unwrap(),
                            &serialize(&key).unwrap().to_vec(),
                        )?;
                    }
                    Key::BlockTransactions(key) => {
                        wb.delete_cf(
                            self.db.cf_handle(COLUMN_FAMILIES[2]).unwrap(),
                            &serialize(&key).unwrap().to_vec(),
                        )?;
                    }
                    Key::Meta(key) => {
                        wb.delete_cf(
                            self.db.cf_handle(COLUMN_FAMILIES[3]).unwrap(),
                            &serialize(&key).unwrap().to_vec(),
                        )?;
                    }
                    Key::Transaction(key) => {
                        wb.delete_cf(
                            self.db.cf_handle(COLUMN_FAMILIES[4]).unwrap(),
                            &serialize(&key).unwrap().to_vec(),
                        )?;
                    }
                    Key::TransactionMeta(key) => {
                        wb.delete_cf(
                            self.db.cf_handle(COLUMN_FAMILIES[5]).unwrap(),
                            &serialize(&key).unwrap().to_vec(),
                        )?;
                    }
                    Key::BlockHeight(key) => {
                        wb.delete_cf(
                            self.db.cf_handle(COLUMN_FAMILIES[6]).unwrap(),
                            &serialize(&key).unwrap().to_vec(),
                        )?;
                    }
                },
            }
        }
        self.db.write(wb).map_err(|err| err.into())
    }

    fn read(&self, key: &Key) -> Result<Option<Value>> {
        let result = match *key {
            Key::BlockHash(ref key) => self.db
                .get_cf(
                    self.db.cf_handle(COLUMN_FAMILIES[0]).unwrap(),
                    &serialize(&key).unwrap().to_vec(),
                )
                .map(|v| v.and_then(|ref v| Some(Value::BlockHash(deserialize(v).unwrap())))),
            Key::BlockHeader(ref key) => self.db
                .get_cf(
                    self.db.cf_handle(COLUMN_FAMILIES[1]).unwrap(),
                    &serialize(&key).unwrap().to_vec(),
                )
                .map(|v| v.and_then(|ref v| Some(Value::BlockHeader(deserialize(v).unwrap())))),
            Key::BlockTransactions(ref key) => self.db
                .get_cf(
                    self.db.cf_handle(COLUMN_FAMILIES[2]).unwrap(),
                    &serialize(&key).unwrap().to_vec(),
                )
                .map(|v| {
                    v.and_then(|ref v| Some(Value::BlockTransactions(deserialize(v).unwrap())))
                }),
            Key::Meta(key) => self.db
                .get_cf(
                    self.db.cf_handle(COLUMN_FAMILIES[3]).unwrap(),
                    &serialize(&key).unwrap().to_vec(),
                )
                .map(|v| v.and_then(|v| Some(Value::Meta(v.to_vec())))),
            Key::Transaction(ref key) => self.db
                .get_cf(
                    self.db.cf_handle(COLUMN_FAMILIES[4]).unwrap(),
                    &serialize(&key).unwrap().to_vec(),
                )
                .map(|v| v.and_then(|ref v| Some(Value::Transaction(deserialize(v).unwrap())))),
            Key::TransactionMeta(ref key) => self.db
                .get_cf(
                    self.db.cf_handle(COLUMN_FAMILIES[5]).unwrap(),
                    &serialize(&key).unwrap().to_vec(),
                )
                .map(|v| v.and_then(|ref v| Some(Value::TransactionMeta(deserialize(v).unwrap())))),
            Key::BlockHeight(ref key) => self.db
                .get_cf(
                    self.db.cf_handle(COLUMN_FAMILIES[6]).unwrap(),
                    &serialize(&key).unwrap().to_vec(),
                )
                .map(|v| v.and_then(|ref v| Some(Value::BlockHeight(deserialize(v).unwrap())))),
        };
        result.map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn save_and_get_meta() {
        let dir = TempDir::new("_nervos_test_db").unwrap();
        let db = RocksKeyValueDB::open(dir);
        let mut batch = Batch::default();
        batch.insert(KeyValue::Meta("abc", [0, 1, 2].to_vec()));
        db.write(batch).unwrap();

        assert_eq!(
            db.read(&Key::Meta("abc")).unwrap().unwrap(),
            Value::Meta([0, 1, 2].to_vec())
        );
    }

    #[test]
    fn get_empty() {
        let dir = TempDir::new("_nervos_test_db").unwrap();
        let db = RocksKeyValueDB::open(dir);
        assert_eq!(db.read(&Key::BlockHash(0)).unwrap(), None);
    }
}
