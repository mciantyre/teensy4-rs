#![no_std]

pub use imxrt1060_pac as pac;

pub mod ccm;
pub mod gpio;
pub mod iomuxc;
pub mod pit;

pub struct Peripherals {
    pub iomuxc: iomuxc::IOMUXC,
    pub systick: pac::SYST,
    pub ccm: ccm::CCM,
    pub pit: pit::PIT<pit::Unclocked>,
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
            ccm: ccm::CCM::new(p.CCM, p.CCM_ANALOG),
            pit: pit::PIT::new(p.PIT),
        }
    }
}
