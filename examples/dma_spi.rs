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
//! WHO_AM_I register value. We read MPU measurements at 2Hz. The first time
//! our interrupt handler is hit, we set pin 20 high.
//!
//! This example is very similar to the blocking SPI example. If this
//! example isn't working, make sure `spi.rs` works with the same
//! pinout.

#![no_main]
#![no_std]

mod systick;
mod usb_io;

use teensy4_panic as _;

use bsp::hal::dma;
use bsp::interrupt;
use cortex_m_rt::{entry, interrupt};
use teensy4_bsp as bsp;

use core::{
    cell::RefCell,
    sync::atomic::{AtomicBool, Ordering},
};
use cortex_m::interrupt::{free, Mutex};

const SPI_BAUD_RATE_HZ: u32 = 1_000_000;

/// DMA interrupt matches the two selected DMA channels
///
/// It clears the interrupt, and completes the transfer.
/// We'll WFI, then check the flag to see if we triggered.
#[interrupt]
unsafe fn DMA9_DMA25() {
    let spi = SPI_DMA.as_mut().unwrap();
    while spi.is_receive_interrupt() {
        spi.receive_clear_interrupt();
    }

    let cs = cortex_m::interrupt::CriticalSection::new();
    if spi.is_receive_complete() {
        let rx_buffer = RX_BUFFER.borrow(&cs);
        *rx_buffer.borrow_mut() = spi.receive_complete();
    }

    if spi.is_transfer_complete() {
        let tx_buffer = TX_BUFFER.borrow(&cs);
        *tx_buffer.borrow_mut() = spi.transfer_complete();
    }

    let hardware_flag = HARDWARE_FLAG.as_mut().unwrap();
    use embedded_hal::digital::v2::OutputPin;
    hardware_flag.set_high().unwrap();

    FLAG.store(true, Ordering::Release);
}

type TxBuffer = dma::Linear<u16>;
type RxBuffer = dma::Linear<u16>;

static TX_MEM: dma::Buffer<[u16; 32]> = dma::Buffer::new([0; 32]);
static RX_MEM: dma::Buffer<[u16; 32]> = dma::Buffer::new([0; 32]);

static TX_BUFFER: Mutex<RefCell<Option<TxBuffer>>> = Mutex::new(RefCell::new(None));
static RX_BUFFER: Mutex<RefCell<Option<RxBuffer>>> = Mutex::new(RefCell::new(None));

type SpiDma =
    dma::Peripheral<bsp::hal::spi::SPI<bsp::hal::iomuxc::consts::U4>, u16, TxBuffer, RxBuffer>;

// TODO types should be Send
static mut SPI_DMA: Option<SpiDma> = None;

static FLAG: AtomicBool = AtomicBool::new(false);

/// Prepares a SPI transaction that writes and reads data using DMA
///
/// `prepare_transfer` never returns if there is an error.
///
/// # Safety
///
/// Caller must ensure that the lifetime of the two buffers is greater
/// than the lifetime of the DMA transfer.
fn prepare_transfer(spi: &mut SpiDma) {
    //   Prime the transfers
    //
    // Start the receive first, then the transfer, since the
    // transfer causes data to be received.
    let (tx_buffer, rx_buffer) = take_buffers();
    if let Err((_, err)) = spi.start_receive(rx_buffer) {
        log::error!("Unable to start DMA receive: {:?}", err);
        if let dma::Error::Setup(es) = err {
            log::error!("{}", es);
        }
        panic!();
    }

    if let Err((_, err)) = spi.start_transfer(tx_buffer) {
        log::error!("Unable to start DMA transfer: {:?}", err);
        if let dma::Error::Setup(es) = err {
            log::error!("{}", es);
        }
        spi.receive_cancel();
        panic!();
    }
}

fn take_buffers() -> (TxBuffer, RxBuffer) {
    let (tx_buffer, rx_buffer) = free(|cs| {
        let tx_buffer = TX_BUFFER.borrow(cs).borrow_mut().take();
        let rx_buffer = RX_BUFFER.borrow(cs).borrow_mut().take();
        (tx_buffer, rx_buffer)
    });
    if tx_buffer.is_none() || rx_buffer.is_none() {
        panic!(
            "Buffers are none! tx.is_none() == {}, rx.is_none() == {}",
            tx_buffer.is_none(),
            rx_buffer.is_none()
        );
    }

    (tx_buffer.unwrap(), rx_buffer.unwrap())
}

fn tx_buffer_mut<F: FnOnce(&mut TxBuffer) -> R, R>(act: F) -> Option<R> {
    free(move |cs| {
        let mut tx_buffer = TX_BUFFER.borrow(cs).borrow_mut();
        tx_buffer.as_mut().map(act)
    })
}

fn rx_buffer_mut<F: FnOnce(&mut RxBuffer) -> R, R>(act: F) -> Option<R> {
    free(move |cs| {
        let mut rx_buffer = RX_BUFFER.borrow(cs).borrow_mut();
        rx_buffer.as_mut().map(act)
    })
}

// Pin 20
type HardwareFlag = bsp::hal::gpio::GPIO<bsp::hal::iomuxc::ad_b1::AD_B1_10, bsp::hal::gpio::Output>;
static mut HARDWARE_FLAG: Option<HardwareFlag> = None;

#[entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    let mut systick = systick::new(cortex_m::Peripherals::take().unwrap().SYST);
    usb_io::init().unwrap();
    let pins = bsp::pins::t40::from_pads(peripherals.iomuxc);

    peripherals.ccm.pll1.set_arm_clock(
        bsp::hal::ccm::PLL1::ARM_HZ,
        &mut peripherals.ccm.handle,
        &mut peripherals.dcdc,
    );

    unsafe {
        let p20 = pins.p20;
        HARDWARE_FLAG = Some(bsp::hal::gpio::GPIO::new(p20).output());
    }

    //
    // SPI setup
    //

    systick.delay_ms(5000);
    log::info!("Initializing SPI4 clocks...");

    let (_, _, _, spi4_builder) = peripherals.spi.clock(
        &mut peripherals.ccm.handle,
        bsp::hal::ccm::spi::ClockSelect::Pll2,
        bsp::hal::ccm::spi::PrescalarSelect::LPSPI_PODF_5,
    );

    log::info!("Constructing SPI4 peripheral...");
    let mut spi4 = spi4_builder.build(pins.p11, pins.p12, pins.p13);
    spi4.enable_chip_select_0(pins.p10);

    match spi4.set_clock_speed(bsp::hal::spi::ClockSpeed(SPI_BAUD_RATE_HZ)) {
        Ok(()) => {
            log::info!("Set clock speed to {}Hz", SPI_BAUD_RATE_HZ);
        }
        Err(err) => {
            panic!(
                "Unable to set clock speed to {}Hz: {:?}",
                SPI_BAUD_RATE_HZ, err
            );
        }
    };

    //
    // DMA setup
    //

    let mut dma_channels = peripherals.dma.clock(&mut peripherals.ccm.handle);
    let tx_channel = dma_channels[9].take().unwrap();
    let mut rx_channel = dma_channels[25].take().unwrap();
    rx_channel.set_interrupt_on_completion(true);

    // We only want to interrupt when the receive completes. When
    // the receive completes, we know that we're also done transferring
    // data.
    let spi = unsafe {
        SPI_DMA = Some(dma::bidirectional_u16(spi4, tx_channel, rx_channel));
        cortex_m::peripheral::NVIC::unmask(interrupt::DMA9_DMA25);
        SPI_DMA.as_mut().unwrap()
    };

    // Prepare transfer and receive buffers
    let tx_buffer = dma::Linear::new(&TX_MEM).unwrap();
    let rx_buffer = dma::Linear::new(&RX_MEM).unwrap();
    free(move |cs| {
        *TX_BUFFER.borrow(cs).borrow_mut() = Some(tx_buffer);
        *RX_BUFFER.borrow(cs).borrow_mut() = Some(rx_buffer);
    });

    //
    // Query and wait for response to WHO_AM_I
    //

    'who_am_i: loop {
        let prep_tx = |tx: &mut dma::Linear<u16>| {
            tx.as_mut_elements()[0] = read(WHO_AM_I);
            tx.set_transfer_len(1);
        };
        if tx_buffer_mut(prep_tx).is_none() {
            panic!("Cannot prepare transfer buffer");
        }

        let prep_rx = |rx: &mut dma::Linear<u16>| rx.set_transfer_len(1);
        if rx_buffer_mut(prep_rx).is_none() {
            panic!("Cannot prepare receive buffer");
        }
        systick.delay_ms(500);

        log::info!("Started DMA transfers for WHO_AM_I");
        FLAG.store(false, Ordering::Release);
        prepare_transfer(spi);
        loop {
            cortex_m::asm::wfi();
            if FLAG.load(Ordering::Acquire) {
                if let Some(who_am_i) = rx_buffer_mut(|rx| rx.as_elements()[0]) {
                    if 0x71 == who_am_i {
                        log::info!("Completed SPI tranfer! WHO_AM_I = {:#X}", who_am_i & 0xFF);
                        break 'who_am_i;
                    } else {
                        log::warn!("Incorrect WHO_AM_I {:#X} received!", who_am_i);
                    }
                } else {
                    panic!("RX buffer was inaccessible!");
                }
            }
        }
    }

    //
    // Loop and report measurements
    //
    log::info!("Dropping into loop for readings...");
    'measurements: loop {
        let cmds = command_3dof();
        let set_tx_buf = |tx: &mut dma::Linear<u16>| {
            tx.as_mut_elements()[..cmds.len()].copy_from_slice(&cmds);
            tx.set_transfer_len(cmds.len());
        };
        if tx_buffer_mut(set_tx_buf).is_none() {
            panic!("Unable to modify transfer buffer!");
        }

        let set_rx_buf = |rx: &mut dma::Linear<u16>| {
            rx.set_transfer_len(cmds.len());
        };
        if rx_buffer_mut(set_rx_buf).is_none() {
            panic!("Unable to modify receive buffer!");
        }

        systick.delay_ms(500);
        FLAG.store(false, Ordering::Release);
        prepare_transfer(spi);
        loop {
            cortex_m::asm::wfi();
            if FLAG.load(Ordering::Acquire) {
                if let Some(rx_buffer) = rx_buffer_mut(|rx| {
                    let mut readings = [0; 6];
                    readings.copy_from_slice(&rx.as_elements()[..6]);
                    readings
                }) {
                    log_6dof(&rx_buffer);
                    continue 'measurements;
                }
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
