#![no_std]
#![no_main]

use embedded_hal::digital::v2::ToggleableOutputPin;
use panic_halt as _;
use teensy4_bsp as bsp;

#[rtic::app(device = teensy4_bsp, peripherals = true)]
const APP: () = {
    struct Resources {
        led: bsp::LED,
    }

    #[init]
    fn init(mut cx: init::Context) -> init::LateResources {
        init_delay();

        // Initialize the USB stack with the default logging settings.
        let usb_rx = cx.device.usb.init(bsp::usb::LoggingConfig {
            filters: &[("usb", None)],
            ..Default::default()
        });

        let led = bsp::configure_led(&mut cx.device.gpr, cx.device.pins.p13);
        init::LateResources { led }
    }

    #[task(binds = USB_OTG1, resources = [led])]
    fn usb(cx: usb::Context) {
        cx.resources.led.toggle().unwrap();
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            core::sync::atomic::spin_loop_hint();
        }
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
