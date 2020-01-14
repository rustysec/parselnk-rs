//! Definitions for the
//! [LinkTargetIdList](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-shllink/881d7a83-07a5-4702-93e3-f9fc34c3e1e4)
//! type.
//!

use crate::{error::LinkTargetIdListError, LinkFlags, Result, ShellLinkHeader};
use byteorder::{ReadBytesExt, LE};
use std::io::Cursor;

/// The LinkTargetIDList structure specifies the target of the link. The presence of this optional structure
/// is specified by the HasLinkTargetIDList bit (LinkFlags section 2.1.1) in the
/// ShellLinkHeader (section 2.1).
#[derive(Clone, Debug)]
pub struct LinkTargetIdList {}

impl LinkTargetIdList {
    /// Construct a new `LinkTargetIdList`
    pub fn new(cursor: &mut Cursor<Vec<u8>>, header: &ShellLinkHeader) -> Result<Self> {
        if header
            .link_flags
            .contains(LinkFlags::HAS_LINK_TARGET_ID_LIST)
        {
            let id_list_size = cursor
                .read_u16::<LE>()
                .map_err(LinkTargetIdListError::Read)?;

            let current = cursor.position();

            cursor.set_position(current + id_list_size as u64);
        }

        Ok(Self {})
    }
}
