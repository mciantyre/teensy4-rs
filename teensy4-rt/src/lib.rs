//! Rust entry point

#![no_std]

#[macro_use]
mod iomuxc;

mod fault;
mod fcb;
mod fpu;
mod gpio;
mod nvic;

pub use cortex_m_rt_macros::{entry, exception, interrupt};
pub use gpio::{disable_led, enable_led};
pub use nvic::exception;

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
        fn tcm_init();
        fn init_data(start: *mut u32, end: *mut u32, input: *const u32);
        fn zero_bss(start: *mut u32, end: *mut u32);
    }
    tcm_init();
    init_data(&mut __stext, &mut __etext, &__sitext);
    init_data(&mut __sdata, &mut __edata, &__sidata);
    zero_bss(&mut __sbss, &mut __ebss);

    nvic::init();
    fpu::init();
    gpio::init();
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
