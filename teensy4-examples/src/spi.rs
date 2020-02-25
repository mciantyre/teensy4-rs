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

const READ_BIT: u8 = 1 << 7;
const WRITE_BIT: u8 = 0;

fn transact<'a, O: OutputPin, F: FnOnce() -> R + 'a, R>(cs: &'a mut O, act: F) -> R {
    let _ = cs.set_low();
    let res = act();
    let _ = cs.set_high();
    res
}

fn ak8963_init<SPI: Transfer<u8>, CS: OutputPin>(
    spi: &mut SPI,
    cs: &mut CS,
) -> Result<(), SPI::Error> {
    const USER_CTRL: u8 = 0x6A;
    const I2C_MST_CTRL: u8 = 0x24;
    const USER_CTRL_I2C_MST_RST: u8 = 1 << 1;
    const USER_CTRL_I2C_IF_DIS: u8 = 1 << 4;
    const USER_CTRL_I2C_MST_EN: u8 = 1 << 5;
    bsp::delay(100);
    transact(cs, || {
        spi.transfer(&mut [
            USER_CTRL | WRITE_BIT,
            USER_CTRL_I2C_IF_DIS | USER_CTRL_I2C_MST_EN | USER_CTRL_I2C_MST_RST,
        ])
        .map(|_| ())
    })?;
    bsp::delay(100);
    transact(cs, || {
        spi.transfer(&mut [I2C_MST_CTRL | WRITE_BIT, 0x0D]) // 400KHz
            .map(|_| ())
    })?;
    bsp::delay(100);
    Ok(())
}

/// Query the MPU9250 for its WHO_AM_I value. It should be `0x71`.
fn who_am_i<SPI: Transfer<u8>, CS: OutputPin>(
    spi: &mut SPI,
    cs: &mut CS,
) -> Result<u8, SPI::Error> {
    const WHO_AM_I: u8 = 0x75;
    let mut buffer: [u8; 2] = [WHO_AM_I | READ_BIT, 0];
    transact(cs, || spi.transfer(&mut buffer))?;
    Ok(buffer[1])
}

fn ak8963_who_am_i<SPI: Transfer<u8>, CS: OutputPin>(
    spi: &mut SPI,
    cs: &mut CS,
) -> Result<u8, SPI::Error> {
    const AK8963_ADDRESS: u8 = 0x0C;
    const AK8964_WHO_AM_I: u8 = 0x00;

    const I2C_SLV4_ADDR: u8 = 0x31;
    const I2C_SLV4_REG: u8 = 0x32;
    const I2C_SLV4_CTRL: u8 = 0x34;
    const I2C_SLV4_DI: u8 = 0x35;

    const I2C_MST_STATUS: u8 = 0x36;
    const I2C_SLV4_DONE: u8 = 1 << 6;
    const I2C_LOST_ARB: u8 = 1 << 5;
    const I2C_SLV4_NACK: u8 = 1 << 4;
    const I2C_SLV_EN: u8 = 1 << 7;

    transact(cs, || {
        spi.transfer(&mut [I2C_SLV4_ADDR | WRITE_BIT, AK8963_ADDRESS | READ_BIT])
            .map(|_| ())
    })?;
    transact(cs, || {
        spi.transfer(&mut [I2C_SLV4_REG | WRITE_BIT, AK8964_WHO_AM_I])
            .map(|_| ())
    })?;
    transact(cs, || {
        spi.transfer(&mut [I2C_SLV4_CTRL | WRITE_BIT, I2C_SLV_EN])
            .map(|_| ())
    })?;

    bsp::delay(1);
    let mut attempts = 0;
    loop {
        let mut buffer = [I2C_MST_STATUS | READ_BIT, 0];
        transact(cs, || spi.transfer(&mut buffer))?;
        if buffer[1] & I2C_SLV4_DONE > 0 {
            break;
        } else if buffer[1] & I2C_SLV4_NACK > 0 {
            log::warn!("I2C_SLV4_NACK");
            break;
        } else if buffer[1] & I2C_LOST_ARB > 0 {
            log::warn!("I2C_LOST_ARB");
            break;
        }
        attempts += 1;
        if attempts >= 10_000 {
            log::warn!("Tried too many times");
            return Ok(0);
        }
    }

    let mut buffer = [I2C_SLV4_DI | READ_BIT, 0];
    transact(cs, || spi.transfer(&mut buffer))?;
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

    let mut cs4 = peripherals
        .pins
        .p10
        .into_gpio()
        .fast(&mut peripherals.gpr)
        .output();
    cs4.set_high().unwrap();
    log::info!("Waiting 5 seconds before querying MPU9250...");
    bsp::delay(4000);
    match ak8963_init(&mut spi4, &mut cs4) {
        Ok(()) => (),
        Err(err) => {
            log::warn!("Unable to initialize AK8963: {:?}", err);
            loop {
                core::sync::atomic::spin_loop_hint()
            }
        }
    };
    loop {
        bsp::delay(1000);
        match who_am_i(&mut spi4, &mut cs4) {
            Ok(who) => log::info!("Received {:#X} for WHO_AM_I", who),
            Err(err) => log::warn!("Error when querying WHO_AM_I: {:?}", err),
        }
        match ak8963_who_am_i(&mut spi4, &mut cs4) {
            Ok(who) => log::info!("Received {:#X} for AK8963_WHO_AM_I", who),
            Err(err) => log::warn!("Error when querying AK8963_WHO_AM_I: {:?}", err),
        }
    }
}
