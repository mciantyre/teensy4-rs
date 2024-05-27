//! Demonstrates how to configure an internal pull
//! on a GPIO. It samples pin 7 to determine the LED's
//! state.

#![no_std]
#![no_main]

use teensy4_panic as _;

#[rtic::app(device = teensy4_bsp, peripherals = true)]
mod app {
    use bsp::board;
    use bsp::{
        hal::{gpio, iomuxc},
        pins,
    };
    use teensy4_bsp as bsp;

    type Input = gpio::Input<pins::t40::P7>;

    const PIN_CONFIG: iomuxc::Config =
        iomuxc::Config::zero().set_pull_keeper(Some(iomuxc::PullKeeper::Pulldown100k));

    #[local]
    struct Local {
        led: board::Led,
        input: Input,
    }

    #[shared]
    struct Shared {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let board::Resources {
            mut pins,
            mut gpio2,
            ..
        } = board::t40(cx.device);
        let led = board::led(&mut gpio2, pins.p13);

        iomuxc::configure(&mut pins.p7, PIN_CONFIG);
        let input = gpio2.input(pins.p7);

        (Shared {}, Local { led, input })
    }

    #[idle(local = [led, input])]
    fn idle(cx: idle::Context) -> ! {
        let idle::LocalResources { led, input, .. } = cx.local;
        loop {
            if input.is_set() {
                led.set();
            } else {
                led.clear();
            }
        }
    }
}
