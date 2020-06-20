//! General purpose timer (GPT) example
//!
//! Success: we interrupt every 400ms, and we
//! toggle the LED.

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::hal::gpt;
use bsp::interrupt;
use bsp::rt::{entry, interrupt};
use embedded_hal::digital::v2::ToggleableOutputPin;
use teensy4_bsp as bsp;

use core::time::Duration;

static mut TIMER: Option<gpt::GPT> = None;

/// GPT output compare register selection
const OCR: gpt::OutputCompareRegister = gpt::OutputCompareRegister::Three;

#[interrupt]
unsafe fn GPT1() {
    TIMER.as_mut().unwrap().output_compare_status(OCR).clear();
}

#[entry]
fn main() -> ! {
    let mut periphs = bsp::Peripherals::take().unwrap();

    let (_, ipg_hz) = periphs.ccm.pll1.set_arm_clock(
        bsp::hal::ccm::PLL1::ARM_HZ,
        &mut periphs.ccm.handle,
        &mut periphs.dcdc,
    );

    let mut cfg = periphs.ccm.perclk.configure(
        &mut periphs.ccm.handle,
        bsp::hal::ccm::perclk::PODF::DIVIDE_3,
        bsp::hal::ccm::perclk::CLKSEL::IPG(ipg_hz),
    );

    let mut gpt1 = periphs.gpt1.clock(&mut cfg);

    gpt1.set_output_interrupt_on_compare(OCR, true);
    gpt1.set_wait_mode_enable(true);
    gpt1.set_mode(bsp::hal::gpt::Mode::FreeRunning);

    unsafe {
        TIMER = Some(gpt1);
        cortex_m::peripheral::NVIC::unmask(interrupt::GPT1);
    }

    let mut led = bsp::configure_led(&mut periphs.gpr, periphs.pins.p13);
    loop {
        let gpt1 = unsafe { TIMER.as_mut().unwrap() };
        gpt1.set_enable(false);
        gpt1.set_output_compare_duration(OCR, Duration::from_millis(400));
        gpt1.set_enable(true);
        cortex_m::asm::wfi();
        led.toggle().unwrap();
    }
}
