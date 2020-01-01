#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMP Control Register 0"]
    pub cr0: CR0,
    #[doc = "0x01 - CMP Control Register 1"]
    pub cr1: CR1,
    #[doc = "0x02 - CMP Filter Period Register"]
    pub fpr: FPR,
    #[doc = "0x03 - CMP Status and Control Register"]
    pub scr: SCR,
    #[doc = "0x04 - DAC Control Register"]
    pub daccr: DACCR,
    #[doc = "0x05 - MUX Control Register"]
    pub muxcr: MUXCR,
}
#[doc = "CMP Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr0](cr0) module"]
pub type CR0 = crate::Reg<u8, _CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR0;
#[doc = "`read()` method returns [cr0::R](cr0::R) reader structure"]
impl crate::Readable for CR0 {}
#[doc = "`write(|w| ..)` method takes [cr0::W](cr0::W) writer structure"]
impl crate::Writable for CR0 {}
#[doc = "CMP Control Register 0"]
pub mod cr0;
#[doc = "CMP Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](cr1) module"]
pub type CR1 = crate::Reg<u8, _CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1;
#[doc = "`read()` method returns [cr1::R](cr1::R) reader structure"]
impl crate::Readable for CR1 {}
#[doc = "`write(|w| ..)` method takes [cr1::W](cr1::W) writer structure"]
impl crate::Writable for CR1 {}
#[doc = "CMP Control Register 1"]
pub mod cr1;
#[doc = "CMP Filter Period Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpr](fpr) module"]
pub type FPR = crate::Reg<u8, _FPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPR;
#[doc = "`read()` method returns [fpr::R](fpr::R) reader structure"]
impl crate::Readable for FPR {}
#[doc = "`write(|w| ..)` method takes [fpr::W](fpr::W) writer structure"]
impl crate::Writable for FPR {}
#[doc = "CMP Filter Period Register"]
pub mod fpr;
#[doc = "CMP Status and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](scr) module"]
pub type SCR = crate::Reg<u8, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
#[doc = "`read()` method returns [scr::R](scr::R) reader structure"]
impl crate::Readable for SCR {}
#[doc = "`write(|w| ..)` method takes [scr::W](scr::W) writer structure"]
impl crate::Writable for SCR {}
#[doc = "CMP Status and Control Register"]
pub mod scr;
#[doc = "DAC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daccr](daccr) module"]
pub type DACCR = crate::Reg<u8, _DACCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACCR;
#[doc = "`read()` method returns [daccr::R](daccr::R) reader structure"]
impl crate::Readable for DACCR {}
#[doc = "`write(|w| ..)` method takes [daccr::W](daccr::W) writer structure"]
impl crate::Writable for DACCR {}
#[doc = "DAC Control Register"]
pub mod daccr;
#[doc = "MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [muxcr](muxcr) module"]
pub type MUXCR = crate::Reg<u8, _MUXCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MUXCR;
#[doc = "`read()` method returns [muxcr::R](muxcr::R) reader structure"]
impl crate::Readable for MUXCR {}
#[doc = "`write(|w| ..)` method takes [muxcr::W](muxcr::W) writer structure"]
impl crate::Writable for MUXCR {}
#[doc = "MUX Control Register"]
pub mod muxcr;
