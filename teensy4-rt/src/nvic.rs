//! Nested Vector Interrupt Controller (NVIC) configuration
use crate::{disable_led, enable_led};
use core::ptr;

use cortex_m::asm::delay;

/// The number of chip-specific interrupt count.
///
/// See page 44 of the IMXRT1060 reference manual (skipping the two 'reserved' at the end,
/// since it seems that the SVD doesn't document these, or svd2rust removes them.)
const NUM_IMXRT106X_INTERRUPTS: usize = 158;

#[inline(always)]
fn long_sleep() {
    delay(300_000_000);
}

#[inline(always)]
fn short_sleep() {
    delay(100_000_000);
}

#[inline(never)]
fn s() {
    enable_led();
    short_sleep();
    disable_led();
    short_sleep();
    enable_led();
    short_sleep();
    disable_led();
    short_sleep();
    enable_led();
    short_sleep();
    disable_led();
    short_sleep();
}

#[inline(never)]
fn o() {
    enable_led();
    long_sleep();
    disable_led();
    short_sleep();
    enable_led();
    long_sleep();
    disable_led();
    short_sleep();
    enable_led();
    long_sleep();
    disable_led();
    short_sleep();
}

#[inline(always)]
fn blink_irq_number(mut irq: u8) {
    while (irq / 100) > 0 {
        enable_led();
        long_sleep();
        long_sleep();
        long_sleep();
        disable_led();
        short_sleep();
        irq -= 100;
    }
    long_sleep();
    while (irq / 10) > 0 {
        enable_led();
        long_sleep();
        disable_led();
        short_sleep();
        irq -= 10;
    }
    long_sleep();
    const BREAK_COUNT: u32 = 5;
    let mut break_count = BREAK_COUNT;
    while irq > 0 {
        enable_led();
        short_sleep();
        disable_led();
        short_sleep();
        irq -= 1;
        break_count -= 1;
        if 0 == break_count {
            short_sleep();
            break_count = BREAK_COUNT;
        }
    }
}

/// A default interrupt handler that emits SOS on the LED.
#[doc(hidden)]
#[no_mangle]
unsafe extern "C" fn DefaultHandler_() -> ! {
    loop {
        s();
        o();
        s();

        long_sleep();
        long_sleep();
        long_sleep();

        // Blink the active exception number
        const SCB_ICSR: *const u32 = 0xE000_ED04 as *const u32;
        let icsr = ptr::read_volatile(SCB_ICSR);
        blink_irq_number((icsr & 0xFF) as u8);

        long_sleep();
        long_sleep();
        long_sleep();
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
    for irq in (2 + 14)..NUM_IMXRT106X_INTERRUPTS {
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
pub static __EXCEPTIONS: [Vector; 14] = [
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
