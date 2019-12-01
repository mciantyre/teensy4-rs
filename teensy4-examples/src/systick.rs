//! Implements the systick exception and uses it
//! to toggle the LED every second.

#![no_std]
#![no_main]
extern crate panic_halt;

use embedded_hal::digital::v2::ToggleableOutputPin;
use teensy4_bsp as bsp;
use teensy4_rt::{entry, exception};

const LED_PERIOD_MS: u32 = 1_000;

static mut LED: Option<bsp::LED> = None;

#[exception]
fn SysTick() {
    static mut COUNT: u32 = 0;
    if *COUNT > LED_PERIOD_MS {
        unsafe { LED.as_mut().unwrap().toggle().unwrap() };
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
    let mut p = bsp::Peripherals::take().unwrap();
    unsafe {
        LED = Some(p.led);
        cortex_m::interrupt::enable();
    }
    p.systick.disable_counter();
    p.systick
        .set_clock_source(cortex_m::peripheral::syst::SystClkSource::External);
    p.systick.set_reload((SYSTICK_EXT_FREQ / 1000) - 1);
    p.systick.clear_current();
    p.systick.enable_counter();
    p.systick.enable_interrupt();
    loop {
        core::sync::atomic::spin_loop_hint();
    }
}
