//! Enables a PIT timer to test interrupts

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::hal::pit::U32Ext;
use bsp::rt::entry;
use embedded_hal::{digital::v2::ToggleableOutputPin, timer::CountDown};
use teensy4_bsp as bsp;

const fn ms_to_ticks(ms: u32) -> u32 {
    const CLOCK_FREQUENCY_MHZ: u32 = 24;
    const CLOCK_PERIOD_NS: u32 = 1_000_000_000 / (CLOCK_FREQUENCY_MHZ * 1_000_000);
    ((ms * 1_000_000) / CLOCK_PERIOD_NS) - 1
}
const BLINK_PERIOD_TICKS: u32 = ms_to_ticks(500);

#[entry]
fn main() -> ! {
    let mut periphs = bsp::Peripherals::take().unwrap();
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

    let cfg = periphs.ccm.perclk.configure(
        &mut periphs.ccm.handle,
        bsp::hal::ccm::perclk::PODF::DIVIDE_1,
        bsp::hal::ccm::perclk::CLKSEL::PERCLK_CLK_SEL_1,
    );

    let [_, _, _, mut timer3] = periphs.pit.clock(cfg);
    let mut led = periphs.led;

    loop {
        timer3.start(BLINK_PERIOD_TICKS.ms());
        nb::block!(timer3.wait()).unwrap();
        led.toggle().unwrap();
    }
}
