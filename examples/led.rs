//! The example simply enables the LED
//!
//! Success criteria: the LED turns on!

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::rt::entry;
use cortex_m::asm::wfi;
use teensy4_bsp as bsp;

use embedded_hal::digital::v2::OutputPin;

#[entry]
fn main() -> ! {
    let peripherals = bsp::Peripherals::take().unwrap();
    let pins = bsp::t40::pins(peripherals.iomuxc);
    let mut led = bsp::configure_led(pins.p13);

    loop {
        led.set_high().unwrap();
        wfi();
    }
}
