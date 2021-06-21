//! Helper module for SysTick configuration.

use cortex_m::{
    delay::Delay,
    peripheral::{syst::SystClkSource, SYST},
};

/// Re-export of the SYSTICK type for these examples.
pub type SysTick = Delay;

/// Create a blocking delay over SYSTICK.
pub fn new(syst: SYST) -> SysTick {
    Delay::with_source(syst, teensy4_bsp::EXT_SYSTICK_HZ, SystClkSource::External)
}
