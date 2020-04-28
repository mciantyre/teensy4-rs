//! Demonstrates a DMA-based UART transfer and receive
//!
//! The example waits for a UART character, then echos that
//! character back to the user with a header. Each received
//! character causes the LED to toggle. Received bytes and
//! replies are logged over USB.
//!
//! Each receive is a DMA transfer of one byte from a UART
//! peripheral. Then, each response is a multi-byte DMA
//! transfer to the UART peripheral.
//!
//! Pinout:
//! - Pin 14: UART2_TX
//! - Pin 15: UART2_RX

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::interrupt;
use bsp::rt::entry;
use teensy4_bsp as bsp;

use embedded_hal::digital::v2::ToggleableOutputPin;

use core::sync::atomic::{AtomicBool, Ordering};

/// Modify your baud rate here
const BAUD: u32 = 115_200;

/// A DMA peripheral adapter around the UART2 peripheral. It sends bytes.
static mut DMA_PERIPHERAL: Option<
    bsp::hal::dma::Peripheral<bsp::hal::uart::UART<bsp::hal::uart::module::_2>, u8>,
> = None;

/// Flags for the main loop
static TX_READY: AtomicBool = AtomicBool::new(false);
static RX_READY: AtomicBool = AtomicBool::new(false);

/// The DMA interrupt matches the selected DMA channels in the demo's setup.
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
    let mut led = bsp::configure_led(&mut peripherals.gpr, peripherals.pins.p13);

    let mut rx_buffer: [u8; 1] = [0; 1];
    // Received: X
    const REPLY_OFFSET: usize = 10;
    let mut tx_buffer: [u8; 13] = [
        b'R', b'e', b'c', b'e', b'i', b'v', b'e', b'd', b':', b' ', 0, b'\r', b'\n',
    ];

    'start: loop {
        RX_READY.store(false, Ordering::Release);
        TX_READY.store(false, Ordering::Release);

        // Schedule an initial receive
        let res = unsafe { dma_uart.start_receive(&mut rx_buffer) };
        if let Err(err) = res {
            log::warn!("Error scheduling DMA receive: {:?}", err);
            loop {
                core::sync::atomic::spin_loop_hint();
            }
        }

        loop {
            cortex_m::asm::wfi();
            if RX_READY.load(Ordering::Acquire) {
                led.toggle().unwrap();
                RX_READY.store(false, Ordering::Release);
                log::info!("Received: {}", rx_buffer[0]);
                tx_buffer[REPLY_OFFSET] = rx_buffer[0];
                let res = unsafe { dma_uart.start_transfer(&tx_buffer) };
                if let Err(err) = res {
                    log::warn!("Error scheduling DMA transfer: {:?}", err);
                    loop {
                        core::sync::atomic::spin_loop_hint();
                    }
                }
            } else if TX_READY.load(Ordering::Acquire) {
                continue 'start;
            }
        }
    }
}
