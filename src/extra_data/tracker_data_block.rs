use super::Result;
use crate::error::ExtraDataError;
use byteorder::{ReadBytesExt, LE};
use std::io::Cursor;

/// The TrackerDataBlock structure specifies data that can be used to resolve a link target if it is not found in its original location when the link is resolved. This data is passed to the Link Tracking service [MS-DLTW] to find the link target.
#[derive(Clone, Debug, Default)]
pub struct TrackerDataBlock {
    /// A 32-bit, unsigned integer that specifies the size of the TrackerDataBlock structure. This value MUST be 0x00000060.
    pub block_size: u32,

    /// A 32-bit, unsigned integer that specifies the signature of the TrackerDataBlock extra data section. This value MUST be 0xA0000003.
    pub block_signature: u32,

    /// A 32-bit, unsigned integer that specifies the size of the rest of the TrackerDataBlock structure, including this Length field. This value MUST be 0x00000058.
    pub length: u32,

    /// A 32-bit, unsigned integer. This value MUST be 0x00000000.
    pub version: u32,

    /// A NULL–terminated character string, as defined by the system default code page, which specifies the NetBIOS name of the machine where the link target was last known to reside.
    pub machine_id: u128,

    /// Two values in GUID packet representation ([MS-DTYP] section 2.3.4.2) that are used to find the link target with the Link Tracking service, as described in [MS-DLTW].
    pub droid: [u128; 2],

    /// Two values in GUID packet representation that are used to find the link target with the Link Tracking service
    pub droid_birth: [u128; 2],
}

impl TrackerDataBlock {
    /// Construct a new `TrackerDataBlock`
    pub(crate) fn new(
        block_size: u32,
        block_signature: u32,
        cursor: &mut Cursor<Vec<u8>>,
    ) -> Result<Self> {
        let this = Self {
            block_size,
            block_signature,
            length: cursor.read_u32::<LE>().map_err(ExtraDataError::Read)?,
            version: cursor.read_u32::<LE>().map_err(ExtraDataError::Read)?,
            machine_id: cursor.read_u128::<LE>().map_err(ExtraDataError::Read)?,
            droid: {
                [
                    cursor.read_u128::<LE>().map_err(ExtraDataError::Read)?,
                    cursor.read_u128::<LE>().map_err(ExtraDataError::Read)?,
                ]
            },
            droid_birth: {
                [
                    cursor.read_u128::<LE>().map_err(ExtraDataError::Read)?,
                    cursor.read_u128::<LE>().map_err(ExtraDataError::Read)?,
                ]
            },
        };

        Ok(this)
    }
}
