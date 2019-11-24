//! The example simply enables the LED

#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m::asm::wfi;
use teensy4_bsp as bsp;
use teensy4_rt::entry;

#[entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    loop {
        peripherals.led.high();
        wfi();
    }
}
