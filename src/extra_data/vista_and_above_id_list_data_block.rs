use super::Result;
use crate::error::ExtraDataError;
use std::io::{Cursor, Read};

/// The VistaAndAboveIDListDataBlock structure specifies an alternate IDList that can be used instead of the LinkTargetIDList structure (section 2.2) on platforms that support it.
#[derive(Clone, Debug, Default)]
pub struct VistaAndAboveIDListDataBlock {
    /// A 32-bit, unsigned integer that specifies the size of the VistaAndAboveIDListDataBlock structure. This value MUST be greater than or equal to 0x0000000A.
    pub block_size: u32,

    /// A 32-bit, unsigned integer that specifies the signature of the VistaAndAboveIDListDataBlock extra data section. This value MUST be 0xA000000C.
    pub block_signature: u32,

    /// An IDList structure (section 2.2.1).
    pub id_list: Vec<u8>,
}

impl VistaAndAboveIDListDataBlock {
    /// Construct a new `VistaAndAboveIDListDataBlock`
    pub(crate) fn new(
        block_size: u32,
        block_signature: u32,
        cursor: &mut Cursor<Vec<u8>>,
    ) -> Result<Self> {
        let this = Self {
            block_size,
            block_signature,
            id_list: {
                let id_list_size = block_size as usize - (std::mem::size_of::<u32>() * 2);
                let mut id_list = vec![0; id_list_size];
                cursor
                    .read_exact(&mut id_list)
                    .map_err(ExtraDataError::Read)?;
                id_list
            },
        };

        Ok(this)
    }
}
