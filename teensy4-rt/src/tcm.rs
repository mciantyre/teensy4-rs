//! Tightly-Coupled Memory (TCM)
//!
//! The module is responsible for setting-up the instruction
//! and data TCM regions (ITCM and DTCM, respectively).

use core::ptr;

/// Initializes the TCM regions given the size of the ITCM text
/// region. Returns the number of RAM banks used for the ITCM
/// memory region.
///
/// The function will ensure that the ITCM region has enough 32KiB
/// RAM banks specified for ITCM, then will allocate the remaining
/// RAM banks to DTCM.
///
/// See Chapter 30 of the IMXRT1060 Reference manual, as well as
/// section 10.4.18 for the bit patterns necessary to write to GPR17.
///
/// The function is put into its own linker section so that we can know
/// that it's not placed in ITCM before ITCM is configured (since this
/// has to set up ITCM...).
///
/// # Safety
///
/// Writes to system memory and should only be called once. It should be
/// the first thing called before copying text / data.
#[link_section = ".boot.tcm"]
pub unsafe fn init() {
    extern "C" {
        static __flexram_bank_config: u32;
    }

    ptr::write_volatile(
        IOMUXC_GPR_GPR!(17),
        &__flexram_bank_config as *const u32 as u32,
    );
    // Enable both ITCM and DTCM, and use the bank configuration settings
    ptr::write_volatile(IOMUXC_GPR_GPR!(16), 0x0000_0007u32);
    // ITCM and DTCM are both 512KiB
    ptr::write_volatile(IOMUXC_GPR_GPR!(14), 0x00AA_0000u32);
}
