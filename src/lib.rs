//! A Rust board support package (BSP) for the Teensy 4. Supports the Teensy 4.0 and
//! 4.1 boards.
//!
//! Peripherals are re-exported from the [`imxrt-rs`](https://docs.rs/imxrt-hal/latest/imxrt_hal/)
//! hardware abstraction layer. See the HAL's documentation for more information on creating
//! and using peripherals.
//!
//! The BSP restricts the processor pads that are available, since the physical Teensy
//! only has a few user-accessible pins. From these pins, you may construct peripherals
//! and perform I/O. The two Teensy 4 boards support many of the same pins; see the
//! [`common`](common/index.html) module for those similar pins. To construct Teensy 4.0
//! or 4.1 pins, see the corresponding `pins` function in each of the corresponding modules.
//!
//! The BSP assumes some facilities of the processor, both which are required for the
//! USB stack. Each are controllable through feature-flags. Each feature is on by default.
//!
//! - it registers the `SysTick` exception handler, and configures
//!   SYSTICK for a 1ms interrupt. Enabled with the `"systick"` feature,
//!   which is on by default.
//! - it registers the `USB_OTG1` interrupt, and uses the USB1
//!   peripheral for logging. Enabled with the `"usb-logging"` feature,
//!   which is on by default. Depends on the `"systick"` feature.
//!
//! If a user also registers a `SysTick` or `USB_OTG1` handler, it may
//! result in a duplicate definition error.
//!
//! # Re-exports
//!
//! The BSP re-exports the following:
//!
//! - the `teensy4-rt` crate, as `rt`
//! - the `imxrt-hal` crate, as `hal`
//!
//! See the accompanying documentation of each crate for more
//! information.
//!
//! For simplicity, there may be other choice APIs from either crate that
//! are re-exported in the BSP namespace.
//!
//! Although it's not exported publicly, the BSP crate links in the
//! `teensy4-fcb` crate, which provides a Firmware Configuration Block (FCB)
//! specific to the Teensy 4. See the `teensy4-fcb` crate for details
//! on FCBs.
//!
//! # Examples
//!
//! Turn on a Teensy 4.0's LED:
//!
//! ```no_run
//! # #![feature(lang_items)]
//! #![no_std]
//! #![no_main]
//!
//! extern crate panic_halt;
//!
//! use bsp::rt::entry;
//! use cortex_m::asm::wfi;
//! use teensy4_bsp as bsp;
//!
//! use embedded_hal::digital::v2::OutputPin;
//!
//! #[entry]
//! fn main() -> ! {
//!     let peripherals = bsp::Peripherals::take().unwrap();
//!     let pins = bsp::t40::pins(peripherals.iomuxc);
//!     let mut led = bsp::configure_led(pins.p13);
//!
//!     loop {
//!         led.set_high().unwrap();
//!         wfi();
//!     }
//! }
//! # #[lang = "eh_personality"] extern fn eh_personality() {}
//! ```
//!
//! See the `teensy4-examples` crate for build-able, run-able
//! examples. The examples utilize this BSP crate to blink LEDs,
//! establish timers, and log data over USB.
//!
//! # Using RTIC
//!
//! To develop Teensy 4 applications with the RTIC framework,
//!
//! 1. disables the `teensy4-bsp`'s default features
//! 2. enable the BSP's `"rtic"` feature
//! 3. patch the `cortex-m-rt` crate with the Teensy 4's special runtime, available in
//!    the `teensy4-bsp`'s repository.
//!
//! All three steps can be summarized by this `Cargo.toml` snippet:
//!
//! ```toml
//! [dependencies.teensy4-bsp]
//! git = "https://github.com/mciantyre/teensy4-rs"
//! branch = "master"
//! default-features = false
//! features = ["rtic"]
//!
//! [patch.crates-io.cortex-m-rt]
//! git = "https://github.com/mciantyre/teensy4-rs"
//! branch = "master"
//! ```
//!
//! You need to disable the BSP's default features, which disables the `"systick"` feature,
//! to enable RTIC's SYSTICK handler. This means that you cannot use the USB logger when
//! developing RTIC applications. Consider using the [`imxrt-uart-log`](https://crates.io/crates/imxrt-uart-log)
//! crate for an alternate logging implementation.
//!
//! You need to replace the typical `cortex-m-rt` runtime with our custom runtime. The patch
//! above lets us replace RTIC's runtime crate with an API-compatible runtime.
//!
//! ## Notice of alpha status
//!
//! We've made some assumptions in this MVP BSP.
//!
//! - SYSTICK and delay implementation is very naive. Do not run for 49
//!   continuous days, or risk a millisecond counter wrap-around.

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

// Need to reference this so that it doesn't get stripped out
#[cfg(target_arch = "arm")]
extern crate teensy4_fcb;

pub use teensy4_pins::common;
pub use teensy4_pins::t40;
pub use teensy4_pins::t41;

#[cfg(feature = "systick")]
mod systick;
#[cfg(feature = "usb-logging")]
#[cfg_attr(docsrs, doc(cfg(feature = "usb-logging")))]
pub mod usb;

#[cfg(feature = "systick")]
pub use systick::SysTick;

pub use hal::ral::interrupt;
// `rtic` expects these in the root.
#[doc(hidden)]
#[cfg(feature = "rtic")]
pub use hal::ral::{interrupt as Interrupt, NVIC_PRIO_BITS};

pub use cortex_m_rt as rt;
pub use hal::Peripherals;
pub use imxrt_hal as hal;

/// The LED
///
/// See [`configure_led`](fn.configure_led.html) to prepare the LED.
pub type LED = hal::gpio::GPIO<common::P13, hal::gpio::Output>;

/// Configure the board's LED
///
/// Returns a GPIO that's physically tied to the LED. Use the returned handle
/// to drive the LED.
pub fn configure_led(pad: common::P13) -> LED {
    let mut led = hal::gpio::GPIO::new(pad);
    led.set_fast(true);
    led.output()
}

/// TODO(mciantyre) define a better yield
#[no_mangle]
fn r#yield() {
    // 'yield' is a Rust keyword. But, it needs to be called 'yield' for the C USB stack
    cortex_m::asm::delay(1024);
}
