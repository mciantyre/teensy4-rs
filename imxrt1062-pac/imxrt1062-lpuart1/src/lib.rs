#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub verid: VERID,
    #[doc = "0x04 - Parameter Register"]
    pub param: PARAM,
    #[doc = "0x08 - LPUART Global Register"]
    pub global: GLOBAL,
    #[doc = "0x0c - LPUART Pin Configuration Register"]
    pub pincfg: PINCFG,
    #[doc = "0x10 - LPUART Baud Rate Register"]
    pub baud: BAUD,
    #[doc = "0x14 - LPUART Status Register"]
    pub stat: STAT,
    #[doc = "0x18 - LPUART Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x1c - LPUART Data Register"]
    pub data: DATA,
    #[doc = "0x20 - LPUART Match Address Register"]
    pub match_: MATCH,
    #[doc = "0x24 - LPUART Modem IrDA Register"]
    pub modir: MODIR,
    #[doc = "0x28 - LPUART FIFO Register"]
    pub fifo: FIFO,
    #[doc = "0x2c - LPUART Watermark Register"]
    pub water: WATER,
}
#[doc = "Version ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [verid](verid) module"]
pub type VERID = crate::Reg<u32, _VERID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERID;
#[doc = "`read()` method returns [verid::R](verid::R) reader structure"]
impl crate::Readable for VERID {}
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [param](param) module"]
pub type PARAM = crate::Reg<u32, _PARAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PARAM;
#[doc = "`read()` method returns [param::R](param::R) reader structure"]
impl crate::Readable for PARAM {}
#[doc = "Parameter Register"]
pub mod param;
#[doc = "LPUART Global Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [global](global) module"]
pub type GLOBAL = crate::Reg<u32, _GLOBAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GLOBAL;
#[doc = "`read()` method returns [global::R](global::R) reader structure"]
impl crate::Readable for GLOBAL {}
#[doc = "`write(|w| ..)` method takes [global::W](global::W) writer structure"]
impl crate::Writable for GLOBAL {}
#[doc = "LPUART Global Register"]
pub mod global;
#[doc = "LPUART Pin Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pincfg](pincfg) module"]
pub type PINCFG = crate::Reg<u32, _PINCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINCFG;
#[doc = "`read()` method returns [pincfg::R](pincfg::R) reader structure"]
impl crate::Readable for PINCFG {}
#[doc = "`write(|w| ..)` method takes [pincfg::W](pincfg::W) writer structure"]
impl crate::Writable for PINCFG {}
#[doc = "LPUART Pin Configuration Register"]
pub mod pincfg;
#[doc = "LPUART Baud Rate Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baud](baud) module"]
pub type BAUD = crate::Reg<u32, _BAUD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAUD;
#[doc = "`read()` method returns [baud::R](baud::R) reader structure"]
impl crate::Readable for BAUD {}
#[doc = "`write(|w| ..)` method takes [baud::W](baud::W) writer structure"]
impl crate::Writable for BAUD {}
#[doc = "LPUART Baud Rate Register"]
pub mod baud;
#[doc = "LPUART Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "LPUART Status Register"]
pub mod stat;
#[doc = "LPUART Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "LPUART Control Register"]
pub mod ctrl;
#[doc = "LPUART Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "LPUART Data Register"]
pub mod data;
#[doc = "LPUART Match Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [match_](match_) module"]
pub type MATCH = crate::Reg<u32, _MATCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCH;
#[doc = "`read()` method returns [match_::R](match_::R) reader structure"]
impl crate::Readable for MATCH {}
#[doc = "`write(|w| ..)` method takes [match_::W](match_::W) writer structure"]
impl crate::Writable for MATCH {}
#[doc = "LPUART Match Address Register"]
pub mod match_;
#[doc = "LPUART Modem IrDA Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modir](modir) module"]
pub type MODIR = crate::Reg<u32, _MODIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODIR;
#[doc = "`read()` method returns [modir::R](modir::R) reader structure"]
impl crate::Readable for MODIR {}
#[doc = "`write(|w| ..)` method takes [modir::W](modir::W) writer structure"]
impl crate::Writable for MODIR {}
#[doc = "LPUART Modem IrDA Register"]
pub mod modir;
#[doc = "LPUART FIFO Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo](fifo) module"]
pub type FIFO = crate::Reg<u32, _FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO;
#[doc = "`read()` method returns [fifo::R](fifo::R) reader structure"]
impl crate::Readable for FIFO {}
#[doc = "`write(|w| ..)` method takes [fifo::W](fifo::W) writer structure"]
impl crate::Writable for FIFO {}
#[doc = "LPUART FIFO Register"]
pub mod fifo;
#[doc = "LPUART Watermark Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [water](water) module"]
pub type WATER = crate::Reg<u32, _WATER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WATER;
#[doc = "`read()` method returns [water::R](water::R) reader structure"]
impl crate::Readable for WATER {}
#[doc = "`write(|w| ..)` method takes [water::W](water::W) writer structure"]
impl crate::Writable for WATER {}
#[doc = "LPUART Watermark Register"]
pub mod water;
