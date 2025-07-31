//! Demonstrates a simple USB logging stack.
//!
//! As of version 0.5 of the BSP, the BSP doesn't include an internal
//! logging stack. You're responsible for sourcing or building your
//! own. This example shows an easy approach: use imxrt-log. The benefit
//! is that it's easy to set up. The drawback is that it's not easy to
//! use the USB stack for anything else other than logging.
//!
//! If you want to use the USB stack for other functions, see rtic_defmt_usb_log.rs,
//! which sets up its own USB device stack.
//!
//! Build this example just like any other example in this project. See
//! the README for more information.
//!
//! You can then observe text logging by connecting to your USB serial
//! device.

#![no_std]
#![no_main]

use teensy4_panic as _;

#[rtic::app(device = teensy4_bsp, peripherals = true, dispatchers = [KPP])]
mod app {
    use bsp::board;
    use teensy4_bsp as bsp;

    use imxrt_log as logging;

    use rtic_monotonics::systick::*;

    #[local]
    struct Local {
        /// For driving the logging endpoint.
        poller: logging::Poller,
        /// For periodically signaling activity.
        led: board::Led,
    }

    #[shared]
    struct Shared {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let board::Resources {
            usb,
            pins,
            mut gpio2,
            ..
        } = board::t40(cx.device);
        let led = board::led(&mut gpio2, pins.p13);

        // Set up the logging system.
        //
        // There's various ways to control log levels at build- and run-time.
        // See the imxrt-log documentation for more information. This example
        // doesn't demonstrate any of that.
        let poller = logging::log::usbd(usb, logging::Interrupts::Enabled).unwrap();

        // Set up a system timer for our software task.
        {
            Systick::start(
                cx.core.SYST,
                board::ARM_FREQUENCY,
                rtic_monotonics::create_systick_token!(),
            );
        }

        // Schedule that software task to run.
        make_logs::spawn().unwrap();

        // If the LED turns on, we've made it past init.
        led.set();

        (Shared {}, Local { poller, led })
    }

    /// This task periodically logs data.
    ///
    /// You won't see all the log levels until you configure your build. See the
    /// top-level docs for more information.
    #[task(local = [led])]
    async fn make_logs(cx: make_logs::Context) {
        let make_logs::LocalResources { led, .. } = cx.local;

        let mut counter = 0u32;
        loop {
            led.toggle();
            Systick::delay(250.millis()).await;

            log::trace!("TRACE: {counter}");

            if counter % 3 == 0 {
                log::debug!("DEBUG: {counter}");
            }

            if counter % 5 == 0 {
                log::info!("INFO: {counter}");
            }

            if counter % 7 == 0 {
                log::warn!("WARN: {counter}");
            }

            if counter % 31 == 0 {
                log::error!("ERROR: {counter}");
            }

            counter = counter.wrapping_add(1);
        }
    }

    /// This task runs when the USB1 interrupt activates.
    /// Simply poll the logger to control the logging process.
    #[task(binds = USB_OTG1, local = [poller])]
    fn usb_interrupt(cx: usb_interrupt::Context) {
        cx.local.poller.poll();
    }
}
