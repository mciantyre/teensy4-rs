use std::collections::HashSet;
use std::fmt;
use std::io;

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
/// Call `write()` to export it as a Rust array.
pub struct FCB {
    pub(crate) raw: [u8; FCB_SIZE],
    comments: Vec<String>,
    reserved: HashSet<usize>,
}

impl FCB {
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

    pub(super) fn field(&mut self, offset: usize, bytes: &[u8]) {
        if self.reserved.contains(&offset) {
            panic!(
                "Attempting to access reserved offset 0x{:03X} in the FCB",
                offset
            );
        } else {
            self.raw[offset..offset + bytes.len()].copy_from_slice(bytes);
        }
    }

    pub(super) fn field_comment<S: ToString>(&mut self, offset: usize, bytes: &[u8], comment: S) {
        self.field(offset, bytes);
        self.comments[offset] = comment.to_string();
    }

    /// Write the FCB as an unmangled Rust array called `FIRMWARE_CONFIGURATION_BLOCK`. The link
    /// section is `".fcb".
    pub fn write<W: io::Write>(&mut self, mut writer: W) -> io::Result<()> {
        write!(writer, "{}", self)
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
            writeln!(f, "    0x{:02X}, // 0x{:03X} {}", *value, idx * 4, comment)?;
        }
        write!(f, "];")?;
        Ok(())
    }
}
