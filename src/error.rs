//! Error types used for `parselnk`.
//!

use thiserror::Error;

#[derive(Debug, Error)]
/// Errors arising from parsing a .lnk file
pub enum Error {
    /// Specified file could not be opened
    #[error("Could not open file: {0}")]
    Open(#[from] std::io::Error),

    /// An error occurred while parsing the header fields
    #[error("Error parsing header: {0}")]
    HeaderError(#[from] HeaderError),

    /// An error occurred while parsing the `StringData` section(s)
    #[error("Error parsing string data: {0}")]
    StringDataError(#[from] StringDataError),

    /// An error occurred while parsing the `LinkTargetIdList` section
    #[error("Error parsing link target id list: {0}")]
    LinkTargetIdListError(#[from] LinkTargetIdListError),

    /// An Error occured while parsing the `LinkInfo` section
    #[error("Error parsing link info: {0}")]
    LinkInfoError(#[from] LinkInfoError),

    /// An Error occured while parsing the `ExtraData` section
    #[error("Error parsing extra data: {0}")]
    ExtraDataError(#[from] ExtraDataError),
}

#[derive(Debug, Error)]
/// An error occurred while parsing the header fields
pub enum HeaderError {
    /// An error occurred while reading the data
    #[error("could not read header: {0}")]
    Read(#[from] std::io::Error),
}

#[derive(Debug, Error)]
/// An error occurred while parsing the `LinkTargetIdList` section
pub enum LinkTargetIdListError {
    /// An error occurred while reading the data
    #[error("could not read link target id list data: {0}")]
    Read(#[from] std::io::Error),
}

#[derive(Debug, Error)]
/// An Error occured while parsing the `LinkInfo` section
pub enum LinkInfoError {
    /// An error occurred while reading the data
    #[error("could not read link info data: {0}")]
    Read(#[from] std::io::Error),
}

#[derive(Debug, Error)]
/// An error occurred while parsing the `StringData` section(s)
pub enum StringDataError {
    /// An error occurred while reading the data
    #[error("could not read string data: {0}")]
    Read(#[from] std::io::Error),

    /// Unable to convert `StringData` element to a `WideString`
    #[error("string conversion failed: {0}")]
    WideStringConversion(#[from] std::string::FromUtf16Error),

    /// Unable to convert `StringData` element to a `String`
    #[error("string conversion failed: {0}")]
    StringConversion(#[from] std::string::FromUtf8Error),

    /// Unable to read string data into `WideString`
    #[error("string conversion failed: {0}")]
    WideStringRead(#[from] widestring::MissingNulError<u16>),
}

#[derive(Debug, Error)]
/// An Error occured while parsing the `ExtraData` section
pub enum ExtraDataError {
    /// An error occurred while reading the data
    #[error("could not read extra data: {0}")]
    Read(#[from] std::io::Error),
}
