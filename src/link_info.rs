//! Definitions for the
//! [LinkInfo](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-shllink/6813269d-0cc8-4be2-933f-e96e8e3412dc)
//! type.
//!

use super::Result;
use crate::{error::LinkInfoError, header::ShellLinkHeader, LinkFlags};
use bitflags::bitflags;
use byteorder::{ReadBytesExt, LE};
use std::io::{Cursor, Read};

#[derive(Clone, Debug, Default)]
/// The LinkInfo structure specifies information necessary to resolve a link target if it is not found in its
/// original location. This includes information about the volume that the target was stored on, the
/// mapped drive letter, and a Universal Naming Convention (UNC) form of the path if one existed
/// when the link was created. For more details about UNC paths, see [MS-DFSNM] section 2.2.1.4.
pub struct LinkInfo {
    /// A 32-bit, unsigned integer that specifies the size, in bytes, of the LinkInfo
    /// structure. All offsets specified in this structure MUST be less than this value, and all strings
    /// contained in this structure MUST fit within the extent defined by this size.
    link_info_size: u32,

    /// A 32-bit, unsigned integer that specifies the size, in bytes, of the
    /// LinkInfo header section, which is composed of the LinkInfoSize, LinkInfoHeaderSize,
    /// LinkInfoFlags, VolumeIDOffset, LocalBasePathOffset,
    /// CommonNetworkRelativeLinkOffset, CommonPathSuffixOffset fields, and, if included, the
    /// LocalBasePathOffsetUnicode and CommonPathSuffixOffsetUnicode fields.
    link_info_header_size: u32,

    /// Flags that specify whether the VolumeID, LocalBasePath,
    /// LocalBasePathUnicode, and CommonNetworkRelativeLink fields are present in this
    /// structure.
    pub link_info_flags: Option<LinkInfoFlags>,

    /// A 32-bit, unsigned integer that specifies the location of the VolumeID
    /// field. If the VolumeIDAndLocalBasePath flag is set, this value is an offset, in bytes, from the
    /// start of the LinkInfo structure; otherwise, this value MUST be zero.
    #[allow(dead_code)]
    volume_id_offset: u32,

    /// A 32-bit, unsigned integer that specifies the location of the
    /// LocalBasePath field. If the VolumeIDAndLocalBasePath flag is set, this value is an offset, in
    /// bytes, from the start of the LinkInfo structure; otherwise, this value MUST be zero.
    local_base_path_offset: u32,

    /// A 32-bit, unsigned integer that specifies the
    /// location of the CommonNetworkRelativeLink field. If the
    /// CommonNetworkRelativeLinkAndPathSuffix flag is set, this value is an offset, in bytes, from
    /// the start of the LinkInfo structure; otherwise, this value MUST be zero.
    common_network_relative_link_offset: u32,

    /// A 32-bit, unsigned integer that specifies the location of the
    /// CommonPathSuffix field. This value is an offset, in bytes, from the start of the LinkInfo
    /// structure.
    common_path_suffix_offset: u32,

    /// An optional, 32-bit, unsigned integer that specifies the
    /// location of the LocalBasePathUnicode field. If the VolumeIDAndLocalBasePath flag is set,
    /// this value is an offset, in bytes, from the start of the LinkInfo structure; otherwise, this value
    /// MUST be zero. This field can be present only if the value of the LinkInfoHeaderSize field is
    /// greater than or equal to 0x00000024.
    local_base_path_offset_unicode: u32,

    /// An optional, 32-bit, unsigned integer that specifies
    /// the location of the CommonPathSuffixUnicode field. This value is an offset, in bytes, from the
    /// start of the LinkInfo structure. This field can be present only if the value of the
    /// LinkInfoHeaderSize field is greater than or equal to 0x00000024.
    common_path_suffix_offset_unicode: u32,

    /// An optional VolumeID structure (section 2.3.1) that specifies information
    /// about the volume that the link target was on when the link was created. This field is present if
    /// the VolumeIDAndLocalBasePath flag is set.
    pub volume_id: Option<()>,

    /// An optional, NULL–terminated string, defined by the system default code
    /// page, which is used to construct the full path to the link item or link target by appending the
    /// string in the CommonPathSuffix field. This field is present if the VolumeIDAndLocalBasePath
    /// flag is set.
    pub local_base_path: Option<String>,

    /// An optional CommonNetworkRelativeLink structure
    /// (section 2.3.2) that specifies information about the network location where the link target is
    /// stored.
    pub common_network_relative_link: Option<()>,

    /// A NULL–terminated string, defined by the system default code
    /// page, which is used to construct the full path to the link item or link target by being appended to
    /// the string in the LocalBasePath field.
    pub common_path_suffix: Option<String>,

    /// An optional, NULL–terminated, Unicode string that is used to
    /// construct the full path to the link item or link target by appending the string in the
    /// CommonPathSuffixUnicode field. This field can be present only if the
    /// VolumeIDAndLocalBasePath flag is set and the value of the LinkInfoHeaderSize field is
    /// greater than or equal to 0x00000024.
    pub local_base_path_unicode: Option<String>,

    /// An optional, NULL–terminated, Unicode string that is used
    /// to construct the full path to the link item or link target by being appended to the string in the
    /// LocalBasePathUnicode field. This field can be present only if the value of the
    /// LinkInfoHeaderSize field is greater than or equal to 0x00000024.
    pub common_path_suffix_unicode: Option<String>,
}

bitflags! {
    /// Flags that specify whether the VolumeID, LocalBasePath, LocalBasePathUnicode, and CommonNetworkRelativeLink fields are present in this structure.
    pub struct LinkInfoFlags: u32 {
        /// If set, the VolumeID and LocalBasePath fields are present, and their locations are specified by the values of the VolumeIDOffset and LocalBasePathOffset fields, respectively. If the value of the LinkInfoHeaderSize field is greater than or equal to 0x00000024, the LocalBasePathUnicode field is present, and its location is specified by the value of the LocalBasePathOffsetUnicode field.
        ///
        /// If not set, the VolumeID, LocalBasePath, and LocalBasePathUnicode fields are not present, and the values of the VolumeIDOffset and LocalBasePathOffset fields are zero. If the value of the LinkInfoHeaderSize field is greater than or equal to 0x00000024, the value of the LocalBasePathOffsetUnicode field is zero.
        const VOLUME_ID_AND_LOCAL_BASE_PATH = 0b0000_0000_0000_0000_0000_0000_0000_0001;

        /// If set, the CommonNetworkRelativeLink field is present, and its location is specified by the value of the CommonNetworkRelativeLinkOffset field.
        ///
        /// If not set, the CommonNetworkRelativeLink field is not present, and the value of the CommonNetworkRelativeLinkOffset field is zero.
        const COMMON_NETWORK_RELATIVE_LINK_AND_PATH_SUFFIX = 0b0000_0000_0000_0000_0000_0000_0000_0010;
    }
}

impl LinkInfo {
    /// Construct a new `LinkInfo` from the data in `cursor`
    pub fn new(cursor: &mut Cursor<Vec<u8>>, header: &ShellLinkHeader) -> Result<Self> {
        if header.link_flags.contains(LinkFlags::HAS_LINK_INFO) {
            let start_pos = cursor.position();

            let mut this = Self {
                link_info_size: cursor.read_u32::<LE>().map_err(LinkInfoError::Read)?,
                link_info_header_size: cursor.read_u32::<LE>().map_err(LinkInfoError::Read)?,
                link_info_flags: Some(LinkInfoFlags::from_bits_truncate(
                    cursor.read_u32::<LE>().map_err(LinkInfoError::Read)?,
                )),
                volume_id_offset: cursor.read_u32::<LE>().map_err(LinkInfoError::Read)?,
                local_base_path_offset: cursor.read_u32::<LE>().map_err(LinkInfoError::Read)?,
                common_network_relative_link_offset: cursor
                    .read_u32::<LE>()
                    .map_err(LinkInfoError::Read)?,
                common_path_suffix_offset: cursor.read_u32::<LE>().map_err(LinkInfoError::Read)?,
                local_base_path_offset_unicode: cursor
                    .read_u32::<LE>()
                    .map_err(LinkInfoError::Read)?,
                common_path_suffix_offset_unicode: cursor
                    .read_u32::<LE>()
                    .map_err(LinkInfoError::Read)?,
                volume_id: None,
                local_base_path: None,
                common_network_relative_link: None,
                common_path_suffix: None,
                local_base_path_unicode: None,
                common_path_suffix_unicode: None,
            };
            cursor.set_position(start_pos);

            if let Some(ref link_info_flags) = this.link_info_flags {
                if link_info_flags.contains(LinkInfoFlags::VOLUME_ID_AND_LOCAL_BASE_PATH) {
                    this.local_base_path = this.read_local_base_path(cursor, *link_info_flags);
                    this.common_path_suffix = this.read_common_path_suffix(cursor);
                    this.local_base_path_unicode =
                        this.read_local_base_path_unicode(cursor, *link_info_flags);
                    this.common_path_suffix_unicode =
                        this.read_common_path_suffix_unicode(cursor, *link_info_flags);

                    // TODO: Parse `VolumeID` structure
                }

                if link_info_flags
                    .contains(LinkInfoFlags::COMMON_NETWORK_RELATIVE_LINK_AND_PATH_SUFFIX)
                {
                    // TODO: Parse `CommonNetworkRelativeLink` structure
                }
            }

            cursor.set_position(this.link_info_size as u64 + start_pos);

            Ok(this)
        } else {
            Ok(Default::default())
        }
    }

    fn read_local_base_path(
        &self,
        cursor: &mut Cursor<Vec<u8>>,
        link_info_flags: LinkInfoFlags,
    ) -> Option<String> {
        let start_pos = cursor.position();
        let end_pos = if link_info_flags
            .contains(LinkInfoFlags::COMMON_NETWORK_RELATIVE_LINK_AND_PATH_SUFFIX)
        {
            self.common_network_relative_link_offset as u64 + start_pos
        } else {
            self.common_path_suffix_offset as u64 + start_pos
        } - 1;

        let begin = start_pos + self.local_base_path_offset as u64;

        if end_pos > begin {
            Self::read_string(cursor, begin, end_pos - begin).ok()
        } else {
            None
        }
    }

    fn read_common_path_suffix(&self, cursor: &mut Cursor<Vec<u8>>) -> Option<String> {
        let start_pos = cursor.position();

        let end_pos = if self.link_info_header_size >= 0x0000_0024 {
            self.local_base_path_offset_unicode as u64
        } else {
            self.link_info_size as u64
        } + start_pos
            - 1;

        let begin = start_pos + self.common_path_suffix_offset as u64;

        if end_pos > begin {
            Self::read_widestring(cursor, begin, end_pos - begin).ok()
        } else {
            None
        }
    }

    fn read_local_base_path_unicode(
        &self,
        cursor: &mut Cursor<Vec<u8>>,
        _link_info_flags: LinkInfoFlags,
    ) -> Option<String> {
        if self.link_info_header_size >= 0x0000_0024 {
            let start_pos = cursor.position();

            let end_pos = self.common_path_suffix_offset_unicode as u64 + start_pos - 1;

            let begin = start_pos + self.local_base_path_offset as u64;

            if end_pos > begin {
                Self::read_widestring(cursor, begin, end_pos - begin).ok()
            } else {
                None
            }
        } else {
            None
        }
    }

    fn read_common_path_suffix_unicode(
        &self,
        cursor: &mut Cursor<Vec<u8>>,
        _link_info_flags: LinkInfoFlags,
    ) -> Option<String> {
        if self.link_info_header_size >= 0x0000_0024 {
            let start_pos = cursor.position();

            let end_pos = self.link_info_size as u64 + start_pos - 1;

            let begin = start_pos + self.common_path_suffix_offset_unicode as u64;

            if end_pos > begin {
                Self::read_widestring(cursor, begin, end_pos - begin).ok()
            } else {
                None
            }
        } else {
            None
        }
    }

    fn read_widestring(
        cursor: &mut Cursor<Vec<u8>>,
        from: u64,
        size: u64,
    ) -> std::result::Result<String, LinkInfoError> {
        let reset = cursor.position();
        let mut data = vec![0; size as usize];

        cursor.set_position(from);
        cursor.read_exact(&mut data).map_err(LinkInfoError::Read)?;
        cursor.set_position(reset);

        let wide_data = data
            .chunks_exact(2)
            .map(|chunks| u16::from_ne_bytes([chunks[0], chunks[1]]))
            .collect::<Vec<u16>>();

        let wide = widestring::U16Str::from_slice(&wide_data).to_ustring();

        wide.to_string()
            .map_err(LinkInfoError::WideStringConversion)
    }

    fn read_string(
        cursor: &mut Cursor<Vec<u8>>,
        from: u64,
        size: u64,
    ) -> std::result::Result<String, LinkInfoError> {
        let reset = cursor.position();
        let mut data = vec![0; size as usize];

        cursor.set_position(from);
        cursor.read_exact(&mut data).map_err(LinkInfoError::Read)?;
        cursor.set_position(reset);

        String::from_utf8(data).map_err(LinkInfoError::StringConversion)
    }
}
