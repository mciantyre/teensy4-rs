#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Config security level register"]
    pub csl: [CSL; 32],
    _reserved1: [u8; 384usize],
    #[doc = "0x200 - HP0 register"]
    pub hp0: HP0,
    _reserved2: [u8; 20usize],
    #[doc = "0x218 - Secure access register"]
    pub sa: SA,
    _reserved3: [u8; 316usize],
    #[doc = "0x358 - HPCONTROL0 register"]
    pub hpcontrol0: HPCONTROL0,
}
#[doc = "Config security level register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csl](csl) module"]
pub type CSL = crate::Reg<u32, _CSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSL;
#[doc = "`read()` method returns [csl::R](csl::R) reader structure"]
impl crate::Readable for CSL {}
#[doc = "`write(|w| ..)` method takes [csl::W](csl::W) writer structure"]
impl crate::Writable for CSL {}
#[doc = "Config security level register"]
pub mod csl;
#[doc = "HP0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hp0](hp0) module"]
pub type HP0 = crate::Reg<u32, _HP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HP0;
#[doc = "`read()` method returns [hp0::R](hp0::R) reader structure"]
impl crate::Readable for HP0 {}
#[doc = "`write(|w| ..)` method takes [hp0::W](hp0::W) writer structure"]
impl crate::Writable for HP0 {}
#[doc = "HP0 register"]
pub mod hp0;
#[doc = "Secure access register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sa](sa) module"]
pub type SA = crate::Reg<u32, _SA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SA;
#[doc = "`read()` method returns [sa::R](sa::R) reader structure"]
impl crate::Readable for SA {}
#[doc = "`write(|w| ..)` method takes [sa::W](sa::W) writer structure"]
impl crate::Writable for SA {}
#[doc = "Secure access register"]
pub mod sa;
#[doc = "HPCONTROL0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpcontrol0](hpcontrol0) module"]
pub type HPCONTROL0 = crate::Reg<u32, _HPCONTROL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPCONTROL0;
#[doc = "`read()` method returns [hpcontrol0::R](hpcontrol0::R) reader structure"]
impl crate::Readable for HPCONTROL0 {}
#[doc = "`write(|w| ..)` method takes [hpcontrol0::W](hpcontrol0::W) writer structure"]
impl crate::Writable for HPCONTROL0 {}
#[doc = "HPCONTROL0 register"]
pub mod hpcontrol0;
