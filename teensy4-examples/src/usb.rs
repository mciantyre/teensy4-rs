//! Demonstrates our ability to log / print over USB.

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::rt;
use teensy4_bsp as bsp;

use embedded_hal::digital::v2::ToggleableOutputPin;

#[rt::entry]
fn main() -> ! {
    let mut p = bsp::Peripherals::take().unwrap();
    // Initialize logging with the default settings
    p.log.init(Default::default());
    bsp::delay(2000);
    p.ccm
        .pll1
        .set_arm_clock(bsp::hal::ccm::PLL1::ARM_HZ, &mut p.ccm.handle, &mut p.dcdc);
    let mut led = p.led;
    loop {
        log::error!("Something terrible happened!");
        log::warn!("Something happened, but we fixed it");
        log::info!("It's 31'C outside");
        log::debug!("Sleeping for 1 second...");
        log::trace!("{} + {} = {}", 3, 2, 3 + 2);
        led.toggle().unwrap();
        bsp::delay(5000);
    }
}
