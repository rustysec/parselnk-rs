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
    path: Option<PathBuf>,
    header: ShellLinkHeader,
    string_data: StringData,
    link_target_id_list: LinkTargetIdList,
    link_info: LinkInfo,
    extra_data: ExtraData,
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
