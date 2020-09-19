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

extern crate panic_halt;

use embedded_hal::digital::v2::OutputPin;
use teensy4_bsp as bsp;

#[rtic::app(device = teensy4_bsp, peripherals = true)]
const APP: () = {
    #[init]
    fn init(cx: init::Context) {
        // Cortex-M peripherals
        let _core: cortex_m::Peripherals = cx.core;

        // Device-specific peripherals
        let device: bsp::Peripherals = cx.device;
        let pins = bsp::t40::into_pins(device.iomuxc);
        let mut led = bsp::configure_led(pins.p13);
        led.set_high().unwrap();
    }
    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            core::sync::atomic::spin_loop_hint();
        }
    }
};
