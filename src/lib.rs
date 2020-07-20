//! A Rust board support package (BSP) for the Teensy 4.
//!
//! The BSP is mainly a pass-through of the `imxrt-hal` hardware abstraction layer.
//! The BSP restricts the processor pads that are available, since the physical Teensy
//! only has a few user-accessible pins. From these pins, you may construct peripherals
//! and perform I/O.
//!
//! The BSP also exposes a USB logging interface. See the [`usb`](usb/index.html) module
//! for more details.
//!
//! The BSP does assume some facilities of the processor, both which are required for the
//! USB stack. Each are controllable through feature-flags. Each feature is on by default.
//!
//! - it registers the `SysTick` exception handler, and configures
//!   SYSTICK for a 1ms interrupt. Enabled with the `"systick"` feature,
//!   which is on by default.
//! - it registers the `USB_OTG1` interrupt, and uses the USB1
//!   peripheral for logging. Enabled with the `"usb-logging"` feature,
//!   which is on by default. Depends on the `"systick"` feature.
//!
//! These peripherals and capabilities are not exported from the BSP.
//! If a user also registers a `SysTick` or `USB_OTG1` handler, it may
//! result in a duplicate definition error.
//!
//! ## Re-exports
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
//! ## Examples
//!
//! See the `teensy4-examples` crate for build-able, run-able
//! examples. The examples utilize this BSP crate to blink LEDs,
//! establish timers, and log data over USB.
//!
//! ## Notice of alpha status
//!
//! We've made some assumptions in this MVP BSP.
//!
//! - SYSTICK and delay implementation is very naive. Do not run for 49
//!   continuous days, or risk a millisecond counter wrap-around.

#![no_std]

// Need to reference this so that it doesn't get stripped out
extern crate teensy4_fcb;

pub mod t40;

#[cfg(feature = "systick")]
mod systick;
#[cfg(feature = "usb-logging")]
pub mod usb;

#[cfg(feature = "systick")]
pub use systick::SysTick;
#[cfg(feature = "usb-logging")]
pub use usb::{init as usb_init, LoggingConfig};

pub use hal::ral::interrupt;
// `rtic` expects these in the root.
#[cfg(feature = "rtic")]
pub use hal::ral::{interrupt as Interrupt, NVIC_PRIO_BITS};

pub use cortex_m_rt as rt;
pub use imxrt_hal as hal;

use hal::iomuxc;

/// The LED in its final configuration
pub type LED = hal::gpio::GPIO<iomuxc::b0::B0_03, hal::gpio::Output>;

/// Configure the board's LED
///
/// Returns a GPIO that's physically tied to the LED. Use the returned handle
/// to drive the LED.
pub fn configure_led(pad: iomuxc::b0::B0_03) -> LED {
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
