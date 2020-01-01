#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Keypad Control Register"]
    pub kpcr: KPCR,
    #[doc = "0x02 - Keypad Status Register"]
    pub kpsr: KPSR,
    #[doc = "0x04 - Keypad Data Direction Register"]
    pub kddr: KDDR,
    #[doc = "0x06 - Keypad Data Register"]
    pub kpdr: KPDR,
}
#[doc = "Keypad Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kpcr](kpcr) module"]
pub type KPCR = crate::Reg<u16, _KPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KPCR;
#[doc = "`read()` method returns [kpcr::R](kpcr::R) reader structure"]
impl crate::Readable for KPCR {}
#[doc = "`write(|w| ..)` method takes [kpcr::W](kpcr::W) writer structure"]
impl crate::Writable for KPCR {}
#[doc = "Keypad Control Register"]
pub mod kpcr;
#[doc = "Keypad Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kpsr](kpsr) module"]
pub type KPSR = crate::Reg<u16, _KPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KPSR;
#[doc = "`read()` method returns [kpsr::R](kpsr::R) reader structure"]
impl crate::Readable for KPSR {}
#[doc = "`write(|w| ..)` method takes [kpsr::W](kpsr::W) writer structure"]
impl crate::Writable for KPSR {}
#[doc = "Keypad Status Register"]
pub mod kpsr;
#[doc = "Keypad Data Direction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kddr](kddr) module"]
pub type KDDR = crate::Reg<u16, _KDDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KDDR;
#[doc = "`read()` method returns [kddr::R](kddr::R) reader structure"]
impl crate::Readable for KDDR {}
#[doc = "`write(|w| ..)` method takes [kddr::W](kddr::W) writer structure"]
impl crate::Writable for KDDR {}
#[doc = "Keypad Data Direction Register"]
pub mod kddr;
#[doc = "Keypad Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kpdr](kpdr) module"]
pub type KPDR = crate::Reg<u16, _KPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KPDR;
#[doc = "`read()` method returns [kpdr::R](kpdr::R) reader structure"]
impl crate::Readable for KPDR {}
#[doc = "`write(|w| ..)` method takes [kpdr::W](kpdr::W) writer structure"]
impl crate::Writable for KPDR {}
#[doc = "Keypad Data Register"]
pub mod kpdr;
