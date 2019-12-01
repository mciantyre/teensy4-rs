//! Enables a PIT timer to test interrupts

#![no_std]
#![no_main]

extern crate panic_halt;

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
    // Rearm the timer
    unsafe {
        let pit = &*pac::PIT::ptr();
        pit.timer[1].tflg.write(|reg| reg.tif().tif_1());

        while !pit.timer[1].tflg.read().tif().is_tif_1() {
            core::sync::atomic::spin_loop_hint();
        }
    }
}

#[inline(always)]
fn configure_clocks(ccm: &mut pac::CCM) {
    ccm.cscmr1.modify(|_, reg: &mut pac::ccm::cscmr1::W| {
        reg.perclk_podf()
            .divide_1()
            .perclk_clk_sel()
            .perclk_clk_sel_1()
    });
    ccm.ccgr1.modify(|_, reg: &mut pac::ccm::ccgr1::W| {
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
fn enable_pit(pit: &mut pac::PIT) {
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
    disable_led();
    let mut periphs = pac::Peripherals::take().unwrap();
    unsafe {
        cortex_m::interrupt::enable();
        cortex_m::peripheral::NVIC::unmask(pac::interrupt::PIT);
    }
    configure_clocks(&mut periphs.CCM);
    enable_pit(&mut periphs.PIT);
    loop {
        cortex_m::asm::wfi();
    }
}
