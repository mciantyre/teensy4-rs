//! Demonstrates how to use a periodic interrupt
//! timer (PIT) and general purpose timer
//! (GPT) to measure a duration.
//!
//! Success criteria: the chained timer measures 500ms, and
//! GPT 2 measures the 200us delay implemented by timer 3.
//! The times are reported over USB. While in the spin loops,
//! the LED is enabled, so it should be possible to measure
//! the delay by watching the LED.

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::hal::pit;
use bsp::rt::entry;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::timer::CountDown;
use teensy4_bsp as bsp;

#[entry]
fn main() -> ! {
    let mut periphs = bsp::Peripherals::take().unwrap();
    periphs.systick.delay(25);
    periphs.usb.init(Default::default());

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

    let mut gpt2 = periphs.gpt2.clock(&mut cfg);
    gpt2.set_mode(bsp::hal::gpt::Mode::FreeRunning);
    gpt2.set_enable(true);

    let (timer0, timer1, _, mut timer3) = periphs.pit.clock(&mut cfg);
    let mut timer = pit::chain(timer0, timer1);
    let mut led: bsp::LED = bsp::configure_led(&mut periphs.gpr, periphs.pins.p13);
    let mut systick = periphs.systick;
    loop {
        let (_, period) = timer.time(|| {
            led.set_high().unwrap();
            systick.delay(500);
            led.set_low().unwrap();
        });
        match period {
            Some(period) => log::info!("Timed a {:?} long event with the lifetime timer", period),
            None => log::warn!("Lifetime timer expired!"),
        };

        systick.delay(100);

        timer3.start(core::time::Duration::from_micros(200));
        let (_, period) = gpt2.time(|| {
            led.set_high().unwrap();
            nb::block!(timer3.wait()).unwrap();
            led.set_low().unwrap();
        });
        log::info!("GPT2 timed a {:?} long event", period);

        systick.delay(100);
    }
}
