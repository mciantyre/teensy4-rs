//! Configure pin 6 with a pull-down resistor, and sample it to control
//! whether the LED is flashing or not.

#![no_std]
#![no_main]

use teensy4_panic as _;

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use imxrt_hal::{
    gpio::{Input, GPIO},
    iomuxc::{configure, gpio::Pin, Config, Hysteresis, PullKeeper},
};
use teensy4_bsp::{configure_led, pins::t40, Peripherals};

fn configure_switch<P: Pin>(mut switch_pin: P) -> GPIO<P, Input> {
    const LOW_SWITCH_CONFIG: Config = Config::zero()
        .set_hysteresis(Hysteresis::Enabled)
        .set_pull_keeper(Some(PullKeeper::Pulldown100k));
    configure(&mut switch_pin, LOW_SWITCH_CONFIG);
    GPIO::new(switch_pin)
}

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let pins = t40::from_pads(p.iomuxc);
    let mut led = configure_led(pins.p13);
    // Unlike configure_led, configure_switch will accept any pin (except for
    //  pins.p13, which has already been used)
    let switch = configure_switch(pins.p6);

    loop {
        if switch.is_set() {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }
    }
}
