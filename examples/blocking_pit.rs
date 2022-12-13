//! Blinks the board's LED using a periodic interrupt timer.
//!
//! It demonstrates how to use the imxrt-hal's timer adapters
//! for simple blocking delays. Since the board configures clock
//! frequencies, we use that frequency to describe our timers.

#![no_std]
#![no_main]

use teensy4_bsp as bsp;
use teensy4_panic as _;

use bsp::{board, hal::timer::Blocking};

#[bsp::rt::entry]
fn main() -> ! {
    let board::Resources {
        pit,
        pins,
        mut gpio2,
        ..
    } = board::t40(board::instances());
    let led = board::led(&mut gpio2, pins.p13);
    let mut delay = Blocking::<_, { board::PERCLK_FREQUENCY }>::from_pit_channel(pit.0);
    loop {
        delay.block_ms(500);
        led.toggle();
    }
}
