//! EEPROM emulation
//!
//! EEPROM emulation uses a small (~1KiB) region of flash to persist data.

use core::sync::atomic::{AtomicBool, Ordering};

/// Bindings to the Teensy 4's EEPROM module
#[cfg_attr(target_arch = "arm", link(name = "t4eeprom"))]
extern "C" {
    pub fn eeprom_initialize();
    pub fn eeprom_read_byte(addr_ptr: *const u8) -> u8;
    pub fn eeprom_write_byte(addr_ptr: *const u8, data: u8);
    // TODO the rest...
}

static TAKEN: AtomicBool = AtomicBool::new(false);

/// Provides read/write access to EEPROM
///
/// There's only one of these available in a given program.
pub struct Eeprom {}

impl Eeprom {
    /// Create an `Eeprom` that controls I/O with the EEPROM emulation region
    ///
    /// Returns `None` if the `Eeprom` has already been created.
    pub fn new() -> Option<Self> {
        let taken = TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            unsafe { eeprom_initialize() };
            Some(Eeprom {})
        }
    }
    /// Read a byte from the EEPROM emulation region
    ///
    /// TODO what happens when out of range? Should handle here...
    pub fn read_byte(&self, index: usize) -> u8 {
        unsafe { eeprom_read_byte(index as _) }
    }
    /// Write a byte into the EEPROM emulated region
    ///
    /// TODO what happens when out of range? Should handle here...
    pub fn write_byte(&mut self, index: usize, byte: u8) {
        unsafe { eeprom_write_byte(index as _, byte) };
    }
}
