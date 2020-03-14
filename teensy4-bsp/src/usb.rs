//! Teensy 4 USB, taken from the original Teensy 4 C libraries.
//!
//! The USB stack provides a [`log`] implementation for logging over USB
//!
//! This is `Serial.println()` in Rust. Use the macros of the
//! [`log`] crate to write data over USB. Messages can be read
//! back using `screen` or `PuTTY`.
//!
//! [`log`]: https://crates.io/crates/log

use crate::interrupt; // bring in interrupt variants for #[interrupt] macro
use core::fmt;
use teensy4_usb_sys as usbsys;

/// Logging configuration
///
/// Allows a user to specify certain configurations of the logging
/// system. By default, the max log level is the log level set at
/// compile time. See the [compile time filters](https://docs.rs/log/0.4.8/log/#compile-time-filters)
/// section for more information. We also enable logging for all targets.
/// Set the `filters` collection to specify log targets of interest.
///
/// If the default configuration is good for you, use `Default::default()`
/// as the argument to `init()`.
pub struct LoggingConfig {
    /// The max log level
    ///
    /// By default, we select the static max level. Users may
    /// override this if they'd like to bypass the statically-assigned
    /// max level
    pub max_level: ::log::LevelFilter,
    /// A list of filtered targets to log.
    ///
    /// If set to an empty slice (default), the logger performs no
    /// filtering. Otherwise, we filter the specified targets by
    /// the accompanying log level. If there is no level, we default
    pub filters: &'static [(&'static str, Option<::log::LevelFilter>)],
}

impl Default for LoggingConfig {
    fn default() -> LoggingConfig {
        LoggingConfig {
            max_level: ::log::STATIC_MAX_LEVEL,
            filters: &[],
        }
    }
}

/// A handle that enables logging
///
/// Calling `init()` will initialize the USB stack and enable the USB interrupt.
/// Once initialized, messages will be written over USB.
pub struct USB(&'static mut Logger);

impl USB {
    /// Initializes the USB stack. This prepares the logging back-end. Returns a `Reader`
    /// that can read USB serial messages.
    ///
    /// To select the default logger behavior, specify `Default::default()` as the
    /// argument for `config`.
    ///
    /// This may only be called once. If this is not called, we do not initialize the logger,
    /// and log messages will not be written to the USB host.
    pub fn init(self, config: LoggingConfig) -> Reader {
        self.0.enabled = true;
        self.0.filters = config.filters;
        ::log::set_logger(self.0)
            .map(|_| ::log::set_max_level(config.max_level))
            .unwrap();
        unsafe {
            usbsys::usb_pll_start();
            usbsys::usb_init();
            cortex_m::peripheral::NVIC::unmask(crate::interrupt::USB_OTG1);
        }
        Reader(())
    }

    /// # Safety
    ///
    /// This is only called once, when we're setting up peripherals.
    /// If `init()` is called, we will set the members of the struct
    /// into their state. There can only be one Logging struct, so
    /// there's only one reference to the logger singleton.
    pub(super) fn new() -> Self {
        unsafe { USB(&mut LOGGER) }
    }
}

#[crate::rt::interrupt]
fn USB_OTG1() {
    unsafe {
        usbsys::isr();
    }
}

struct Logger {
    /// Tracks if we are (not) enabled
    enabled: bool,
    /// A collection of targets that we are expected
    /// to filter. If this is empty, we allow everything
    filters: &'static [(&'static str, Option<::log::LevelFilter>)],
}

impl Logger {
    /// Returns true if the target is in the filter, else false if the target is
    /// not in the list of kept targets. If the filter collection is empty, return
    /// true.
    fn filtered(&self, metadata: &::log::Metadata) -> bool {
        if self.filters.is_empty() {
            true
        } else if let Some(idx) = self
            .filters
            .iter()
            .position(|&(target, _)| target == metadata.target())
        {
            let (_, lvl) = self.filters[idx];
            lvl.is_none() || lvl.filter(|lvl| metadata.level() <= *lvl).is_some()
        } else {
            false
        }
    }
}

static mut LOGGER: Logger = Logger {
    enabled: false,
    filters: &[],
};

impl ::log::Log for Logger {
    fn enabled(&self, metadata: &::log::Metadata) -> bool {
        self.enabled // We're enabled
            && metadata.level() <= ::log::max_level() // The log level is appropriate
            && self.filtered(metadata) // The target is in the filter list
    }

    fn log(&self, record: &::log::Record) {
        if self.enabled(record.metadata()) {
            use core::fmt::Write;
            // Stack space for writing
            let mut buffer = [0; 256];
            let mut cursor = Cursor::new(&mut buffer);
            write!(&mut cursor, "[{} {}]: ", record.level(), record.target()).expect("infallible");
            usbsys::serial_write(&cursor);
            cursor.clear();
            writeln!(&mut cursor, "{}\r", record.args()).expect("infallible");
            usbsys::serial_write(&cursor);
            cursor.clear();
        }
    }

    fn flush(&self) {
        unsafe { usbsys::usb_serial_flush_output() }
    }
}

/// Custom cursor for writing into buffers
struct Cursor<'a> {
    buffer: &'a mut [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    fn new(buffer: &'a mut [u8]) -> Self {
        Cursor { buffer, offset: 0 }
    }

    fn clear(&mut self) {
        self.offset = 0;
    }
}

impl<'a> fmt::Write for Cursor<'a> {
    fn write_str(&mut self, msg: &str) -> fmt::Result {
        let src = msg.as_bytes();
        let buf = &mut self.buffer[self.offset..];
        let len = core::cmp::min(buf.len(), src.len());
        let buf = &mut buf[..len];
        buf.copy_from_slice(src);
        self.offset += len;
        Ok(())
    }
}

impl<'a> AsRef<[u8]> for Cursor<'a> {
    fn as_ref(&self) -> &[u8] {
        &self.buffer[..self.offset]
    }
}

/// A type that can read USB serial messages from a host
pub struct Reader(());

/// OK to transfer across 'thread' boundaries, but not safe for
/// multi-threaded access (Sync).
unsafe impl Send for Reader {}

impl Reader {
    /// Read from the USB serial endpoint into buffer. Returns the number
    /// of bytes read, or zero if there is no data.
    pub fn read(&mut self, buffer: &mut [u8]) -> usize {
        usbsys::serial_read(buffer)
    }
}
