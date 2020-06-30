//! Demonstrates DMA-powered memory copies
//!
//! Success: the byte pattern written to the transfer buffer is read from
//! the receive buffer. We can repeat the transfer multiple times.

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::{hal::dma, rt::entry};
use core::iter::ExactSizeIterator;
use teensy4_bsp as bsp;

/// Update me to play with different element types!
/// Valid types include any of the unsigned integers.
type Element = u32;

const BUFFER_SIZE: usize = 4096;

// We're using circular buffers for both the TX and RX buffers.
// When using the circular buffers, we need to align the underlying
// memory to a multiple of the memory size.
#[repr(align(16384))]
struct Alignment(dma::Buffer<[Element; BUFFER_SIZE]>);

// Memory that backs the DMA buffers
static TX_MEMORY: Alignment = Alignment(dma::Buffer::new([0; BUFFER_SIZE]));
static RX_MEMORY: Alignment = Alignment(dma::Buffer::new([0; BUFFER_SIZE]));

// Number of elements to move for each DMA memcpy
const NUMBER_OF_ELEMENTS: Element = (BUFFER_SIZE - 7) as Element;

#[entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    peripherals.usb.init(Default::default());
    peripherals.systick.delay(5_000);

    let mut dma_channels = peripherals.dma.clock(&mut peripherals.ccm.handle);
    let channel = dma_channels[13].take().unwrap();
    let mut memcpy = dma::Memcpy::new(channel);

    let mut tx_buffer = match dma::Circular::new(&TX_MEMORY.0) {
        Ok(buffer) => buffer,
        Err(error) => {
            log::error!("Unable to create the transfer buffer: {:?}", error);
            loop {
                core::sync::atomic::spin_loop_hint();
            }
        }
    };

    let mut rx_buffer = match dma::Circular::new(&RX_MEMORY.0) {
        Ok(buffer) => buffer,
        Err(error) => {
            log::error!("Unable to create the receive buffer: {:?}", error);
            loop {
                core::sync::atomic::spin_loop_hint();
            }
        }
    };

    let mut start: Element = 0;
    loop {
        let pattern = start..(start + NUMBER_OF_ELEMENTS);
        log::info!(
            "Inserting {}..{} into the transfer buffer...",
            start,
            start + NUMBER_OF_ELEMENTS
        );
        tx_buffer.insert(pattern.clone());
        rx_buffer.reserve(pattern.len());

        if let Err(error) = memcpy.transfer(tx_buffer, rx_buffer) {
            log::error!("Unable to start memcpy: {:?}", error);
            loop {
                core::sync::atomic::spin_loop_hint();
            }
        } else {
            log::info!("Transfer started...");
        }

        if (start + 1) % 3 != 0 {
            while !memcpy.is_complete() {}
        } else {
            log::info!("Cancelling this, and every third, transfer...");
        }

        let (tx, rx) = match memcpy.complete() {
            Some(Ok(buffers)) => buffers,
            Some(Err(buffers)) => {
                log::info!("Memcpy was cancelled!");
                buffers
            }
            None => {
                log::error!("Memcpy didn't give us back the buffers!");
                loop {
                    core::sync::atomic::spin_loop_hint();
                }
            }
        };

        tx_buffer = tx;
        rx_buffer = rx;

        let mut ok = if rx_buffer.len() != NUMBER_OF_ELEMENTS as usize {
            log::warn!(
                "Expected {} elements in the receive queue, but found {} elements",
                NUMBER_OF_ELEMENTS,
                rx_buffer.len()
            );
            false
        } else if !tx_buffer.is_empty() {
            log::warn!(
                "Expected there to be 0 elements in the transfer queue, but found {} elements",
                tx_buffer.len()
            );
            false
        } else {
            true
        };

        for (actual, expected) in rx_buffer.drain().zip(pattern) {
            if actual != expected {
                log::warn!(
                    "Expected {expected}, but actually found {actual} in the receive buffer",
                    expected = expected,
                    actual = actual
                );
                ok = false;
                break;
            }
        }

        if ok {
            log::info!("Transfer completed OK, and all data matched!");
        }

        rx_buffer.clear();
        tx_buffer.clear();

        start += 1;
        peripherals.systick.delay(5_000);
    }
}
