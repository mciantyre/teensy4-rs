//! Demonstrates a blocking SPI peripheral.
//!
//! Connect pins 11 and 12 together, then run this example.
//! The example prints success / errors to the USB serial console.
//! You should see a 1MHz SPI clock, and that the elements of a write /
//! transfer operation occur within a single low PCS.
//!
//! If you disconnect pins 11 and 12, you should observe a data mismatch
//! warning after each transfer.

#![no_std]
#![no_main]

use bsp::board;
use teensy4_bsp as bsp;
use teensy4_panic as _;

use bsp::hal::{
    lpspi::{Direction, LpspiError},
    timer::Blocking,
};

use embedded_hal::blocking::spi::{Transfer, Write};

/// Change me to experiment with different word sizes.
/// Valid types: u8, u16, u32.
type Elem = u8;

fn write_error<T>(result: Result<T, LpspiError>) {
    match result {
        Err(LpspiError::Busy) => {
            log::error!("Error: BUSY\r\n");
        }
        Err(LpspiError::Fifo(Direction::Rx)) => {
            log::error!("Error: RX FIFO\r\n");
        }
        Err(LpspiError::Fifo(Direction::Tx)) => {
            log::error!("Error: TX FIFO\r\n");
        }
        Err(LpspiError::NoData) => {
            log::error!("Error: NO DATA\r\n");
        }
        Err(LpspiError::FrameSize) => {
            log::error!("Error: FRAME SIZE\r\n");
        }
        Ok(_) => {}
    }
}

#[bsp::rt::entry]
fn main() -> ! {
    let board::Resources {
        lpspi4,
        mut pins,
        pit: (pit, _, _, _),
        usb,
        mut gpio2,
        ..
    } = board::t40(board::instances());

    bsp::LoggingFrontend::default_log().register_usb(usb);
    let mut timer = Blocking::<_, { board::PERCLK_FREQUENCY }>::from_pit_channel(pit);

    // Configure LPSPI4 clock, serial data out, and serial data in pins.
    {
        use bsp::hal::iomuxc; // <-- Pad configuration APIs are here.
        iomuxc::lpspi::prepare(&mut pins.p13); // <-- LPSPI4 SCK
        iomuxc::lpspi::prepare(&mut pins.p11); // <-- LPSPI4 SDO
        iomuxc::lpspi::prepare(&mut pins.p12); // <-- LPSPI4 SDI

        // Drop / forget pins so that they're no longer accessible.
        // We don't want to accidentally change them later.
        //
        // (There's no impl Drop on iomuxc pads, so this is basically
        // a forget.)
        drop(pins.p13);
        drop(pins.p12);
        drop(pins.p11);
    }

    // Use the chip select line as a software-controlled GPIO.
    let cs = gpio2.output(pins.p10);
    // TODO add extra data / command pin here.

    // Construct the driver:
    let mut spi = bsp::hal::lpspi::Lpspi::without_pins(lpspi4);
    // Set the baud rate.
    spi.disabled(|spi| {
        spi.set_clock_hz(bsp::board::LPSPI_FREQUENCY, 1_000_000);
    });

    // Have fun! You're responsible for calling set and clear on the chip select object.

    let data: [Elem; 5] = [0xDE, 0xAD, 0xBE, 0xEF, 0xA5];
    loop {
        let mut buffer: [Elem; 5] = data;

        timer.block_ms(500);

        log::info!("Transfer...");
        let result = spi.transfer(&mut buffer);
        if result.is_err() {
            write_error(result);
        } else if buffer != data {
            log::warn!("Data mismatch");
        } else {
            log::info!("OK");
        }

        timer.block_ms(500);

        log::info!("Write...");
        let result = spi.write(&buffer[..3]);
        if result.is_err() {
            write_error(result);
        } else {
            log::info!("OK");
        }
    }
}
