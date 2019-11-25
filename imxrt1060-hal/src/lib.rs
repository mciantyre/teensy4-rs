#![no_std]

use imxrt1060_pac as pac;

pub mod gpio;
pub mod iomuxc;

pub use pac::interrupt;
pub use pac::{CCM, PIT, SYST};

pub struct Peripherals {
    pub iomuxc: iomuxc::IOMUXC,
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
            iomuxc: iomuxc::IOMUXC::new(p.IOMUXC),
            systick: cp.SYST,
            ccm: p.CCM,
            pit: p.PIT,
        }
    }
}
