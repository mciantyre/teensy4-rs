//! Sets duty cycles for PWM pins

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::rt;
use embedded_hal::PwmPin;
use teensy4_bsp as bsp;

fn percent(duty: u16) -> f32 {
    ((duty as f32) * 100.0f32) / (core::u16::MAX as f32)
}

#[rt::entry]
fn main() -> ! {
    let mut p = bsp::Peripherals::take().unwrap();
    p.ccm
        .pll1
        .set_arm_clock(bsp::hal::ccm::PLL1::ARM_HZ, &mut p.ccm.handle, &mut p.dcdc);
    p.log.init(Default::default());
    let mut pwm2 = p.pwm2.clock(&mut p.ccm.handle);
    let (mut pin_a, mut pin_b) = pwm2.outputs(p.pins.p6.alt2(), p.pins.p9.alt2()).split();

    let (mut duty1, mut duty2) = (core::u16::MAX / 4, core::u16::MAX / 2);
    loop {
        log::info!(
            "Setting duty cycles {} and {}...",
            percent(duty1),
            percent(duty2)
        );
        pin_a.enable();
        pin_b.enable();
        pin_a.set_duty(duty1);
        pin_b.set_duty(duty2);
        bsp::delay(200);

        log::info!("Disabling PWMs...");
        pin_a.disable();
        pin_b.disable();
        bsp::delay(500);

        log::info!("Swapping duty cycles...");
        core::mem::swap(&mut duty1, &mut duty2);
    }
}
