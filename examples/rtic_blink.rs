//! An adaptation of the `rtic_led.rs` example that demonstrates:
//!
//! 1. how to define resources and
//! 2. how to use the systick interrupt to cause the LED to blink.
//!
//! Please refer to the [RTIC book](https://rtic.rs) for more information on RTIC.
//!
//! NOTE: This example requires the `rtic` feature to be enabled.

#![no_std]
#![no_main]

use teensy4_panic as _;

#[rtic::app(device = teensy4_bsp, peripherals = true, dispatchers = [LPUART8])]
mod app {
    use dwt_systick_monotonic::{fugit::ExtU32, DwtSystick};
    use teensy4_bsp as bsp;

    const MONO_HZ: u32 = bsp::hal::ccm::PLL1::ARM_HZ;
    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<MONO_HZ>;

    #[local]
    struct Local {
        led: bsp::Led,
    }

    #[shared]
    struct Shared {}

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut dcb = cx.core.DCB;
        let dwt = cx.core.DWT;
        let systick = cx.core.SYST;

        let mono = DwtSystick::new(&mut dcb, dwt, systick, MONO_HZ);

        // Ensure the ARM clock is configured for the default speed seeing as we use this speed to
        // determine a 1 second `PERIOD`.
        cx.device
            .ccm
            .pll1
            .set_arm_clock(MONO_HZ, &mut cx.device.ccm.handle, &mut cx.device.dcdc);

        // Schedule the first blink.
        blink::spawn_after(1_u32.secs()).unwrap();
        let pins = bsp::t40::into_pins(cx.device.iomuxc);
        let mut led = bsp::configure_led(pins.p13);
        led.set();

        (Shared {}, Local { led }, init::Monotonics(mono))
    }

    #[task(local = [led])]
    fn blink(cx: blink::Context) {
        cx.local.led.toggle();
        // Schedule the following blink.
        blink::spawn_after(1_u32.secs()).unwrap();
    }
}
