//! Demonstrates I2C communication with an MPU9250.
//!
//! Requires an MPU9250. The board queries the sensor's WHO_AM_I
//! The clock should be 400KHz. Check the USB serial logger to
//! see responses.
//!
//! This example use the alpha board with LPI2C3.
//!
//! - Pin 16 is SCL.
//! - Pin 17 is SDA.

#![no_main]
#![no_std]

use teensy4_bsp as bsp;
use teensy4_panic as _;

use bsp::{
    board,
    hal::{lpi2c::ControllerStatus, timer::Blocking},
};

use embedded_hal::blocking::i2c;

/// MPU9250 I2C address
const MPU9250_ADDRESS: u8 = 0x68;
const WHO_AM_I_REG: u8 = 0x75;
const WHO_AM_I_RESP: u8 = 0x71;

/// This should show a repeated start.
fn who_am_i_write_read<I>(i2c: &mut I) -> Result<bool, I::Error>
where
    I: i2c::WriteRead,
{
    let mut out = [0; 1];
    i2c.write_read(MPU9250_ADDRESS, &[WHO_AM_I_REG], &mut out)?;
    Ok(out[0] == WHO_AM_I_RESP)
}

/// This should show a stop after the write, then a new start
/// for the read.
fn who_am_i_write_then_read<I, E>(i2c: &mut I) -> Result<bool, E>
where
    I: i2c::Write<Error = E> + i2c::Read<Error = E>,
{
    i2c.write(MPU9250_ADDRESS, &[WHO_AM_I_REG])?;
    let mut out = [0; 1];
    i2c.read(MPU9250_ADDRESS, &mut out)?;
    Ok(out[0] == WHO_AM_I_RESP)
}

/// This should show a repeated start when using the transaction API.
fn who_am_i_transaction_write_read<I>(i2c: &mut I) -> Result<bool, I::Error>
where
    I: i2c::Transactional,
{
    use i2c::Operation;
    let mut out = [0u8; 1];
    let mut ops = [Operation::Write(&[WHO_AM_I_REG]), Operation::Read(&mut out)];
    i2c.exec(MPU9250_ADDRESS, &mut ops)?;
    Ok(out[0] == WHO_AM_I_RESP)
}

/// This should show a stop after the write, then a new start
/// for the read.
fn who_am_i_transaction_write_then_read<I>(i2c: &mut I) -> Result<bool, I::Error>
where
    I: i2c::Transactional,
{
    use i2c::Operation;
    let mut ops = [Operation::Write(&[WHO_AM_I_REG])];
    i2c.exec(MPU9250_ADDRESS, &mut ops)?;
    let mut out = [0u8; 1];
    ops[0] = Operation::Read(&mut out);
    i2c.exec(MPU9250_ADDRESS, &mut ops)?;
    Ok(out[0] == WHO_AM_I_RESP)
}

fn query_mpu(ctx: &str, func: impl FnOnce() -> Result<bool, ControllerStatus>) {
    match func() {
        Ok(true) => {
            log::info!("{ctx} OK");
        }
        Ok(false) => {
            log::warn!("{ctx} Wrong response");
        }
        Err(err) => {
            log::error!("{ctx} Error performing I2C I/O: {err:?}");
        }
    }
}

#[bsp::rt::entry]
fn main() -> ! {
    let board::Resources {
        pit: (pit, _, _, _),
        pins,
        mut gpio2,
        lpi2c3,
        usb,
        ..
    } = board::t40(board::instances());
    let led = board::led(&mut gpio2, pins.p13);
    bsp::LoggingFrontend::default_log().register_usb(usb);
    let mut timer = Blocking::<_, { board::PERCLK_FREQUENCY }>::from_pit(pit);
    let mut lpi2c3: board::Lpi2c3 =
        board::lpi2c(lpi2c3, pins.p16, pins.p17, board::Lpi2cClockSpeed::KHz400);

    led.set();
    let mut iteration = 0u32;

    loop {
        timer.block_ms(500);

        log::info!("Iteration {iteration}...");
        query_mpu("Querying WHO_AM_I with write-read... ", || {
            who_am_i_write_read(&mut lpi2c3)
        });

        query_mpu("Querying WHO_AM_I with write-then-read... ", || {
            who_am_i_write_then_read(&mut lpi2c3)
        });

        query_mpu(
            "Querying WHO_AM_I with transactional write-read... ",
            || who_am_i_transaction_write_read(&mut lpi2c3),
        );

        query_mpu(
            "Querying WHO_AM_I with transactional write-then-read... ",
            || who_am_i_transaction_write_then_read(&mut lpi2c3),
        );

        led.toggle();
        iteration = iteration.wrapping_add(1);
    }
}
