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
use core::{
    fmt,
    sync::atomic::{AtomicBool, Ordering},
};
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

/// Indicate an error when preparing the USB stack
#[derive(Debug)]
pub enum Error {
    /// The error indicates that you've already set the logger, either from this
    /// interface or through another logging interface.
    SetLogger,
    /// The USB stack is already in use
    AlreadySet,
}

impl From<::log::SetLoggerError> for Error {
    fn from(_: ::log::SetLoggerError) -> Self {
        Error::SetLogger
    }
}

static TAKEN: AtomicBool = AtomicBool::new(false);

/// Initializes the USB stack. This prepares the logging back-end. Returns a `Reader`
/// that can read USB serial messages.
///
/// To select the default logger behavior, specify `Default::default()` as the
/// argument for `config`.
///
/// Before configuring the USB logger, you'll need to configure [`SysTick`](struct.SysTick.html).
/// Once you've configured `SysTick`, supply its reference here.
///
/// This may only be called once. If this is not called, we do not initialize the logger,
/// and log messages will not be written to the USB host. Returns a
/// [`SetLoggerError`](struct.SetLoggerError.html) if the logging subsystem already has a
/// logger.
pub fn init(_: &crate::SysTick, config: LoggingConfig) -> Result<Reader, Error> {
    let taken = TAKEN.swap(true, Ordering::SeqCst);
    if taken {
        return Err(Error::AlreadySet);
    }
    unsafe {
        LOGGER.enabled = true;
        LOGGER.filters = config.filters;

        ::log::set_logger(&LOGGER).map(|_| ::log::set_max_level(config.max_level))?;

        usbsys::usb_pll_start();
        usbsys::usb_init();
        cortex_m::peripheral::NVIC::unmask(crate::interrupt::USB_OTG1);
    }
    Ok(Reader(core::marker::PhantomData))
}

pub fn split() -> Result<(Reader, Writer), Error> {
    let taken = TAKEN.swap(true, Ordering::SeqCst);
    if taken {
        return Err(Error::AlreadySet);
    }
    Ok((Reader(core::marker::PhantomData), unsafe { Writer::new() }))
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
            writeln!(
                unsafe { Writer::new() },
                "[{} {}]: {}",
                record.level(),
                record.target(),
                record.args()
            )
            .expect("infallible");
        }
    }

    fn flush(&self) {
        unsafe { usbsys::usb_serial_flush_output() }
    }
}

/// A type that can send data to a USB serial host
///
/// Use [`Writer::write`](struct.Writer.html#method.write) to write byte
/// buffers. Or, use the standard `write!()` macro to serialize data to
/// the writer.
pub struct Writer(core::marker::PhantomData<*const ()>);

impl Writer {
    const unsafe fn new() -> Self {
        Writer(core::marker::PhantomData)
    }

    /// Writes raw bytes to the USB serial host
    pub fn write<B: AsRef<[u8]>>(&mut self, buffer: B) -> usize {
        usbsys::serial_write(buffer) as usize
    }
}

unsafe impl Send for Writer {}

impl fmt::Write for Writer {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        let mut at_linefeed = false;
        for line in string.split('\n') {
            if at_linefeed {
                usbsys::serial_write("\r\n");
            }
            let bytes = line.as_bytes();
            if !bytes.is_empty() {
                self.write(bytes);
            }
            at_linefeed = true;
        }
        Ok(())
    }
}

/// A type that can read USB serial messages from a host
// Uses a raw `*const ()` to ensure that Reader is not Send or Sync
pub struct Reader(core::marker::PhantomData<*const ()>);

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
