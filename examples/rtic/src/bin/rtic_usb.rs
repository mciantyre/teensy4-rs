//! An adaptation of the `rtic_blink.rs` example that adds USB logging.
//!
//! The logging support is handled by the BSP. Specifically, the BSP manages the
//! USB interrupt, and configures the peripheral for high-speed USB CDC support.
//! Simply use the logging macros in your RTIC application to send data to your
//! USB host.
//!
//! Please refer to the [RTIC book](https://rtic.rs) for more information on RTIC.
//!
//! NOTE: This example requires the `rtic` and `usb-logging` features to be enabled.

#![no_std]
#![no_main]

use bsp::hal::ral::usb::USB1;
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
        poller: bsp::usb::Poller,
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

        // Initialize the USB system
        let (poller, _) = bsp::usb::init(USB1::take().unwrap(), Default::default()).unwrap();

        init::LateResources { led, poller }
    }

    #[task(resources = [led], schedule = [blink])]
    fn blink(cx: blink::Context) {
        static mut COUNTER: u32 = 0;
        cx.resources.led.toggle();
        // Schedule the following blink.
        cx.schedule.blink(cx.scheduled + PERIOD.cycles()).unwrap();
        log::info!("Hello from RTIC! Count = {}", *COUNTER);
        *COUNTER += 1;
    }

    #[task(binds = USB_OTG1, resources = [poller])]
    fn usb_otg1(cx: usb_otg1::Context) {
        cx.resources.poller.poll();
    }

    // RTIC requires that unused interrupts are declared in an extern block when
    // using software tasks; these free interrupts will be used to dispatch the
    // software tasks.
    extern "C" {
        fn LPUART8();
    }
};
