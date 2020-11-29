//! Nested Vector Interrupt Controller (NVIC) configuration

use core::ptr;

/// The number of chip-specific interrupt count.
///
/// See page 44 of the IMXRT1062 reference manual (skipping the two 'reserved' at the end,
/// since it seems that the SVD doesn't document these, or svd2rust removes them.)
const NUM_IMXRT106X_INTERRUPTS: usize = 158;

#[inline(always)]
unsafe fn set_priority(irqn: usize, priority: u8) {
    const INTERRUPT_PRIORITY_BASE: *mut u8 = 0xE000_E400 as *mut u8;
    ptr::write_volatile(INTERRUPT_PRIORITY_BASE.add(irqn), priority);
}

/// Initialize the NVIC, registering default handlers for
/// all interrupts.
///
/// # Safety
///
/// Writes to registers and should only be called once.
#[inline(always)]
pub unsafe fn init() {
    for irq in 0..NUM_IMXRT106X_INTERRUPTS {
        set_priority(irq, 128);
    }
}
