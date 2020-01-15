use super::Result;
use crate::error::ExtraDataError;
use bitflags::bitflags;
use byteorder::{ReadBytesExt, LE};
use std::io::{Cursor, Read};

/// The ConsoleDataBlock structure specifies the display settings to use when a link target specifies an application that is run in a console window.
#[derive(Clone, Debug, Default)]
pub struct ConsoleDataBlock {
    /// A 32-bit, unsigned integer that specifies the size of the ConsoleDataBlock
    /// structure. This value MUST be 0x000000CC.
    pub block_size: u32,

    /// A 32-bit, unsigned integer that specifies the signature of the
    /// ConsoleDataBlock extra data section. This value MUST be 0xA0000002.
    pub block_signature: u32,

    /// A 16-bit, unsigned integer that specifies the fill attributes that control the
    /// foreground and background text colors in the console window. The following bit definitions can be
    /// combined to specify 16 different values each for the foreground and background colors:
    pub file_attributes: FileAttributes,

    /// A 16-bit, unsigned integer that specifies the fill attributes that
    /// control the foreground and background text color in the console window popup. The values are the
    /// same as for the FillAttributes field.
    pub popup_file_attributes: u16,

    /// A 16-bit, signed integer that specifies the horizontal size (X axis), in
    /// characters, of the console window buffer.
    pub screen_buffer_size_x: u16,

    /// A 16-bit, signed integer that specifies the vertical size (Y axis), in
    /// characters, of the console window buffer.
    pub screen_buffer_size_y: u16,

    /// A 16-bit, signed integer that specifies the horizontal size (X axis), in
    /// characters, of the console window.
    pub window_size_x: u16,

    /// A 16-bit, signed integer that specifies the vertical size (Y axis), in
    /// characters, of the console window.
    pub window_size_y: u16,

    /// A 16-bit, signed integer that specifies the horizontal coordinate (X axis),
    /// in pixels, of the console window origin.
    pub window_origin_x: u16,

    /// A 16-bit, signed integer that specifies the vertical coordinate (Y axis), in
    /// pixels, of the console window origin.
    pub window_origin_y: u16,

    /// A 16-bit, signed integer that specifies the vertical coordinate (Y axis), in
    /// pixels, of the console window origin.
    _unused_1: u32,

    /// A 16-bit, signed integer that specifies the vertical coordinate (Y axis), in
    /// pixels, of the console window origin.
    _unused_2: u32,

    /// A 32-bit, unsigned integer that specifies the size, in pixels, of the font used in
    /// the console window. The two most significant bytes contain the font height and the two least
    /// significant bytes contain the font width. For vector fonts, the width is set to zero.
    pub font_size: u32,

    /// A 32-bit, unsigned integer that specifies the family of the font used in the
    /// console window. This value MUST be comprised of a font family and a font pitch. The values for
    /// the font family are shown in the following table:
    pub font_family: FontFamily,

    /// A 32-bit, unsigned integer that specifies the stroke weight of the font used in
    /// the console window.
    ///
    /// **700 ≤ value** - A bold font.
    ///
    /// **value < 700** - A regular-weight font.
    pub font_weight: u32,

    /// A 32-character Unicode string that specifies the face name of the font used
    /// in the console window.
    pub face_name: Vec<u8>,

    /// A 32-bit, unsigned integer that specifies the size of the cursor, in pixels, used
    /// in the console window.
    ///
    /// **value ≤ 25** - A small cursor.
    ///
    /// **26 — 50** - A medium cursor.
    ///
    /// **51 — 100** - A large cursor.
    pub cursor_size: u32,

    /// A 32-bit, unsigned integer that specifies whether to open the console window
    /// in full-screen mode.
    ///
    /// **0x00000000** - Full-screen mode is off.
    ///
    /// **0x00000000 < value** - Full-screen mode is on.
    pub full_screen: u32,

    /// A 32-bit, unsigned integer that specifies whether to open the console window in
    /// QuikEdit mode. In QuickEdit mode, the mouse can be used to cut, copy, and paste text in the
    /// console window.
    ///
    /// *0x00000000* - QuikEdit mode is off.
    ///
    /// *0x00000000 < value* - QuikEdit mode is on.
    pub quick_edit: u32,

    /// A 32-bit, unsigned integer that specifies insert mode in the console window.
    ///
    /// **0x00000000** - Insert mode is disabled.
    ///
    /// **0x00000000 < value** - Insert mode is enabled.
    pub insert_mode: u32,

    /// A 32-bit, unsigned integer that specifies auto-position mode of the console
    /// window.
    ///
    /// **0x00000000** - The values of the WindowOriginX and WindowOriginY fields are used to
    /// position the console window.
    ///
    /// **0x00000000 < value** - The console window is positioned automatically.
    pub auto_position: u32,

    /// A 32-bit, unsigned integer that specifies the size, in characters, of the
    /// buffer that is used to store a history of user input into the console window.
    pub history_buffer_size: u32,

    /// A 32-bit, unsigned integer that specifies the number of history
    /// buffers to use.
    pub number_of_history_buffers: u32,

    /// A 32-bit, unsigned integer that specifies whether to remove duplicates in
    /// the history buffer.
    ///
    /// **0x00000000** - Duplicates are not allowed.
    ///
    /// **0x00000000 < value** - Duplicates are allowed.
    pub history_no_dup: u32,

    /// A table of 16 32-bit, unsigned integers specifying the RGB colors that are
    /// used for text in the console window. The values of the fill attribute fields FillAttributes and
    /// PopupFillAttributes are used as indexes into this table to specify the final foreground and
    /// background color for a character.
    pub color_table: Vec<u8>,
}

bitflags! {
    /// A 16-bit, unsigned integer that specifies the fill attributes that control the
    /// foreground and background text colors in the console window. The following bit definitions can be
    /// combined to specify 16 different values each for the foreground and background colors:
    #[derive(Default)]
    pub struct FileAttributes: u16 {
        /// The foreground text color contains blue.
        const FOREGROUND_BLUE = 0x0001;

        /// The foreground text color contains green.
        const FOREGROUND_GREEN = 0x0002;

        /// The foreground text color contains red.
        const FOREGROUND_RED = 0x0004;

        /// The foreground text color is intensified.
        const FOREGROUND_INTENSITY = 0x0008;

        /// The background text color contains blue.
        const BACKGROUND_BLUE = 0x0010;

        /// The background text color contains green.
        const BACKGROUND_GREEN = 0x0020;

        /// The background text color contains red.
        const BACKGROUND_RED = 0x0040;

        /// The background text color is intensified.
        const BACKGROUND_INTENSITY = 0x0080;
    }
}

bitflags! {
    /// A 32-bit, unsigned integer that specifies the family of the font used in the
    /// console window. This value MUST be comprised of a font family and a font pitch. The values for
    /// the font family are shown in the following table:
    #[derive(Default)]
    pub struct FontFamily: u32 {
        /// The font family is unknown.
        const FF_DONTCARE = 0x0000;

        /// The font is variable-width with serifs; for example, "Times New Roman".
        const FF_ROMAN = 0x0000;

        /// The font is variable-width without serifs; for example, "Arial".
        const FF_SWISS = 0x0000;

        /// The font is fixed-width, with or without serifs; for example, "Courier New".
        const FF_MODERN = 0x0000;

        /// The font is designed to look like handwriting; for example, "Cursive".
        const FF_SCRIPT = 0x0000;

        /// The font is a novelty font; for example, "Old English".
        const FF_DECORATIVE = 0x0000;

        /// A font pitch does not apply.
        const TMPF_NONE= 0x0000;

        /// The font is a fixed-pitch font.
        const TMPF_FIXED_PITCH = 0x0000;

        /// The font is a vector font.
        const TMPF_VECTOR = 0x0000;

        /// The font is a true-type font.
        const TMPF_TRUETYPE = 0x0000;

        /// The font is specific to the device.
        const TMPF_DEVICE = 0x0000;
    }
}

impl ConsoleDataBlock {
    /// Construct a new `ConsoleDataBlock`
    pub(crate) fn new(
        block_size: u32,
        block_signature: u32,
        cursor: &mut Cursor<Vec<u8>>,
    ) -> Result<Self> {
        let cdb = ConsoleDataBlock {
            block_size,
            block_signature,
            file_attributes: FileAttributes::from_bits_truncate(
                cursor.read_u16::<LE>().map_err(ExtraDataError::Read)?,
            ),
            popup_file_attributes: cursor.read_u16::<LE>().map_err(ExtraDataError::Read)?,
            screen_buffer_size_x: cursor.read_u16::<LE>().map_err(ExtraDataError::Read)?,
            screen_buffer_size_y: cursor.read_u16::<LE>().map_err(ExtraDataError::Read)?,
            window_size_x: cursor.read_u16::<LE>().map_err(ExtraDataError::Read)?,
            window_size_y: cursor.read_u16::<LE>().map_err(ExtraDataError::Read)?,
            window_origin_x: cursor.read_u16::<LE>().map_err(ExtraDataError::Read)?,
            window_origin_y: cursor.read_u16::<LE>().map_err(ExtraDataError::Read)?,
            _unused_1: cursor.read_u32::<LE>().map_err(ExtraDataError::Read)?,
            _unused_2: cursor.read_u32::<LE>().map_err(ExtraDataError::Read)?,
            font_size: cursor.read_u32::<LE>().map_err(ExtraDataError::Read)?,
            font_family: FontFamily::from_bits_truncate(
                cursor.read_u32::<LE>().map_err(ExtraDataError::Read)?,
            ),
            font_weight: cursor.read_u32::<LE>().map_err(ExtraDataError::Read)?,
            face_name: {
                let mut face_name = [0u8; 64];
                cursor
                    .read_exact(&mut face_name)
                    .map_err(ExtraDataError::Read)?;
                face_name.to_vec()
            },
            cursor_size: cursor.read_u32::<LE>().map_err(ExtraDataError::Read)?,
            full_screen: cursor.read_u32::<LE>().map_err(ExtraDataError::Read)?,
            quick_edit: cursor.read_u32::<LE>().map_err(ExtraDataError::Read)?,
            insert_mode: cursor.read_u32::<LE>().map_err(ExtraDataError::Read)?,
            auto_position: cursor.read_u32::<LE>().map_err(ExtraDataError::Read)?,
            history_buffer_size: cursor.read_u32::<LE>().map_err(ExtraDataError::Read)?,
            number_of_history_buffers: cursor.read_u32::<LE>().map_err(ExtraDataError::Read)?,
            history_no_dup: cursor.read_u32::<LE>().map_err(ExtraDataError::Read)?,
            color_table: {
                let mut face_name = [0u8; 64];
                cursor
                    .read_exact(&mut face_name)
                    .map_err(ExtraDataError::Read)?;
                face_name.to_vec()
            },
        };

        Ok(cdb)
    }
}
