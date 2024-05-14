//! Shows how to use RTIC with a periodic interrupt timer.

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
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let board::Resources {
            pins,
            pit: (_, _, mut pit, _),
            mut gpio2,
            ..
        } = board::t40(cx.device);
        let led = board::led(&mut gpio2, pins.p13);
        pit.set_interrupt_enable(true);
        pit.set_load_timer_value(PIT_DELAY_MS);
        pit.enable();
        (Shared {}, Local { led, pit })
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    #[task(binds = PIT, local = [led, pit])]
    fn blink(cx: blink::Context) {
        let pit = cx.local.pit;
        let led = cx.local.led;

        led.toggle();
        while pit.is_elapsed() {
            pit.clear_elapsed();
        }
    }
}
