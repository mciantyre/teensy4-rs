//! Demonstrates how to use a FlexCAN peripheral.
//!
//! Pinout:
//!
//! - Teensy 4 Pin 22 (CAN1 TX) to TX pin of CAN Transceiver
//! - Teensy 4 Pin 23 (CAN1 RX) to RX pin of CAN Transceiver
//!
//! A Can transceiver (such as the Texas Instruments SN65HVD230) is required for this demo.
//!

#![no_std]
#![no_main]

mod systick;
mod usb_io;

use teensy4_panic as _;

use cortex_m_rt::entry;
use teensy4_bsp as bsp;

#[entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    let mut systick = systick::new(cortex_m::Peripherals::take().unwrap().SYST);

    usb_io::init().unwrap();
    let pins = bsp::pins::t40::from_pads(peripherals.iomuxc);

    let mut led = bsp::configure_led(pins.p13);
    led.set();

    peripherals.ccm.pll1.set_arm_clock(
        bsp::hal::ccm::PLL1::ARM_HZ,
        &mut peripherals.ccm.handle,
        &mut peripherals.dcdc,
    );

    systick.delay_ms(1000);
    log::info!("Initializing CAN clocks...");

    let (can1_builder, _) = peripherals.can.clock(
        &mut peripherals.ccm.handle,
        bsp::hal::ccm::can::ClockSelect::OSC,
        bsp::hal::ccm::can::PrescalarSelect::DIVIDE_1,
    );

    let mut can1 = can1_builder.build(pins.p22, pins.p23);

    can1.set_baud_rate(1_000_000);
    can1.set_max_mailbox(16);
    can1.enable_fifo();
    can1.set_fifo_interrupt(false);
    can1.set_fifo_accept_all();
    can1.print_registers();

    let id = bsp::hal::can::Id::from(bsp::hal::can::StandardId::new(0x00).unwrap());
    let data: [u8; 8] = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    let frame = bsp::hal::can::Frame::new_data(id, data);

    unsafe { cortex_m::peripheral::NVIC::unmask(bsp::interrupt::CAN1) };
    loop {
        systick.delay_ms(1000);
        led.toggle();
        can1.read_mailboxes();
        can1.transmit(&frame).ok();
    }
}

use bsp::interrupt;
#[cortex_m_rt::interrupt]
fn CAN1() {
    log::warn!("CAN1 interrupt fired");
}
