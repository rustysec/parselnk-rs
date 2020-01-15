use super::Result;
use crate::error::ExtraDataError;
use byteorder::{ReadBytesExt, LE};
use std::io::Cursor;

/// The SpecialFolderDataBlock structure specifies the location of a special folder. This data can be used when a link target is a special folder to keep track of the folder, so that the link target IDList can be translated when the link is loaded.
#[derive(Clone, Debug, Default)]
pub struct SpecialFolderDataBlock {
    /// A 32-bit, unsigned integer that specifies the size of the SpecialFolderDataBlock structure. This value MUST be 0x00000010.
    pub block_size: u32,

    /// A 32-bit, unsigned integer that specifies the signature of the SpecialFolderDataBlock extra data section. This value MUST be 0xA0000005.
    pub block_signature: u32,

    /// A 32-bit, unsigned integer that specifies the folder integer ID.
    pub special_folder_id: u32,

    /// A 32-bit, unsigned integer that specifies the location of the ItemID of the first child segment of the IDList specified by SpecialFolderID. This value is the offset, in bytes, into the link target IDList.
    pub offset: u32,
}

impl SpecialFolderDataBlock {
    /// Construct a new `SpecialFolderDataBlock`
    pub(crate) fn new(
        block_size: u32,
        block_signature: u32,
        cursor: &mut Cursor<Vec<u8>>,
    ) -> Result<Self> {
        let this = Self {
            block_size,
            block_signature,
            special_folder_id: cursor.read_u32::<LE>().map_err(ExtraDataError::Read)?,
            offset: cursor.read_u32::<LE>().map_err(ExtraDataError::Read)?,
        };

        Ok(this)
    }
}
