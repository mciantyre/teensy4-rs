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

extern crate panic_halt;

use bsp::hal::dma;
use bsp::interrupt;
use bsp::rt::{entry, interrupt};
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
    dma::Peripheral<bsp::hal::spi::SPI<bsp::hal::spi::module::_4>, u16, TxBuffer, RxBuffer>;

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
        loop {
            core::sync::atomic::spin_loop_hint();
        }
    }

    if let Err((_, err)) = spi.start_transfer(tx_buffer) {
        log::error!("Unable to start DMA transfer: {:?}", err);
        if let dma::Error::Setup(es) = err {
            log::error!("{}", es);
        }
        spi.receive_cancel();
        loop {
            core::sync::atomic::spin_loop_hint();
        }
    }
}

fn take_buffers() -> (TxBuffer, RxBuffer) {
    let (tx_buffer, rx_buffer) = free(|cs| {
        let tx_buffer = TX_BUFFER.borrow(cs).borrow_mut().take();
        let rx_buffer = RX_BUFFER.borrow(cs).borrow_mut().take();
        (tx_buffer, rx_buffer)
    });
    if tx_buffer.is_none() || rx_buffer.is_none() {
        log::error!(
            "Buffers are none! tx.is_none() == {}, rx.is_none() == {}",
            tx_buffer.is_none(),
            rx_buffer.is_none()
        );
        loop {
            core::sync::atomic::spin_loop_hint();
        }
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
type HardwareFlag = bsp::hal::gpio::GPIO1IO26<bsp::hal::gpio::GPIO1, bsp::hal::gpio::Output>;
static mut HARDWARE_FLAG: Option<HardwareFlag> = None;

#[entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    peripherals.usb.init(Default::default());

    peripherals.ccm.pll1.set_arm_clock(
        bsp::hal::ccm::PLL1::ARM_HZ,
        &mut peripherals.ccm.handle,
        &mut peripherals.dcdc,
    );

    unsafe {
        let p20 = peripherals.pins.p20;
        use bsp::hal::gpio::IntoGpio;
        HARDWARE_FLAG = Some(p20.alt5().into_gpio().output());
    }

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
        SPI_DMA = Some(dma::bidirectional_u16(
            spi4,
            (tx_channel, dma::ConfigBuilder::new().build()),
            (rx_channel, rx_config),
        ));
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
        if tx_buffer_mut(|tx| {
            tx.as_mut_elements()[0] = read(WHO_AM_I);
            tx.set_transfer_len(1);
        })
        .is_none()
        {
            log::error!("Cannot prepare transfer buffer");
            loop {
                core::sync::atomic::spin_loop_hint();
            }
        }
        if rx_buffer_mut(|rx| {
            rx.set_transfer_len(1);
        })
        .is_none()
        {
            log::error!("Cannot prepare receive buffer");
            loop {
                core::sync::atomic::spin_loop_hint();
            }
        }
        bsp::delay(500);

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
                    log::error!("RX buffer was inaccessible!");
                    loop {
                        core::sync::atomic::spin_loop_hint();
                    }
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
        if let None = tx_buffer_mut(|tx| {
            tx.as_mut_elements()[..cmds.len()].copy_from_slice(&cmds);
            tx.set_transfer_len(cmds.len());
        }) {
            log::error!("Unable to modify transfer buffer!");
            loop {
                core::sync::atomic::spin_loop_hint();
            }
        }
        if let None = rx_buffer_mut(|rx| {
            rx.set_transfer_len(cmds.len());
        }) {
            log::error!("Unable to modify receive buffer!");
            loop {
                core::sync::atomic::spin_loop_hint();
            }
        }

        bsp::delay(500);
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
