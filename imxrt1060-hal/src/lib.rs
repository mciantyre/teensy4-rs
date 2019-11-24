#![no_std]

use imxrt1060_pac as pac;

pub mod gpio;

pub use pac::interrupt;
pub use pac::{CCM, PIT, SYST};

pub struct Peripherals {
    pub gpio2: gpio::GPIO2Pins,
    pub iomuxc: pac::IOMUXC,
    pub iomuxc_gpr: pac::IOMUXC_GPR,
    pub systick: pac::SYST,
    pub ccm: pac::CCM,
    pub pit: pac::PIT,
}

impl Peripherals {
    pub fn take() -> Option<Self> {
        let cp = cortex_m::Peripherals::take()?;
        let p = pac::Peripherals::take()?;
        Some(Peripherals::new(p, cp))
    }

    fn new(p: pac::Peripherals, cp: pac::CorePeripherals) -> Self {
        Peripherals {
            gpio2: gpio::GPIO2Pins::new(),
            iomuxc: p.IOMUXC,
            iomuxc_gpr: p.IOMUXC_GPR,
            systick: cp.SYST,
            ccm: p.CCM,
            pit: p.PIT,
        }
    }
}
