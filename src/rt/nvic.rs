//! Nested Vector Interrupt Controller (NVIC) configuration

use core::ptr;

/// A pointer to a function
type Vector = u32;

/// The number of Cortex M exceptions
const NUM_EXCEPTIONS: usize = 14;

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

/// +2 for stack pointer, reset handler
const NUM_VECTORS: usize = 2 + NUM_EXCEPTIONS + NUM_IMXRT106X_INTERRUPTS;

/// A vector table that will be allocated in DTCM
#[repr(align(1024))]
struct VectorTable([Vector; NUM_VECTORS]);

#[link_section = ".vector_table.ram"] // In data section, but won't be initialized
static mut VECTOR_TABLE: VectorTable = VectorTable([0; NUM_VECTORS]);

/// Initialize the NVIC, registering default handlers for
/// all interrupts.
///
/// # Safety
///
/// Writes to registers and should only be called once.
#[inline(always)]
pub unsafe fn init() {
    extern "C" {
        static __svectors: u32;
    }

    let input_table = core::slice::from_raw_parts(&__svectors as *const u32, NUM_VECTORS);

    VECTOR_TABLE
        .0
        .iter_mut()
        .zip(input_table.iter())
        .for_each(|(dst, src)| {
            ptr::write_volatile(dst, *src);
        });

    for irq in 0..NUM_IMXRT106X_INTERRUPTS {
        set_priority(irq, 128);
    }

    const SCB_VTOR: *mut u32 = 0xE000_ED08 as *mut u32;
    ptr::write_volatile(SCB_VTOR, VECTOR_TABLE.0.as_ptr() as u32);
}
