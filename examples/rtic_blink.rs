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

        // Set the clock mode to 'RUN'
        //
        // i.MX RT (106x) processors will not wake on SYSTICK. When we enter
        // WFI or WFE, we'll enter WAIT mode by default. This will disable
        // SYSTICK. So, if you're waiting for SYSTICK to wake you up, it won't
        // happen. By setting ourselves to 'RUN' low-power mode, SYSTICK will
        // still wake us up.
        //
        // See the CCM_CLPCR register for more information. The HAL docs also note
        // this aspect of the processor.
        cx.device.ccm.set_mode(bsp::hal::ccm::ClockMode::Run);

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
