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

use teensy4_panic as _;

#[rtic::app(device = teensy4_bsp, peripherals = true, dispatchers = [LPUART8])]
mod app {
    use bsp::hal::ral::usb::USB1;
    use embedded_hal::digital::v2::OutputPin;
    use teensy4_bsp as bsp;

    use dwt_systick_monotonic::{fugit::ExtU32, DwtSystick};

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<{ bsp::hal::ccm::PLL1::ARM_HZ }>;

    #[local]
    struct Local {
        counter: u32,
        led: bsp::Led,
        poller: bsp::usb::Poller,
    }

    #[shared]
    struct Shared {}

    #[init()]
    fn init(mut cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut dcb = cx.core.DCB;
        let dwt = cx.core.DWT;
        let systick = cx.core.SYST;

        let mono = DwtSystick::new(&mut dcb, dwt, systick, bsp::hal::ccm::PLL1::ARM_HZ);

        cx.device.ccm.pll1.set_arm_clock(
            bsp::hal::ccm::PLL1::ARM_HZ,
            &mut cx.device.ccm.handle,
            &mut cx.device.dcdc,
        );

        // Schedule the first blink.
        blink::spawn_after(1_u32.secs()).unwrap();
        let pins = bsp::t40::into_pins(cx.device.iomuxc);
        let mut led = bsp::configure_led(pins.p13);
        led.set_high().unwrap();

        // Initialize the USB system
        let (poller, _) = bsp::usb::init(USB1::take().unwrap(), Default::default()).unwrap();

        (
            Shared {},
            Local {
                counter: 0,
                led,
                poller,
            },
            init::Monotonics(mono),
        )
    }

    #[task(local = [led, counter])]
    fn blink(cx: blink::Context) {
        cx.local.led.toggle();
        // Schedule the following blink.
        blink::spawn_after(1_u32.secs()).unwrap();
        log::info!("Hello from RTIC! Count = {}", *cx.local.counter);
        *cx.local.counter += 1;
    }

    #[task(binds = USB_OTG1, local = [poller])]
    fn usb_otg1(cx: usb_otg1::Context) {
        cx.local.poller.poll();
    }
}
