//! Demonstrates our ability to log / print over USB.

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::rt;
use teensy4_bsp as bsp;

use embedded_hal::digital::v2::ToggleableOutputPin;

#[rt::entry]
fn main() -> ! {
    let p = bsp::Peripherals::take().unwrap();
    let mut led = p.led;
    loop {
        bsp::delay(1000);
        bsp::serial_write(b"hello world\r\n");
        led.toggle().unwrap();
    }
}
