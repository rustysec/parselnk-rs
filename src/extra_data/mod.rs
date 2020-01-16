//! Types defining the
//! [ExtraData](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-shllink/c41e062d-f764-4f13-bd4f-ea812ab9a4d1)
//! type.
//!

mod console_data_block;
mod console_fe_data_block;
mod darwin_data_block;
mod environment_variable_data_block;
mod icon_environment_data_block;
mod known_folder_data_block;
mod property_store_data_block;
mod shim_data_block;
mod special_folder_data_block;
mod tracker_data_block;
mod vista_and_above_id_list_data_block;

use crate::{error::ExtraDataError, header::ShellLinkHeader};
use byteorder::{ReadBytesExt, LE};
pub use console_data_block::*;
pub use console_data_block::*;
pub use console_fe_data_block::*;
pub use darwin_data_block::*;
pub use environment_variable_data_block::*;
pub use icon_environment_data_block::*;
pub use known_folder_data_block::*;
pub use property_store_data_block::*;
pub use shim_data_block::*;
pub use special_folder_data_block::*;
use std::io::Cursor;
pub use tracker_data_block::*;
pub use vista_and_above_id_list_data_block::*;

/// Result for parsing `ExtraData` blocks
type Result<T> = std::result::Result<T, ExtraDataError>;

#[derive(Clone, Debug, Default)]
/// ExtraData refers to a set of structures that convey additional information about a link target. These optional structures can be present in an extra data section that is appended to the basic Shell Link Binary File Format.
/// The ExtraData structures conform to the following ABNF rules [RFC5234]:
pub struct ExtraData {
    /// The DarwinDataBlock structure specifies an application identifier that can be used instead of a link target IDList to install an application when a shell link is activated.
    darwin_props: Option<DarwinDataBlock>,

    /// The SpecialFolderDataBlock structure specifies the location of a special folder. This data can be used when a link target is a special folder to keep track of the folder, so that the link target IDList can be translated when the link is loaded.
    special_folder_props: Option<SpecialFolderDataBlock>,

    /// The ConsoleDataBlock structure specifies the display settings to use when a link target specifies an application that is run in a console window.
    pub console_props: Option<ConsoleDataBlock>,

    /// The ConsoleFEDataBlock structure specifies the code page to use for displaying text when a link target specifies an application that is run in a console window.
    pub console_fe_props: Option<ConsoleFEDataBlock>,

    /// The EnvironmentVariableDataBlock structure specifies a path to environment variable information when the link target refers to a location that has a corresponding environment variable.
    environment_props: Option<EnvironmentVariableDataBlock>,

    /// The IconEnvironmentDataBlock structure specifies the path to an icon. The path is encoded using environment variables, which makes it possible to find the icon across machines where the locations vary but are expressed using environment variables.
    icon_environment_props: Option<IconEnvironmentDataBlock>,

    /// The KnownFolderDataBlock structure specifies the location of a known folder. This data can be used when a link target is a known folder to keep track of the folder so that the link target IDList can be translated when the link is loaded.
    known_folder_props: Option<KnownFolderDataBlock>,

    /// A PropertyStoreDataBlock structure specifies a set of properties that can be used by applications to store extra data in the shell link.
    property_store_props: Option<PropertyStoreDataBlock>,

    /// The ShimDataBlock structure specifies the name of a shim that can be applied when activating a link target.
    pub shim_props: Option<ShimDataBlock>,

    /// The TrackerDataBlock structure specifies data that can be used to resolve a link target if it is not found in its original location when the link is resolved. This data is passed to the Link Tracking service [MS-DLTW] to find the link target.
    tracker_props: Option<TrackerDataBlock>,

    /// The VistaAndAboveIDListDataBlock structure specifies an alternate IDList that can be used instead of the LinkTargetIDList structure (section 2.2) on platforms that support it.
    vista_and_above_idlist_props: Option<VistaAndAboveIDListDataBlock>,
}

impl ExtraData {
    /// Construct a new `ExtraData` instance from the data in `cursor`
    pub fn new(cursor: &mut Cursor<Vec<u8>>, _header: &ShellLinkHeader) -> Result<Self> {
        let mut this = Self::default();

        while {
            match this.parse_next_block(cursor) {
                Err(ExtraDataError::UnknownBlock(a, b)) => Err(ExtraDataError::UnknownBlock(a, b)),
                Err(_) => Ok(false),
                Ok(_) => Ok(true),
            }?
        } {}

        Ok(this)
    }

    fn parse_next_block(
        &mut self,
        cursor: &mut Cursor<Vec<u8>>,
    ) -> std::result::Result<(), ExtraDataError> {
        let block_size = cursor.read_u32::<LE>().map_err(ExtraDataError::Read)?;
        let block_signature = cursor.read_u32::<LE>().map_err(ExtraDataError::Read)?;

        match (block_size, block_signature) {
            (0x0000_0314, 0xa000_0001) => {
                self.environment_props =
                    EnvironmentVariableDataBlock::new(block_size, block_signature, cursor)
                        .map(Some)?;
                Ok(())
            }
            (0x0000_00cc, 0xa000_0002) => {
                self.console_props =
                    ConsoleDataBlock::new(block_size, block_signature, cursor).map(Some)?;
                Ok(())
            }
            (0x0000_0060, 0xa000_0003) => {
                self.tracker_props =
                    TrackerDataBlock::new(block_size, block_signature, cursor).map(Some)?;
                Ok(())
            }
            (0x0000_000c, 0xa000_0004) => {
                self.console_fe_props =
                    ConsoleFEDataBlock::new(block_size, block_signature, cursor).map(Some)?;
                Ok(())
            }
            (0x0000_0010, 0xa000_0005) => {
                self.special_folder_props =
                    SpecialFolderDataBlock::new(block_size, block_signature, cursor).map(Some)?;
                Ok(())
            }
            (0x0000_0314, 0xa000_0006) => {
                self.darwin_props =
                    DarwinDataBlock::new(block_size, block_signature, cursor).map(Some)?;
                Ok(())
            }
            (0x0000_0314, 0xa000_0007) => {
                self.icon_environment_props =
                    IconEnvironmentDataBlock::new(block_size, block_signature, cursor).map(Some)?;
                Ok(())
            }
            (_, 0xa000_0008) => {
                self.shim_props =
                    ShimDataBlock::new(block_size, block_signature, cursor).map(Some)?;
                Ok(())
            }
            (_, 0xa000_0009) => {
                self.property_store_props =
                    PropertyStoreDataBlock::new(block_size, block_signature, cursor).map(Some)?;
                Ok(())
            }
            (0x0000_001c, 0xa000_000b) => {
                self.known_folder_props =
                    KnownFolderDataBlock::new(block_size, block_signature, cursor).map(Some)?;
                Ok(())
            }
            (_, 0xa000_000c) => {
                self.vista_and_above_idlist_props =
                    VistaAndAboveIDListDataBlock::new(block_size, block_signature, cursor)
                        .map(Some)?;
                Ok(())
            }
            (size, signature) => Err(ExtraDataError::UnknownBlock(size, signature)),
        }
    }
}
