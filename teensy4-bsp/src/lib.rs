#![no_std]

use imxrt1060_hal as hal;

pub type LED = hal::gpio::Pin<hal::gpio::GPIO7>;

pub struct Peripherals {
    pub led: LED,
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
        }
    }
}
