//! Teensy 4 USB, taken from the original Teensy 4 C libraries.
//!
//! The USB stack provides a [`log`] implementation for logging over USB
//!
//! This is `Serial.println()` in Rust. Use the macros of the
//! [`log`] crate to write data over USB. Or, acquire a raw [`Reader`]
//! and [`Writer`] to perform your own USB I/O.
//!
//! [`log`]: https://crates.io/crates/log
//!
//! Most initialization functions require the `imxrt_ral`'s `USB1` instance.
//! You can acquire the instance through the HAL, which is exported by the
//! BSP:
//!
//! ```no_run
//! use teensy4_bsp as bsp;
//! use bsp::hal::ral::usb::USB1;
//!
//! let instance = USB1::take().unwrap();
//! ```
//!
//! # Logging Example
//!
//! ```no_run
//! use teensy4_bsp as bsp;
//! use bsp::hal::ral::usb::USB1;
//!
//! let core_peripherals = cortex_m::Peripherals::take().unwrap();
//! bsp::usb::init(
//!     USB1::take().unwrap(),
//!     bsp::usb::LoggingConfig {
//!         filters: &[("motor", None)],
//!         ..Default::default()
//!     },
//! )
//! .unwrap();
//!
//! log::info!("Hello world! 3 + 2 = {}", 5);
//! ```
//!
//! # Reader / Writer Example
//!
//! ```no_run
//! use teensy4_bsp as bsp;
//! use bsp::hal::ral::usb::USB1;
//! use core::fmt::Write;
//!
//! let (mut reader, mut writer) = bsp::usb::split(USB1::take().unwrap()).unwrap();
//!
//! write!(writer, "Hello world! 3 + 2 = {}", 5);
//! ```

//
// Developer notes:
//
// - We intentionally drop the RAL instance while we own it. This
//   should mean that the end user can't use the USB stack for anything
//   else, unless they mix-and-match RALs, or use another register access
//   layer.
//

#[cfg(all(target_arch = "arm", feature = "rt"))]
use crate::interrupt; // bring in interrupt variants for #[interrupt] macro
use core::fmt;
mod bindings;
mod filters;

pub use filters::Filter;
use filters::Filters;

use crate::hal::ral::usb::{Instance, USB1};

/// Logging configuration
///
/// Allows a user to specify certain configurations of the logging
/// system. By default, the max log level is the log level set at
/// compile time. See the [compile time filters](log#compile-time-filters)
/// section for more information. We also enable logging for all targets.
/// Set the `filters` collection to specify log targets of interest.
///
/// If the default configuration is good for you, use `Default::default()`
/// as the argument to [`init`](init()).
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
    pub filters: &'static [Filter],
}

impl Default for LoggingConfig {
    fn default() -> LoggingConfig {
        LoggingConfig {
            max_level: ::log::STATIC_MAX_LEVEL,
            filters: &[],
        }
    }
}

/// Indicate an error when preparing or using the USB stack
#[derive(Debug)]
pub enum Error {
    /// The error indicates that you've already set the logger, either from this
    /// interface or through another logging interface.
    SetLogger,
    /// Arbitrary IO error
    IO,
    /// Incorrect USB instance; use the `imrt_ral`'s `USB1` instance
    WrongInstance,
}

impl From<::log::SetLoggerError> for Error {
    fn from(_: ::log::SetLoggerError) -> Self {
        Error::SetLogger
    }
}

/// Initializes the USB stack. This prepares the logging back-end. Returns a `Reader`
/// that can read USB serial messages.
///
/// To select the default logger behavior, specify `Default::default()` as the
/// argument for `config`.
///
/// The `inst` argument must be the `imxrt_ral`'s `USB1` instance. An incorrect instance
/// results in a [`Error::WrongInstance`] error.
///
/// This may only be called once. If this is not called, we do not initialize the logger,
/// and log messages will not be written to the USB host. Returns a
/// [`Error::SetLogger`](Error::SetLogger) if the logging subsystem already has a
/// logger.
///
/// See the [module-level documentation](mod@crate::usb) for an example.
pub fn init(inst: Instance, config: LoggingConfig) -> Result<Reader, Error> {
    if &*inst as *const _ != USB1 {
        return Err(Error::WrongInstance);
    }
    unsafe {
        LOGGER.enabled = true;
        LOGGER.filters = Filters::new(config.filters);

        ::log::set_logger(&LOGGER).map(|_| ::log::set_max_level(config.max_level))?;
        start();
    }
    Ok(Reader(core::marker::PhantomData))
}

/// Splits the USB stack into reading and writing halves, and returns both halves
///
/// The `inst` argument must be the `imxrt_ral`'s `USB1` instance. An incorrect instance
/// results in a [`Error::WrongInstance`] error.
///
/// See the [module-level documentation](mod@crate::usb) for an example.
pub fn split(inst: Instance) -> Result<(Reader, Writer), Error> {
    if &*inst as *const _ != USB1 {
        return Err(Error::WrongInstance);
    }
    unsafe { start() };
    Ok((Reader(core::marker::PhantomData), unsafe { Writer::new() }))
}

/// Initialize the USB stack
///
/// # Safety
///
/// Must only be called once.
#[inline(always)]
unsafe fn start() {
    bindings::usb_pll_start();
    bindings::usb_init();
    cortex_m::peripheral::NVIC::unmask(crate::interrupt::USB_OTG1);
}

#[cfg(all(target_arch = "arm", feature = "rt"))]
#[crate::rt::interrupt]
fn USB_OTG1() {
    unsafe {
        bindings::isr();
    }
}

struct Logger {
    /// Tracks if we are (not) enabled
    enabled: bool,
    /// A collection of targets that we are expected
    /// to filter. If this is empty, we allow everything
    filters: Filters,
}

static mut LOGGER: Logger = Logger {
    enabled: false,
    filters: Filters::empty(),
};

impl ::log::Log for Logger {
    fn enabled(&self, metadata: &::log::Metadata) -> bool {
        self.enabled // We're enabled
            && metadata.level() <= ::log::max_level() // The log level is appropriate
            && self.filters.is_enabled(metadata) // The target is in the filter list
    }

    fn log(&self, record: &::log::Record) {
        if self.enabled(record.metadata()) {
            use core::fmt::Write;
            assert!(writeln!(
                unsafe { Writer::new() },
                "[{} {}]: {}",
                record.level(),
                record.target(),
                record.args()
            )
            .is_ok());
        }
    }

    fn flush(&self) {
        unsafe { bindings::usb_serial_flush_output() }
    }
}

/// A type that can send data to a USB serial host
///
/// Use [`Writer::write`](Writer::write()) to write byte
/// buffers. Or, use the standard `write!()` macro to serialize data to
/// the writer.
pub struct Writer(core::marker::PhantomData<*const ()>);

impl Writer {
    /// # Safety
    ///
    /// There should only be one `Writer` instance in the program. It's
    /// either given to the user, or it's used in the logger. The implementor
    /// must ensure that `Writer` isn't used in both places!
    const unsafe fn new() -> Self {
        Writer(core::marker::PhantomData)
    }

    /// Writes raw bytes to the USB serial host
    ///
    /// `write` may return a size smaller than `buffer`. This indicates that
    /// the driver could only write that many elements from the buffer. If it's
    /// important that you write a complete message, you'll need to retry the
    /// call with the rest of the data.
    ///
    /// If there was an error, the error is [`Error::IO`].
    pub fn write<B: AsRef<[u8]>>(&mut self, buffer: B) -> Result<usize, Error> {
        let res = unsafe { bindings::serial_write(buffer) };
        if res < 0 {
            Err(Error::IO)
        } else {
            Ok(res as usize)
        }
    }

    /// Flush the written USB data
    ///
    /// `flush` may schedule an additional USB transfer to write USB
    /// data. However, it will not make your USB data appear to the host
    /// faster. You should not call `flush` in a tight USB writing loop,
    /// since the driver will attempt to pack multiple writes into a
    /// single USB transfer.
    ///
    /// If there was an error, the error variant is [`Error::IO`].
    pub fn flush(&mut self) -> Result<(), Error> {
        unsafe { bindings::usb_serial_flush_output() };
        Ok(())
    }
}

unsafe impl Send for Writer {}

impl fmt::Write for Writer {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        let mut at_linefeed = false;
        for line in string.split('\n') {
            if at_linefeed {
                self.write("\r\n")
                    .map_err(|_| fmt::Error)
                    .and_then(|size| if size < 2 { Err(fmt::Error) } else { Ok(size) })?;
            }
            let bytes = line.as_bytes();
            if !bytes.is_empty() {
                self.write(bytes).map_err(|_| fmt::Error).and_then(|size| {
                    if size < bytes.len() {
                        Err(fmt::Error)
                    } else {
                        Ok(size)
                    }
                })?;
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
    ///
    /// If there is an error, the error type is [`Error::IO`].
    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        let res = unsafe { bindings::serial_read(buffer) };
        if res < 0 {
            Err(Error::IO)
        } else {
            Ok(res as usize)
        }
    }
}
