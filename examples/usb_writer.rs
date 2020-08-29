//! Demonstrates our ability to write over USB, and read
//! USB serial messages from a USB host.

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::rt;
use core::fmt::Write;
use teensy4_bsp as bsp;

#[rt::entry]
fn main() -> ! {
    let mut p = bsp::Peripherals::take().unwrap();
    let pins = bsp::t40::pins(p.iomuxc);
    let mut systick = bsp::SysTick::new(cortex_m::Peripherals::take().unwrap().SYST);
    // Split the USB stack into read / write halves
    let (mut reader, mut writer) = bsp::usb::split(&systick).unwrap();
    systick.delay(2000);
    p.ccm
        .pll1
        .set_arm_clock(bsp::hal::ccm::PLL1::ARM_HZ, &mut p.ccm.handle, &mut p.dcdc);
    let mut led: bsp::LED = bsp::configure_led(pins.p13);
    let mut buffer = [0; 256];
    loop {
        let bytes_read = reader.read(&mut buffer);
        if bytes_read > 0 {
            let bytes = &buffer[..bytes_read];
            match core::str::from_utf8(bytes) {
                Ok(msg) => writeln!(writer, "Received message: {} ({:?})", msg, bytes).unwrap(),
                Err(e) => writeln!(
                    writer,
                    "Read {} bytes, but could not interpret message {:?}: {:?}",
                    bytes_read, bytes, e
                )
                .unwrap(),
            }
        }

        writeln!(writer, "Hello world! 3 + 2 = {}", 3 + 2).unwrap();
        led.toggle();
        systick.delay(5000);
    }
}
