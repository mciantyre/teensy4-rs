//! Demonstrates an I2C master. We try to read data from
//! a MPU9250 9-DOF IMU.

#![no_std]
#![no_main]

extern crate panic_halt;

use embedded_hal::blocking::i2c;
use teensy4_bsp as bsp;

const SLAVE_ADDRESS: u8 = 0x68; // MPU9250

/// Returns the MPU's WHO_AM_I value. This should be a static
/// value that's specific for a MPU variant.
fn who_am_i<I>(i2c: &mut I) -> Result<u8, I::Error>
where
    I: i2c::WriteRead,
{
    const WHO_AM_I: u8 = 0x75;
    let mut out = [0; 1];
    i2c.write_read(SLAVE_ADDRESS, &[WHO_AM_I], &mut out)?;
    Ok(out[0])
}

#[bsp::rt::entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    peripherals.usb.init(Default::default());
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
        log::info!("Querying WHO_AM_I...");
        match who_am_i(&mut i2c3) {
            Ok(who) => log::info!("Received 0x{:X} for WHO_AM_I", who),
            Err(err) => {
                log::warn!("Error reading WHO_AM_I: {:?}", err);
                continue;
            }
        }
    }
}
