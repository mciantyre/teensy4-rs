#![no_std]

use imxrt1060_pac as pac;

pub struct GPIO;

pub struct Peripherals {
    pub gpio: GPIO,
}

impl Peripherals {
    pub fn take() -> Option<Self> {
        let cp = cortex_m::Peripherals::take()?;
        let p = pac::Peripherals::take()?;
        Some(Peripherals::new(p, cp))
    }

    fn new(p: pac::Peripherals, cp: pac::CorePeripherals) -> Self {
        Peripherals {
            gpio: GPIO,
        }
    }
}