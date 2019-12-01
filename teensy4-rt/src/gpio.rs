//! General Purpose I/O (GPIO)
//!
//! Configures fast GPIOs for GPIO6, GPIO7, GPIO8, GPIO9. Provides
//! a function for enabling / disabling the LED.

use crate::iomuxc::*;
use core::ptr;

#[macro_export]
macro_rules! IOMUXC_PAD_DSE {
    ($n:expr) => {
        (($n & 0x07) << 3) as u32
    };
}

/// Initialize GPIOs
///
/// Use fast GPIOs for GPIO6 through GPIO9. Sets up the LED pin.
///
/// # Safety
///
/// Initialization writes to CPU registers, and it only needs to be
/// called once.
#[inline(always)]
pub unsafe fn init() {
    ptr::write_volatile(IOMUXC_GPR_GPR!(26), 0xFFFF_FFFFu32);
    ptr::write_volatile(IOMUXC_GPR_GPR!(27), 0xFFFF_FFFFu32);
    ptr::write_volatile(IOMUXC_GPR_GPR!(28), 0xFFFF_FFFFu32);
    ptr::write_volatile(IOMUXC_GPR_GPR!(29), 0xFFFF_FFFFu32);

    ptr::write_volatile(IOMUXC_SW_MUX_CTL_PAD_GPIO_B0_03 as *mut u32, 5);
    ptr::write_volatile(
        IOMUXC_SW_PAD_CTL_PAD_GPIO_B0_03 as *mut u32,
        IOMUXC_PAD_DSE!(7),
    );
    let gdir = ptr::read_volatile(GPIO7_GDIR);
    ptr::write_volatile(GPIO7_GDIR, gdir | (1 << 3));
}

const IMXRT_GPIO7: u32 = 0x4200_4000;
const GPIO7_GDIR: *mut u32 = (IMXRT_GPIO7 + 0x004) as *mut u32;
const GPIO7_DR_SET: *mut u32 = (IMXRT_GPIO7 + 0x084) as *mut u32;
const GPIO7_DR_CLEAR: *mut u32 = (IMXRT_GPIO7 + 0x088) as *mut u32;

#[inline(always)]
fn write_led(reg: *mut u32) {
    unsafe {
        ptr::write_volatile(reg, 1 << 3);
    }
}

/// Turns on the pin 13 LED on the Teensy
#[inline(always)]
pub fn enable_led() {
    write_led(GPIO7_DR_SET);
}

#[inline(always)]
pub fn disable_led() {
    write_led(GPIO7_DR_CLEAR);
}
