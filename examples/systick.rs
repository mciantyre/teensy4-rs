//! Implements the systick exception and uses it
//! to toggle the LED every second.
//!
//! Success critera: the LED is on for 1 second, then off
//! for 1 second, then on for 1 second, then off for 1 second.

#![no_std]
#![no_main]

use teensy4_panic as _;

use cortex_m_rt as rt;
use teensy4_bsp as bsp;

const LED_PERIOD_MS: u32 = 1_000;

#[rt::entry]
fn main() -> ! {
    let p = bsp::Peripherals::take().unwrap();
    let mut systick = bsp::SysTick::new(cortex_m::Peripherals::take().unwrap().SYST);
    let pins = bsp::t40::from_pads(p.iomuxc);
    let mut led = bsp::configure_led(pins.p13);

    loop {
        systick.delay(LED_PERIOD_MS);
        led.toggle();
    }
}
