//! Rust entry point

#![no_std]

mod cache;
mod fault;
mod fpu;
mod nvic;

pub use cortex_m_rt_macros::{entry, exception, interrupt};
pub use nvic::exception;

/// System entrypoint
///
/// # Safety
///
/// The function is unsafe since it directly modifies registers, and invokes
/// other functions that do the same.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    nvic::init();
    fpu::init();
    cache::init();

    extern "Rust" {
        fn main() -> !;
    }

    #[inline(never)]
    fn trampoline() -> ! {
        unsafe { main() };
    }

    trampoline();
}
