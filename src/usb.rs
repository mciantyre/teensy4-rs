//! Teensy 4 USB, taken from the original Teensy 4 C libraries
//!
//! The USB stack provides a [`log`] implementation for logging over USB.
//! It also provides a simpler [`Reader`] and [`Writer`] for performing
//! I/O over the CDC interface. Use the macros of the
//! [`log`] crate to write data over USB. Or, acquire a raw [`Reader`]
//! and [`Writer`] to perform your own USB I/O.
//!
//! [`log`]: https://crates.io/crates/log
//!
//! You're responsible for driving the USB driver with repeated calls to
//! [`poll`]. See the `poll` documentation for considerations on where to
//! call `poll`.
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
//! This example drives the USB logging system through the USB ISR.
//!
//! ```no_run
//! use teensy4_bsp as bsp;
//! use bsp::hal::ral::usb::USB1;
//! use bsp::interrupt;
//!
//! use cortex_m::interrupt::Mutex;
//! use core::cell::RefCell;
//!
//! static POLLER: Mutex<RefCell<Option<bsp::usb::Poller>>> = Mutex::new(RefCell::new(None));
//!
//! // Enable this macro for your real system!
//! // #[cortex_m_rt::interrupt]
//! fn USB_OTG1() {
//!     cortex_m::interrupt::free(|cs| {
//!         POLLER
//!             .borrow(cs)
//!             .borrow_mut()
//!             .as_mut()
//!             .map(|poller| poller.poll());
//!     });
//! }
//!
//! let (poller, _) = bsp::usb::init(
//!     USB1::take().unwrap(),
//!     bsp::usb::LoggingConfig {
//!         filters: &[("motor", None)],
//!         ..Default::default()
//!     },
//! )
//! .unwrap();
//!
//! cortex_m::interrupt::free(|cs| {
//!     *POLLER.borrow(cs).borrow_mut() = Some(poller);
//!     // Safety: invoked in a critical section that also prepares the ISR
//!     // shared memory. ISR memory is ready by the time the ISR runs.
//!     unsafe { cortex_m::peripheral::NVIC::unmask(bsp::interrupt::USB_OTG1) };
//! });
//!
//! // You can now begin logging.
//! log::info!("Hello world! 3 + 2 = {}", 5);
//! ```
//!
//! # Reader / Writer Example
//!
//! This example will manually call `poll` in an idle loop to drive USB I/O.
//!
//! ```no_run
//! use teensy4_bsp as bsp;
//! use bsp::hal::ral::usb::USB1;
//! use core::fmt::Write;
//!
//! let (mut poller, mut reader, mut writer) = bsp::usb::split(USB1::take().unwrap()).unwrap();
//!
//! write!(writer, "Hello world! 3 + 2 = {}", 5).unwrap();
//!
//! 'idle: loop {
//!     // Other work...
//!
//!     let status = poller.poll();
//!     if status.cdc_rx_complete() {
//!         // Read the data
//!         let mut buffer: [u8; 256] = [0; 256];
//!         reader.read(&mut buffer).unwrap();
//!     }
//! }
//! ```

//
// Developer notes:
//
// - We intentionally drop the RAL instance while we own it. This
//   should mean that the end user can't use the USB stack for anything
//   else, unless they mix-and-match RALs, or use another register access
//   layer.
//

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
    ///
    /// The [`init`] function may return this error.
    SetLogger,
    /// Incorrect USB instance; use the `imrt_ral`'s `USB1` instance
    ///
    /// The [`init`] or [`split`] functions may return this error.
    WrongInstance,
    /// The USB device hasn't been configured by the host
    ///
    /// Try again after the host has a chance to configure
    /// the device. If you receive this repeatedly, you might
    /// not be connected to a USB host.
    ///
    /// Any USB CDC I/O method may return this error.
    NotConfigured,
    /// Arbitrary I/O error
    ///
    /// Any USB CDC I/O method may return this error.
    Io,
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
pub fn init(inst: Instance, config: LoggingConfig) -> Result<(Poller, Reader), Error> {
    if &*inst as *const _ != USB1 {
        return Err(Error::WrongInstance);
    }
    unsafe {
        LOGGER.enabled = true;
        LOGGER.filters = Filters::new(config.filters);

        ::log::set_logger(&LOGGER).map(|_| ::log::set_max_level(config.max_level))?;
        start();
    }
    Ok((
        Poller(core::marker::PhantomData),
        Reader(core::marker::PhantomData),
    ))
}

/// Splits the USB stack into reading and writing halves, and returns both halves
///
/// The `inst` argument must be the `imxrt_ral`'s `USB1` instance. An incorrect instance
/// results in a [`Error::WrongInstance`] error.
///
/// See the [module-level documentation](mod@crate::usb) for an example.
pub fn split(inst: Instance) -> Result<(Poller, Reader, Writer), Error> {
    if &*inst as *const _ != USB1 {
        return Err(Error::WrongInstance);
    }
    unsafe { start() };
    Ok((
        Poller(core::marker::PhantomData),
        Reader(core::marker::PhantomData),
        unsafe { Writer::new() },
    ))
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
}

/// An object that can poll the USB device and driver
/// USB device I/O
///
/// Acquire `Poller` from [`init`] or [`split`].
pub struct Poller(core::marker::PhantomData<*const ()>);

impl Poller {
    /// Drive the USB device event loop
    ///
    /// `poll` must be called fast enough to handled the speed of your
    /// USB host. It will typically run as a USB high speed device.
    /// Consider calling `poll` in the `USB_OTG1` ISR, or in your idle loop.
    /// If calling `poll` in a USB ISR, make sure you unmask the interrupt.
    ///
    /// For an unsafe interface, see the [`poll`](mod@crate::usb::poll()) function.
    pub fn poll(&mut self) -> PollStatus {
        // Safety: users can only safely create one poller. That poller
        // "owns" the state modified by the poll function.
        unsafe { poll() }
    }
}

// Safety: OK to move across execution contexts; never
// safe to share across those contexts.
unsafe impl Send for Poller {}

/// The status of a [`poll`] call
///
/// ```no_run
/// use teensy4_bsp as bsp;
///
/// let status = unsafe { bsp::usb::poll() };
/// if status.cdc_rx_complete() {
///     // Received USB serial data from host
/// }
/// ```
#[derive(Debug)]
pub struct PollStatus {
    flags: u32,
}

impl PollStatus {
    /// Indicates if a CDC RX transfer was completed (true)
    /// or not completed (false) in this poll
    ///
    /// In this context, "rx" means "USB host to USB device."
    /// Check this flag to understand if your [`Reader`]
    /// might have data.
    #[inline(always)]
    pub fn cdc_rx_complete(&self) -> bool {
        self.flags & bindings::POLL_CDC_RX_COMPLETE != 0
    }

    /// Indicates if a CDC TX transfer was completed (true)
    /// or not completed (false) in this poll
    ///
    /// In this context, "tx" means "USB device to USB host."
    /// Check this flag to understand if your [`Writer`] or
    /// USB logger has scheduled a transfer.
    #[inline(always)]
    pub fn cdc_tx_complete(&self) -> bool {
        self.flags & bindings::POLL_CDC_TX_COMPLETE != 0
    }
}

/// Drive the USB device event loop
///
/// `poll` must be called fast enough to handled the speed of your
/// USB host. It will typically run as a USB high speed device.
/// Consider calling `poll` in the `USB_OTG1` ISR, or in your idle loop.
/// If calling `poll` in a USB ISR, make sure you unmask the interrupt.
///
/// For a safer polling interface, see [`Poller`].
///
/// # Safety
///
/// `poll` modifies USB driver state, and this may happen without synchronization.
/// Users are responsible for serializing calls to `poll`.
///
/// # Example
///
/// How to set up the USB ISR using the unsafe `poll` function.
///
/// ```no_run
/// use teensy4_bsp as bsp;
/// use bsp::interrupt;
///
/// // #[cortex_m_rt::interrupt]
/// unsafe fn USB_OTG1() { bsp::usb::poll(); }
///
/// // Unmask you interrupt once the USB system is enabled,
/// // and your ISR state is ready.
/// unsafe { cortex_m::peripheral::NVIC::unmask(interrupt::USB_OTG1) };
/// ```
pub unsafe fn poll() -> PollStatus {
    let flags = bindings::poll();
    PollStatus { flags }
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
            && bindings::is_configured() // The host has configured the USB device
            && metadata.level() <= ::log::max_level() // The log level is appropriate
            && self.filters.is_enabled(metadata) // The target is in the filter list
    }

    fn log(&self, record: &::log::Record) {
        if self.enabled(record.metadata()) {
            use core::fmt::Write;
            let result = cortex_m::interrupt::free(|_| {
                writeln!(
                    unsafe { Writer::new() },
                    "[{} {}]: {}",
                    record.level(),
                    record.target(),
                    record.args()
                )
            });
            assert!(result.is_ok());
        }
    }

    fn flush(&self) {
        cortex_m::interrupt::free(|_| unsafe { bindings::usb_serial_flush_output() });
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
    pub fn write<B: AsRef<[u8]>>(&mut self, buffer: B) -> Result<usize, Error> {
        let res = unsafe { bindings::serial_write(buffer) };
        match res {
            bindings::SERIAL_NOT_CONFIGURED => Err(Error::NotConfigured),
            res if res >= 0 => Ok(res as usize),
            _ => Err(Error::Io),
        }
    }

    /// Flush the written USB data
    ///
    /// `flush` may schedule an additional USB transfer to write USB
    /// data. However, it will not make your USB data appear to the host
    /// faster. You should not call `flush` in a tight USB writing loop,
    /// since the driver will attempt to pack multiple writes into a
    /// single USB transfer.
    pub fn flush(&mut self) -> Result<(), Error> {
        if !bindings::is_configured() {
            return Err(Error::NotConfigured);
        }

        unsafe { bindings::usb_serial_flush_output() };
        Ok(())
    }
}

unsafe impl Send for Writer {}

impl fmt::Write for Writer {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        if !bindings::is_configured() {
            return Ok(());
        }

        let mut at_linefeed = false;
        for line in string.split('\n') {
            if at_linefeed {
                match self.write("\r\n") {
                    Err(Error::NotConfigured) => return Ok(()),
                    Err(_) => return Err(fmt::Error),
                    Ok(size) if size < 2 => return Err(fmt::Error),
                    Ok(_) => (),
                };
            }
            let bytes = line.as_bytes();
            if !bytes.is_empty() {
                match self.write(bytes) {
                    Err(Error::NotConfigured) => return Ok(()),
                    Err(_) => return Err(fmt::Error),
                    Ok(size) if size < bytes.len() => return Err(fmt::Error),
                    Ok(_) => (),
                };
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
    pub fn read<B: AsMut<[u8]>>(&mut self, buffer: B) -> Result<usize, Error> {
        if !bindings::is_configured() {
            return Err(Error::NotConfigured);
        }

        let res = unsafe { bindings::serial_read(buffer) };
        if res < 0 {
            Err(Error::Io)
        } else {
            Ok(res as usize)
        }
    }
}
