//! Demonstrates simple USB logging.
//!
//! This example uses `LoggingFrontend` to quickly register the USB logger.

#![no_std]
#![no_main]

use teensy4_bsp as bsp;
use teensy4_panic as _;

use bsp::{board, hal::timer::Blocking};

#[bsp::rt::entry]
fn main() -> ! {
    let board::Resources {
        pins,
        mut gpio2,
        pit,
        usb,
        ..
    } = board::t40(board::instances());
    let led = board::led(&mut gpio2, pins.p13);
    bsp::LoggingFrontend::default_log().register_usb(usb);

    let mut delay = Blocking::<_, { board::PERCLK_FREQUENCY }>::from_pit(pit.0);
    let mut counter = 0;
    loop {
        delay.block_ms(500);
        led.toggle();
        log::info!("Hello world! The count is {counter}");
        defmt::println!("Hello world! The count is {=u32}", counter);
        counter = counter.wrapping_add(1);
    }
}
