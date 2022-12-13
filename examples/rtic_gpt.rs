//! Shows how to use RTIC with two general purpose timers.

#![no_std]
#![no_main]

use teensy4_panic as _;

#[rtic::app(device = teensy4_bsp, peripherals = true)]
mod app {
    use bsp::board;
    use bsp::hal::{self, gpt};
    use teensy4_bsp as bsp;

    // Given this GPT clock source...
    const GPT_CLOCK_SOURCE: gpt::ClockSource = gpt::ClockSource::PeripheralClock;
    // ...and this GPT-specific divider...
    const GPT_DIVIDER: u32 = 8;
    /// ...the GPT frequency is
    const GPT_FREQUENCY: u32 = board::PERCLK_FREQUENCY / GPT_DIVIDER;

    const GPT_DELAY_MS: u32 = GPT_FREQUENCY / 1_000 * 250;
    const OCR: hal::gpt::OutputCompareRegister = hal::gpt::OutputCompareRegister::OCR3;

    #[local]
    struct Local {}

    #[shared]
    struct Shared {
        led: board::Led,
        gpt1: hal::gpt::Gpt1,
        gpt2: hal::gpt::Gpt2,
    }

    fn init_gpt<const N: u8>(gpt: &mut hal::gpt::Gpt<N>) {
        gpt.set_clock_source(GPT_CLOCK_SOURCE);
        gpt.set_divider(GPT_DIVIDER);
        gpt.set_output_compare_count(OCR, GPT_DELAY_MS);
        gpt.set_mode(hal::gpt::Mode::Restart);
        gpt.set_reset_on_enable(true);
        gpt.set_output_interrupt_on_compare(OCR, true);
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let board::Resources {
            pins,
            mut gpt1,
            mut gpt2,
            mut gpio2,
            ..
        } = board::t40(cx.device);
        let led = board::led(&mut gpio2, pins.p13);
        init_gpt(&mut gpt1);
        init_gpt(&mut gpt2);

        gpt1.enable();
        (Shared { led, gpt1, gpt2 }, Local {}, init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    #[task(binds = GPT1, shared = [led, gpt1, gpt2])]
    fn turn_on(cx: turn_on::Context) {
        let gpt1 = cx.shared.gpt1;
        let gpt2 = cx.shared.gpt2;
        let led = cx.shared.led;

        (gpt1, gpt2, led).lock(|gpt1, gpt2, led| {
            gpt1.clear_elapsed(OCR);
            gpt1.disable();
            led.set();

            while gpt1.is_elapsed(OCR) {}
            gpt2.enable();
        })
    }

    #[task(binds = GPT2, shared = [led, gpt1, gpt2])]
    fn turn_off(cx: turn_off::Context) {
        let gpt1 = cx.shared.gpt1;
        let gpt2 = cx.shared.gpt2;
        let led = cx.shared.led;

        (gpt1, gpt2, led).lock(|gpt1, gpt2, led| {
            gpt2.clear_elapsed(OCR);
            gpt2.disable();
            led.clear();

            while gpt2.is_elapsed(OCR) {}
            gpt1.enable();
        })
    }
}
