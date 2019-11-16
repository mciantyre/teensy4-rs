#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]

include!("../../generic.rs");

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO data register"]
    pub dr: DR,
    #[doc = "0x04 - GPIO direction register"]
    pub gdir: GDIR,
    #[doc = "0x08 - GPIO pad status register"]
    pub psr: PSR,
    #[doc = "0x0c - GPIO interrupt configuration register1"]
    pub icr1: ICR1,
    #[doc = "0x10 - GPIO interrupt configuration register2"]
    pub icr2: ICR2,
    #[doc = "0x14 - GPIO interrupt mask register"]
    pub imr: IMR,
    #[doc = "0x18 - GPIO interrupt status register"]
    pub isr: ISR,
    #[doc = "0x1c - GPIO edge select register"]
    pub edge_sel: EDGE_SEL,
    _reserved8: [u8; 100usize],
    #[doc = "0x84 - GPIO data register SET"]
    pub dr_set: DR_SET,
    #[doc = "0x88 - GPIO data register CLEAR"]
    pub dr_clear: DR_CLEAR,
    #[doc = "0x8c - GPIO data register TOGGLE"]
    pub dr_toggle: DR_TOGGLE,
}
#[doc = "GPIO data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "`write(|w| ..)` method takes [dr::W](dr::W) writer structure"]
impl crate::Writable for DR {}
#[doc = "GPIO data register"]
pub mod dr;
#[doc = "GPIO direction register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gdir](gdir) module"]
pub type GDIR = crate::Reg<u32, _GDIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GDIR;
#[doc = "`read()` method returns [gdir::R](gdir::R) reader structure"]
impl crate::Readable for GDIR {}
#[doc = "`write(|w| ..)` method takes [gdir::W](gdir::W) writer structure"]
impl crate::Writable for GDIR {}
#[doc = "GPIO direction register"]
pub mod gdir;
#[doc = "GPIO pad status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [psr](psr) module"]
pub type PSR = crate::Reg<u32, _PSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSR;
#[doc = "`read()` method returns [psr::R](psr::R) reader structure"]
impl crate::Readable for PSR {}
#[doc = "GPIO pad status register"]
pub mod psr;
#[doc = "GPIO interrupt configuration register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [icr1](icr1) module"]
pub type ICR1 = crate::Reg<u32, _ICR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR1;
#[doc = "`read()` method returns [icr1::R](icr1::R) reader structure"]
impl crate::Readable for ICR1 {}
#[doc = "`write(|w| ..)` method takes [icr1::W](icr1::W) writer structure"]
impl crate::Writable for ICR1 {}
#[doc = "GPIO interrupt configuration register1"]
pub mod icr1;
#[doc = "GPIO interrupt configuration register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [icr2](icr2) module"]
pub type ICR2 = crate::Reg<u32, _ICR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR2;
#[doc = "`read()` method returns [icr2::R](icr2::R) reader structure"]
impl crate::Readable for ICR2 {}
#[doc = "`write(|w| ..)` method takes [icr2::W](icr2::W) writer structure"]
impl crate::Writable for ICR2 {}
#[doc = "GPIO interrupt configuration register2"]
pub mod icr2;
#[doc = "GPIO interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "`write(|w| ..)` method takes [imr::W](imr::W) writer structure"]
impl crate::Writable for IMR {}
#[doc = "GPIO interrupt mask register"]
pub mod imr;
#[doc = "GPIO interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "`write(|w| ..)` method takes [isr::W](isr::W) writer structure"]
impl crate::Writable for ISR {}
#[doc = "GPIO interrupt status register"]
pub mod isr;
#[doc = "GPIO edge select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [edge_sel](edge_sel) module"]
pub type EDGE_SEL = crate::Reg<u32, _EDGE_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EDGE_SEL;
#[doc = "`read()` method returns [edge_sel::R](edge_sel::R) reader structure"]
impl crate::Readable for EDGE_SEL {}
#[doc = "`write(|w| ..)` method takes [edge_sel::W](edge_sel::W) writer structure"]
impl crate::Writable for EDGE_SEL {}
#[doc = "GPIO edge select register"]
pub mod edge_sel;
#[doc = "GPIO data register SET\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dr_set](dr_set) module"]
pub type DR_SET = crate::Reg<u32, _DR_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR_SET;
#[doc = "`write(|w| ..)` method takes [dr_set::W](dr_set::W) writer structure"]
impl crate::Writable for DR_SET {}
#[doc = "GPIO data register SET"]
pub mod dr_set;
#[doc = "GPIO data register CLEAR\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dr_clear](dr_clear) module"]
pub type DR_CLEAR = crate::Reg<u32, _DR_CLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR_CLEAR;
#[doc = "`write(|w| ..)` method takes [dr_clear::W](dr_clear::W) writer structure"]
impl crate::Writable for DR_CLEAR {}
#[doc = "GPIO data register CLEAR"]
pub mod dr_clear;
#[doc = "GPIO data register TOGGLE\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dr_toggle](dr_toggle) module"]
pub type DR_TOGGLE = crate::Reg<u32, _DR_TOGGLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR_TOGGLE;
#[doc = "`write(|w| ..)` method takes [dr_toggle::W](dr_toggle::W) writer structure"]
impl crate::Writable for DR_TOGGLE {}
#[doc = "GPIO data register TOGGLE"]
pub mod dr_toggle;
