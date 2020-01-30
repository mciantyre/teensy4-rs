//! Demonstrates how to use a periodic interrupt
//! timer (PIT) to measure a duration.

//! Enables a PIT timer to test interrupts

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::hal::pit;
use bsp::rt::entry;
use embedded_hal::timer::CountDown;
use teensy4_bsp as bsp;

#[entry]
fn main() -> ! {
    let mut periphs = bsp::Peripherals::take().unwrap();
    bsp::delay(25);
    periphs.log.init(Default::default());

    let (_, ipg_hz) = periphs.ccm.pll1.set_arm_clock(
        bsp::hal::ccm::PLL1::ARM_HZ,
        &mut periphs.ccm.handle,
        &mut periphs.dcdc,
    );

    let cfg = periphs.ccm.perclk.configure(
        &mut periphs.ccm.handle,
        bsp::hal::ccm::perclk::PODF::DIVIDE_3,
        bsp::hal::ccm::perclk::CLKSEL::IPG(ipg_hz),
    );

    let (timer0, timer1, mut timer2, mut timer3) = periphs.pit.clock(cfg);
    let mut timer = pit::chain(timer0, timer1);
    loop {
        let (_, period) = timer.time(|| {
            bsp::delay(7_000);
        });
        match period {
            Some(period) => log::info!("Timed a {:?} long event with the lifetime timer", period),
            None => log::warn!("Lifetime timer expired!"),
        };

        timer3.start(core::time::Duration::from_micros(217));
        let (_, period) = timer2.time(|| {
            nb::block!(timer3.wait()).unwrap();
        });
        match period {
            Some(period) => log::info!("Timed a {:?} long event with timer 2", period),
            None => log::warn!("Timer 2 expired!"),
        }
    }
}
