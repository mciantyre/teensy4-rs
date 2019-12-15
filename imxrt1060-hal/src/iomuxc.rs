//! IOMUX Controller

#![allow(non_camel_case_types)]


#[macro_use]
mod macros;

pub mod gpio;

// IOMUXC section of docs originally state that there are up to 8
// alternative modes. However, some have up to 10 (like GPIO_AD_B1_00)

/// Pad alternative 0 (type tag)
pub struct Alt0;
/// Pad alternative 1 (type tag)
pub struct Alt1;
/// Pad alternative 2 (type tag)
pub struct Alt2;
/// Pad alternative 3 (type tag)
pub struct Alt3;
/// Pad alternative 4 (type tag)
pub struct Alt4;
/// Pad alternative 5 (type tag)
pub struct Alt5;
/// Pad alternative 6 (type tag)
pub struct Alt6;
/// Pad alternative 7 (type tag)
pub struct Alt7;
/// Pad alternative 8 (type tag)
pub struct Alt8;
/// Pad alternative 9 (type tag)
pub struct Alt9;

pub struct IOMUXC {
    pub gpio_b0_03: gpio::GPIO_B0_03<Alt5>,
    pub gpr: GPR,
}

impl IOMUXC {
    pub(crate) fn new(iomuxc: crate::pac::IOMUXC) -> Self {
        Self {
            gpio_b0_03: gpio::GPIO_B0_03::new(&iomuxc),
            gpr: GPR(()),
        }
    }
}

pub struct GPR(());

impl GPR {
    pub(crate) fn gpr27(&mut self) -> &crate::pac::iomuxc_gpr::GPR27 {
        unsafe { &(*crate::pac::IOMUXC_GPR::ptr()).gpr27 }
    }
}
