//! Demonstrates a manual USB logger setup for use with RTIC.
//!
//! The BSP's `LoggingFrontend` helper will conveniently implement a USB ISR
//! that drives USB logging. But, when you're using a framework like RTIC, you
//! may want to expose the USB poller directly to the framework. Since we're
//! registering `USB_OTG1` ourself, we do not enable the BSP's `"usb-logging"`
//! feature. For more information, see the BSP's feature documentation.

#![no_std]
#![no_main]

use teensy4_panic as _;

#[rtic::app(device = teensy4_bsp, peripherals = true)]
mod app {
    use bsp::board;
    use teensy4_bsp as bsp;

    const PIT_DELAY_MS: u32 = board::PERCLK_FREQUENCY / 1_000 * 250;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: board::Led,
        pit: bsp::hal::pit::Pit<2>,
        poller: bsp::logging::Poller,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let board::Resources {
            pins,
            mut gpio2,
            pit: (_, _, mut pit, _),
            usb,
            ..
        } = board::t40(cx.device);
        let led = board::led(&mut gpio2, pins.p13);
        pit.set_interrupt_enable(true);
        pit.set_load_timer_value(PIT_DELAY_MS);
        pit.enable();

        // let poller = bsp::logging::log::usbd(usb, bsp::logging::Interrupts::Enabled).unwrap();
        let poller = bsp::logging::defmt::usbd(usb, bsp::logging::Interrupts::Enabled).unwrap();
        (Shared {}, Local { led, pit, poller }, init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    #[task(binds = PIT, local = [led, pit])]
    fn blink_and_log(cx: blink_and_log::Context) {
        let pit = cx.local.pit;
        let led = cx.local.led;

        led.toggle();
        while pit.is_elapsed() {
            pit.clear_elapsed();
        }

        #[derive(defmt::Format)]
        struct ImuData {
            acc: [i32; 3],
            gyro: [i32; 3],
            mag: [i32; 3],
        }

        impl ImuData {
            const fn fake() -> Self {
                ImuData {
                    acc: [1, 2, 3],
                    gyro: [4, 5, 6],
                    mag: [7, 8, 9],
                }
            }
        }

        let fake_data = ImuData::fake();
        defmt::println!("{}", fake_data);
    }

    #[task(binds = USB_OTG1, local = [poller])]
    fn poll_logger(cx: poll_logger::Context) {
        cx.local.poller.poll();
    }
}
