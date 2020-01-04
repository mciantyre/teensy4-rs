#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Control and Status Register"]
    pub cs: CS,
    #[doc = "0x04 - Watchdog Counter Register"]
    pub cnt: CNT,
    #[doc = "0x08 - Watchdog Timeout Value Register"]
    pub toval: TOVAL,
    #[doc = "0x0c - Watchdog Window Register"]
    pub win: WIN,
}
#[doc = "Watchdog Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cs](cs) module"]
pub type CS = crate::Reg<u32, _CS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS;
#[doc = "`read()` method returns [cs::R](cs::R) reader structure"]
impl crate::Readable for CS {}
#[doc = "`write(|w| ..)` method takes [cs::W](cs::W) writer structure"]
impl crate::Writable for CS {}
#[doc = "Watchdog Control and Status Register"]
pub mod cs;
#[doc = "Watchdog Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt](cnt) module"]
pub type CNT = crate::Reg<u32, _CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT;
#[doc = "`read()` method returns [cnt::R](cnt::R) reader structure"]
impl crate::Readable for CNT {}
#[doc = "`write(|w| ..)` method takes [cnt::W](cnt::W) writer structure"]
impl crate::Writable for CNT {}
#[doc = "Watchdog Counter Register"]
pub mod cnt;
#[doc = "Watchdog Timeout Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [toval](toval) module"]
pub type TOVAL = crate::Reg<u32, _TOVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOVAL;
#[doc = "`read()` method returns [toval::R](toval::R) reader structure"]
impl crate::Readable for TOVAL {}
#[doc = "`write(|w| ..)` method takes [toval::W](toval::W) writer structure"]
impl crate::Writable for TOVAL {}
#[doc = "Watchdog Timeout Value Register"]
pub mod toval;
#[doc = "Watchdog Window Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [win](win) module"]
pub type WIN = crate::Reg<u32, _WIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WIN;
#[doc = "`read()` method returns [win::R](win::R) reader structure"]
impl crate::Readable for WIN {}
#[doc = "`write(|w| ..)` method takes [win::W](win::W) writer structure"]
impl crate::Writable for WIN {}
#[doc = "Watchdog Window Register"]
pub mod win;
