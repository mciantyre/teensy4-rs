//! An adaptation of the `rtic_led.rs` example that demonstrates:
//!
//! 1. how to share late resources and
//! 2. how to use the systick interrupt to cause the LED to blink.
//!
//! Please refer to the [RTIC book](https://rtic.rs) for more information on RTIC.
//!
//! NOTE: This example requires the `rtic` feature to be enabled.

#![no_std]
#![no_main]

use embedded_hal::digital::v2::{OutputPin, ToggleableOutputPin};
use panic_halt as _;
use rtic::cyccnt::U32Ext;
use teensy4_bsp as bsp;

// The CYCCNT counts in clock cycles. Using the clock hz should give us a ~1 second period.
const PERIOD: u32 = bsp::hal::ccm::PLL1::ARM_HZ;

#[rtic::app(device = teensy4_bsp, monotonic = rtic::cyccnt::CYCCNT, peripherals = true)]
const APP: () = {
    struct Resources {
        led: bsp::LED,
    }

    #[init(schedule = [blink])]
    fn init(mut cx: init::Context) -> init::LateResources {
        init_delay();

        // Initialise the monotonic CYCCNT timer.
        cx.core.DWT.enable_cycle_counter();

        // Ensure the ARM clock is configured for the default speed seeing as we use this speed to
        // determine a 1 second `PERIOD`.
        cx.device.ccm.pll1.set_arm_clock(
            bsp::hal::ccm::PLL1::ARM_HZ,
            &mut cx.device.ccm.handle,
            &mut cx.device.dcdc,
        );

        // Schedule the first blink.
        cx.schedule.blink(cx.start + PERIOD.cycles()).unwrap();

        let mut led = bsp::configure_led(&mut cx.device.gpr, cx.device.pins.p13);
        led.set_high().unwrap();

        init::LateResources { led }
    }

    #[task(resources = [led], schedule = [blink])]
    fn blink(cx: blink::Context) {
        cx.resources.led.toggle().unwrap();
        // Schedule the following blink.
        cx.schedule.blink(cx.scheduled + PERIOD.cycles()).unwrap();
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            core::sync::atomic::spin_loop_hint();
        }
    }

    // RTIC requires that unused interrupts are declared in an extern block when
    // using software tasks; these free interrupts will be used to dispatch the
    // software tasks.
    extern "C" {
        fn LPUART8();
    }
};

// If we reach WFI on teensy 4.0 too quickly it seems to halt. Here we wait a short while in `init`
// to avoid this issue. The issue only appears to occur when rebooting the device (via the button),
// however there appears to be no issue when power cycling the device.
//
// TODO: Investigate exactly why this appears to be necessary.
fn init_delay() {
    for _ in 0..10_000_000 {
        core::sync::atomic::spin_loop_hint();
    }
}
