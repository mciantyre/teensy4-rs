//! Enables a PIT timer to test interrupts
//!
//! Success criteria: the LED is on for 250ms,
//! then off for 250ms, then on for 250ms, then off
//! for 250ms... This is observable using a scope, or
//! a really good eye, finger, and stopwatch.

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::hal::pit;
use bsp::interrupt;
use bsp::rt::{entry, interrupt};
use embedded_hal::{digital::v2::ToggleableOutputPin, timer::CountDown};
use teensy4_bsp as bsp;

static mut TIMER: Option<pit::PIT<pit::channel::_3>> = None;

#[interrupt]
unsafe fn PIT() {
    // If this timer expired, `wait()` returns `Ok(())`
    // after re-arming the timer. On the other hand,
    // `wait()` returns `Err(WouldBlock)` if the timer
    // didn't expire. Use this to change state.
    let _ = TIMER.as_mut().unwrap().wait();
}

#[entry]
fn main() -> ! {
    let mut periphs = bsp::Peripherals::take().unwrap();
    // When flashing a debug build, I'm finding that
    // the chip is likely to crash if we don't put this
    // delay here. I've narrowed it down to something
    // with the WFI in the loop, maybe...? If I instead
    // busy-loop on an atomic U32, I don't crash in debug
    // builds.
    bsp::delay(25);
    let (_, ipg_hz) = periphs.ccm.pll1.set_arm_clock(
        bsp::hal::ccm::PLL1::ARM_HZ,
        &mut periphs.ccm.handle,
        &mut periphs.dcdc,
    );
    periphs.ccm.pll2.set(
        &mut periphs.ccm.handle,
        [
            Some(bsp::hal::ccm::pll2::MHZ_352),
            Some(bsp::hal::ccm::pll2::MHZ_594),
            Some(bsp::hal::ccm::pll2::MHZ_396),
            Some(bsp::hal::ccm::pll2::MHZ_297),
        ],
    );
    periphs.ccm.pll3.set(
        &mut periphs.ccm.handle,
        [
            Some(bsp::hal::ccm::pll3::MHZ_720),
            Some(bsp::hal::ccm::pll3::MHZ_664),
            Some(bsp::hal::ccm::pll3::MHZ_508),
            Some(bsp::hal::ccm::pll3::MHZ_454),
        ],
    );

    let mut cfg = periphs.ccm.perclk.configure(
        &mut periphs.ccm.handle,
        bsp::hal::ccm::perclk::PODF::DIVIDE_3,
        bsp::hal::ccm::perclk::CLKSEL::IPG(ipg_hz),
    );

    let (_, _, _, mut timer) = periphs.pit.clock(&mut cfg);
    timer.set_interrupt_enable(true);
    unsafe {
        TIMER = Some(timer);
        TIMER
            .as_mut()
            .unwrap()
            .start(core::time::Duration::from_millis(250));
        cortex_m::peripheral::NVIC::unmask(interrupt::PIT);
    }
    let mut led = bsp::configure_led(&mut periphs.gpr, periphs.pins.p13);
    loop {
        led.toggle().unwrap();
        cortex_m::asm::wfi();
    }
}
