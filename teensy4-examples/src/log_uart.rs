//! Logging over UART
//!
//! This uses the `imxrt_uart_log` crate as a `log` back-end.
//! Connect a serial receiver to pin 14, and you should see
//! messages.

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::rt::entry;
use embedded_hal::digital::v2::ToggleableOutputPin;
use imxrt_uart_log::set_logger;
use teensy4_bsp as bsp;

const BAUD: u32 = 115_200;
const TX_FIFO_SIZE: u8 = 4;

#[entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
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
    uart.set_tx_fifo(core::num::NonZeroU8::new(TX_FIFO_SIZE));
    let mut led = bsp::configure_led(&mut peripherals.gpr, peripherals.pins.p13);
    let (tx, _) = uart.split();
    if let Err(_) = set_logger(tx, Default::default()) {
        led.toggle().unwrap();
        loop {
            core::sync::atomic::spin_loop_hint();
        }
    }
    loop {
        led.toggle().unwrap();
        log::info!("Hello world! 3 + 2 = {}", 3 + 2);
        bsp::delay(1_000);
    }
}
