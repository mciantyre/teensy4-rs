//! Loopback over UART
//!
//! Connect Teensy pins 16 and 17 together. We transfer
//! from one pin, and receive on the other.

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::rt::entry;
use teensy4_bsp as bsp;

use embedded_hal::digital::v2::ToggleableOutputPin;
use embedded_hal::serial::Write;

#[entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    let uarts = peripherals.uart.clock(
        &mut peripherals.ccm.handle,
        bsp::hal::ccm::uart::ClockSelect::OSC,
        bsp::hal::ccm::uart::PrescalarSelect::DIVIDE_1,
    );
    let mut uart3 = uarts
        .uart3
        .init(
            peripherals.pins.p17.alt2(),
            peripherals.pins.p16.alt2(),
            9600,
        )
        .unwrap();
    loop {
        peripherals.led.toggle().unwrap();
        nb::block!(uart3.write(0xBE)).unwrap();
        nb::block!(uart3.write(0xEF)).unwrap();
        bsp::delay(1_000);
    }
}
