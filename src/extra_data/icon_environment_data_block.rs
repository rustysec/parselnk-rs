use super::Result;
use crate::error::ExtraDataError;
use std::io::{Cursor, Read};

/// The IconEnvironmentDataBlock structure specifies the path to an icon. The path is encoded using environment variables, which makes it possible to find the icon across machines where the locations vary but are expressed using environment variables.
#[derive(Clone, Debug, Default)]
pub struct IconEnvironmentDataBlock {
    /// A 32-bit, unsigned integer that specifies the size of the IconEnvironmentDataBlock structure. This value MUST be 0x00000314.
    pub block_size: u32,

    /// A 32-bit, unsigned integer that specifies the signature of the IconEnvironmentDataBlock extra data section. This value MUST be 0xA0000007.
    pub block_signature: u32,

    /// A NULL-terminated string, defined by the system default code page, which specifies a path that is constructed with environment variables.
    pub target_ansi: Option<Vec<u8>>,

    /// An optional, NULL-terminated, Unicode string that specifies a path that is constructed with environment variables.
    pub target_unicode: Option<Vec<u8>>,
}

impl IconEnvironmentDataBlock {
    /// Construct a new `IconEnvironmentDataBlock`
    pub(crate) fn new(
        block_size: u32,
        block_signature: u32,
        cursor: &mut Cursor<Vec<u8>>,
    ) -> Result<Self> {
        let this = Self {
            block_size,
            block_signature,
            target_ansi: {
                let mut target_ansi = vec![0; 260];
                cursor
                    .read_exact(&mut target_ansi)
                    .map_err(ExtraDataError::Read)?;
                Some(target_ansi)
            },
            target_unicode: {
                let mut target_unicode = vec![0; 520];
                cursor
                    .read_exact(&mut target_unicode)
                    .map_err(ExtraDataError::Read)?;
                Some(target_unicode)
            },
        };

        Ok(this)
    }
}
