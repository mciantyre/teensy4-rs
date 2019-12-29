//! Firmware configuration block (FCB)

use std::collections::HashSet;
use std::fmt;

const FCB_SIZE: usize = 512;

/// Offsets that are listed as 'reserved' in the main FlexSPI
/// FCB table. We panic if we try to access one of these.
static OFFSETS_OF_RESERVED: &[std::ops::Range<usize>] = &[
    0x008..0x00C,
    0x011..0x012,
    0x01D..0x020,
    0x02C..0x030,
    0x03C..0x040,
    0x048..0x050,
    0x1B0..0x1C0,
    0x1CC..0x200,
];

/// The firmware configuration block
///
/// The `FCB` contains all the information to create the FCB code.
/// It implements `Display`, and it will display itself as a formatted,
/// commented Rust array. The array is called `FIRMWARE_CONFIGURATION_BLOCK`.
/// It's 512 bytes large. It has comments that describe the element offsets
/// and usage of the value.
pub struct FCB {
    /// The raw contents of the FCB. Exposed in the crate so that
    /// we can test its contents.
    pub(crate) raw: [u8; FCB_SIZE],
    /// Associated comments for each byte in the FCB. These
    /// will become Rust `//` comment tags
    comments: Vec<String>,
    /// The indices of reserved fields in the FCB.
    reserved: HashSet<usize>,
}

impl FCB {
    /// Allocates a new FCB
    ///
    /// The FCB will have the required FCB header already provided.
    pub(super) fn new() -> Self {
        let mut fcb = FCB {
            raw: [0; FCB_SIZE],
            comments: vec![String::new(); FCB_SIZE],
            reserved: OFFSETS_OF_RESERVED.iter().cloned().flatten().collect(),
        };

        fcb.comments[0] = String::from("Tag 'FCFB'");
        fcb.raw[0..4].copy_from_slice(&0x4246_4346u32.to_le_bytes());

        fcb.comments[4] = String::from("Version 'bugfix'");
        fcb.comments[5] = String::from("Version 'minor'");
        fcb.comments[6] = String::from("Version 'major");
        fcb.comments[7] = String::from("Version 'V'");
        fcb.raw[4..8].copy_from_slice(&0x5601_0000u32.to_le_bytes());

        for reserved_offset in fcb.reserved.iter() {
            fcb.comments[*reserved_offset] = String::from("RESERVED");
        }

        fcb
    }

    /// Insert a field of bytes into the FCB at the specified `offset`
    ///
    /// # Panics
    ///
    /// Panics if the offset is in a reserved range, or if the field
    /// will index past the bounds of the FCB.
    pub(super) fn field<B: AsRef<[u8]>>(&mut self, offset: usize, bytes: &B) {
        let bytes = bytes.as_ref();
        if self.reserved.contains(&offset) {
            panic!(
                "Attempting to access reserved offset 0x{:03X} in the FCB",
                offset
            );
        } else {
            self.raw[offset..offset + bytes.len()].copy_from_slice(bytes);
        }
    }

    /// Insert a field of bytes into the FCB at the specified `offset`, along with a
    /// documentation comment
    ///
    /// The comment will be placed at the `offset` element. This means that, if the field
    /// is multiple bytes, the comment will appear at the first element.
    ///
    ///  # Panics
    ///
    /// Panics if the offset is in a reserved range, or if the field
    /// will index past the bounds of the FCB.
    pub(super) fn field_comment<B: AsRef<[u8]>, S: ToString>(
        &mut self,
        offset: usize,
        bytes: &B,
        comment: S,
    ) {
        self.field(offset, bytes);
        self.comments[offset] = comment.to_string();
    }
}

impl fmt::Display for FCB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            r#"
#[link_section = ".fcb"]
#[no_mangle]
pub static FIRMWARE_CONFIGURATION_BLOCK: [u8; 512] = ["#,
        )?;
        for (idx, (value, comment)) in self.raw.iter().zip(self.comments.iter()).enumerate() {
            writeln!(f, "    0x{:02X}, // 0x{:03X} {}", *value, idx, comment)?;
        }
        writeln!(f, "];")?;
        Ok(())
    }
}
