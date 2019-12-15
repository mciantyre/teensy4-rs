//! Enables a PIT timer to test interrupts

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::rt::entry;
use embedded_hal::{digital::v2::ToggleableOutputPin, timer::CountDown};
use teensy4_bsp as bsp;

#[entry]
fn main() -> ! {
    let mut periphs = bsp::Peripherals::take().unwrap();
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

    let cfg = periphs.ccm.perclk.configure(
        &mut periphs.ccm.handle,
        bsp::hal::ccm::perclk::PODF::DIVIDE_3,
        bsp::hal::ccm::perclk::CLKSEL::IPG(ipg_hz),
    );

    let (_, _, timer2, timer3) = periphs.pit.clock(cfg);
    let mut timer = bsp::hal::pit::chain(timer2, timer3);
    let mut led = periphs.led;

    timer.start(core::time::Duration::from_millis(250));
    loop {
        nb::block!(timer.wait()).unwrap();
        led.toggle().unwrap();
    }
}
