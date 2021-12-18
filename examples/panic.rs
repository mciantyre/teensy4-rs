//! Demonstrates a simple panic handler.
//!
//! This example does not write a panic message, nor does it set up
//! logging. You should only observe a blinking LED. For a more advanced
//! panic example, see panic_log.rs.

#![no_std]
#![no_main]

use teensy4_bsp as bsp;
use teensy4_panic as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut p = bsp::Peripherals::take().unwrap();
    p.ccm
        .pll1
        .set_arm_clock(bsp::hal::ccm::PLL1::ARM_HZ, &mut p.ccm.handle, &mut p.dcdc);

    panic!();
}
