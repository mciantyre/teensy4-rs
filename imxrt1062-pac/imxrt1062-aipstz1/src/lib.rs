#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Priviledge Registers"]
    pub mpr: MPR,
    _reserved1: [u8; 60usize],
    #[doc = "0x40 - Off-Platform Peripheral Access Control Registers"]
    pub opacr: OPACR,
    #[doc = "0x44 - Off-Platform Peripheral Access Control Registers"]
    pub opacr1: OPACR1,
    #[doc = "0x48 - Off-Platform Peripheral Access Control Registers"]
    pub opacr2: OPACR2,
    #[doc = "0x4c - Off-Platform Peripheral Access Control Registers"]
    pub opacr3: OPACR3,
    #[doc = "0x50 - Off-Platform Peripheral Access Control Registers"]
    pub opacr4: OPACR4,
}
#[doc = "Master Priviledge Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpr](mpr) module"]
pub type MPR = crate::Reg<u32, _MPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPR;
#[doc = "`read()` method returns [mpr::R](mpr::R) reader structure"]
impl crate::Readable for MPR {}
#[doc = "`write(|w| ..)` method takes [mpr::W](mpr::W) writer structure"]
impl crate::Writable for MPR {}
#[doc = "Master Priviledge Registers"]
pub mod mpr;
#[doc = "Off-Platform Peripheral Access Control Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacr](opacr) module"]
pub type OPACR = crate::Reg<u32, _OPACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPACR;
#[doc = "`read()` method returns [opacr::R](opacr::R) reader structure"]
impl crate::Readable for OPACR {}
#[doc = "`write(|w| ..)` method takes [opacr::W](opacr::W) writer structure"]
impl crate::Writable for OPACR {}
#[doc = "Off-Platform Peripheral Access Control Registers"]
pub mod opacr;
#[doc = "Off-Platform Peripheral Access Control Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacr1](opacr1) module"]
pub type OPACR1 = crate::Reg<u32, _OPACR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPACR1;
#[doc = "`read()` method returns [opacr1::R](opacr1::R) reader structure"]
impl crate::Readable for OPACR1 {}
#[doc = "`write(|w| ..)` method takes [opacr1::W](opacr1::W) writer structure"]
impl crate::Writable for OPACR1 {}
#[doc = "Off-Platform Peripheral Access Control Registers"]
pub mod opacr1;
#[doc = "Off-Platform Peripheral Access Control Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacr2](opacr2) module"]
pub type OPACR2 = crate::Reg<u32, _OPACR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPACR2;
#[doc = "`read()` method returns [opacr2::R](opacr2::R) reader structure"]
impl crate::Readable for OPACR2 {}
#[doc = "`write(|w| ..)` method takes [opacr2::W](opacr2::W) writer structure"]
impl crate::Writable for OPACR2 {}
#[doc = "Off-Platform Peripheral Access Control Registers"]
pub mod opacr2;
#[doc = "Off-Platform Peripheral Access Control Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacr3](opacr3) module"]
pub type OPACR3 = crate::Reg<u32, _OPACR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPACR3;
#[doc = "`read()` method returns [opacr3::R](opacr3::R) reader structure"]
impl crate::Readable for OPACR3 {}
#[doc = "`write(|w| ..)` method takes [opacr3::W](opacr3::W) writer structure"]
impl crate::Writable for OPACR3 {}
#[doc = "Off-Platform Peripheral Access Control Registers"]
pub mod opacr3;
#[doc = "Off-Platform Peripheral Access Control Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacr4](opacr4) module"]
pub type OPACR4 = crate::Reg<u32, _OPACR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPACR4;
#[doc = "`read()` method returns [opacr4::R](opacr4::R) reader structure"]
impl crate::Readable for OPACR4 {}
#[doc = "`write(|w| ..)` method takes [opacr4::W](opacr4::W) writer structure"]
impl crate::Writable for OPACR4 {}
#[doc = "Off-Platform Peripheral Access Control Registers"]
pub mod opacr4;
