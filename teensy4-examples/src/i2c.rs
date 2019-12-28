//! Demonstrates an I2C master

#![no_std]
#![no_main]

extern crate panic_halt;

use embedded_hal::blocking::i2c::Write;
use teensy4_bsp as bsp;

const SLAVE_ADDRESS: u8 = 0x67;

#[bsp::rt::entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    peripherals.log.init(Default::default());
    bsp::delay(5000);
    log::info!("Enabling I2C clocks...");
    let (_, _, i2c3_builder, _) = peripherals.i2c.clock(
        &mut peripherals.ccm.handle,
        bsp::hal::ccm::i2c::ClockSelect::OSC,
        bsp::hal::ccm::i2c::PrescalarSelect::DIVIDE_1,
    );
    log::info!("Constructing I2C3 instance on pins 16 and 17...");
    let mut i2c3 = i2c3_builder.build(peripherals.pins.p16.alt1(), peripherals.pins.p17.alt1());
    if let Err(err) = i2c3.set_bus_idle_timeout(core::time::Duration::from_micros(200)) {
        log::warn!("Error when setting bus idle timeout: {:?}", err);
    }
    if let Err(err) = i2c3.set_pin_low_timeout(core::time::Duration::from_millis(1)) {
        log::warn!("Error when setting pin low timeout: {:?}", err);
    }
    log::info!("Starting I/O loop...");
    loop {
        bsp::delay(1000);
        log::info!("Writing [1, 2, 3] to slave address {}", SLAVE_ADDRESS);
        match i2c3.write(SLAVE_ADDRESS, &[1, 2, 3]) {
            Ok(_) => log::info!("Write OK"),
            Err(err) => {
                log::error!("Error: {:?}", err);
                continue;
            }
        }
    }
}
