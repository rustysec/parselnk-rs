//! Types defining the
//! [ExtraData](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-shllink/c41e062d-f764-4f13-bd4f-ea812ab9a4d1)
//! type.
//!

mod console_data_block;
mod darwin_data_block;

use crate::{
    header::{LinkFlags, ShellLinkHeader},
    Result,
};
pub use console_data_block::*;
pub use darwin_data_block::*;
use std::convert::TryFrom;
use std::io::Cursor;

#[derive(Clone, Debug, Default)]
/// ExtraData refers to a set of structures that convey additional information about a link target. These optional structures can be present in an extra data section that is appended to the basic Shell Link Binary File Format.
/// The ExtraData structures conform to the following ABNF rules [RFC5234]:
pub struct ExtraData {
    darwin_props: Option<DarwinDataBlock>,
}

impl ExtraData {
    /// Construct a new `ExtraData` instance from the data in `cursor`
    pub fn new(cursor: &mut Cursor<Vec<u8>>, header: &ShellLinkHeader) -> Result<Self> {
        let mut this = Self::default();

        if header.link_flags.contains(LinkFlags::HAS_DARWIN_ID) {
            this.darwin_props = DarwinDataBlock::try_from(cursor).ok();
        }

        Ok(this)
    }
}
