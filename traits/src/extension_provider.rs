use ckb_types::packed;

pub trait ExtensionProvider {
    /// Get the header of the given block hash
    fn get_block_extension(&self, hash: &packed::Byte32) -> Option<packed::Bytes>;
}
