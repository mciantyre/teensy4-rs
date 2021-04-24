//! Panic handler for the Teensy 4
//!
//! When you link `teensy4-panic` into your program, any `panic!()` will cause
//! your Teensy's LED to blink S.O.S. in Morse code. Supports both Teensy 4.0 and
//! 4.1 boards.
//!
//! # Usage
//!
//! Depend on `teensy4-panic`:
//!
//! ```toml
//! [dependencies]
//! teensy4-panic = "0.2"
//! ```
//!
//! Then, include the crate in your final program:
//!
//! ```rust
//! use teensy4_panic as _;
//! ```
//!
//! Finally, use `panic!()` to stop the program, and blink the LED.
//!
//! # Custom panic handlers
//!
//! By default, `teensy4-panic` enables the `panic-handler` feature. If you want
//! to use the S.O.S. routine in your own, custom panic handler, disable the default
//! features, and call `sos()`:
//!
//! ```toml
//! [dependencies]
//! teensy4-panic = "0.2"
//! default-features = false
//! ```
//!
//! ```no_run
//! # #![feature(lang_items)]
//! # #![no_std]
//! use teensy4_panic::sos;
//!
//! #[panic_handler]
//! fn panic(_: &core::panic::PanicInfo) -> ! {
//!     // ...
//!     sos()
//! }
//! # fn main() {}
//! # #[lang = "eh_personality"] extern fn rust_eh_personality() {}
//! ```

#![no_std]

/// The LED
struct Led();

const GPIO2_BASE: u32 = 0x401BC000;

impl Led {
    /// Construct the LED
    ///
    /// When `new` returns, the LED will be ready to set, clear, and toggle.
    ///
    /// # Safety
    ///
    /// There should only be one LED in the program. This will modify peripheral memory
    /// to enable the LED.
    unsafe fn new() -> Self {
        const IOMUXC_SW_MUX_CTL_PAD_GPIO_B0_03: *mut u32 = 0x401F_8148 as *mut u32;
        const IOMUXC_SW_PAD_CTL_PAD_GPIO_B0_03: *mut u32 = 0x401F_8338 as *mut u32;
        const IOMUXC_GPR_GPR27: *mut u32 = 0x400A_C06C as *mut u32;

        const fn drive_strength_enable(dse: u32) -> u32 {
            (dse & 0x07) << 3
        }

        const GPIO2_GDIR: *mut u32 = (GPIO2_BASE + 0x04) as *mut u32;

        // Set the LED pad into Alt5
        IOMUXC_SW_MUX_CTL_PAD_GPIO_B0_03.write_volatile(5);
        // Increase drive strength, clearing all other fields
        IOMUXC_SW_PAD_CTL_PAD_GPIO_B0_03.write_volatile(drive_strength_enable(7));
        // Disable fast mode so that GPIO2 registers drive the pad
        IOMUXC_GPR_GPR27.write_volatile(IOMUXC_GPR_GPR27.read_volatile() & !(1 << 3));
        GPIO2_GDIR.write_volatile(GPIO2_GDIR.read_volatile() | (1 << 3));

        Led()
    }

    /// Drive the LED high
    fn set(&mut self) {
        const GPIO2_DR_SET: *mut u32 = (GPIO2_BASE + 0x84) as *mut u32;
        unsafe { GPIO2_DR_SET.write_volatile(1 << 3) };
    }

    /// Drive the LED low
    fn clear(&mut self) {
        const GPIO2_DR_CLEAR: *mut u32 = (GPIO2_BASE + 0x88) as *mut u32;
        unsafe { GPIO2_DR_CLEAR.write_volatile(1 << 3) };
    }
}

fn delay(factor: u32) {
    for _ in 0..(factor * 50_000_000) {
        core::hint::spin_loop();
    }
}

fn triple(led: &mut Led, factor: u32) {
    for _ in 0..3 {
        led.set();
        delay(factor);
        led.clear();
        delay(factor);
    }
}

fn s(led: &mut Led) {
    triple(led, 1);
}

fn o(led: &mut Led) {
    triple(led, 3);
}

#[cfg(feature = "panic-handler")]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    sos()
}

/// Blink S.O.S. on the LED, forever
///
/// This is the implementation of the `teensy4-panic` handler.
/// See the crate-level docs for how you could use this in your
/// own panic handler.
pub fn sos() -> ! {
    let mut led = unsafe { Led::new() };
    loop {
        s(&mut led);
        o(&mut led);
        s(&mut led);
        delay(6);
    }
}
