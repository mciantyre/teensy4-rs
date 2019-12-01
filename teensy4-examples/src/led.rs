//! The example simply enables the LED

#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m::asm::wfi;
use teensy4_rt::{enable_led, entry};

#[entry]
fn main() -> ! {
    enable_led();
    loop {
        wfi();
    }
}
