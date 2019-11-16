//! Floating-point unit (FPU) configuration

use core::ptr;

/// Initialize the floating point unit
///
/// # Safety
///
/// Initialization writes to CPU registers, and it only needs to be
/// called once.
#[inline(always)]
pub unsafe fn init() {
    const SCB_CPACR: *mut u32 = 0xE000_ED88 as *mut u32;
    const SCB_CPACR_FPU_ENABLE: u32 = 0b01_01 << 20;
    const SCB_CPACR_FPU_USER: u32 = 0b10_10 << 20;

    ptr::write_volatile(
        SCB_CPACR,
        *SCB_CPACR | SCB_CPACR_FPU_ENABLE | SCB_CPACR_FPU_USER,
    );
}
