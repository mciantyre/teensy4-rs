//! Demonstrates how to use `teensy4_bsp` alongside `rtic`.
//!
//! NOTE: This example requires the `rtic` feature to be enabled.
//!
//! `rtic` stands for "Real-Time Interrupt-driven Concurrency". It is a convenient concurrency
//! framework for building real-time systems. If you are unfamiliar with `rtic`, I recommend
//! reading the online book: https://rtic.rs
//!
//! Success criteria: the LED turns on!

#![no_std]
#![no_main]

use teensy4_panic as _;

#[rtic::app(device = teensy4_bsp, peripherals = true)]
mod app {
    use teensy4_bsp as bsp;

    #[local]
    struct Local {}

    #[shared]
    struct Shared {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        // Cortex-M peripherals
        let _core: cortex_m::Peripherals = cx.core;

        // Device-specific peripherals
        let device: bsp::Peripherals = cx.device;
        let pins = bsp::t40::into_pins(device.iomuxc);
        let mut led = bsp::configure_led(pins.p13);
        led.set();

        (Shared {}, Local {}, init::Monotonics())
    }
    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            core::hint::spin_loop();
        }
    }
}
