#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 212usize],
    #[doc = "0xd4 - ROMC Data Registers"]
    pub rompatch7d: ROMPATCHD,
    #[doc = "0xd8 - ROMC Data Registers"]
    pub rompatch6d: ROMPATCHD,
    #[doc = "0xdc - ROMC Data Registers"]
    pub rompatch5d: ROMPATCHD,
    #[doc = "0xe0 - ROMC Data Registers"]
    pub rompatch4d: ROMPATCHD,
    #[doc = "0xe4 - ROMC Data Registers"]
    pub rompatch3d: ROMPATCHD,
    #[doc = "0xe8 - ROMC Data Registers"]
    pub rompatch2d: ROMPATCHD,
    #[doc = "0xec - ROMC Data Registers"]
    pub rompatch1d: ROMPATCHD,
    #[doc = "0xf0 - ROMC Data Registers"]
    pub rompatch0d: ROMPATCHD,
    #[doc = "0xf4 - ROMC Control Register"]
    pub rompatchcntl: ROMPATCHCNTL,
    #[doc = "0xf8 - ROMC Enable Register High"]
    pub rompatchenh: ROMPATCHENH,
    #[doc = "0xfc - ROMC Enable Register Low"]
    pub rompatchenl: ROMPATCHENL,
    #[doc = "0x100 - ROMC Address Registers"]
    pub rompatcha: [ROMPATCHA; 16],
    _reserved12: [u8; 200usize],
    #[doc = "0x208 - ROMC Status Register"]
    pub rompatchsr: ROMPATCHSR,
}
#[doc = "ROMC Data Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rompatchd](rompatchd) module"]
pub type ROMPATCHD = crate::Reg<u32, _ROMPATCHD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROMPATCHD;
#[doc = "`read()` method returns [rompatchd::R](rompatchd::R) reader structure"]
impl crate::Readable for ROMPATCHD {}
#[doc = "`write(|w| ..)` method takes [rompatchd::W](rompatchd::W) writer structure"]
impl crate::Writable for ROMPATCHD {}
#[doc = "ROMC Data Registers"]
pub mod rompatchd;
#[doc = "ROMC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rompatchcntl](rompatchcntl) module"]
pub type ROMPATCHCNTL = crate::Reg<u32, _ROMPATCHCNTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROMPATCHCNTL;
#[doc = "`read()` method returns [rompatchcntl::R](rompatchcntl::R) reader structure"]
impl crate::Readable for ROMPATCHCNTL {}
#[doc = "`write(|w| ..)` method takes [rompatchcntl::W](rompatchcntl::W) writer structure"]
impl crate::Writable for ROMPATCHCNTL {}
#[doc = "ROMC Control Register"]
pub mod rompatchcntl;
#[doc = "ROMC Enable Register High\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rompatchenh](rompatchenh) module"]
pub type ROMPATCHENH = crate::Reg<u32, _ROMPATCHENH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROMPATCHENH;
#[doc = "`read()` method returns [rompatchenh::R](rompatchenh::R) reader structure"]
impl crate::Readable for ROMPATCHENH {}
#[doc = "ROMC Enable Register High"]
pub mod rompatchenh;
#[doc = "ROMC Enable Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rompatchenl](rompatchenl) module"]
pub type ROMPATCHENL = crate::Reg<u32, _ROMPATCHENL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROMPATCHENL;
#[doc = "`read()` method returns [rompatchenl::R](rompatchenl::R) reader structure"]
impl crate::Readable for ROMPATCHENL {}
#[doc = "`write(|w| ..)` method takes [rompatchenl::W](rompatchenl::W) writer structure"]
impl crate::Writable for ROMPATCHENL {}
#[doc = "ROMC Enable Register Low"]
pub mod rompatchenl;
#[doc = "ROMC Address Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rompatcha](rompatcha) module"]
pub type ROMPATCHA = crate::Reg<u32, _ROMPATCHA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROMPATCHA;
#[doc = "`read()` method returns [rompatcha::R](rompatcha::R) reader structure"]
impl crate::Readable for ROMPATCHA {}
#[doc = "`write(|w| ..)` method takes [rompatcha::W](rompatcha::W) writer structure"]
impl crate::Writable for ROMPATCHA {}
#[doc = "ROMC Address Registers"]
pub mod rompatcha;
#[doc = "ROMC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rompatchsr](rompatchsr) module"]
pub type ROMPATCHSR = crate::Reg<u32, _ROMPATCHSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROMPATCHSR;
#[doc = "`read()` method returns [rompatchsr::R](rompatchsr::R) reader structure"]
impl crate::Readable for ROMPATCHSR {}
#[doc = "`write(|w| ..)` method takes [rompatchsr::W](rompatchsr::W) writer structure"]
impl crate::Writable for ROMPATCHSR {}
#[doc = "ROMC Status Register"]
pub mod rompatchsr;
