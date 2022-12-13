//! Blinks the board's LED using a general purpose timer.
//!
//! It demonstrates how to use the imxrt-hal's timer adapters
//! for simple blocking delays. Since the board configures clock
//! frequencies, we use that frequency to describe our timers.

#![no_std]
#![no_main]

use teensy4_bsp as bsp;
use teensy4_panic as _;

use bsp::{
    board,
    hal::{gpt, timer::Blocking},
};

// Given this GPT clock source...
const GPT_CLOCK_SOURCE: gpt::ClockSource = gpt::ClockSource::PeripheralClock;
// ...and this GPT-specific divider...
const GPT_DIVIDER: u32 = 8;
/// ...the GPT frequency is
const GPT_FREQUENCY: u32 = board::PERCLK_FREQUENCY / GPT_DIVIDER;

fn init_gpt<const N: u8>(gpt: &mut gpt::Gpt<N>) {
    gpt.set_clock_source(GPT_CLOCK_SOURCE);
    gpt.set_divider(GPT_DIVIDER);
}

#[bsp::rt::entry]
fn main() -> ! {
    let board::Resources {
        pins,
        mut gpio2,
        mut gpt1,
        mut gpt2,
        ..
    } = board::t40(board::instances());

    init_gpt(&mut gpt1);
    init_gpt(&mut gpt2);

    let led = board::led(&mut gpio2, pins.p13);
    let mut turn_on = Blocking::<_, { GPT_FREQUENCY }>::from_gpt(gpt1);
    let mut turn_off = Blocking::<_, { GPT_FREQUENCY }>::from_gpt(gpt2);

    loop {
        turn_on.block_ms(500);
        led.set();
        turn_off.block_ms(500);
        led.clear();
    }
}
