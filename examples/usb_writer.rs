//! Demonstrates our ability to write over USB, and read
//! USB serial messages from a USB host.

#![no_std]
#![no_main]

mod systick;
mod usb_io;

use teensy4_panic as _;

use core::fmt::Write;
use cortex_m_rt as rt;
use teensy4_bsp as bsp;

#[rt::entry]
fn main() -> ! {
    let mut p = bsp::Peripherals::take().unwrap();
    let pins = bsp::pins::t40::from_pads(p.iomuxc);
    let mut systick = systick::new(cortex_m::Peripherals::take().unwrap().SYST);
    // Split the USB stack into read / write halves
    let (mut reader, mut writer) = usb_io::split().unwrap();

    systick.delay_ms(2000);
    p.ccm
        .pll1
        .set_arm_clock(bsp::hal::ccm::PLL1::ARM_HZ, &mut p.ccm.handle, &mut p.dcdc);
    let mut led = bsp::configure_led(pins.p13);
    let mut buffer = [0; 256];
    loop {
        let bytes_read = reader.read(&mut buffer).unwrap();
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
        writer.flush().unwrap();
        led.toggle();
        systick.delay_ms(5000);
    }
}
