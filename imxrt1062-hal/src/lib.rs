//! NXP iMXRT1062 hardware abstraction layer
//!
//! An [`embedded-hal`](https://crates.io/crates/embedded-hal) implementation
//! targeting processors in NXP's IMXRT106x family.
//!
//! The HAL is a WIP. More documentation will become available once more capabilities
//! are exposed.
//!
//! In some cases, the HAL simply re-exports peripherals from the peripheral access
//! crates (PAC). If they are not re-exported, all PAC components are available
//! in the `pac` module.
//!
//! To see examples of the HAL, check out the `teensy4-bsp` and the `teensy4-examples` crates.
//! We will skip documentation example and tests, since we cannot yet test them as part
//! of the `cargo test` workflow...

#![no_std]

pub use imxrt1062_pac as pac;

pub mod ccm;
pub mod gpio;
pub mod i2c;
pub mod iomuxc;
pub mod pit;
pub mod pwm;
pub mod spi;
pub mod uart;

pub mod dcdc {
    use imxrt1062_pac as pac;
    pub struct DCDC(pub(crate) pac::DCDC);
    impl DCDC {
        pub fn raw(&mut self) -> &pac::DCDC {
            &self.0
        }
    }
}

pub struct Peripherals {
    pub iomuxc: iomuxc::IOMUXC,
    pub systick: pac::SYST,
    pub ccm: ccm::CCM,
    pub pit: pit::UnclockedPIT,
    pub dcdc: dcdc::DCDC,
    pub pwm1: pwm::UnclockedController<pwm::module::_1>,
    pub pwm2: pwm::UnclockedController<pwm::module::_2>,
    pub pwm3: pwm::UnclockedController<pwm::module::_3>,
    pub pwm4: pwm::UnclockedController<pwm::module::_4>,
    pub i2c: i2c::Unclocked,
    pub spi: spi::Unclocked,
    pub uart: uart::Unclocked,
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
            pit: pit::UnclockedPIT::new(p.PIT),
            dcdc: dcdc::DCDC(p.DCDC),
            pwm1: pwm::UnclockedController::new(),
            pwm2: pwm::UnclockedController::new(),
            pwm3: pwm::UnclockedController::new(),
            pwm4: pwm::UnclockedController::new(),
            i2c: i2c::Unclocked::new(),
            spi: spi::Unclocked::new(),
            uart: uart::Unclocked {
                uart1: p.LPUART1,
                uart2: p.LPUART2,
                uart3: p.LPUART3,
                uart4: p.LPUART4,
                uart5: p.LPUART5,
                uart6: p.LPUART6,
                uart7: p.LPUART7,
                uart8: p.LPUART8,
            },
        }
    }
}
