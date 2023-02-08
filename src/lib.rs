//! A Rust board support package (BSP) for the Teensy 4.
//!
//! `teensy4-bsp` supports the following boards:
//!
//! - Teensy 4.0
//! - Teensy 4.1
//! - Teensy MicroMod
//!
//! If you're just getting started with embedded Rust development on the Teensy 4, take
//! a look at [the `board` module](crate::board). This module provides pre-configured drivers
//! and helper functions to define hardware drivers.
//!
//! Peripherals are re-exported from the [`imxrt-hal`](crate::hal)
//! hardware abstraction layer. For more information on drivers, consult the `imxrt-hal` documentation.
//! Note that `imxrt-hal` drivers depend on low-level resources from `imxrt-ral`. For convenience,
//! the BSP also exposes [`imxrt-ral`](crate::ral). Combine `imxrt-hal` and `imxrt-ral` to have full
//! control of your hardware.
//!
//! Finally, the BSP provides a runtime to simplify application development. It exposes board pins through
//! the [`pins`](crate::pins) module. And, it provides the [`imxrt-log`](crate::logging) API for advanced
//! logging features.
//!
//! # Features
//!
//! `teensy4-bsp` supports these features.
//!
//! | Flag            |         Description                                                  |
//! | --------------- | -------------------------------------------------------------------- |
//! | `"rt"`          | Adds runtime support using `imxrt-rt`.                               |
//! | `"usb-logging"` | Enables the [`LoggingFrontend`](crate::LoggingFrontend) convenience. |
//!
//! # Runtime
//!
//! When the runtime is enabled, `teensy4-bsp` defines the memory map. In order to use the memory map,
//! you must **link your program with `t4link.x`**.
//!
//! The memory organization includes
//!
//! - 320 KiB of DTCM, comprised of
//!   - a 16 KiB stack.
//!   - the vector table.
//!   - all zero- and runtime-initialized data (`.bss`, `.data`).
//! - 192 KiB of ITCM, containing _all_ instructions (`.text`).
//! - 512 KiB of OCRAM, comprised of
//!   - any uninitialized data (`.uninit`)
//!   - a 16 KiB heap.
//!
//! If the runtime is disabled, then `teensy4-bsp` does no define the memory map, and it does not
//! depend on `imxrt-rt`. Consider disabling the BSP's runtime feature if you want to implement your
//! own runtime, or if you want to use `imxrt-rt` to define your own memory map.
//!
//! # Notes
//!
//! ## SRTC reset by loader
//!
//! When the SRTC is enabled, setting the board into program mode then using the Teensy Loader
//! application (GUI) to reboot it will set the current time (Unix epoch, but time in local
//! timezone). This will overwrite whatever time you may have previously set and is ambiguous
//! around the backwards daylight savings transition point.

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use imxrt_hal as hal;
pub use imxrt_log as logging;
pub use imxrt_ral as ral;
#[cfg(all(feature = "rt", target_arch = "arm", target_os = "none"))]
#[cfg_attr(docsrs, doc(cfg(feature = "rt")))]
pub use imxrt_rt as rt;

pub use teensy4_pins as pins;

// Need to reference this so that it doesn't get stripped out
use teensy4_fcb as _;

/// Exported for RTIC. Do not use.
#[doc(hidden)]
pub struct Peripherals(ral::Instances);

#[doc(hidden)]
impl Peripherals {
    #[inline]
    pub unsafe fn steal() -> Self {
        Self(board::instances())
    }
}

#[doc(hidden)]
impl From<Peripherals> for ral::Instances {
    #[inline]
    fn from(periphs: Peripherals) -> Self {
        periphs.0
    }
}

/// Exported for RTIC. Do not use.
#[doc(hidden)]
pub use ral::{interrupt, Interrupt, NVIC_PRIO_BITS};

pub mod board;
mod clock_power;

/// SYSTICK external clock frequency.
///
/// This represents the frequency (Hz) of the external clock
/// that can supply SYSTICK.
// See Section 12.3.2.1 of the reference manual. The note
// explains that the 24MHz clock is divided down to 100KHz
// before reaching SYSTICK.
pub const EXT_SYSTICK_HZ: u32 = 100_000;

/// The logging frontend.
///
/// `LoggingFrontend` provides a convenient API for instantiating a USB1 logger.
/// It works with two different logging front-ends, described by the enum values.
///
/// When used for USB logging, the implementation registers the USB1 interrupt handler (`USB_OTG1`).
/// This requires the BSP's `"rt"` feature. Registering an interrupt handler may not be appropriate
/// for environments where interrupts are defined and registered elsewhere. If that's the case,
/// you should directly use [`logging`](crate::logging) APIs.
///
/// For advanced logging configurations, see [`logging`](crate::logging).
///
/// # Example
///
/// Register a USB logger that uses the `log` front-end.
///
/// ```no_run
/// use teensy4_bsp as bsp;
/// use bsp::board;
///
/// let board::Resources { usb, .. } = board::t40(board::instances());
/// bsp::LoggingFrontend::default_log().register_usb(usb);
/// log::info!("Hello world!");
/// ```
///
/// Register a USB logger that uses the `defmt` front-end.
///
/// ```no_run
/// # use teensy4_bsp as bsp;
/// # use bsp::board;
/// # let board::Resources { usb, .. } = board::t40(board::instances());
/// // Same as above...
/// bsp::LoggingFrontend::Defmt.register_usb(usb);
/// defmt::info!("Hello world!");
/// ```
#[cfg(all(feature = "rt", feature = "usb-logging"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "rt", feature = "usb-logging"))))]
pub enum LoggingFrontend {
    /// Use the [`log` crate](https://docs.rs/log/0.4) to write textual log messages.
    ///
    /// The logging configuration is optional; use [`default_log()`](LoggingFrontend::default_log)
    /// to select a reasonable default.
    Log(logging::log::LoggingConfig),
    /// Use the [`defmt` crate](https://docs.rs/defmt/0.3) to write compressed messages.
    ///
    /// *`defmt` requires additional setup* in order to properly build your application.
    /// Consult the `defmt` documentation for specifics.
    Defmt,
}

#[cfg(all(feature = "rt", feature = "usb-logging"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "rt", feature = "usb-logging"))))]
impl LoggingFrontend {
    /// Creates a `log` front-end with a default configuration.
    pub const fn default_log() -> Self {
        Self::Log(logging::log::LoggingConfig::new())
    }
    /// Register the USB logger.
    ///
    /// This method internally defines a USB1 interrupt handler named `USB_OTG1`.
    /// When this call returns, the interrupt is unmasked and may periodically
    /// execute.
    pub fn register_usb(self, _: crate::hal::usbd::Instances<1>) {
        #[cfg(all(target_arch = "arm", target_os = "none"))]
        {
            static mut ISR_CONFIG: LoggingFrontend = LoggingFrontend::default_log();
            #[crate::rt::interrupt]
            fn USB_OTG1() {
                static mut POLLER: Option<crate::logging::Poller> = None;
                if let Some(poller) = &mut *POLLER {
                    poller.poll();
                } else {
                    // Safety: we've "taken ownership" of the USB instances.
                    // We can fabricate those instances here.
                    let instances = unsafe {
                        crate::hal::usbd::Instances {
                            usb: crate::ral::usb::USB1::instance(),
                            usbphy: crate::ral::usbphy::USBPHY1::instance(),
                            usbnc: crate::ral::usbnc::USBNC1::instance(),
                        }
                    };

                    // Safety: memory is always initialized. It's written while the ISR
                    // is masked, then read from the ISR.
                    let poller = unsafe {
                        match &ISR_CONFIG {
                            LoggingFrontend::Log(config) => crate::logging::log::usbd_with_config(
                                instances,
                                crate::logging::Interrupts::Enabled,
                                config,
                                &crate::logging::UsbdConfigBuilder::new().build(),
                            )
                            .unwrap(),
                            LoggingFrontend::Defmt => crate::logging::defmt::usbd(
                                instances,
                                crate::logging::Interrupts::Enabled,
                            )
                            .unwrap(),
                        }
                    };
                    *POLLER = Some(poller);
                }
            }

            cortex_m::peripheral::NVIC::mask(interrupt::USB_OTG1);
            core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
            // Safety: ISR is masked, so we can safely write without a
            // torn read in the ISR.
            unsafe {
                ISR_CONFIG = self;
            }
            core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);

            // Invoke USB_OTG1 as soon as it's unmasked to initialize the USB driver.
            cortex_m::peripheral::NVIC::pend(interrupt::USB_OTG1);
            // Safety: interrupt handler state is ready.
            unsafe { cortex_m::peripheral::NVIC::unmask(interrupt::USB_OTG1) };
        }
    }
}
