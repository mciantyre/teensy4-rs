//! Demonstrates our ability to log over USB, and read
//! USB serial messages from a USB host.
//!
//! Success criteria: you see log messages when connecting
//! to the Teensy 4 using PuTTY of another serial console.
//! Baud rate is 9600.

#![no_std]
#![no_main]

mod systick;
mod usb_io;

use teensy4_panic as _;

use cortex_m_rt as rt;
use teensy4_bsp as bsp;

#[rt::entry]
fn main() -> ! {
    let mut p = bsp::Peripherals::take().unwrap();
    let pins = bsp::pins::t40::from_pads(p.iomuxc);
    let mut systick = systick::new(cortex_m::Peripherals::take().unwrap().SYST);
    let mut usb_reader = usb_io::init().unwrap();

    log::error!("You might not see this message if the USB device isn't configured by the host");
    systick.delay_ms(2000);
    p.ccm
        .pll1
        .set_arm_clock(bsp::hal::ccm::PLL1::ARM_HZ, &mut p.ccm.handle, &mut p.dcdc);
    let mut led = bsp::configure_led(pins.p13);
    let mut buffer = [0; 256];
    let mut counter = 0;
    loop {
        let bytes_read = usb_reader.read(&mut buffer).unwrap();
        if bytes_read > 0 {
            let bytes = &buffer[..bytes_read];
            match core::str::from_utf8(bytes) {
                Ok(msg) => log::info!("Received message: {} ({:?})", msg, bytes),
                Err(e) => log::warn!(
                    "Read {} bytes, but could not interpret message {:?}: {:?}",
                    bytes_read,
                    bytes,
                    e
                ),
            }
        }

        log::error!("Something terrible happened! Count {}", counter);
        log::warn!("Something happened, but we fixed it");
        log::info!("It's 31'C outside");
        log::debug!("Sleeping for 1 second...");
        log::trace!("{} + {} = {}", 3, 2, 3 + 2);
        counter += 1;
        led.toggle();
        systick.delay_ms(500);
    }
}
