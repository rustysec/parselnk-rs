use super::Result;
use crate::error::ExtraDataError;
use std::io::{Cursor, Read};

/// The ShimDataBlock structure specifies the name of a shim that can be applied when activating a link target.
#[derive(Clone, Debug, Default)]
pub struct ShimDataBlock {
    /// A 32-bit, unsigned integer that specifies the size of the ShimDataBlock structure. This value MUST be greater than or equal to 0x00000088.
    pub block_size: u32,

    /// A 32-bit, unsigned integer that specifies the signature of the ShimDataBlock extra data section. This value MUST be 0xA0000008.
    pub block_signature: u32,

    /// A Unicode string that specifies the name of a shim layer to apply to a link target when it is being activated.
    pub layer_name: Option<Vec<u8>>,
}

impl ShimDataBlock {
    /// Construct a new `ShimDataBlock`
    pub(crate) fn new(
        block_size: u32,
        block_signature: u32,
        cursor: &mut Cursor<Vec<u8>>,
    ) -> Result<Self> {
        let this = Self {
            block_size,
            block_signature,
            layer_name: {
                let layer_name_size = block_size as usize - (std::mem::size_of::<u32>() * 2);
                let mut layer_name = vec![0; layer_name_size];
                cursor
                    .read_exact(&mut layer_name)
                    .map_err(ExtraDataError::Read)?;
                Some(layer_name)
            },
        };

        Ok(this)
    }
}
