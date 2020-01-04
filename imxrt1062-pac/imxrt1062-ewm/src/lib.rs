#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x01 - Service Register"]
    pub serv: SERV,
    #[doc = "0x02 - Compare Low Register"]
    pub cmpl: CMPL,
    #[doc = "0x03 - Compare High Register"]
    pub cmph: CMPH,
    #[doc = "0x04 - Clock Control Register"]
    pub clkctrl: CLKCTRL,
    #[doc = "0x05 - Clock Prescaler Register"]
    pub clkprescaler: CLKPRESCALER,
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u8, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Service Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [serv](serv) module"]
pub type SERV = crate::Reg<u8, _SERV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SERV;
#[doc = "`read()` method returns [serv::R](serv::R) reader structure"]
impl crate::Readable for SERV {}
#[doc = "`write(|w| ..)` method takes [serv::W](serv::W) writer structure"]
impl crate::Writable for SERV {}
#[doc = "Service Register"]
pub mod serv;
#[doc = "Compare Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpl](cmpl) module"]
pub type CMPL = crate::Reg<u8, _CMPL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPL;
#[doc = "`read()` method returns [cmpl::R](cmpl::R) reader structure"]
impl crate::Readable for CMPL {}
#[doc = "`write(|w| ..)` method takes [cmpl::W](cmpl::W) writer structure"]
impl crate::Writable for CMPL {}
#[doc = "Compare Low Register"]
pub mod cmpl;
#[doc = "Compare High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmph](cmph) module"]
pub type CMPH = crate::Reg<u8, _CMPH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPH;
#[doc = "`read()` method returns [cmph::R](cmph::R) reader structure"]
impl crate::Readable for CMPH {}
#[doc = "`write(|w| ..)` method takes [cmph::W](cmph::W) writer structure"]
impl crate::Writable for CMPH {}
#[doc = "Compare High Register"]
pub mod cmph;
#[doc = "Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkctrl](clkctrl) module"]
pub type CLKCTRL = crate::Reg<u8, _CLKCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKCTRL;
#[doc = "`read()` method returns [clkctrl::R](clkctrl::R) reader structure"]
impl crate::Readable for CLKCTRL {}
#[doc = "`write(|w| ..)` method takes [clkctrl::W](clkctrl::W) writer structure"]
impl crate::Writable for CLKCTRL {}
#[doc = "Clock Control Register"]
pub mod clkctrl;
#[doc = "Clock Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkprescaler](clkprescaler) module"]
pub type CLKPRESCALER = crate::Reg<u8, _CLKPRESCALER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKPRESCALER;
#[doc = "`read()` method returns [clkprescaler::R](clkprescaler::R) reader structure"]
impl crate::Readable for CLKPRESCALER {}
#[doc = "`write(|w| ..)` method takes [clkprescaler::W](clkprescaler::W) writer structure"]
impl crate::Writable for CLKPRESCALER {}
#[doc = "Clock Prescaler Register"]
pub mod clkprescaler;
