//! Demonstrates an I2C master. We try to read data from
//! a MPU9250 9-DOF IMU.
//!
//! Teensy pin 16 => SCL (I2C3)
//! Teensy pin 17 => SDA (I2C3)
//!
//! Success criteria:
//!
//! - The MPU correctly reports its `WHO_AM_I` address. The slave
//!   address is printed over USB logging.
//! - The clock is running at its selected bit rate; either 100KHz
//!   or 400KHz. Measure it with a logic analyzer.
//! - There's a repeated start in the `write_read` call; observable
//!   via a logic analyzer. Changing it to a `write`, followed by a
//!   `read`, should show that there is are two transactions.

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::hal::i2c::ClockSpeed;
use embedded_hal::blocking::i2c;
use teensy4_bsp as bsp;

/// MPU9250 I2C slave address
const SLAVE_ADDRESS: u8 = 0x68;
/// Our I2C clock speed. Change me to try 400KHz
const I2C_CLOCK_SPEED: ClockSpeed = ClockSpeed::KHz400;

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
    peripherals.systick.delay(5000);

    log::info!("Enabling I2C clocks...");
    let (_, _, i2c3_builder, _) = peripherals.i2c.clock(
        &mut peripherals.ccm.handle,
        bsp::hal::ccm::i2c::ClockSelect::OSC, // 2MHz clock...
        bsp::hal::ccm::i2c::PrescalarSelect::DIVIDE_3, // Divide by 3
    );

    log::info!("Constructing I2C3 instance on pins 16 and 17...");
    let mut i2c3 = i2c3_builder.build(peripherals.pins.p16.alt1(), peripherals.pins.p17.alt1());

    if let Err(err) = i2c3.set_bus_idle_timeout(core::time::Duration::from_micros(200)) {
        log::warn!("Error when setting bus idle timeout: {:?}", err);
    }
    if let Err(err) = i2c3.set_pin_low_timeout(core::time::Duration::from_millis(1)) {
        log::warn!("Error when setting pin low timeout: {:?}", err);
    }
    if let Err(err) = i2c3.set_clock_speed(I2C_CLOCK_SPEED) {
        log::warn!(
            "Error when setting I2C clock speed to {:?}: {:?}",
            I2C_CLOCK_SPEED,
            err
        );
    }

    log::info!("Starting I/O loop...");
    loop {
        peripherals.systick.delay(1000);
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
