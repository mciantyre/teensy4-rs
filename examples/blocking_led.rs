//! This is the simplest example: it turns on the LED.

#![no_std]
#![no_main]

use teensy4_bsp as bsp;
use teensy4_panic as _;

use bsp::board;

#[bsp::rt::entry]
fn main() -> ! {
    let board::Resources {
        pins, mut gpio2, ..
    } = board::t40(board::instances());
    let led = board::led(&mut gpio2, pins.p13);
    loop {
        led.set();
    }
}
