use crate::error::ExtraDataError;
use byteorder::{ReadBytesExt, LE};
use std::convert::TryFrom;
use std::io::Cursor;

#[derive(Clone, Debug, Default)]
/// The DarwinDataBlock structure specifies an application identifier that can be used instead of a link target IDList to install an application when a shell link is activated.
pub struct DarwinDataBlock {
    block_size: u32,
    block_signature: u32,
    darwin_data_ansi: Vec<u8>,
    darwin_data_unicode: Option<Vec<u8>>,
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
