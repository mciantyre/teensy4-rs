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

use embedded_hal::digital::v2::OutputPin;
use rtic::cyccnt::U32Ext;
use teensy4_bsp as bsp;
use teensy4_panic as _;

// The CYCCNT counts in clock cycles. Using the clock hz should give us a ~1 second period.
const PERIOD: u32 = bsp::hal::ccm::PLL1::ARM_HZ;

#[rtic::app(device = teensy4_bsp, monotonic = rtic::cyccnt::CYCCNT, peripherals = true)]
const APP: () = {
    struct Resources {
        led: bsp::Led,
    }

    #[init(schedule = [blink])]
    fn init(mut cx: init::Context) -> init::LateResources {
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
        let pins = bsp::t40::into_pins(cx.device.iomuxc);
        let mut led = bsp::configure_led(pins.p13);
        led.set_high().unwrap();

        init::LateResources { led }
    }

    #[task(resources = [led], schedule = [blink])]
    fn blink(cx: blink::Context) {
        cx.resources.led.toggle();
        // Schedule the following blink.
        cx.schedule.blink(cx.scheduled + PERIOD.cycles()).unwrap();
    }

    // RTIC requires that unused interrupts are declared in an extern block when
    // using software tasks; these free interrupts will be used to dispatch the
    // software tasks.
    extern "C" {
        fn LPUART8();
    }
};
