//! A Rust board support package (BSP) for the Teensy 4.
//!
//! As of this writing, the BSP is very primitive. It exposes
//! only the LED, and it configures facilities for logging over USB.
//! Otherwise, it simply forwards components from the HAL for your
//! own usage. This will be addressed as the HAL becomes more developed.
//!
//! The BSP does assume some facilities of the processor:
//!
//! - it registers the `SysTick` exception handler, and configures
//!   SYSTICK for a 1ms interrupt.
//! - it registers the `USB_OTG1` interrupt, and uses the USB1
//!   peripheral for logging.
//!
//! These peripherals and capabilities are not exported from the BSP.
//! If a user also registers a `SysTick` or `USB_OTG1` handler, it may
//! result in a duplicate definition error.
//!
//! ## Re-exports
//!
//! The BSP re-exports the following:
//!
//! - the `imxrt1062-rt` crate, as `rt`
//! - the `imxrt1062-hal` crate, as `hal`
//!
//! See the accompanying documentation of each crate for more
//! information.
//!
//! For simplicity, there may be other choice APIs from either crate that
//! are re-exported in the BSP namespace.
//!
//! Although it's not exported publicly, the BSP crate links in the
//! `teensy4-fcb` crate, which provides a Firmware Configuration Block (FCB)
//! specific to the Teensy 4. See the `imxrt1062-fcb-gen` crate for details
//! on FCBs.
//!
//! ## Physical Pins to Pads
//!
//! The list below describes the mapping between a Teensy 4 pin and its pad.
//! Parentheticals describe possible alternatives of the pins. Alternatives
//! are not listed in any particular order, and they may be incomplete. See
//! the reference manual for the mapping of alternative to functionality.
//!
//! 0. `AD_B0_03`  (`FlexPWM1_1_X`)
//! 1. `AD_B0_02`  (`FlexPWM1_0_X`)
//! 2. `EMC_04`  (`FlexPWM4_2_A`)
//! 3. `EMC_05`  (`FlexPWM4_2_B`)
//! 4. `EMC_06`  (`FlexPWM2_0_A`)
//! 5. `EMC_08`  (`FlexPWM2_1_A`)
//! 6. `B0_10` (`FlexPWM2_2_A`)
//! 7. `B1_01`  (`FlexPWM1_3_B`)
//! 8. `B1_00`  (`FlexPWM1_3_A`)
//! 9. `B0_11`  (`FlexPWM2_2_B`)
//! 10. `B0_00`  (`QuadTimer1_0`)
//! 11. `B0_02`  (`QuadTimer1_2`)
//! 12. `B0_01`  (`QuadTimer1_1`)
//! 13. `B0_03`  (`QuadTimer2_0`, `GPIO2_3` (`LED`))
//! 14. `AD_B1_02` (`QuadTimer3_2`)
//! 15. `AD_B1_03` (`QuadTimer3_3`)
//! 16. `AD_B1_07` (`I2C3_SCL`)
//! 17. `AD_B1_06` (`I2C3_SDA`)
//! 18. `AD_B1_01` (`QuadTimer3_1`, `I2C1_SDA`)
//! 19. `AD_B1_00` (`QuadTimer3_0`, `I2C1_SCL`)
//! 20. `AD_B1_10
//! 21. `AD_B1_11`
//! 22. `AD_B1_08` (`FlexPWM4_0_A`)
//! 23. `AD_B1_09` (`FlexPWM4_1_A`)
//! 24. `AD_B0_12` (`FlexPWM1_2_X`, `I2C4_SCL`)
//! 25. `AD_B0_13` (`FlexPWM1_3_X`, `I2C4_SDA`)
//! 26. `AD_B0_14`
//! 27. `AD_B0_15`
//! 28. `EMC_32` (`FlexPWM3_1_B`)
//! 29. `EMC_31` (`FlexPWM3_1_A`)
//! 30. `EMC_37`
//! 31. `EMC_36`
//! 32. `B0_12`
//! 33. `EMC_07` (`FlexPWM2_0_B`)
//! 34. `SD_B0_03` (`FlexPWM1_1_B`)
//! 35. `SD_B0_02` (`FlexPWM1_1_A`)
//! 36. `SD_B0_01` (`FlexPWM1_0_B`, `I2C3_SDA`)
//! 37. `SD_B0_00` (`FlexPWM1_0_A`, `I2C3_SCL`)
//! 38. `SD_B0_05` (`FlexPWM1_2_B`)
//! 39. `SD_B0_04` (`FlexPWM1_2_A`)
//!
//! References:
//! - [Teensy 4.0 Schematic Diagram](https://www.pjrc.com/teensy/schematic.html)
//!
//! ## Examples
//!
//! See the `teensy4-examples` crate for build-able, run-able
//! examples. The examples utilize this BSP crate to blink LEDs,
//! establish timers, and log data over USB.
//!
//! ## Notice of alpha status
//!
//! We've made some assumptions in this MVP BSP.
//!
//! - SYSTICK and delay implementation is very naive. Do not run for 49
//!   continuous days, or risk a millisecond counter wrap-around.

#![no_std]

// Need to reference this so that it doesn't get stripped out
extern crate teensy4_fcb;

pub mod log;

pub use hal::pac::interrupt;
pub use imxrt1062_hal as hal;
pub use imxrt1062_rt as rt;
pub use teensy4_usb_sys::serial_write;
pub type LED = hal::gpio::IO03<hal::gpio::GPIO7, hal::gpio::Output>;

pub use hal::ccm::CCM;
pub use hal::pac::PIT;
pub use hal::pac::SYST;

/// Teensy pins that do not yet have a function
pub struct Pins {
    /// Pin 6
    pub p6: hal::iomuxc::gpio::GPIO_B0_10<hal::iomuxc::Alt5>,
    /// Pin 9
    pub p9: hal::iomuxc::gpio::GPIO_B0_11<hal::iomuxc::Alt5>,
}

/// All peripherals available on the Teensy4
pub struct Peripherals {
    /// The LED (AKA, pin 13)
    pub led: LED,
    /// Clock control module (forwarded from the HAL)
    pub ccm: hal::ccm::CCM,
    /// PIT timers (forwarded from the HAL)
    pub pit: hal::pit::UnclockedPIT,
    /// The USB logger
    pub log: log::Logging,
    /// DCDC converters
    pub dcdc: hal::dcdc::DCDC,
    /// PWM2 controller
    pub pwm2: hal::pwm::UnclockedController<hal::pwm::module::_2>,
    /// Teensy pin
    pub pins: Pins,
}

/// SYSTICK external clock frequency
///
/// See Section 12.3.2.1 of the reference manual. The note
/// explains that the 24MHz clock is divided down to 100KHz
/// before reaching SYSTICK.
const SYSTICK_EXT_FREQ: u32 = 100_000;

impl Peripherals {
    /// Instantiate the system peripherals. This may only be called once!
    pub fn take() -> Option<Self> {
        let p = hal::Peripherals::take()?;
        Some(Peripherals::new(p))
    }

    fn new(mut p: hal::Peripherals) -> Peripherals {
        p.systick.disable_counter();
        p.systick
            .set_clock_source(cortex_m::peripheral::syst::SystClkSource::External);
        p.systick.set_reload((SYSTICK_EXT_FREQ / 1000) - 1);
        p.systick.clear_current();
        p.systick.enable_counter();
        p.systick.enable_interrupt();
        Peripherals {
            led: {
                let pad = p.iomuxc.gpio_b0_03;
                hal::gpio::IO03::gpio2(pad).fast(&mut p.iomuxc.gpr).output()
            },
            ccm: p.ccm,
            pit: p.pit,
            log: log::Logging::new(),
            dcdc: p.dcdc,
            pwm2: p.pwm2,
            pins: Pins {
                p6: p.iomuxc.gpio_b0_10,
                p9: p.iomuxc.gpio_b0_11,
            },
        }
    }
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
    let start = systick::read();
    let target = start + millis;
    loop {
        let count = systick::read();
        if count >= target {
            return;
        }
    }
}

/// Scoping of data related to SYSTICK
mod systick {
    use imxrt1062_rt::exception;

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
}

/// TODO(mciantyre) define a better yield
#[no_mangle]
fn r#yield() {
    // 'yield' is a Rust keyword. But, it needs to be called 'yield' for the C USB stack
    cortex_m::asm::delay(1024);
}
