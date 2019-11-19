//! Implements the systick exception and uses it
//! to toggle the LED every second.

#![no_std]
#![no_main]
extern crate panic_halt;

use teensy4_rt::{disable_led, enable_led, entry, exception};

const LED_PERIOD_MS: u32 = 1_000;

#[exception]
fn SysTick() {
    static mut COUNT: u32 = 0;
    if LED_PERIOD_MS == *COUNT {
        enable_led();
    } else if *COUNT > LED_PERIOD_MS * 2 {
        disable_led();
        *COUNT = 0;
    }
    *COUNT += 1;
}

// See Section 12.3.2.1 of the reference manual. The note
// explains that the 24MHz clock is divided down to 100KHz
// before reaching SYSTICK.
const SYSTICK_EXT_FREQ: u32 = 100_000;

#[entry]
fn main() -> ! {
    let mut cm = cortex_m::Peripherals::take().unwrap();
    cm.SYST.disable_counter();
    cm.SYST
        .set_clock_source(cortex_m::peripheral::syst::SystClkSource::External);
    cm.SYST.set_reload((SYSTICK_EXT_FREQ / 1000) - 1);
    cm.SYST.clear_current();
    cm.SYST.enable_counter();
    cm.SYST.enable_interrupt();
    loop {
        core::sync::atomic::spin_loop_hint();
    }
}
