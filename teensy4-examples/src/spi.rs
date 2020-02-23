//! Demonstrates how to use a SPI master.
//! Similar to the I2C example, we try to
//! read the WHO_AM_I register from an MPU9250.
//! 
//! Pinout:
//! 
//! - Teensy 4 Pin 13 (SCK), to MPU's SCL (Note that we lose the LED here)
//! - Teensy 4 Pin 11 (MOSI), to MPU's SDA/SDI
//! - Teensy 4 Pin 12 (MISO), to MPU's AD0/SDO
//! - Teensy 4 Pin 2 to MPU's NCS. This is will be our SPI chip select (CS)
//! 
//! The CS pin choice is arbitrary; we could use any digital output as CS.

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::hal::gpio::IntoGpio;
use bsp::rt::entry;
use teensy4_bsp as bsp;

use embedded_hal::{blocking::spi::Transfer, digital::v2::OutputPin};

const SPI_BAUD_RATE_HZ: u32 = 1_000_000;

/// Query the MPU9250 for its WHO_AM_I value. It should be `0x71`.
fn who_am_i<SPI: Transfer<u8>, CS: OutputPin>(
    spi: &mut SPI,
    cs: &mut CS,
) -> Result<u8, SPI::Error> {
    const WHO_AM_I: u8 = 0x75;
    const READ_BIT: u8 = 1 << 7;
    let mut buffer: [u8; 2] = [WHO_AM_I | READ_BIT, 0];
    let _ = cs.set_low();
    spi.transfer(&mut buffer)?;
    let _ = cs.set_high();
    Ok(buffer[1])
}

#[entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    peripherals.usb.init(Default::default());

    peripherals.ccm.pll1.set_arm_clock(
        bsp::hal::ccm::PLL1::ARM_HZ,
        &mut peripherals.ccm.handle,
        &mut peripherals.dcdc,
    );

    bsp::delay(5000);
    log::info!("Initializing SPI4 clocks...");

    let (_, _, _, spi4_builder) = peripherals.spi.clock(
        &mut peripherals.ccm.handle,
        bsp::hal::ccm::spi::ClockSelect::Pll2,
        bsp::hal::ccm::spi::PrescalarSelect::LPSPI_PODF_5,
    );

    log::info!("Constructing SPI4 peripheral...");
    let mut spi4 = spi4_builder.build(
        peripherals.pins.p11.alt3(),
        peripherals.pins.p12.alt3(),
        peripherals.pins.p13.alt3(),
    );

    match spi4.set_clock_speed(bsp::hal::spi::ClockSpeed(SPI_BAUD_RATE_HZ)) {
        Ok(()) => {
            log::info!("Set clock speed to {}Hz", SPI_BAUD_RATE_HZ);
        }
        Err(err) => {
            log::error!(
                "Unable to set clock speed to {}Hz: {:?}",
                SPI_BAUD_RATE_HZ,
                err
            );
            loop {
                core::sync::atomic::spin_loop_hint()
            }
        }
    };

    let mut cs4 = peripherals.pins.p2.into_gpio().output();
    cs4.set_high().unwrap();
    log::info!("Waiting 5 seconds before querying MPU9250...");
    bsp::delay(4000);
    loop {
        bsp::delay(1000);
        match who_am_i(&mut spi4, &mut cs4) {
            Ok(who) => log::info!("Received {:#X} for WHO_AM_I", who),
            Err(err) => log::warn!("Error when querying WHO_AM_I: {:?}", err),
        }
    }
}
