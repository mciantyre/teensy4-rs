//! Loopback over UART
//!
//! Connect Teensy pins 14 and 15 together. We transfer
//! from one pin, and receive on the other. Demonstrates
//! the usage of the TX and RX FIFOs.
//!
//! It's not the most advanced example. The RX FIFO could
//! overrun if we're not reading fast enough.
//!
//! See the `const` configurations for settings.
//!
//! Success criteria: the loopback is reported as successful
//! over USB logging. Changing the settings below demonstrate
//! the same ideal behavior. When decreasing the `TX_FIFO_SIZE`,
//! we see an increase of blocked reads. The transfer
//! content is `0xDEADBEEF`.

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::rt::entry;
use teensy4_bsp as bsp;

use embedded_hal::digital::v2::ToggleableOutputPin;
use embedded_hal::serial::{Read, Write};

const BAUD: u32 = 115_200;
/// Change the TX FIFO sizes to see how the FIFO affects the number
/// of `WouldBlock`s that we would see. Setting this to zero disables
/// the FIFO.
const TX_FIFO_SIZE: u8 = 4;
/// Change me to affect the partity bit generation
const PARITY: Option<bsp::hal::uart::Parity> = None;
/// Change me to invert all output data, and to treat all input data as inverted
const INVERTED: bool = false;
/// The data you want to send and receive
const DATA: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];

/// Writes `bytes` to the provided `uart`. The function will count the
/// number of blocks that we hit, and will log how many blocks we
/// required to transmit `bytes`.
fn write<W: Write<u8>>(uart: &mut W, bytes: &[u8]) -> Result<(), W::Error> {
    let mut blocks = 0;
    for byte in bytes {
        loop {
            match uart.write(*byte) {
                Ok(()) => break,
                Err(nb::Error::WouldBlock) => blocks += 1,
                Err(nb::Error::Other(err)) => return Err(err),
            }
        }
    }
    log::info!("{} blocks to transmit {:?}", blocks, bytes);
    Ok(())
}

/// Reads from `uart` into `bytes`. The function will count the
/// number of blocks that we hit, and it will log how many blocks
/// we required to receive `bytes`.
fn read<R: Read<u8>>(uart: &mut R, bytes: &mut [u8]) -> Result<(), R::Error> {
    let mut blocks = 0;
    for byte in bytes.iter_mut() {
        loop {
            match uart.read() {
                Ok(b) => {
                    *byte = b;
                    break;
                }
                Err(nb::Error::WouldBlock) => blocks += 1,
                Err(nb::Error::Other(err)) => return Err(err),
            }
        }
    }
    log::info!("{} blocks to receive {:?}", blocks, bytes);
    Ok(())
}

#[entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    peripherals.usb.init(Default::default());
    bsp::delay(5_000);
    let uarts = peripherals.uart.clock(
        &mut peripherals.ccm.handle,
        bsp::hal::ccm::uart::ClockSelect::OSC,
        bsp::hal::ccm::uart::PrescalarSelect::DIVIDE_1,
    );
    let mut uart = uarts
        .uart2
        .init(
            peripherals.pins.p14.alt2(),
            peripherals.pins.p15.alt2(),
            BAUD,
        )
        .unwrap();
    let fifo_size = uart.set_tx_fifo(core::num::NonZeroU8::new(TX_FIFO_SIZE));
    log::info!("Setting TX FIFO to {}", fifo_size);
    // If this is disabled, we won't receive the four bytes from the transfer!
    uart.set_rx_fifo(true);
    uart.set_parity(PARITY);
    uart.set_rx_inversion(INVERTED);
    uart.set_tx_inversion(INVERTED);
    let mut led = bsp::configure_led(&mut peripherals.gpr, peripherals.pins.p13);
    let (mut tx, mut rx) = uart.split();
    loop {
        bsp::delay(1_000);
        led.toggle().unwrap();
        let mut buffer = DATA;
        write(&mut tx, &buffer).unwrap();
        bsp::delay(1);
        match read(&mut rx, &mut buffer) {
            Ok(_) => continue,
            Err(err) => log::warn!("Receiver error: {:?}", err.flags),
        }
    }
}
