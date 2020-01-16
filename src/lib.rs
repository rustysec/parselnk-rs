//! Parse windows .lnk files using only safe rust. Windows lnk files
//! describe links to data objects as defined by
//! [this specification](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-shllink/16cb4ca1-9339-4d0c-a68d-bf1d6cc0f943).
//!
//! # Examples
//!
//! You can process the `Lnk` data from a memory buffer that implements
//! `std::io::Read`.
//!
//! ```no_run
//! use parselnk::Lnk;
//! use std::convert::TryFrom;
//!
//! let mut lnk_data: Vec<u8> = Vec::new();
//! // read your link into `lnk_data` here...
//! let lnk = Lnk::try_from(lnk_data);
//! ```
//!
//! Or you can process any `Lnk` on disk.
//! ```no_run
//! use parselnk::Lnk;
//! use std::convert::TryFrom;
//!
//! let path = std::path::Path::new("c:\\users\\me\\shortcut.lnk");
//!
//! let lnk = Lnk::try_from(path).unwrap();
//! ```

#![warn(missing_docs)]

pub mod error;
pub mod extra_data;
pub mod header;
pub mod link_info;
pub mod link_target_id_list;
pub mod string_data;

pub use extra_data::*;
pub use header::*;
pub use link_info::*;
pub use link_target_id_list::*;
use std::convert::TryFrom;
use std::path::PathBuf;
pub use string_data::*;

/// Result type wrapping around `parselnk::error::Error`
pub type Result<T> = std::result::Result<T, error::Error>;

/// Represents a windows .lnk file
#[derive(Clone, Debug)]
pub struct Lnk {
    /// Path to the `.lnk` file
    path: Option<PathBuf>,

    /// The ShellLinkHeader structure contains identification information, timestamps, and flags that specify the presence of optional structures, including LinkTargetIDList (section 2.2), LinkInfo (section 2.3), and StringData (section 2.4).
    pub header: ShellLinkHeader,

    /// StringData refers to a set of structures that convey user interface and path identification information. The presence of these optional structures is controlled by LinkFlags (section 2.1.1) in the ShellLinkHeader (section 2.1).
    pub string_data: StringData,

    /// The LinkTargetIDList structure specifies the target of the link. The presence of this optional structure is specified by the HasLinkTargetIDList bit (LinkFlags section 2.1.1) in the ShellLinkHeader (section 2.1).
    pub link_target_id_list: LinkTargetIdList,

    /// The LinkInfo structure specifies information necessary to resolve a link target if it is not found in its original location. This includes information about the volume that the target was stored on, the mapped drive letter, and a Universal Naming Convention (UNC) form of the path if one existed when the link was created. For more details about UNC paths, see [MS-DFSNM] section 2.2.1.4.:w
    pub link_info: LinkInfo,

    /// ExtraData refers to a set of structures that convey additional information about a link target. These optional structures can be present in an extra data section that is appended to the basic Shell Link Binary File Format.
    pub extra_data: ExtraData,
}

impl Lnk {
    /// Creates a new `Lnk` from a `Read` source.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use parselnk::Lnk;
    /// use std::fs::File;
    ///
    /// let mut file = File::open(r"c:\users\me\desktop\firefox.lnk").unwrap();
    /// let lnk = Lnk::new(&mut file);
    /// ```
    ///
    pub fn new<S: std::io::Read>(reader: &mut S) -> Result<Lnk> {
        let mut data_buf = Vec::new();
        reader
            .read_to_end(&mut data_buf)
            .map_err(error::HeaderError::Read)?;

        let mut cursor = std::io::Cursor::new(data_buf);

        let header = ShellLinkHeader::try_from(&mut cursor)?;
        let link_target_id_list = LinkTargetIdList::new(&mut cursor, &header)?;
        let link_info = LinkInfo::new(&mut cursor, &header)?;
        let string_data = StringData::new(&mut cursor, &header)?;
        let extra_data = ExtraData::new(&mut cursor, &header)?;

        Ok(Lnk {
            path: None,
            header,
            string_data,
            link_target_id_list,
            link_info,
            extra_data,
        })
    }

    /// The command line arguments supplied via the `Lnk`
    pub fn arguments(&self) -> Option<String> {
        self.string_data.command_line_arguments.clone()
    }

    /// The relative path to the resource of the `Lnk``
    pub fn relative_path(&self) -> Option<PathBuf> {
        self.string_data.relative_path.clone()
    }

    /// The working directory of the `Lnk`
    pub fn working_dir(&self) -> Option<PathBuf> {
        self.string_data.working_dir.clone()
    }

    /// The description of the `Lnk`
    pub fn description(&self) -> Option<String> {
        self.string_data.name_string.clone()
    }

    /// The creation `FileTime` as a u64
    pub fn creation_time(&self) -> u64 {
        self.header.creation_time
    }

    /// The access `FileTime` as a u64
    pub fn access_time(&self) -> u64 {
        self.header.access_time
    }

    /// The write `FileTime` as a u64
    pub fn write_time(&self) -> u64 {
        self.header.write_time
    }

    /// The creation `FileTime` as a `DateTime`
    #[cfg(feature = "chrono")]
    pub fn created_on(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.header.created_on
    }

    /// The access `FileTime` as a `DateTime`
    #[cfg(feature = "chrono")]
    pub fn accessed_on(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.header.accessed_on
    }

    /// The write `FileTime` as a `DateTime`
    #[cfg(feature = "chrono")]
    pub fn modified_on(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.header.modified_on
    }
}

impl TryFrom<&std::path::Path> for Lnk {
    type Error = crate::error::Error;

    fn try_from(p: &std::path::Path) -> std::result::Result<Self, Self::Error> {
        let mut f = std::fs::File::open(p).map_err(crate::error::Error::from)?;
        Lnk::new(&mut f).map(|mut lnk| {
            lnk.path = Some(p.to_path_buf());
            lnk
        })
    }
}

impl TryFrom<&[u8]> for Lnk {
    type Error = crate::error::Error;

    fn try_from(mut p: &[u8]) -> std::result::Result<Self, Self::Error> {
        Lnk::new(&mut p)
    }
}

impl TryFrom<Vec<u8>> for Lnk {
    type Error = crate::error::Error;

    fn try_from(p: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Lnk::new(&mut &p[0..])
    }
}

impl TryFrom<&Vec<u8>> for Lnk {
    type Error = crate::error::Error;

    fn try_from(p: &Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Lnk::new(&mut &p[0..])
    }
}

#[cfg(test)]
mod tests {
    use crate::Lnk;
    use std::convert::TryFrom;
    use std::path::Path;

    #[test]
    fn firefox() {
        let path = Path::new("./test_data/firefox.lnk");
        assert!(Lnk::try_from(path).is_ok());
    }

    #[test]
    fn commander() {
        let path = Path::new("./test_data/commander.lnk");
        assert!(Lnk::try_from(path).is_ok());
    }

    #[test]
    fn notepad() {
        let path = Path::new("./test_data/notepad.lnk");
        assert!(Lnk::try_from(path).is_ok());
    }

    #[test]
    fn xp_outlook_express() {
        let path = Path::new("./test_data/outlook_express.lnk");
        assert!(Lnk::try_from(path).is_ok());
    }
}
