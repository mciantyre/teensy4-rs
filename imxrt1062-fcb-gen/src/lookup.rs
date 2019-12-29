//! Lookup table
//!
//! The lookup table is a 256 byte array of commands that's part of the general
//! FCB. We provide accessors that let you interact with the lookup table as either
//! a byte slice or slice of `u32`s.

use std::mem;
use std::ops::{Deref, DerefMut};

/// Size of the lookup table in bytes
const LOOKUP_TABLE_SIZE_BYTES: usize = 256;

/// The lookup table, part of the general FCB memory region.
///
/// `LookupTable` is a fixed-sized byte array. We provide convenience
/// methods for inserting `u32`s into the table. Just as with slices,
/// insertion of larger-sized types will panic if the computed offset
/// is out of range.
///
/// ```
/// use imxrt1062_fcb_gen::LookupTable;
///
/// let mut lookup_table = LookupTable::new();
/// lookup_table.insert_u32(0, 0xDEADBEEF);
/// assert_eq!(lookup_table[3], 0xDE);
/// ```
pub struct LookupTable([u8; LOOKUP_TABLE_SIZE_BYTES]);

impl Default for LookupTable {
    fn default() -> LookupTable {
        LookupTable([0; LOOKUP_TABLE_SIZE_BYTES])
    }
}

impl LookupTable {
    /// Create a new lookup table. All memory is set to zero.
    pub fn new() -> Self {
        Self::default()
    }

    /// Treat the underlying memory as a `u32` slice, and insert
    /// `value` at the `u32_index`.
    ///
    /// The value will be serialized as a little-endian value.
    ///
    /// # Panic
    ///
    /// Panics if the computed index is out of range.
    pub fn insert_u32(&mut self, u32_index: usize, value: u32) {
        let start = u32_index * mem::size_of::<u32>();
        let end = start + mem::size_of::<u32>();
        self.0[start..end].copy_from_slice(&value.to_le_bytes());
    }
}

impl Deref for LookupTable {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LookupTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<[u8]> for LookupTable {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for LookupTable {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn little_endian() {
        let mut lt = LookupTable::default();
        lt.insert_u32(0, 0xDEADBEEF);
        assert_eq!(lt[0], 0xEF);
        assert_eq!(lt[1], 0xBE);
        assert_eq!(lt[2], 0xAD);
        assert_eq!(lt[3], 0xDE);
    }

    #[test]
    #[should_panic]
    fn out_of_range() {
        let mut lt = LookupTable::default();
        lt.insert_u32(64, 0);
    }

    #[test]
    fn end_of_table() {
        let mut lt = LookupTable::default();
        lt.insert_u32(63, (0xAC1D << 16) | 0x1D1C);
        assert_eq!(lt[255], 0xAC);
        assert_eq!(lt[254], 0x1D);
        assert_eq!(lt[253], 0x1D);
        assert_eq!(lt[252], 0x1C);
    }
}
