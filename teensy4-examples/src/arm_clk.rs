//! Demonstrates setting the ARM clock
//!
//! By delaying for the same number of cycles as the
//! ARM clock, we should observe a 1Hz blink.

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::rt;
use embedded_hal::digital::v2::ToggleableOutputPin;
use teensy4_bsp as bsp;

const ARM_HZ: u32 = 600_000_000;

#[rt::entry]
fn main() -> ! {
    let p = bsp::Peripherals::take().unwrap();
    let (ccm, ccm_analog) = p.ccm.handle.raw();
    let dcdc = p.dcdc.raw();
    bsp::hal::set_arm_clock(ARM_HZ, ccm, ccm_analog, dcdc);
    let mut led = p.led;
    loop {
        cortex_m::asm::delay(ARM_HZ);
        led.toggle().unwrap();
    }
}
