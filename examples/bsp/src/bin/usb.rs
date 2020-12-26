//! Demonstrates our ability to log over USB, and read
//! USB serial messages from a USB host.
//!
//! Success criteria: you see log messages when connecting
//! to the Teensy 4 using PuTTY of another serial console.
//! Baud rate is 9600.

#![no_std]
#![no_main]

use teensy4_panic as _;

use bsp::hal::ral::usb::USB1;
use bsp::interrupt;
use cortex_m_rt as rt;
use teensy4_bsp as bsp;

#[cortex_m_rt::interrupt]
unsafe fn USB_OTG1() {
    bsp::usb::poll();
}

/// Specify (optional) logging filters
///
/// The "usb" filter matches the name of this example.
/// If you copy and paste this example somewhere else,
/// consider updating the filter, or removing the filter
/// entirely.
///
/// See the `LoggingConfig` documentation for more information.
const LOG_FILTERS: &[bsp::usb::Filter] = &[
    // +------------- Module name to include in log messages
    // v     v------- Maximum log level (subject to the statically-defined max level)
    ("usb", None),
];

#[rt::entry]
fn main() -> ! {
    let mut p = bsp::Peripherals::take().unwrap();
    let pins = bsp::t40::into_pins(p.iomuxc);
    let mut systick = bsp::SysTick::new(cortex_m::Peripherals::take().unwrap().SYST);
    // Initialize the USB stack with the default logging settings
    let mut usb_reader = bsp::usb::init(
        USB1::take().unwrap(),
        bsp::usb::LoggingConfig {
            filters: LOG_FILTERS,
            ..Default::default()
        },
    )
    .unwrap();

    unsafe { cortex_m::peripheral::NVIC::unmask(bsp::interrupt::USB_OTG1) };

    log::error!("You might not see this message if the USB device isn't configured by the host");
    systick.delay(2000);
    p.ccm
        .pll1
        .set_arm_clock(bsp::hal::ccm::PLL1::ARM_HZ, &mut p.ccm.handle, &mut p.dcdc);
    let mut led: bsp::LED = bsp::configure_led(pins.p13);
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
        systick.delay(500);
    }
}
