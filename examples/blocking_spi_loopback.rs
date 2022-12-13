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
        pins,
        pit: (pit, _, _, _),
        usb,
        ..
    } = board::t40(board::instances());

    bsp::LoggingFrontend::default_log().register_usb(usb);
    let mut timer = Blocking::<_, { board::PERCLK_FREQUENCY }>::from_pit_channel(pit);

    let mut spi: board::Lpspi4 = board::lpspi(
        lpspi4,
        board::LpspiPins {
            pcs0: pins.p10,
            sck: pins.p13,
            sdo: pins.p11,
            sdi: pins.p12,
        },
        1_000_000,
    );

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
