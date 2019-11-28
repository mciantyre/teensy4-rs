//! Enables a PIT timer to test interrupts

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::interrupt;
use embedded_hal::digital::v2::ToggleableOutputPin;
use teensy4_bsp as bsp;
use teensy4_rt::{entry, interrupt};

static mut LED: Option<bsp::LED> = None;
static mut REARM: Option<bsp::hal::pit::Rearm> = None;

#[interrupt]
fn PIT() {
    unsafe {
        LED.as_mut().unwrap().toggle().unwrap();
        REARM.as_mut().unwrap().rearm();
    }
}

const fn ms_to_ticks(ms: u32) -> u32 {
    const CLOCK_FREQUENCY_MHZ: u32 = 24;
    const CLOCK_PERIOD_NS: u32 = 1_000_000_000 / (CLOCK_FREQUENCY_MHZ * 1_000_000);
    ((ms * 1_000_000) / CLOCK_PERIOD_NS) - 1
}
const BLINK_PERIOD_TICKS: u32 = ms_to_ticks(500);

#[entry]
fn main() -> ! {
    let mut periphs = bsp::Peripherals::take().unwrap();
    unsafe {
        LED = Some(periphs.led);
        cortex_m::peripheral::NVIC::unmask(bsp::interrupt::PIT);
    }
    let cfg = periphs.ccm.perclk.configure(
        &mut periphs.ccm.handle,
        bsp::hal::ccm::perclk::PODF::DIVIDE_1,
        bsp::hal::ccm::perclk::CLKSEL::PERCLK_CLK_SEL_1,
    );

    let mut pit = periphs.pit.clock(cfg);
    let (mut timer, rearm) = pit.timer::<bsp::hal::pit::T1>();
    unsafe { REARM = Some(rearm) };
    timer.load(BLINK_PERIOD_TICKS).enable();

    loop {
        cortex_m::asm::wfi();
    }
}
