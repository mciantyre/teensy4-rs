//! Demonstrates a simple panic handler.
//!
//! This example does not write a panic message, nor does it set up
//! logging. You should only observe an blinking LED in an SOS pattern.

#![no_std]
#![no_main]

use teensy4_bsp as bsp;
use teensy4_panic as _;

#[bsp::rt::entry]
fn main() -> ! {
    panic!();
}
