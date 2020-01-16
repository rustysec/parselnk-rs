//! Definitions for the  
//! [ShellLinkHeader](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-shllink/c3376b21-0931-45e4-b2fc-a48ac0e60d15)
//! type.
//!

use bitflags::bitflags;
use byteorder::{ReadBytesExt, LE};
use std::io::Cursor;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// The ShellLinkHeader structure contains identification information, timestamps, and flags that specify
/// the presence of optional structures, including LinkTargetIDList (section 2.2), LinkInfo (section 2.3),
/// and StringData (section 2.4).
pub struct ShellLinkHeader {
    /// The size, in bytes, of this structure. This value MUST be 0x0000004C.
    pub header_size: u32,

    /// A class identifier (CLSID). This value MUST be 00021401-0000-0000-C000-000000000046.
    pub link_clsid: u128,

    /// A LinkFlags structure (section 2.1.1) that specifies information about the shell
    /// link and the presence of optional portions of the structure.
    pub link_flags: LinkFlags,

    /// FileAttributes (4 bytes): A FileAttributesFlags structure (section 2.1.2) that specifies information
    /// about the link target.
    pub file_attributes: FileAttributeFlags,

    /// A FILETIME structure ([MS-DTYP] section 2.3.3) that specifies the creation
    /// time of the link target in UTC (Coordinated Universal Time). If the value is zero, there is no
    /// creation time set on the link target.
    pub creation_time: u64,

    /// A FILETIME structure ([MS-DTYP] section 2.3.3) that specifies the access
    /// time of the link target in UTC (Coordinated Universal Time). If the value is zero, there is no access
    /// time set on the link target.
    pub access_time: u64,

    /// A FILETIME structure ([MS-DTYP] section 2.3.3) that specifies the write time
    /// of the link target in UTC (Coordinated Universal Time). If the value is zero, there is no write time
    /// set on the link target.
    pub write_time: u64,

    /// A 32-bit unsigned integer that specifies the size, in bytes, of the link target. If the
    /// link target file is larger than 0xFFFFFFFF, this value specifies the least significant 32 bits of the link
    /// target file size.
    pub file_size: u32,

    /// IconIndex (4 bytes): A 32-bit signed integer that specifies the index of an icon within a given icon
    /// location.
    pub icon_index: u32,

    /// ShowCommand (4 bytes): A 32-bit unsigned integer that specifies the expected
    pub show_command: ShowCommand,

    /// HotKey (2 bytes): A HotKeyFlags structure (section 2.1.3) that specifies the keystrokes used to
    /// launch the application referenced by the shortcut key. This value is assigned to the application
    /// after it is launched, so that pressing the key activates that application.
    pub hot_key: HotKeyFlags,

    /// Reserved1 (2 bytes): A value that MUST be zero.
    pub reserved1: u16,

    /// Reserved2 (4 bytes): A value that MUST be zero.
    pub reserved2: u32,

    /// Reserved3 (4 bytes): A value that MUST be zero.
    pub reserved3: u32,

    /// Human readable created on date
    #[cfg(feature = "chrono")]
    pub created_on: Option<chrono::DateTime<chrono::Utc>>,

    /// Human readable modified on date
    #[cfg(feature = "chrono")]
    pub modified_on: Option<chrono::DateTime<chrono::Utc>>,

    /// Human readable accessed on date
    #[cfg(feature = "chrono")]
    pub accessed_on: Option<chrono::DateTime<chrono::Utc>>,
}

impl std::convert::TryFrom<&mut Cursor<Vec<u8>>> for ShellLinkHeader {
    type Error = crate::error::HeaderError;
    fn try_from(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, Self::Error> {
        let mut header = Self {
            header_size: cursor.read_u32::<LE>().map_err(Self::Error::Read)?,
            link_clsid: cursor.read_u128::<LE>().map_err(Self::Error::Read)?,
            link_flags: LinkFlags::from_bits_truncate(
                cursor.read_u32::<LE>().map_err(Self::Error::Read)?,
            ),
            file_attributes: FileAttributeFlags::from_bits_truncate(
                cursor.read_u32::<LE>().map_err(Self::Error::Read)?,
            ),
            creation_time: cursor.read_u64::<LE>().map_err(Self::Error::Read)?,
            access_time: cursor.read_u64::<LE>().map_err(Self::Error::Read)?,
            write_time: cursor.read_u64::<LE>().map_err(Self::Error::Read)?,
            file_size: cursor.read_u32::<LE>().map_err(Self::Error::Read)?,
            icon_index: cursor.read_u32::<LE>().map_err(Self::Error::Read)?,
            show_command: ShowCommand::from_bits_truncate(
                cursor.read_u32::<LE>().map_err(Self::Error::Read)?,
            ),
            hot_key: HotKeyFlags::from(cursor.read_u16::<LE>().map_err(Self::Error::Read)?),
            reserved1: cursor.read_u16::<LE>().map_err(Self::Error::Read)?,
            reserved2: cursor.read_u32::<LE>().map_err(Self::Error::Read)?,
            reserved3: cursor.read_u32::<LE>().map_err(Self::Error::Read)?,
            #[cfg(feature = "chrono")]
            created_on: None,
            #[cfg(feature = "chrono")]
            modified_on: None,
            #[cfg(feature = "chrono")]
            accessed_on: None,
        };

        #[cfg(feature = "chrono")]
        {
            use chrono::{TimeZone, Utc};

            let start = Utc.ymd(1601, 1, 1).and_hms(0, 0, 0);

            header.created_on =
                Some(start + chrono::Duration::milliseconds(header.creation_time as i64 / 10000));

            header.modified_on =
                Some(start + chrono::Duration::milliseconds(header.write_time as i64 / 10000));

            header.accessed_on =
                Some(start + chrono::Duration::milliseconds(header.access_time as i64 / 10000));
        }

        Ok(header)
    }
}

bitflags! {
    /// The LinkFlags structure defines bits that specify which shell link structures are present in the file
    /// format after the ShellLinkHeader structure (section 2.1).
    pub struct LinkFlags: u32 {
        /// The shell link is saved with an item ID list (IDList). If this bit is set, a
        /// LinkTargetIDList structure (section 2.2) MUST follow the ShellLinkHeader.
        /// If this bit is not set, this structure MUST NOT be present.
        const HAS_LINK_TARGET_ID_LIST           = 0b0000_0000_0000_0000_0000_0000_0000_0001;

        /// The shell link is saved with link information. If this bit is set, a LinkInfo
        /// structure (section 2.3) MUST be present. If this bit is not set, this structure
        /// MUST NOT be present.
        const HAS_LINK_INFO                     = 0b0000_0000_0000_0000_0000_0000_0000_0010;

        ///The shell link is saved with a name string. If this bit is set, a
        ///NAME_STRING StringData structure (section 2.4) MUST be present. If
        ///this bit is not set, this structure MUST NOT be present.
        const HAS_NAME                          = 0b0000_0000_0000_0000_0000_0000_0000_0100;

        /// The shell link is saved with a relative path string. If this bit is set, a
        /// RELATIVE_PATH StringData structure (section 2.4) MUST be present. If
        /// this bit is not set, this structure MUST NOT be present.
        const HAS_RELATIVE_PATH                 = 0b0000_0000_0000_0000_0000_0000_0000_1000;

        /// The shell link is saved with a working directory string. If this bit is set, a
        /// WORKING_DIR StringData structure (section 2.4) MUST be present. If
        /// this bit is not set, this structure MUST NOT be present.
        const HAS_WORKING_DIR                   = 0b0000_0000_0000_0000_0000_0000_0001_0000;

        /// The shell link is saved with command line arguments. If this bit is set, a
        /// COMMAND_LINE_ARGUMENTS StringData structure (section 2.4) MUST
        /// be present. If this bit is not set, this structure MUST NOT be present.
        const HAS_ARGUMENTS                     = 0b0000_0000_0000_0000_0000_0000_0010_0000;

        /// The shell link is saved with an icon location string. If this bit is set, an
        /// ICON_LOCATION StringData structure (section 2.4) MUST be present. If
        /// this bit is not set, this structure MUST NOT be present.
        const HAS_ICON_LOCATION                 = 0b0000_0000_0000_0000_0000_0000_0100_0000;

        /// The shell link contains Unicode encoded strings. This bit SHOULD be set. If
        /// this bit is set, the StringData section contains Unicode-encoded strings;
        /// otherwise, it contains strings that are encoded using the system default
        /// code page.
        const IS_UNICODE                        = 0b0000_0000_0000_0000_0000_0000_1000_0000;

        /// The LinkInfo structure (section 2.3) is ignored.
        const FORCE_NO_LINK_INFO                = 0b0000_0000_0000_0000_0000_0001_0000_0000;

        /// The shell link is saved with an
        /// EnvironmentVariableDataBlock (section 2.5.4).
        const HAS_EXP_STRING                    = 0b0000_0000_0000_0000_0000_0010_0000_0000;

        /// The target is run in a separate virtual machine when launching a link
        /// target that is a 16-bit application.
        const RUN_IN_SEPARATE_PROCESS           = 0b0000_0000_0000_0000_0000_0100_0000_0000;

        /// A bit that is undefined and MUST be ignored.
        const UNUSED_1                          = 0b0000_0000_0000_0000_0000_1000_0000_0000;

        /// The shell link is saved with a DarwinDataBlock (section 2.5.3).
        const HAS_DARWIN_ID                     = 0b0000_0000_0000_0000_0001_0000_0000_0000;

        /// The application is run as a different user when the target of the shell link is
        /// activated.
        const RUN_AS_USER                       = 0b0000_0000_0000_0000_0010_0000_0000_0000;

        /// The shell link is saved with an IconEnvironmentDataBlock (section 2.5.5).
        const HAS_EXP_ICON                      = 0b0000_0000_0000_0000_0100_0000_0000_0000;

        /// The file system location is represented in the shell namespace when the
        /// path to an item is parsed into an IDList.
        const NO_PID_I_ALIAS                    = 0b0000_0000_0000_0000_1000_0000_0000_0000;

        /// A bit that is undefined and MUST be ignored.
        const UNUSED_2                          = 0b0000_0000_0000_0001_0000_0000_0000_0000;

        /// The shell link is saved with a ShimDataBlock (section 2.5.8).
        const RUN_WITH_SHIM_LAYER               = 0b0000_0000_0000_0010_0000_0000_0000_0000;

        /// The TrackerDataBlock (section 2.5.10) is ignored.
        const FORCE_NO_LINK_TRACK               = 0b0000_0000_0000_0100_0000_0000_0000_0000;

        /// The shell link attempts to collect target properties and store them in the
        /// PropertyStoreDataBlock (section 2.5.7) when the link target is set.
        const ENABLE_TARGET_METADATA            = 0b0000_0000_0000_1000_0000_0000_0000_0000;

        /// The EnvironmentVariableDataBlock is ignored.
        const DISABLE_LINK_PATH_TRACKING        = 0b0000_0000_0001_0000_0000_0000_0000_0000;

        /// The SpecialFolderDataBlock (section 2.5.9) and the
        /// KnownFolderDataBlock (section 2.5.6) are ignored when loading the shell
        /// link. If this bit is set, these extra data blocks SHOULD NOT be saved when
        /// saving the shell link.
        const DISABLE_KNOWN_FOLDER_TRACKING     = 0b0000_0000_0010_0000_0000_0000_0000_0000;

        /// If the link has a KnownFolderDataBlock (section 2.5.6), the unaliased form
        /// of the known folder IDList SHOULD be used when translating the target
        /// IDList at the time that the link is loaded.
        const DISABLE_KNOWN_FOLDER_ALIAS        = 0b0000_0000_0100_0000_0000_0000_0000_0000;

        /// Creating a link that references another link is enabled. Otherwise,
        /// specifying a link as the target IDList SHOULD NOT be allowed.
        const ALLOW_LINK_TO_LINK                = 0b0000_0000_1000_0000_0000_0000_0000_0000;

        /// When saving a link for which the target IDList is under a known folder,
        /// either the unaliased form of that known folder or the target IDList SHOULD
        /// be used.
        const UNALIAS_ON_SAVE                   = 0b0000_0001_0000_0000_0000_0000_0000_0000;

        /// The target IDList SHOULD NOT be stored; instead, the path specified in the
        /// EnvironmentVariableDataBlock (section 2.5.4) SHOULD be used to refer to
        /// the target.
        const PREFER_ENVIRONMENT_PATH           = 0b0000_0010_0000_0000_0000_0000_0000_0000;

        /// When the target is a UNC name that refers to a location on a local
        /// machine, the local path IDList in the
        /// PropertyStoreDataBlock (section 2.5.7) SHOULD be stored, so it can be
        /// used when the link is loaded on the local machine.
        const KEEP_LOCAL_ID_LIST_FOR_UNC_TARGET = 0b0000_0100_0000_0000_0000_0000_0000_0000;
    }
}

bitflags! {
    /// The FileAttributesFlags structure defines bits that specify the file attributes of the link target, if the
    /// target is a file system item. File attributes can be used if the link target is not available, or if accessing
    /// the target would be inefficient. It is possible for the target items attributes to be out of sync with this
    /// value.
    pub struct FileAttributeFlags: u32 {
        /// The file or directory is read-only. For a file, if this bit is set, applications can read the file but cannot write to it or delete it. For a directory, if this bit is set, applications cannot delete the directory.
        const FILE_ATTRIBUTE_READONLY               = 0b1000_0000_0000_0000_0000_0000_0000_0000;

        /// The file or directory is hidden. If this bit is set, the file or folder is not included in an ordinary directory listing.
        const FILE_ATTRIBUTE_HIDDEN                 = 0b0100_0000_0000_0000_0000_0000_0000_0000;

        /// The file or directory is part of the operating system or is used exclusively by the operating system.
        const FILE_ATTRIBUTE_SYSTEM                 = 0b0010_0000_0000_0000_0000_0000_0000_0000;

        /// A bit that MUST be zero.
        const RESERVED_1                            = 0b0001_0000_0000_0000_0000_0000_0000_0000;

        /// The link target is a directory instead of a file.
        const FILE_ATTRIBUTE_DIRECTORY              = 0b0000_1000_0000_0000_0000_0000_0000_0000;

        /// The file or directory is an archive file. Applications use this flag to mark files for backup or removal.
        const FILE_ATTRIBUTE_ARCHIVE                = 0b0000_0100_0000_0000_0000_0000_0000_0000;

        /// A bit that MUST be zero.
        const RESERVED_2                            = 0b0000_0010_0000_0000_0000_0000_0000_0000;

        /// The file or directory has no other flags set. If this bit is 1, all other bits in this structure MUST be clear.
        const FILE_ATTRIBUTE_NORMAL                 = 0b0000_0001_0000_0000_0000_0000_0000_0000;

        /// The file is being used for temporary storage.
        const FILE_ATTRIBUTE_TEMPORARY              = 0b0000_0000_1000_0000_0000_0000_0000_0000;

        /// The file is a sparse file.
        const FILE_ATTRIBUTE_SPARCE_FILE            = 0b0000_0000_0100_0000_0000_0000_0000_0000;

        /// The file or directory has an associated reparse point.
        const FILE_ATTRIBUTE_REPARSE_POINT          = 0b0000_0000_0010_0000_0000_0000_0000_0000;

        /// The file or directory is compressed. For a file, this means that all data in the file is compressed. For a directory, this means that compression is the default for newly created files and subdirectories.
        const FILE_ATTRIBUTE_COMPRESSED             = 0b0000_0000_0001_0000_0000_0000_0000_0000;

        /// The data of the file is not immediately available.
        const FILE_ATTRIBUTE_OFFLINE                = 0b0000_0000_0000_1000_0000_0000_0000_0000;

        /// The contents of the file need to be indexed.
        const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED    = 0b0000_0000_0000_0100_0000_0000_0000_0000;

        /// The file or directory is encrypted. For a file, this means that all data in the file is encrypted. For a directory, this means that encryption is the default for newly created files and subdirectories.
        const FILE_ATTRIBUTE_ENCRYPTED              = 0b0000_0000_0000_0010_0000_0000_0000_0000;

    }
}

bitflags! {
    /// A 32-bit unsigned integer that specifies the expected window state of an
    /// application launched by the link.
    pub struct ShowCommand: u32 {

        /// The application is open and its window is open in a normal fashion.
        const SW_SHOWNORMAL = 0x0000_0001;

        /// The application is open, and keyboard focus is given to the application, but its window is not shown.
        const SW_SHOWMAXIMIZED = 0x0000_0003;

        /// The application is open, but its window is not shown. It is not given the keyboard focus.
        const SW_SHOWMINNOACTIVE = 0x0000_0007;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// The HotKeyFlags structure specifies input generated by a combination of keyboard keys being
/// pressed.
pub struct HotKeyFlags {
    /// An 8-bit unsigned integer that specifies a virtual key code that corresponds to a
    /// key on the keyboard.
    pub low_byte: u8,

    /// An 8-bit unsigned integer that specifies bits that correspond to modifier keys on
    /// the keyboard.
    pub high_byte: u8,
}

impl From<u16> for HotKeyFlags {
    fn from(i: u16) -> Self {
        let mut cursor = Cursor::new(i.to_le_bytes());
        Self {
            low_byte: cursor.read_u8().unwrap(),
            high_byte: cursor.read_u8().unwrap(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
/// Contains a 64-bit value representing the number of 100-nanosecond intervals since January 1, 1601 (UTC).
pub struct FileTime {
    /// The low-order part of the file time.
    pub low: u32,

    /// The high-order part of the file time.
    pub high: u32,
}
