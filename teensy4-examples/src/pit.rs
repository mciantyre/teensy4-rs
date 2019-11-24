//! Enables a PIT timer to test interrupts

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::interrupt;
use teensy4_bsp as bsp;
use teensy4_rt::{entry, interrupt};

static mut LED: Option<bsp::LED> = None;

#[interrupt]
fn PIT() {
    unsafe { LED.as_mut().unwrap().toggle() };
    // Rearm the timer
    unsafe {
        (*bsp::PIT::ptr()).timer[1]
            .tflg
            .write(|reg| reg.tif().tif_1());
    }
}

#[inline(always)]
fn configure_clocks(ccm: &mut bsp::CCM) {
    ccm.cscmr1.modify(|_, reg| {
        reg.perclk_podf()
            .divide_1()
            .perclk_clk_sel()
            .perclk_clk_sel_1()
    });
    ccm.ccgr1.modify(|_, reg| {
        unsafe {
            reg.cg6().bits(0x3);
        } // Enable PIT timers
        reg
    })
}

const fn ms_to_ticks(ms: u32) -> u32 {
    const CLOCK_FREQUENCY_MHZ: u32 = 24;
    const CLOCK_PERIOD_NS: u32 = 1_000_000_000 / (CLOCK_FREQUENCY_MHZ * 1_000_000);
    ((ms * 1_000_000) / CLOCK_PERIOD_NS) - 1
}
const BLINK_PERIOD_TICKS: u32 = ms_to_ticks(500);

#[inline(always)]
fn enable_pit(pit: &mut bsp::PIT) {
    pit.mcr.write(|reg| {
        reg.mdis().mdis_0() // Enable PIT
    });
    pit.timer[1].ldval.write(|reg| {
        unsafe {
            reg.tsv().bits(BLINK_PERIOD_TICKS);
        }
        reg
    });
    pit.timer[1].tctrl.write(|reg| reg.tie().tie_1());
    pit.timer[1].tctrl.modify(|_, reg| reg.ten().ten_1());
}

#[entry]
fn main() -> ! {
    let mut periphs = bsp::Peripherals::take().unwrap();
    unsafe {
        LED = Some(periphs.led);
        cortex_m::peripheral::NVIC::unmask(bsp::interrupt::PIT);
    }
    configure_clocks(&mut periphs.ccm);
    enable_pit(&mut periphs.pit);
    loop {
        cortex_m::asm::wfi();
    }
}
