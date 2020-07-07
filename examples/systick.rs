//! Implements the systick exception and uses it
//! to toggle the LED every second.
//!
//! Success critera: the LED is on for 1 second, then off
//! for 1 second, then on for 1 second, then off for 1 second.

#![no_std]
#![no_main]
extern crate panic_halt;

use bsp::rt;
use teensy4_bsp as bsp;

const LED_PERIOD_MS: u32 = 1_000;

#[rt::entry]
fn main() -> ! {
    let mut p = bsp::Peripherals::take().unwrap();
    let mut led: bsp::LED = bsp::configure_led(p.pins.p13);

    loop {
        p.systick.delay(LED_PERIOD_MS);
        led.toggle();
    }
}
