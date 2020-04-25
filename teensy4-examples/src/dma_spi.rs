//! Demonstrates DMA-based SPI transfers and receives
//!
//! We read the WHO_AM_I register from an MPU9250, then we read accelerometer
//! values. The MPU9250 supports full-duplex transfers of `u16`s. We can
//! support full-duplex SPI with two DMA channels: one for transfer with
//! SPI, and one for receive with SPI.
//!
//! Pinout:
//!
//! - Teensy 4 Pin 13 (SCK) to MPU's SCL (Note that we lose the LED here)
//! - Teensy 4 Pin 11 (MOSI) to MPU's SDA/SDI
//! - Teensy 4 Pin 12 (MISO) to MPU's AD0/SDO
//! - Teensy 4 Pin 10 (PSC0) to MPU's NCS
//!
//! We utilize the dedicated chip select, since we can't sit there and
//! toggle a GPIO while a transfer is in progress.
//!
//! Success criteria: the clock runs at 1MHz. We read `0x71` as the
//! WHO_AM_I register value. We read MPU measurements at 2Hz
//!
//! This example is very similar to the blocking SPI example. If this
//! example isn't working, make sure `spi.rs` works with the same
//! pinout.

#![no_main]
#![no_std]

extern crate panic_halt;

use bsp::hal::dma;
use bsp::interrupt;
use bsp::rt::{entry, interrupt};
use teensy4_bsp as bsp;

use core::sync::atomic::{AtomicBool, Ordering};

const SPI_BAUD_RATE_HZ: u32 = 1_000_000;

/// DMA interrupt matches the two selected DMA channels
///
/// It clears the interrupt, and completes the transfer.
/// We'll WFI, then check the flag to see if we triggered.
#[interrupt]
unsafe fn DMA9_DMA25() {
    let spi = SPI_DMA.as_mut().unwrap();
    while spi.receive_interrupt() {
        spi.receive_clear_interrupt();
    }
    while spi.receive_complete() {
        spi.receive_clear_complete();
    }
    while spi.transfer_complete() {
        spi.transfer_clear_complete();
    }
    FLAG.store(true, Ordering::Release);
}

static mut SPI_DMA: Option<dma::Peripheral<bsp::hal::spi::SPI<bsp::hal::spi::module::_4>, u16>> =
    None;

static FLAG: AtomicBool = AtomicBool::new(false);

/// Prepares a SPI transaction that writes and reads data using DMA
///
/// `prepare_transfer` never returns if there is an error.
///
/// # Safety
///
/// Caller must ensure that the lifetime of the two buffers is greater
/// than the lifetime of the DMA transfer.
unsafe fn prepare_transfer<P>(
    spi: &mut dma::Peripheral<P, u16>,
    tx_buffer: &[u16],
    rx_buffer: &mut [u16],
) where
    P: dma::Source<u16> + dma::Destination<u16>,
    <P as dma::Source<u16>>::Error: core::fmt::Debug,
    <P as dma::Destination<u16>>::Error: core::fmt::Debug,
{
    //   Prime the transfers
    //
    // Start the receive first, then the transfer, since the
    // transfer causes data to be received.
    if let Err(err) = spi.start_receive(rx_buffer) {
        log::warn!("Unable to start DMA receive: {:?}", err);
        if let dma::Error::Setup(es) = err {
            log::warn!("{}", es);
        }
        loop {
            core::sync::atomic::spin_loop_hint();
        }
    }

    if let Err(err) = spi.start_transfer(tx_buffer) {
        log::warn!("Unable to start DMA transfer: {:?}", err);
        if let dma::Error::Setup(es) = err {
            log::warn!("{}", es);
        }
        spi.receive_cancel();
        loop {
            core::sync::atomic::spin_loop_hint();
        }
    }
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

    //
    // SPI setup
    //

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
    spi4.enable_chip_select_0(peripherals.pins.p10.alt3());

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

    //
    // DMA setup
    //

    let mut dma_channels = peripherals.dma.clock(&mut peripherals.ccm.handle);
    let tx_channel = dma_channels[9].take().unwrap();
    let rx_channel = dma_channels[25].take().unwrap();
    let rx_config = dma::ConfigBuilder::new()
        .interrupt_on_completion(true)
        .build();

    // We only want to interrupt when the receive completes. When
    // the receive completes, we know that we're also done transferring
    // data.
    let spi = unsafe {
        SPI_DMA = Some(dma::transfer_receive_u16(
            spi4,
            (tx_channel, dma::ConfigBuilder::default().build()),
            (rx_channel, rx_config),
        ));
        cortex_m::peripheral::NVIC::unmask(interrupt::DMA9_DMA25);
        SPI_DMA.as_mut().unwrap()
    };

    //
    // Query and wait for response to WHO_AM_I
    //

    'who_am_i: loop {
        let tx_buffer = [read(WHO_AM_I)];
        let mut rx_buffer: [u16; 1] = [0; 1];

        bsp::delay(500);
        unsafe {
            // Safey: memory exists on the stack for the lifetime
            // of the transfer.
            prepare_transfer(spi, &tx_buffer, &mut rx_buffer);
        }

        log::info!("Started DMA transfers for WHO_AM_I");
        FLAG.store(false, Ordering::Release);
        loop {
            cortex_m::asm::wfi();
            if FLAG.load(Ordering::Acquire) {
                if 0x71 == rx_buffer[0] {
                    log::info!(
                        "Completed SPI tranfer! WHO_AM_I = {:#X}",
                        rx_buffer[0] & 0xFF
                    );
                    break 'who_am_i;
                } else {
                    log::warn!("Incorrect WHO_AM_I {:#X} received!", rx_buffer[0]);
                }
            }
        }
    }

    //
    // Loop and report measurements
    //
    log::info!("Dropping into loop for readings...");
    'measurements: loop {
        let tx_buffer = command_3dof();
        let mut rx_buffer: [u16; 6] = [0; 6];

        bsp::delay(500);
        unsafe {
            // Safety: buffer on stack, but always in scope
            prepare_transfer(spi, &tx_buffer, &mut rx_buffer);
        }

        FLAG.store(false, Ordering::Release);
        loop {
            cortex_m::asm::wfi();
            if FLAG.load(Ordering::Acquire) {
                log_6dof(&rx_buffer);
                continue 'measurements;
            }
        }
    }
}

const WHO_AM_I: u8 = 0x75;

/// Creates a read instruction for the MPU9250
const fn read(address: u8) -> u16 {
    ((address as u16) | (1 << 7)) << 8
}

/// Creates a command that can read all accelerometer values
fn command_3dof() -> [u16; 6] {
    let accelerometer_registers = 0x3B..=0x40;
    let commands = accelerometer_registers.map(read);
    let mut buffer: [u16; 6] = [0; 6];
    for (dst, src) in buffer.iter_mut().zip(commands) {
        *dst = src;
    }
    buffer
}

/// Prints the 6DOF values to the info log
fn log_6dof(raw: &[u16; 6]) {
    const LABELS: [&str; 3] = ["ACC_X", "ACC_Y", "ACC_Z"];
    let values = raw
        .chunks(2)
        .map(|pairs| (pairs[0] << 8) | (pairs[1] & 0xFF));
    LABELS
        .iter()
        .zip(values)
        .for_each(|(label, value)| log::info!("{}: {} ", label, value));
}
