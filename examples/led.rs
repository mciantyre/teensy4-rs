//! The example simply enables the LED
//!
//! Success criteria: the LED turns on!

#![no_std]
#![no_main]

use teensy4_panic as _;

use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use teensy4_bsp as bsp;

use embedded_hal::digital::v2::OutputPin;

#[entry]
fn main() -> ! {
    let peripherals = bsp::Peripherals::take().unwrap();
    let pins = bsp::pins::t40::from_pads(peripherals.iomuxc);
    let mut led = bsp::configure_led(pins.p13);

    loop {
        led.set_high().unwrap();
        wfi();
    }
}
