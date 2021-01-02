//! System tick and delay support
//!
//! If we're compiling this module, it's because the `"systick"` feature
//! is enabled.

#[cfg(all(target_arch = "arm", feature = "rt"))]
use crate::rt::exception;

static mut SYSTICK_MILLIS_COUNT: u32 = 0;

#[cfg(all(target_arch = "arm", feature = "rt"))]
#[exception]
fn SysTick() {
    unsafe {
        let ms = core::ptr::read_volatile(&SYSTICK_MILLIS_COUNT);
        let ms = ms.wrapping_add(1);
        core::ptr::write_volatile(&mut SYSTICK_MILLIS_COUNT, ms);
    }
}

/// Read the systick counter. Returns an absolute value describing
/// the number of milliseconds since the SYSTICK handler was enabled.
/// This may be used to implement coarse timing.
fn read() -> u32 {
    unsafe { core::ptr::read_volatile(&SYSTICK_MILLIS_COUNT) }
}

/// Blocks for at least `millis` milliseconds
///
/// `delay` will spin-loop on updates from SYSTICK, until
/// `millis` milliseconds have elapsed. SYSTICK has a 1ms
/// interrupt interval, so the minimal delay is around 1ms.
fn delay(millis: u32) {
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

/// SYSTICK external clock frequency
///
/// See Section 12.3.2.1 of the reference manual. The note
/// explains that the 24MHz clock is divided down to 100KHz
/// before reaching SYSTICK.
const SYSTICK_EXT_FREQ: u32 = 100_000;

/// A type that represents the system timer, SYSTICK
///
/// `SysTick` implements the `embedded_hal`'s `DelayMs` trait. It
/// may be used to implement simple, blocking delays.
///
/// # Example
///
/// ```no_run
/// use teensy4_bsp as bsp;
///
/// let core_peripherals = cortex_m::Peripherals::take().unwrap();
/// let mut systick = bsp::SysTick::new(core_peripherals.SYST);
///
/// systick.delay(50 /* ms */);
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "systick")))]
pub struct SysTick(cortex_m::peripheral::SYST);

impl SysTick {
    /// Convert the normal cortex-m SYST peripheral into a Teensy `SysTick`
    ///
    /// `new` will configure the systick counter for a 1ms tick. When `new()` returns,
    /// systick is counting.
    ///
    /// # Safety
    ///
    /// `new` is safe because it assumes that it has the only `SYST` instance.
    /// The only way you could acquire two `SysTick` is if you've unsafely obtained
    /// a second `SYST` instance.
    pub fn new(mut systick: cortex_m::peripheral::SYST) -> SysTick {
        systick.disable_counter();
        systick.set_clock_source(cortex_m::peripheral::syst::SystClkSource::External);
        systick.set_reload((SYSTICK_EXT_FREQ / 1000) - 1);
        systick.clear_current();
        systick.enable_counter();
        systick.enable_interrupt();
        SysTick(systick)
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
