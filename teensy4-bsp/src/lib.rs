#![no_std]

use imxrt1060_hal as hal;

pub use hal::{interrupt, CCM, PIT};
pub type LED = hal::gpio::IO03<hal::gpio::GPIO7, hal::gpio::Output>;

pub struct Peripherals {
    pub led: LED,
    pub systick: hal::SYST,
    pub ccm: hal::CCM,
    pub pit: hal::PIT,
}

impl Peripherals {
    pub fn take() -> Option<Self> {
        let p = hal::Peripherals::take()?;
        Some(Peripherals::new(p))
    }

    fn new(mut p: hal::Peripherals) -> Peripherals {
        Peripherals {
            led: {
                let pad = p.iomuxc.gpio_b0_03;
                hal::gpio::IO03::gpio2(pad).fast(&mut p.iomuxc.gpr).output()
            },
            systick: p.systick,
            ccm: p.ccm,
            pit: p.pit,
        }
    }
}
