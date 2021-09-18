//! Demonstrates how to react to GPIO inputs using RTIC.
//!
//! Connect LEDs to
//!
//! - pin 14
//! - pin 15 (optional)
//! - pin 16 (more optional).
//!
//! Also, wire together
//!
//! - pin 13 to pin 12
//! - pin 14 to pin 11 (optional)
//! - pin 15 to pin 10 (more optional)
//!
//! On each falling edge of one LED, the adjacent LED will
//! turn on, creating a binary counter. LED toggling happens
//! when the input GPIO (pins 12 and below) detect a falling
//! edge.
//!
//! Please refer to the [RTIC book](https://rtic.rs) for more
//! information on RTIC.
//!
//! NOTE: This example requires the `rtic` feature to be enabled.

#![no_std]
#![no_main]

use teensy4_bsp as bsp;
use teensy4_panic as _;

use bsp::common as pins;
use bsp::hal::gpio::{self, GPIO};

/// CHANGE ME to affect the behaviors of the pins.
const INTERRUPT_CONFIGURATION: gpio::InterruptConfiguration =
    gpio::InterruptConfiguration::FallingEdge;

/// Represents a software connection between two GPIOs.
///
/// Note that these are not the physical connections! See
/// the example documentation to understand the wiring for
/// this example.
//
// TODO why must this be pub? If it's not pub, we get
//
//  error[E0446]: private type `[TYPE_NAME]` in public interface
//
// The error manifests when specifying the gpio_interrupt hardware
// task, within the app.
pub struct GpioConnection<I, O> {
    input: GPIO<I, gpio::Input>,
    output: GPIO<O, gpio::Output>,
}

impl<I, O> GpioConnection<I, O>
where
    I: bsp::hal::iomuxc::gpio::Pin,
    O: bsp::hal::iomuxc::gpio::Pin,
{
    /// Allocate a new GPIO connection.
    ///
    /// This prepares the input pin to respond to interrupts.
    fn new(input_pin: I, output_pin: O) -> Self {
        let mut input = GPIO::new(input_pin);
        input.set_interrupt_configuration(INTERRUPT_CONFIGURATION);
        input.set_interrupt_enable(true);

        let output = GPIO::new(output_pin).output();
        Self { input, output }
    }

    /// Check the GPIO connection.
    ///
    /// If the input interrupt has triggered, toggle the connected
    /// output pin.
    fn check(&mut self) {
        if self.input.is_interrupt_status() {
            self.input.clear_interrupt_status();
            self.output.toggle();
        }
    }
}

/// The connection to drive pin 14's output.
type P12P14 = GpioConnection<pins::P12, pins::P14>;
/// The connection to drive pin 15's output.
type P11P15 = GpioConnection<pins::P11, pins::P15>;
/// The connection to drive pin 16's output.
type P10P16 = GpioConnection<pins::P10, pins::P16>;

#[rtic::app(device = teensy4_bsp, peripherals = true, dispatchers = [LPUART8])]
mod app {
    use dwt_systick_monotonic::{fugit::ExtU32, DwtSystick};
    use teensy4_bsp as bsp;

    // TODO why can't we use super, instead of crate? The app mod
    // is (visually) a submodule of the current module, so super
    // might be expected to work.
    use crate::{GpioConnection, P10P16, P11P15, P12P14};

    const MONO_HZ: u32 = bsp::hal::ccm::PLL1::ARM_HZ;
    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<MONO_HZ>;

    #[local]
    struct Local {
        led: bsp::Led,
        p12_p14: P12P14,
        p11_p15: P11P15,
        p10_p16: P10P16,
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
        let led = bsp::configure_led(pins.p13);

        let p12_p14 = GpioConnection::new(pins.p12, pins.p14);
        let p11_p15 = GpioConnection::new(pins.p11, pins.p15);
        let p10_p16 = GpioConnection::new(pins.p10, pins.p16);

        (
            Shared {},
            Local {
                led,
                p12_p14,
                p11_p15,
                p10_p16,
            },
            init::Monotonics(mono),
        )
    }

    #[task(local = [led])]
    fn blink(cx: blink::Context) {
        cx.local.led.toggle();
        // Schedule the following blink.
        blink::spawn_after(1_u32.secs()).unwrap();
    }

    /// This function represents the GPIO hardware interrupt.
    ///
    /// Why GPIO2? The input pins P12, P11, and P10 are all part of the
    /// GPIO2 port. To see this, look in the common pinout table of the
    /// teensy4-pins documentation. The values under Alt5 show the GPIO
    /// port and pin assignment.
    #[task(binds = GPIO2_Combined_0_15, local = [p12_p14, p11_p15, p10_p16])]
    fn gpio_interrupt(cx: gpio_interrupt::Context) {
        cx.local.p12_p14.check();
        cx.local.p11_p15.check();
        cx.local.p10_p16.check();
    }
}
