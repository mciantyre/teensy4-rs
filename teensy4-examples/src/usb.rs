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
    // Initialize logging with the default settings
    p.log.init(Default::default());
    let mut led = p.led;
    let mut counter = 0;
    loop {
        bsp::delay(1000);
        log::error!("Something terrible happened!");
        log::warn!("Something happened, but we fixed it");
        log::info!("It's 31'C outside");
        log::debug!("Sleeping for 1 second...");
        log::trace!("{} + {} = {}", 3, 2, 3 + 2);
        led.toggle().unwrap();
        counter += 1;
        if counter == 5 {
            let (ccm, ccm_analog) = p.ccm.handle.raw();
            let dcdc = p.dcdc.raw();
            bsp::hal::set_arm_clock(600_000_000, ccm, ccm_analog, dcdc);
            loop {
                core::sync::atomic::spin_loop_hint();
            }
        }
    }
}
