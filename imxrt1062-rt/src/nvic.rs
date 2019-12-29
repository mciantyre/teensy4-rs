//! Nested Vector Interrupt Controller (NVIC) configuration
use core::ptr;

const NUM_EXCEPTIONS: usize = 14;

/// The number of chip-specific interrupt count.
///
/// See page 44 of the IMXRT1062 reference manual (skipping the two 'reserved' at the end,
/// since it seems that the SVD doesn't document these, or svd2rust removes them.)
const NUM_IMXRT106X_INTERRUPTS: usize = 158;

#[doc(hidden)]
#[no_mangle]
unsafe extern "C" fn DefaultHandler_() -> ! {
    loop {
        core::sync::atomic::spin_loop_hint();
    }
}

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

    extern "C" {
        static __svectors: u32;
    }
    const SCB_VTOR: *mut u32 = 0xE000_ED08 as *mut u32;
    ptr::write_volatile(SCB_VTOR, &__svectors as *const u32 as u32);
}

#[doc(hidden)]
#[derive(Copy, Clone)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}

extern "C" {
    fn NonMaskableInt();
    fn HardFaultTrampoline();
    fn MemoryManagement();
    fn BusFault();
    fn UsageFault();
    fn SVCall();
    fn DebugMonitor();
    fn PendSV();
    fn SysTick();
}

#[doc(hidden)]
#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static __EXCEPTIONS: [Vector; NUM_EXCEPTIONS] = [
    // Exception 2: Non Maskable Interrupt.
    Vector {
        _handler: NonMaskableInt,
    },
    // Exception 3: Hard Fault Interrupt.
    Vector {
        _handler: HardFaultTrampoline,
    },
    // Exception 4: Memory Management Interrupt.
    Vector {
        _handler: MemoryManagement,
    },
    // Exception 5: Bus Fault Interrupt
    Vector { _handler: BusFault },
    // Exception 6: Usage Fault Interrupt.
    Vector {
        _handler: UsageFault,
    },
    // Exception 7: Secure Fault Interrupt [only on Armv8-M].
    Vector { _reserved: 0 },
    // 8-10: _Reserved
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    // Exception 11: SV Call Interrupt.
    Vector { _handler: SVCall },
    // Exception 12: Debug Monitor Interrupt.
    Vector {
        _handler: DebugMonitor,
    },
    // 13: Reserved
    Vector { _reserved: 0 },
    // Exception 14: Pend SV Interrupt.
    Vector { _handler: PendSV },
    // Exception 15: System Tick Interrupt.
    Vector { _handler: SysTick },
];

/* Exceptions */
#[doc(hidden)]
pub enum Exception {
    NonMaskableInt,
    // Not overridable
    // HardFault,
    MemoryManagement,
    BusFault,
    UsageFault,
    SVCall,
    DebugMonitor,
    PendSV,
    SysTick,
}

pub use self::Exception as exception;
