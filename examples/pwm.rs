//! The example shows how to allocate PWM controllers and pins,
//! set duty cycles, and enable / disable PWM pins. We use
//! pin 6 and pin 9 of the Teensy board.
//!
//! Success criteria: pin 6 starts at 25% duty cycle, and pin 9
//! starts at 50% duty cycle. They stay that way for 200ms. Pin 9
//! turns off, and pin 6 stays on for another 200ms. Both pins are
//! then disabled for 400ms. Switch the duty cycles between the two
//! pins, and re-run the loop.
//!
//! Success criteria: the switching period is 1ms (1KHz).

#![no_std]
#![no_main]

mod systick;
mod usb_io;

use teensy4_panic as _;

use bsp::hal::pwm::Channel;
use cortex_m_rt as rt;
use embedded_hal::Pwm;
use teensy4_bsp as bsp;

/// Helper function to represent a duty cycle as a percent
fn percent(duty: u16) -> f32 {
    ((duty as f32) * 100.0f32) / (core::u16::MAX as f32)
}

#[rt::entry]
fn main() -> ! {
    // Prepare all the BSP peripherals
    let mut p = bsp::Peripherals::take().unwrap();
    let mut systick = systick::new(cortex_m::Peripherals::take().unwrap().SYST);
    let pins = bsp::pins::t40::from_pads(p.iomuxc);
    usb_io::init().unwrap();
    // Delay is only to let a user set-up their USB serial connection...
    systick.delay_ms(5000);
    // Set the core and IPG clock. The IPG clock frequency drives the PWM (sub)modules
    let (_, ipg_hz) =
        p.ccm
            .pll1
            .set_arm_clock(bsp::hal::ccm::PLL1::ARM_HZ, &mut p.ccm.handle, &mut p.dcdc);
    systick.delay_ms(100);
    // Enable the clocks for the PWM2 module
    let mut pwm2 = p.pwm2.clock(&mut p.ccm.handle);
    // Get the outputs from the PWM2 module, submodule 2.
    // Set a 1KHz switching frequency, using a prescalar of 32.
    let mut sm2 = pwm2
        .sm2
        .outputs(
            &mut pwm2.handle,
            pins.p6,
            pins.p9,
            bsp::hal::pwm::Timing {
                clock_select: bsp::hal::ccm::pwm::ClockSelect::IPG(ipg_hz),
                prescalar: bsp::hal::ccm::pwm::Prescalar::PRSC_5,
                switching_period: core::time::Duration::from_micros(1000),
            },
        )
        .unwrap();

    // Two different duty cycles that will be swapped to show
    // different duty cycles on the same PWM pins
    let (mut duty1, mut duty2) = (core::u16::MAX / 4, core::u16::MAX / 2);
    let mut ctrl = sm2.control(&mut pwm2.handle);
    loop {
        log::info!(
            "Setting duty cycles {} and {}...",
            percent(duty1),
            percent(duty2)
        );
        ctrl.enable(Channel::A);
        ctrl.enable(Channel::B);
        ctrl.set_duty(Channel::A, duty1);
        ctrl.set_duty(Channel::B, duty2);
        systick.delay_ms(200);

        log::info!("Disabling 'B' PWM...");
        ctrl.disable(Channel::B);
        systick.delay_ms(200);

        log::info!("Disabling 'A' PWM...");
        ctrl.disable(Channel::A);
        systick.delay_ms(400);

        core::mem::swap(&mut duty1, &mut duty2);
    }
}
