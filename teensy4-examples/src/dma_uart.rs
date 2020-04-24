//! Demonstrates a DMA-based UART transfer and receive

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::interrupt;
use bsp::rt::entry;
use teensy4_bsp as bsp;

use embedded_hal::digital::v2::ToggleableOutputPin;

use core::sync::atomic::{AtomicBool, Ordering};

const BAUD: u32 = 115_200;
const RX_TRANSFER_SIZE: usize = 1;
const TX_TRANSFER_SIZE: usize = 50;

const BUFFER_SIZE: usize = 128; // Arbitrary size; might not use it all
static mut RX_BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
static mut TX_BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

/// A DMA peripheral adapter around the UART2 peripheral. It sends bytes.
static mut DMA_PERIPHERAL: Option<
    bsp::hal::dma::Peripheral<bsp::hal::uart::UART<bsp::hal::uart::module::_2>, u8>,
> = None;

static TX_READY: AtomicBool = AtomicBool::new(false);
static RX_READY: AtomicBool = AtomicBool::new(false);

static mut LED: Option<bsp::LED> = None;

#[bsp::rt::interrupt]
unsafe fn DMA7_DMA23() {
    let uart = DMA_PERIPHERAL.as_mut().unwrap();

    while uart.transfer_interrupt() {
        uart.transfer_clear_interrupt();
        uart.transfer_clear_complete();
        TX_READY.store(true, Ordering::Release);
    }
    while uart.receive_interrupt() {
        uart.receive_clear_interrupt();
        uart.receive_clear_complete();
        RX_READY.store(true, Ordering::Release);
        LED.as_mut().unwrap().toggle().unwrap();
    }
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
    let uart = uarts
        .uart2
        .init(
            peripherals.pins.p14.alt2(),
            peripherals.pins.p15.alt2(),
            BAUD,
        )
        .unwrap();

    let mut dma_channels = peripherals.dma.clock(&mut peripherals.ccm.handle);
    let tx_channel = dma_channels[7].take().unwrap();
    let rx_channel = dma_channels[23].take().unwrap();

    let config = bsp::hal::dma::ConfigBuilder::new()
        .interrupt_on_completion(true)
        .build();

    let dma_uart = unsafe {
        DMA_PERIPHERAL = Some(bsp::hal::dma::Peripheral::new_transfer_receive(
            uart,
            (tx_channel, config),
            (rx_channel, config),
        ));
        cortex_m::peripheral::NVIC::unmask(interrupt::DMA7_DMA23);
        DMA_PERIPHERAL.as_mut().unwrap()
    };

    let led = unsafe {
        LED = Some(bsp::configure_led(
            &mut peripherals.gpr,
            peripherals.pins.p13,
        ));
        LED.as_mut().unwrap()
    };
    let mut alphabet = (b'A'..b'Z').cycle();

    let rx_buffer = unsafe { &mut RX_BUFFER };
    let tx_buffer = unsafe { &mut TX_BUFFER };

    // Schedule an initial receive
    let res = unsafe { dma_uart.start_receive(&mut rx_buffer[..RX_TRANSFER_SIZE]) };
    if let Err(err) = res {
        log::warn!("Error scheduling initial DMA receive: {:?}", err);
    } else {
        log::info!(
            "Scheduled an intial DMA receive of {} bytes OK",
            RX_TRANSFER_SIZE
        );
    }

    // Start sending some data
    let letter = alphabet.next().unwrap();
    for byte in tx_buffer.iter_mut().take(TX_TRANSFER_SIZE) {
        *byte = letter;
    }
    let res = unsafe { dma_uart.start_transfer(&mut tx_buffer[..TX_TRANSFER_SIZE]) };
    if let Err(err) = res {
        log::warn!("Error scheduling initial DMA transfer: {:?}", err);
    } else {
        log::info!(
            "Scheduled an intial DMA transfer of {} bytes OK",
            TX_TRANSFER_SIZE
        );
    }

    loop {
        bsp::delay(1_000);

        if RX_READY.load(Ordering::Acquire) {
            log::info!("Received data: {:?}", &rx_buffer[..RX_TRANSFER_SIZE]);
            log::info!(
                "Scheduling another receive for {} bytes...",
                RX_TRANSFER_SIZE
            );
            RX_READY.store(false, Ordering::Release);
            let res = unsafe { dma_uart.start_receive(&mut rx_buffer[..RX_TRANSFER_SIZE]) };
            if let Err(err) = res {
                log::warn!("Error scheduling initial DMA receive: {:?}", err);
            } else {
                log::info!(
                    "Scheduled an intial DMA receive of {} bytes OK",
                    RX_TRANSFER_SIZE
                );
            }
        }

        if TX_READY.load(Ordering::Acquire) {
            log::info!("Transferred {} bytes of data!", TX_TRANSFER_SIZE);
            log::info!("Scheduling another transfer...");
            TX_READY.store(false, Ordering::Release);
            let letter = alphabet.next().unwrap();
            for byte in tx_buffer.iter_mut().take(TX_TRANSFER_SIZE) {
                *byte = letter;
            }
            let res = unsafe { dma_uart.start_transfer(&mut tx_buffer[..TX_TRANSFER_SIZE]) };
            if let Err(err) = res {
                log::warn!("Error scheduling DMA transfer: {:?}", err);
            }
        }
    }
}
