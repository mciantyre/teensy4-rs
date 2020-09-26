//! The Teensy USB modules, compiled for Rust.
//!
//! The crate is comprised of the C sources from the
//! [Teensy Core Libraries for Arduino](https://github.com/PaulStoffregen/cores).
//! We compile the USB sources into the crate, then expose
//! the initialization and I/O routines. The crate is
//! intended for use in the `teensy4-bsp` Teensy 4 BSP.
//!
//! ## Notes on compilation
//!
//! The crate should compile, but it may not link without
//! a few additional symbols. The symbols may include
//!
//! - `uint32_t systick_millis_count`, a counter representing
//!   the SYSTICK
//! - `void delay(uint32_t)`, a delay function based on the
//!   SYSTICK counter
//! - `void yield(void)`, which yields control
//!
//! Dependent crates are expected to implement these C functions and
//! expose the memory.
//!
//! ## Notes on source-level changes
//!
//! We made minor changes to the C sources in order to compile without
//! warnings. Changes included fixes for things like implicit switch/case
//! fallthroughs, and unsigned / signed integer comparisons.
//!
//! We also made changes to include paths, in order to reduce the number
//! of source files we introduced. Specifically, we removed
//! instances of `#include "core_pins.h"`, which declared the `delay()`
//! and `yield()` functions. We opted to declare these functions at the
//! top of dependent source files. We also did not bring in the header
//! that defined the `PROGMEM`, `FLASHMEM`, and `DMAMEM` attributes.
//! We define the attributes in the build script, specifing the
//! macro values in the command-line compiler invocation.

#[link(name = "t4usb")]
extern "C" {
    /// Initialize the USB PLL and clocks.
    ///
    /// This must be invoked before `usb_init()`.
    pub fn usb_pll_start();
    /// Initialize the USB module. Configures clocks, endpoints, and descriptors.
    pub fn usb_init();
    /// Runs the interrupt service routine.
    pub fn isr();
    /// Flush the serial buffer
    pub fn usb_serial_flush_output();
    /// Write to the USB host. Returns the number of bytes
    /// written.
    ///
    /// The implementation never appears to return a negative value,
    /// despite returning an integer.
    fn usb_serial_write(buffer: *const u8, size: u32) -> i32;
    /// Read from the USB serila endpoint
    fn usb_serial_read(buffer: *mut u8, size: u32) -> i32;
}

/// Writes the buffer of data to the USB host, returning the number
/// of bytes written
pub fn serial_write<B: AsRef<[u8]>>(buffer: B) -> u32 {
    let buffer = buffer.as_ref();
    unsafe { usb_serial_write(buffer.as_ptr(), buffer.len() as u32) as u32 }
}

/// Reads a buffer of data from the USB serial endpoint
pub fn serial_read<B: AsMut<[u8]>>(mut buffer: B) -> usize {
    let buffer = buffer.as_mut();
    unsafe { usb_serial_read(buffer.as_mut_ptr(), buffer.len() as u32) as usize }
}
