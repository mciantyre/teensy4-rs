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
        pit: bsp::hal::pit::Pit,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let board::Resources {
            pins,
            mut pit,
            mut gpio2,
            ..
        } = board::t40(cx.device);
        let led = board::led(&mut gpio2, pins.p13);
        use bsp::hal::pit::Channel;
        pit.set_interrupt_enable(Channel::Chan2, true);
        pit.set_load_timer_value(Channel::Chan2, PIT_DELAY_MS);
        pit.enable(Channel::Chan2);
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

        use bsp::hal::pit::Channel;
        led.toggle();
        while pit.is_elapsed(Channel::Chan2) {
            pit.clear_elapsed(Channel::Chan2);
        }
    }
}
