//! Demonstrates a panic handler that can log the panic message.
//!
//! Note: this example requires that the teensy4-panic crate's `log`
//! feature is enabled. Otherwise, there will be no log message written
//! over USB. The teensy4-panic crate's `log` feature is enabled for
//! all BSP examples.

#![no_std]
#![no_main]

use bsp::board;
use teensy4_bsp as bsp;
use teensy4_panic as _;

use cortex_m::{delay::Delay, peripheral::syst::SystClkSource};

const DELAY_MS: u32 = 5_000;

#[bsp::rt::entry]
fn main() -> ! {
    let syst = cortex_m::Peripherals::take().unwrap().SYST;
    let mut delay = Delay::with_source(syst, bsp::EXT_SYSTICK_HZ, SystClkSource::External);

    let board::Resources { usb, .. } = board::t40(board::instances());
    bsp::LoggingFrontend::default_log().register_usb(usb);

    delay.delay_ms(DELAY_MS);
    panic!("This is a panic message written after {DELAY_MS}ms");
}
