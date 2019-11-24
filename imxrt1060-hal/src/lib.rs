#![no_std]

use imxrt1060_pac as pac;

pub mod gpio;

pub struct Peripherals {
    pub gpio2: gpio::GPIO2,
    pub iomuxc: pac::IOMUXC,
}

impl Peripherals {
    pub fn take() -> Option<Self> {
        let cp = cortex_m::Peripherals::take()?;
        let p = pac::Peripherals::take()?;
        Some(Peripherals::new(p, cp))
    }

    fn new(p: pac::Peripherals, _cp: pac::CorePeripherals) -> Self {
        Peripherals {
            gpio2: gpio::GPIO2::new(),
            iomuxc: p.IOMUXC,
        }
    }
}
