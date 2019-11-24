//! Rust entry point

#![no_std]

#[macro_use]
mod iomuxc;

mod fault;
mod fcb;
mod fpu;
mod nvic;
mod tcm;

pub use cortex_m_rt_macros::{entry, exception, interrupt};
pub use nvic::exception;

use cortex_m::register;

/// System entrypoint
///
/// # Safety
///
/// The function is unsafe since it directly modifies registers, and invokes
/// other functions that do the same.
#[link_section = ".boot.reset"]
#[no_mangle]
pub unsafe extern "C" fn _reset() -> ! {
    extern "C" {
        static mut __stext: u32;
        static mut __etext: u32;
        static __sitext: u32;

        static mut __sbss: u32;
        static mut __ebss: u32;

        static mut __sdata: u32;
        static mut __edata: u32;
        static __sidata: u32;

        static __estack: u32;

        fn cache_init();
    }
    tcm::init();
    r0::init_data(&mut __stext, &mut __etext, &__sitext);
    r0::init_data(&mut __sdata, &mut __edata, &__sidata);
    r0::zero_bss(&mut __sbss, &mut __ebss);

    // Reconfigure the stack pointers to give us some more room
    // after the TCM is configured. Note that this should be called
    // after the TCM is configured, and we've copied our code / data
    // into the specific regions. There's a good chance that these
    // functions aren't inlined, especially if we're using the cortex_m
    // crate with non-inlined asm.
    register::msp::write(&__estack as *const u32 as u32);
    register::psp::write(&__estack as *const u32 as u32);

    nvic::init();
    fpu::init();
    cache_init();

    extern "Rust" {
        fn main() -> !;
    }

    #[inline(never)]
    fn trampoline() -> ! {
        unsafe { main() };
    }

    trampoline();
}
