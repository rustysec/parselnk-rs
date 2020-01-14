//! Definitions for the
//! [StringData](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-shllink/17b69472-0f34-4bcf-b290-eccdb8de224b)
//! type.
//!

use crate::{error::StringDataError, LinkFlags, Result, ShellLinkHeader};
use byteorder::{ReadBytesExt, LE};
use std::io::{Cursor, Read};
use std::path::PathBuf;

#[derive(Clone, Debug, Default)]
/// StringData refers to a set of structures that convey user interface and path identification information. The presence of these optional structures is controlled by LinkFlags (section 2.1.1) in the ShellLinkHeader (section 2.1).
/// The StringData structures conform to the following ABNF rules [RFC5234].
pub struct StringData {
    /// Description supplied by .lnk creator
    pub name_string: Option<String>,

    /// Relative path from the .lnk to the resource
    pub relative_path: Option<PathBuf>,

    /// Working directory to use when launching the resource
    pub working_dir: Option<PathBuf>,

    /// Any arguments to be passed to the resource
    pub command_line_arguments: Option<String>,

    /// Icon displayed for the .lnk
    pub icon_location: Option<PathBuf>,
}

impl StringData {
    /// Parses the string value found at the beginning of `cursor`. If `unicode`
    /// is `true`, attempt to parse it as a wide string.
    fn parse_string(cursor: &mut Cursor<Vec<u8>>, unicode: bool) -> Result<String> {
        let count_characters =
            if unicode { 2 } else { 1 } * cursor.read_u16::<LE>().map_err(StringDataError::Read)?;

        let mut string_data: Vec<u8> = vec![0; count_characters as usize];

        cursor
            .read_exact(&mut string_data)
            .map_err(StringDataError::Read)?;

        if unicode {
            let wide_data = string_data
                .chunks_exact(2)
                .map(|chunk| u16::from_ne_bytes([chunk[0], chunk[1]]))
                .collect::<Vec<u16>>();

            let wide = widestring::U16Str::from_slice(&wide_data).to_ustring();

            wide.to_string()
                .map_err(|e| StringDataError::WideStringConversion(e).into())
        } else {
            String::from_utf8(string_data).map_err(|e| StringDataError::StringConversion(e).into())
        }
    }

    /// Build new `StringData` from data blob.
    pub fn new(cursor: &mut Cursor<Vec<u8>>, header: &ShellLinkHeader) -> Result<Self> {
        let mut this = StringData::default();

        if header.link_flags.contains(LinkFlags::HAS_NAME) {
            this.name_string =
                Self::parse_string(cursor, header.link_flags.contains(LinkFlags::IS_UNICODE)).ok();
        }
        if header.link_flags.contains(LinkFlags::HAS_RELATIVE_PATH) {
            this.relative_path = Some(PathBuf::from(&Self::parse_string(
                cursor,
                header.link_flags.contains(LinkFlags::IS_UNICODE),
            )?));
        }
        if header.link_flags.contains(LinkFlags::HAS_WORKING_DIR) {
            this.working_dir = Some(PathBuf::from(&Self::parse_string(
                cursor,
                header.link_flags.contains(LinkFlags::IS_UNICODE),
            )?));
        }
        if header.link_flags.contains(LinkFlags::HAS_ARGUMENTS) {
            this.command_line_arguments = Some(Self::parse_string(
                cursor,
                header.link_flags.contains(LinkFlags::IS_UNICODE),
            )?);
        }
        if header.link_flags.contains(LinkFlags::HAS_ICON_LOCATION) {
            this.icon_location = Some(PathBuf::from(&Self::parse_string(
                cursor,
                header.link_flags.contains(LinkFlags::IS_UNICODE),
            )?));
        }
        Ok(this)
    }
}
