//! Sets duty cycles for PWM pins

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::rt;
use embedded_hal::digital::v2::ToggleableOutputPin;
use embedded_hal::PwmPin;
use teensy4_bsp as bsp;

#[rt::entry]
fn main() -> ! {
    let mut p = bsp::Peripherals::take().unwrap();
    p.ccm
        .pll1
        .set_arm_clock(bsp::hal::ccm::PLL1::ARM_HZ, &mut p.ccm.handle, &mut p.dcdc);
    let mut pwm2 = p.pwm2.clock(&mut p.ccm.handle);
    let (mut pin_a, mut pin_b) = pwm2.outputs(p.pins.p6.alt2(), p.pins.p9.alt2()).split();
    pin_a.enable();
    pin_b.enable();
    pin_a.set_duty(core::u16::MAX / 4);
    pin_b.set_duty(core::u16::MAX / 2);
    let mut led = p.led;
    loop {
        bsp::delay(100);
        led.toggle().unwrap();
    }
}
