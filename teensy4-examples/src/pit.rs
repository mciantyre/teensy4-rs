//! Enables a PIT timer to test interrupts

#![no_std]
#![no_main]

use imxrt1060_pac as pac;
use pac::interrupt;
use teensy4_rt::{disable_led, enable_led, entry, interrupt};

#[interrupt]
fn PIT() {
    static mut ON: bool = false;
    if !*ON {
        enable_led();
    } else {
        disable_led();
    }
    *ON = !*ON;
    // TODO rearm
}

#[entry]
fn main() -> ! {
    disable_led();
    let periphs = pac::Peripherals::take().unwrap();
    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::interrupt::PIT);
    }
    loop {
        cortex_m::asm::wfi();
    }
}
