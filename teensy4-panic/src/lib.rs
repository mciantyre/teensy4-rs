//! Panic handler for the Teensy 4.
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
//! Finally, use `panic!()` to stop the program and blink the LED.
//!
//! # Features
//!
//! The table below summarizes this crate's features. Each subsection details the feature.
//!
//! | Feature         |         Description                                | Default feature? |
//! | --------------- | -------------------------------------------------- | ---------------- |
//! | `panic-handler` | Define the Teensy 4's panic handler in this crate  |        ✓         |
//! | `log`           | Log the panic message using `log::error!`          |                  |
//!
//! It does not make sense to _disable_ `panic-handler` and _enable_ `log`, since logging can
//! only happen when `panic-handler` is enabled.
//!
//! ## Custom panic handlers
//!
//! By default, `teensy4-panic` enables the `panic-handler` feature. If you want
//! to use the S.O.S. routine in your own panic handler, disable the default
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
//!     // Your panic handler here...
//!     sos()
//! }
//! # fn main() {}
//! # #[lang = "eh_personality"] extern fn rust_eh_personality() {}
//! ```
//!
//! ## Log the panic message
//!
//! If the `log` feature is enabled, the crate links with [the `log` crate](https://crates.io/crates/log).
//! Before blinking, the panic handler logs the panic message and source location at the error priority,
//! using `log::error!`. The logging target is `teensy4_panic`.
//!
//! The example below shows a `panic!` and an example of its corresponding log message:
//!
//! ```no_run
//! const DELAY_MS: u32 = 5_000;
//! panic!("This is a panic message written after {}ms", DELAY_MS);
//! ```
//! ```text
//! [ERROR teensy4_panic]: panicked at 'This is a panic message written after 5000ms', examples/panic_log.rs:22:5
//! ```
//!
//! The panic handler only emits the log message once. You're responsible for making sure that the
//! log message can reach its destination while a `panic!` is active.

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

/// Assign all DelayTicks to a `const`. If the factor overflows, it's
/// caught at compile time. Use this to explore different delays in
/// debug builds.
#[derive(Clone, Copy)]
struct DelayTicks(u32);
impl DelayTicks {
    const fn new(factor: u32) -> Self {
        Self(80_000_000 * factor)
    }
}

fn delay(_ticks: DelayTicks) {
    #[cfg(all(target_arch = "arm", target_os = "none"))]
    unsafe {
        core::arch::asm! {
            r#"
                5:
                subs {ticks}, #1 @ ticks--; Z = (0 == ticks);
                bne 5b           @ if (0 == Z) goto 5;
            "#,
            ticks = inout(reg) _ticks.0 => _,
        };
    }
}

// Morse timing
const SOS_DIT: DelayTicks = DelayTicks::new(1);
const SOS_DAH: DelayTicks = DelayTicks::new(3);
const SOS_INTRA_CHAR: DelayTicks = DelayTicks::new(1);
const SOS_INTER_CHAR: DelayTicks = DelayTicks::new(3);
const SOS_WORD_SPACE: DelayTicks = DelayTicks::new(7);

fn dit(led: &mut Led) {
    led.set();
    delay(SOS_DIT);
    led.clear();
}

fn dah(led: &mut Led) {
    led.set();
    delay(SOS_DAH);
    led.clear();
}

fn s(led: &mut Led) {
    dit(led);
    delay(SOS_INTRA_CHAR);
    dit(led);
    delay(SOS_INTRA_CHAR);
    dit(led);
}

fn o(led: &mut Led) {
    dah(led);
    delay(SOS_INTRA_CHAR);
    dah(led);
    delay(SOS_INTRA_CHAR);
    dah(led);
}

#[cfg(feature = "panic-handler")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    #[cfg(feature = "log")]
    {
        use core::sync::atomic::{AtomicBool, Ordering};
        // If logging results in another panic, we shouldn't try again.
        static TRY_TO_LOG: AtomicBool = AtomicBool::new(true);
        if TRY_TO_LOG.fetch_and(false, Ordering::Relaxed) {
            log::error!("{_info}");
        }
    }
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
        delay(SOS_INTER_CHAR);
        o(&mut led);
        delay(SOS_INTER_CHAR);
        s(&mut led);

        delay(SOS_WORD_SPACE);
    }
}
