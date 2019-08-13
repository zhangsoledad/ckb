use ckb_db::Error as DBError;
use ckb_types::core::cell::UnresolvableError;
use failure::Fail;

#[derive(Debug, PartialEq, Clone, Eq, Fail)]
pub enum SharedError {
    #[fail(display = "UnresolvableTransaction: {:?}", _0)]
    UnresolvableTransaction(UnresolvableError),
    #[fail(display = "InvalidParentBlock")]
    InvalidParentBlock,
    #[fail(display = "InvalidData error: {}", _0)]
    InvalidData(String),
    #[fail(display = "InvalidBlock error: {}", _0)]
    InvalidBlock(String),
    #[fail(display = "DB error: {}", _0)]
    DB(DBError),
}
