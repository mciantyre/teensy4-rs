#![no_std]

use imxrt1060_hal as hal;

pub use hal::{interrupt, CCM, PIT};
pub type LED = hal::gpio::Pin<hal::gpio::GPIO7>;

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

    fn new(p: hal::Peripherals) -> Peripherals {
        Peripherals {
            led: {
                let pin = p.gpio2.io3.build((
                    &p.iomuxc.sw_mux_ctl_pad_gpio_b0_03,
                    &p.iomuxc.sw_pad_ctl_pad_gpio_b0_03,
                ));
                pin.fast_gpio7(&p.iomuxc_gpr.gpr27)
            },
            systick: p.systick,
            ccm: p.ccm,
            pit: p.pit,
        }
    }
}
