//! Runtime support
//!
//! The useful bits are written in assembly. See `start.s` for the
//! reset handler implementation. The reset handler eventually calls
//! `t4_init()`, which finishes setup, and calls the `cortex-m-rt`
//! `Reset()` handler.
//!
//! Code that's in this module is running before `.data` is initialized,
//! and before `.bss` is zeroed. This code should only touch ARM and
//! peripheral memory.

use core::arch::global_asm;
pub use cortex_m_rt::*;

global_asm!(include_str!("start.s"));

/// System entrypoint, invoked by reset handler
///
/// # Safety
///
/// The function is unsafe since it directly modifies registers, and invokes
/// other functions that do the same. It does not touch any memory that needs
/// to first be initialized by cortex-m-rt.
#[no_mangle]
unsafe extern "C" fn t4_init() {
    // Remain in 'run' when transitioning to low power mode.
    // Do not transition to wait mode.
    //
    // This addresses an edge case identified when reprogramming
    // the Teensy, and then immediately executing WFI.
    // See https://github.com/mciantyre/teensy4-rs/issues/76 for
    // more details.
    //
    // It's also useful for users who expect SYSTICK exceptions
    // to unblock WFI, which isn't the default behavior.
    //
    // Ideally, we could figure out an approach that keeps the processor
    // as close to the reset state as possible, while avoiding #76.
    // For now, we'll go with this, which simplifies some downstream
    // code.
    const CCM_CLPCR: *mut u32 = 0x400F_C054 as *mut _;
    CCM_CLPCR.write_volatile(CCM_CLPCR.read_volatile() & !0b11);

    extern "C" {
        fn Reset();
    }
    Reset(); // cortex-m-rt entrypoint
}

/// Returns the size of the heap, in bytes.
///
/// Use [`heap_start()`](crate::rt::heap_start) to access the start of the heap.
#[inline]
pub fn heap_len() -> usize {
    fn heap_end() -> *mut u32 {
        extern "C" {
            static mut __eheap: u32;
        }
        unsafe { &mut __eheap }
    }

    // Cannot use offset_from, since heap_end
    // and heap_start are not derived from the
    // same allocated object.
    let end = heap_end() as usize;
    let start = cortex_m_rt::heap_start() as usize;
    end - start
}

/// Returns the starting location for a DTCM "heap."
///
/// The recommended heap is in OCRAM2, and available
/// using [`heap_start()`](crate::rt::heap_start). But,
/// you may choose to place the heap in DTCM. This function
/// returns the starting address for that heap.
///
/// The DTCM heap is expected to grow up towards the stack,
/// and has no statically-known maximum size. The pointer
/// is guaranteed to be 4 byte aligned.
#[inline]
pub fn dtcm_heap_start() -> *mut u32 {
    extern "C" {
        static mut __sheap_dtcm: u32;
    }
    unsafe { &mut __sheap_dtcm }
}
