use super::Result;
use crate::error::ExtraDataError;
use std::io::{Cursor, Read};

/// A PropertyStoreDataBlock structure specifies a set of properties that can be used by applications to store extra data in the shell link.
#[derive(Clone, Debug, Default)]
pub struct PropertyStoreDataBlock {
    /// A 32-bit, unsigned integer that specifies the size of the PropertyStoreDataBlock structure. This value MUST be greater than or equal to 0x0000000C.
    pub block_size: u32,

    /// A 32-bit, unsigned integer that specifies the signature of the PropertyStoreDataBlock extra data section. This value MUST be 0xA0000009.
    pub block_signature: u32,

    /// A serialized property storage structure ([MS-PROPSTORE] section 2.2).
    pub property_store: Vec<u8>,
}

impl PropertyStoreDataBlock {
    /// Construct a new `KnownFolderDataBlock`
    pub(crate) fn new(
        block_size: u32,
        block_signature: u32,
        cursor: &mut Cursor<Vec<u8>>,
    ) -> Result<Self> {
        let this = Self {
            block_size,
            block_signature,
            property_store: {
                let store_size = block_size as usize - (std::mem::size_of::<u32>() * 2);
                let mut property_store = vec![0; store_size];
                cursor
                    .read_exact(&mut property_store)
                    .map_err(ExtraDataError::Read)?;
                property_store
            },
        };

        Ok(this)
    }
}
