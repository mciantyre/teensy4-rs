//! Implements the systick exception and uses it
//! to toggle the LED every second.

#![no_std]
#![no_main]
extern crate panic_halt;

use bsp::rt;
use embedded_hal::digital::v2::ToggleableOutputPin;
use teensy4_bsp as bsp;

const LED_PERIOD_MS: u32 = 1_000;

#[rt::entry]
fn main() -> ! {
    let p = bsp::Peripherals::take().unwrap();
    let mut led = p.led;

    loop {
        bsp::delay(LED_PERIOD_MS);
        led.toggle().unwrap();
    }
}
