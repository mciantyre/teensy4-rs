//! Demonstrates a blinking LED based on a hardware timer
//!
//! This is similar to the rtic-blink example, only that it
//! uses a hardware interrupt instead of the scheduler and the
//! monotonic clock.

#![no_std]
#![no_main]

use teensy4_bsp as bsp;
use teensy4_panic as _;

type Timer = bsp::hal::pit::PIT<bsp::hal::pit::channel::_3>;

#[rtic::app(device = teensy4_bsp, peripherals = true)]
mod app {
    use super::{bsp, Timer};
    use embedded_hal::timer::CountDown;

    #[local]
    struct Local {
        led: bsp::Led,
        timer: Timer,
    }

    #[shared]
    struct Shared {}

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let (_, ipg_hz) = cx.device.ccm.pll1.set_arm_clock(
            bsp::hal::ccm::PLL1::ARM_HZ,
            &mut cx.device.ccm.handle,
            &mut cx.device.dcdc,
        );

        let mut cfg = cx.device.ccm.perclk.configure(
            &mut cx.device.ccm.handle,
            bsp::hal::ccm::perclk::PODF::DIVIDE_3,
            bsp::hal::ccm::perclk::CLKSEL::IPG(ipg_hz),
        );

        let (_, _, _, mut timer) = cx.device.pit.clock(&mut cfg);
        timer.set_interrupt_enable(true);
        timer.start(core::time::Duration::from_millis(250));

        let pins = bsp::t40::into_pins(cx.device.iomuxc);
        let mut led = bsp::configure_led(pins.p13);
        led.set();

        (Shared {}, Local { led, timer }, init::Monotonics())
    }

    #[task(binds = PIT, local = [led, timer])]
    fn blink(cx: blink::Context) {
        let timer = cx.local.timer;
        let led = cx.local.led;

        if let Ok(()) = timer.wait() {
            led.toggle();
        }
    }
}
