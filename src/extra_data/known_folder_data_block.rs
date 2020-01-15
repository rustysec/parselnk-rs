use super::Result;
use crate::error::ExtraDataError;
use byteorder::{ReadBytesExt, LE};
use std::io::Cursor;

/// The KnownFolderDataBlock structure specifies the location of a known folder. This data can be used when a link target is a known folder to keep track of the folder so that the link target IDList can be translated when the link is loaded.
#[derive(Clone, Debug, Default)]
pub struct KnownFolderDataBlock {
    /// A 32-bit, unsigned integer that specifies the size of the KnownFolderDataBlock structure. This value MUST be 0x0000001C.
    pub block_size: u32,

    /// A 32-bit, unsigned integer that specifies the signature of the KnownFolderDataBlock extra data section. This value MUST be 0xA000000B.
    pub block_signature: u32,

    /// A value in GUID packet representation ([MS-DTYP] section 2.3.4.2) that specifies the folder GUID ID.
    pub known_folder_id: u128,

    /// A 32-bit, unsigned integer that specifies the location of the ItemID of the first child segment of the IDList specified by KnownFolderID. This value is the offset, in bytes, into the link target IDList.
    pub offset: u32,
}

impl KnownFolderDataBlock {
    /// Construct a new `KnownFolderDataBlock`
    pub(crate) fn new(
        block_size: u32,
        block_signature: u32,
        cursor: &mut Cursor<Vec<u8>>,
    ) -> Result<Self> {
        let this = Self {
            block_size,
            block_signature,
            known_folder_id: cursor.read_u128::<LE>().map_err(ExtraDataError::Read)?,
            offset: cursor.read_u32::<LE>().map_err(ExtraDataError::Read)?,
        };

        Ok(this)
    }
}
