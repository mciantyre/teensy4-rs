//! System tick and delay support
//!
//! If we're compiling this module, it's because the `"systick"` feature
//! is enabled.

use crate::rt::exception;

#[no_mangle]
static mut systick_millis_count: u32 = 0;

#[exception]
fn SysTick() {
    unsafe {
        let ms = core::ptr::read_volatile(&systick_millis_count);
        let ms = ms.wrapping_add(1);
        core::ptr::write_volatile(&mut systick_millis_count, ms);
    }
}

/// Read the systick counter. Returns an absolute value describing
/// the number of milliseconds since the SYSTICK handler was enabled.
/// This may be used to implement coarse timing.
pub fn read() -> u32 {
    unsafe { core::ptr::read_volatile(&systick_millis_count) }
}

/// Blocks for at least `millis` milliseconds
///
/// `delay()` will spin-loop on updates from SYSTICK, until
/// `millis` milliseconds have elapsed. SYSTICK has a 1ms
/// interrupt interval, so the minimal delay is around 1ms.
#[no_mangle]
pub extern "C" fn delay(millis: u32) {
    if 0 == millis {
        return;
    }
    let start = read();
    let target = start + millis;
    loop {
        let count = read();
        if count >= target {
            return;
        }
    }
}

/// A type that represents the system timer, SYSTICK
///
/// `SysTick` implements the `embedded_hal`'s `DelayMs` trait. It
/// may be used to implement simple, blocking delays.
pub struct SysTick(());

impl SysTick {
    pub(crate) fn new() -> Self {
        SysTick(())
    }

    /// Blocks for `ms` milliseconds
    pub fn delay(&mut self, ms: u32) {
        self::delay(ms);
    }
}

impl embedded_hal::blocking::delay::DelayMs<u32> for SysTick {
    fn delay_ms(&mut self, ms: u32) {
        self::delay(ms);
    }
}

impl embedded_hal::blocking::delay::DelayMs<u16> for SysTick {
    fn delay_ms(&mut self, ms: u16) {
        self::delay(ms.into());
    }
}

impl embedded_hal::blocking::delay::DelayMs<u8> for SysTick {
    fn delay_ms(&mut self, ms: u8) {
        self::delay(ms.into());
    }
}
