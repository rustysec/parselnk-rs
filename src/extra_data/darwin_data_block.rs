use super::Result;
use crate::error::ExtraDataError;
use byteorder::{ReadBytesExt, LE};
use std::convert::TryFrom;
use std::io::{Cursor, Read};

#[derive(Clone, Debug, Default)]
/// The DarwinDataBlock structure specifies an application identifier that can be used instead of a link target IDList to install an application when a shell link is activated.
pub struct DarwinDataBlock {
    block_size: u32,
    block_signature: u32,
    darwin_data_ansi: Vec<u8>,
    darwin_data_unicode: Option<Vec<u8>>,
}

impl DarwinDataBlock {
    /// Construct a new `DarwinDataBlock`
    pub(crate) fn new(
        block_size: u32,
        block_signature: u32,
        cursor: &mut Cursor<Vec<u8>>,
    ) -> Result<Self> {
        let this = Self {
            block_size,
            block_signature,
            darwin_data_ansi: {
                let mut darwin_data_ansi = vec![0; 260];
                cursor
                    .read_exact(&mut darwin_data_ansi)
                    .map_err(ExtraDataError::Read)?;
                darwin_data_ansi
            },
            darwin_data_unicode: {
                let mut darwin_data_unicode = vec![0; 520];
                cursor
                    .read_exact(&mut darwin_data_unicode)
                    .map_err(ExtraDataError::Read)?;
                Some(darwin_data_unicode)
            },
        };

        Ok(this)
    }
}

impl TryFrom<&mut Cursor<Vec<u8>>> for DarwinDataBlock {
    type Error = ExtraDataError;

    fn try_from(cursor: &mut Cursor<Vec<u8>>) -> std::result::Result<Self, Self::Error> {
        let mut this = Self::default();

        this.block_size = cursor.read_u32::<LE>().map_err(Self::Error::Read)?;
        this.block_signature = cursor.read_u32::<LE>().map_err(Self::Error::Read)?;

        Ok(this)
    }
}
