use super::Result;
use crate::error::ExtraDataError;
use byteorder::{ReadBytesExt, LE};
use std::io::Cursor;

/// The ConsoleFEDataBlock structure specifies the code page to use for displaying text when a link target specifies an application that is run in a console window.
#[derive(Clone, Debug, Default)]
pub struct ConsoleFEDataBlock {
    /// A 32-bit, unsigned integer that specifies the size of the ConsoleFEDataBlock structure. This value MUST be 0x0000000C.
    pub block_size: u32,

    /// A 32-bit, unsigned integer that specifies the signature of the ConsoleFEDataBlock extra data section. This value MUST be 0xA0000004.
    pub block_signature: u32,

    /// A 32-bit, unsigned integer that specifies a code page language code identifier. For details concerning the structure and meaning of language code identifiers, see [MS-LCID]. For additional background information, see [MSCHARSET], [MSDN-CS], and [MSDOCS-CodePage]
    pub code_page: u32,
}

impl ConsoleFEDataBlock {
    /// Construct a new `ConsoleFEDataBlock`
    pub(crate) fn new(
        block_size: u32,
        block_signature: u32,
        cursor: &mut Cursor<Vec<u8>>,
    ) -> Result<Self> {
        let this = Self {
            block_size,
            block_signature,
            code_page: cursor.read_u32::<LE>().map_err(ExtraDataError::Read)?,
        };

        Ok(this)
    }
}
