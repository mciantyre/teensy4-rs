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
    let p = bsp::Peripherals::take().unwrap();
    let mut systick = bsp::SysTick::new(cortex_m::Peripherals::take().unwrap().SYST);
    let pins = bsp::t40::into_pins(p.iomuxc);
    let mut led: bsp::LED = bsp::configure_led(pins.p13);

    loop {
        systick.delay(LED_PERIOD_MS);
        led.toggle();
    }
}
