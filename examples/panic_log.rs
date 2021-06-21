//! Demonstrates a panic handler that can log the panic message.
//!
//! Note: this example requires that the teensy4-panic crate's `log`
//! feature is enabled. Otherwise, there will be no log message written
//! over USB. The teensy4-panic crate's `log` feature is enabled for
//! all BSP examples.

#![no_std]
#![no_main]

mod systick;
mod usb_io;

use teensy4_bsp as bsp;
use teensy4_panic as _;

const DELAY_MS: u32 = 5_000;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut p = bsp::Peripherals::take().unwrap();
    let mut systick = systick::new(cortex_m::Peripherals::take().unwrap().SYST);
    p.ccm
        .pll1
        .set_arm_clock(bsp::hal::ccm::PLL1::ARM_HZ, &mut p.ccm.handle, &mut p.dcdc);
    usb_io::init().unwrap();
    systick.delay_ms(DELAY_MS);
    panic!("This is a panic message written after {}ms", DELAY_MS);
}
