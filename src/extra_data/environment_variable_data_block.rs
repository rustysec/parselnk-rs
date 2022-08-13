use super::Result;
use crate::error::ExtraDataError;
use std::io::{Cursor, Read};
use widestring::{U16Str, U16String};

/// The EnvironmentVariableDataBlock structure specifies a path to environment variable information when the link target refers to a location that has a corresponding environment variable.
#[derive(Clone, Debug, Default)]
pub struct EnvironmentVariableDataBlock {
    ///A 32-bit, unsigned integer that specifies the size of the EnvironmentVariableDataBlock structure. This value MUST be 0x00000314.
    pub block_size: u32,

    /// A 32-bit, unsigned integer that specifies the signature of the EnvironmentVariableDataBlock extra data section. This value MUST be 0xA0000001.
    pub block_signature: u32,

    /// A NULL-terminated string, defined by the system default code page, which specifies a path to environment variable information.
    pub target_ansi: Option<Vec<u8>>,

    /// An optional, NULL-terminated, Unicode string that specifies a path to environment variable information.
    pub target_unicode: Option<Vec<u16>>,
}

impl EnvironmentVariableDataBlock {
    /// Construct a new `EnvironmentVariableDataBlock`
    pub(crate) fn new(
        block_size: u32,
        block_signature: u32,
        cursor: &mut Cursor<Vec<u8>>,
    ) -> Result<Self> {
        let this = Self {
            block_size,
            block_signature,
            target_ansi: {
                let mut target_ansi = vec![0; 260];
                cursor
                    .read_exact(&mut target_ansi)
                    .map_err(ExtraDataError::Read)?;
                Some(target_ansi)
            },
            target_unicode: {
                let mut target_unicode = vec![0; 520];
                cursor
                    .read_exact(&mut target_unicode)
                    .map_err(ExtraDataError::Read)?;

                let result = target_unicode
                    .chunks_exact(2)
                    .map(|chunks| u16::from_ne_bytes([chunks[0], chunks[1]]))
                    .collect::<Vec<u16>>();

                Some(result)
            },
        };

        Ok(this)
    }

    /// Attempt to parse the Target ANSI property to a valid string
    pub fn target_ansi(&self) -> Result<String> {
        let ansi = self
            .target_ansi
            .clone()
            .ok_or_else(|| ExtraDataError::MissingStringData)?;

        let first_null = ansi.iter().position(|c| c == &0x00);

        let c_str = match first_null {
            Some(pos) => String::from_utf8((&ansi[0..pos]).to_vec()),
            None => String::from_utf8(ansi),
        };

        Ok(c_str
            .map_err(|_| ExtraDataError::MissingStringData)?
            .to_string())
    }

    /// Attempt to parse the Target Unicode property to a valid string
    pub fn target_unicode(&self) -> Result<String> {
        let unicode = self
            .target_unicode
            .clone()
            .ok_or_else(|| ExtraDataError::MissingStringData)?;

        let first_null = unicode.iter().position(|c| c == &0x0000);

        let c_str = match first_null {
            Some(pos) => U16Str::from_slice(&unicode[0..pos]).to_ustring(),
            None => U16String::from_vec(unicode),
        };

        Ok(c_str
            .to_string()
            .map_err(|_| ExtraDataError::MissingStringData)?
            .to_string())
    }
}
