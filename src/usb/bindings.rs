//! The Teensy USB modules, compiled for Rust.
//!
//! The crate is comprised of the C sources from the
//! [Teensy Core Libraries for Arduino](https://github.com/PaulStoffregen/cores).
//! We compile the USB sources into the crate, then expose
//! the initialization and I/O routines. The crate is
//! intended for use in the `teensy4-bsp` Teensy 4 BSP.
//!
//! ## Notes on source-level changes
//!
//! We made minor changes to the C sources in order to compile without
//! warnings. Changes included fixes for things like implicit switch/case
//! fallthroughs, and unsigned / signed integer comparisons.
//!
//! We changed the usb_serial_write re-try behavior. Previously, the
//! function would wait for space in the transfer buffer, then complete the
//! write, or time out. In this implementation, we will allow a partial write.
//! The caller is responsible for implementing any re-try behaviors to complete
//! the write. This change let us remove the dependencies on symbols like
//!
//! - `systick_millis_count`
//! - `delay(uint32_t)`
//! - `yield()`
//!
//! which were used to track timeouts, and yield to the Teensyduino scheduler to
//! do other things.
//!
//! We also made changes to include paths, in order to reduce the number
//! of source files we introduced. We did not bring in the header
//! that defined the `PROGMEM`, `FLASHMEM`, and `DMAMEM` attributes.
//! We define the attributes in the build script, specifing the
//! macro values in the command-line compiler invocation.
//!
//! Finally, we removed some C++ declarations and definitions.

#[cfg_attr(target_arch = "arm", link(name = "t4usb"))]
extern "C" {
    /// Initialize the USB PLL and clocks.
    ///
    /// This must be invoked before `usb_init()`.
    pub fn usb_pll_start();
    /// Initialize the USB module. Configures clocks, endpoints, and descriptors.
    pub fn usb_init();
    /// Runs the interrupt service routine.
    #[cfg(target_arch = "arm")]
    pub fn poll() -> u32;
    /// Flush the serial buffer
    pub fn usb_serial_flush_output();
    /// Write to the USB host. Returns the number of bytes
    /// written, or a negative number if there was an error.
    ///
    /// If there's not enough space to hold the buffer contents,
    /// the function may return fewer bytes. If callers need to
    /// write the complete message, they must try to write the
    /// rest of the data themselves.
    fn usb_serial_write(buffer: *const u8, size: u32) -> i32;
    /// Read from the USB serial endpoint. Returns the number of
    /// bytes read, or a negative number for an error.
    fn usb_serial_read(buffer: *mut u8, size: u32) -> i32;
    /// Returns a non-zero value if the USB device has been configured
    /// by the host, or zero if the device is unconfigured
    fn usb_device_is_configured() -> i32;
}

// Stub for unit and documentation testing
#[cfg(not(target_arch = "arm"))]
pub unsafe fn poll() -> u32 {
    panic!("This `poll()` call should never happen")
}

//
// Keep these constants in sync with the poll_flag_t
// enum in poll.c
//

pub const POLL_CDC_RX_COMPLETE: u32 = 1;
pub const POLL_CDC_TX_COMPLETE: u32 = 2;

/// Writes the buffer of data to the USB host, returning the number
/// of bytes written
pub unsafe fn serial_write<B: AsRef<[u8]>>(buffer: B) -> i32 {
    let buffer = buffer.as_ref();
    usb_serial_write(buffer.as_ptr(), buffer.len() as u32)
}

/// Reads a buffer of data from the USB serial endpoint
pub unsafe fn serial_read<B: AsMut<[u8]>>(mut buffer: B) -> i32 {
    let buffer = buffer.as_mut();
    usb_serial_read(buffer.as_mut_ptr(), buffer.len() as u32)
}

//
// Keep these constants in sync with the error
// enum in usb_serial.c
//

pub const SERIAL_NOT_CONFIGURED: i32 = -1;

/// Indicates if the USB device has been configured by the host
pub fn is_configured() -> bool {
    unsafe { usb_device_is_configured() != 0 }
}
