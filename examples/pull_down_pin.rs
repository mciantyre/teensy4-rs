//! Configure pin 6 with a pull-down resistor, and sample it to control
//! whether the LED is flashing or not.

#![no_std]
#![no_main]

use teensy4_panic as _;

use cortex_m_rt::entry;
use imxrt_hal::gpio::GPIO;
use imxrt_hal::iomuxc;
use imxrt_hal::iomuxc::{Config, Hysteresis, PullKeep, PullKeepSelect, PullUpDown};
use teensy4_bsp::{configure_led, t40, Peripherals, SysTick};

const LED_PERIOD_MS: u32 = 500;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let mut systick = SysTick::new(cortex_m::Peripherals::take().unwrap().SYST);
    let pins = t40::into_pins(p.iomuxc);
    let mut led = configure_led(pins.p13);

    let mut switch_pin = pins.p6;

    let cfg = Config::zero()
        .set_hysteresis(Hysteresis::Enabled)
        .set_pull_keep(PullKeep::Enabled)
        .set_pull_keep_select(PullKeepSelect::Pull)
        .set_pullupdown(PullUpDown::Pulldown100k);
    iomuxc::configure(&mut switch_pin, cfg);

    let switch_gpio = GPIO::new(switch_pin);

    loop {
        if switch_gpio.is_set() {
            led.toggle()
        }
        systick.delay(LED_PERIOD_MS);
    }
}
