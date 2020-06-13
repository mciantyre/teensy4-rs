//! Logging over UART
//!
//! This uses the `imxrt_uart_log` crate as a `log` back-end.
//! Connect a serial receiver to pin 14, and you should see
//! messages.

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::hal::gpt;
use bsp::interrupt;
use bsp::rt::{entry, interrupt};
use core::time::Duration;
use embedded_hal::digital::v2::ToggleableOutputPin;
use teensy4_bsp as bsp;

const BAUD: u32 = 115_200;
const TX_FIFO_SIZE: u8 = 4;

static mut TIMER: Option<gpt::GPT> = None;

/// GPT output compare register selection
const OCR: gpt::OutputCompareRegister = gpt::OutputCompareRegister::Three;
const INTERRUPT_PERIOD: Duration = Duration::from_millis(850);

#[interrupt]
unsafe fn GPT1() {
    let gpt1 = TIMER.as_mut().unwrap();
    gpt1.output_compare_status(OCR).clear();
    gpt1.set_enable(false);
    log::warn!("Called from interrupt!");
    gpt1.set_output_compare_duration(OCR, INTERRUPT_PERIOD);
    gpt1.set_enable(true);
}

#[entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    bsp::delay(5_000);

    let (_, ipg_hz) = peripherals.ccm.pll1.set_arm_clock(
        bsp::hal::ccm::PLL1::ARM_HZ,
        &mut peripherals.ccm.handle,
        &mut peripherals.dcdc,
    );

    //
    // UART initialization
    //
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
    uart.set_tx_fifo(core::num::NonZeroU8::new(TX_FIFO_SIZE));
    let mut led = bsp::configure_led(&mut peripherals.gpr, peripherals.pins.p13);
    let (tx, _) = uart.split();
    if let Err(_) = imxrt_uart_log::init(tx, Default::default()) {
        led.toggle().unwrap();
        loop {
            core::sync::atomic::spin_loop_hint();
        }
    }

    //
    // GPT2 initialization (for timing how long logging takes)
    //
    let mut cfg = peripherals.ccm.perclk.configure(
        &mut peripherals.ccm.handle,
        bsp::hal::ccm::perclk::PODF::DIVIDE_3,
        bsp::hal::ccm::perclk::CLKSEL::IPG(ipg_hz),
    );

    let mut gpt2 = peripherals.gpt2.clock(&mut cfg);
    gpt2.set_mode(bsp::hal::gpt::Mode::FreeRunning);
    gpt2.set_enable(true);

    //
    // GPT1 initialization (for demonstrating logging in an interrupt)
    //
    let mut gpt1 = peripherals.gpt1.clock(&mut cfg);
    gpt1.set_output_interrupt_on_compare(OCR, true);
    gpt1.set_wait_mode_enable(true);
    gpt1.set_mode(bsp::hal::gpt::Mode::FreeRunning);

    gpt1.set_enable(false);
    gpt1.set_output_compare_duration(OCR, INTERRUPT_PERIOD);
    gpt1.set_enable(true);

    unsafe {
        TIMER = Some(gpt1);
        cortex_m::peripheral::NVIC::unmask(interrupt::GPT1);
    }

    loop {
        let (_, duration) = gpt2.time(|| {
            led.toggle().unwrap();
            log::info!("Hello world! 3 + 2 = {}", 3 + 2);
            led.toggle().unwrap();
        });
        log::info!("Logging that took {:?}", duration);
        bsp::delay(1_000);

        let (_, duration) = gpt2.time(|| {
            led.toggle().unwrap();
            log::info!("Hello world! 3 + 2 = 5");
            led.toggle().unwrap();
        });
        log::info!("Logging that took {:?}", duration);
        bsp::delay(1_000);

        let (_, duration) = gpt2.time(|| {
            led.toggle().unwrap();
            log::info!("");
            led.toggle().unwrap();
        });
        log::info!("Logging that took {:?}", duration);
        bsp::delay(1_000);

        let (_, duration) = gpt2.time(|| {
            led.toggle().unwrap();
            // 100 characters
            log::info!("1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890");
            led.toggle().unwrap();
        });
        log::info!("Logging that took {:?}", duration);
        bsp::delay(1_000);
    }
}
