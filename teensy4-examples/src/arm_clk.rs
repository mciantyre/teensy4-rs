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
    let mut p = bsp::Peripherals::take().unwrap();
    p.ccm
        .pll1
        .set_arm_clock(bsp::hal::ccm::PLL1::ARM_HZ, &mut p.ccm.handle, &mut p.dcdc);
    let mut led = p.led;
    loop {
        cortex_m::asm::delay(ARM_HZ);
        led.toggle().unwrap();
    }
}
