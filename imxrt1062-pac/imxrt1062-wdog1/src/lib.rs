#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Control Register"]
    pub wcr: WCR,
    #[doc = "0x02 - Watchdog Service Register"]
    pub wsr: WSR,
    #[doc = "0x04 - Watchdog Reset Status Register"]
    pub wrsr: WRSR,
    #[doc = "0x06 - Watchdog Interrupt Control Register"]
    pub wicr: WICR,
    #[doc = "0x08 - Watchdog Miscellaneous Control Register"]
    pub wmcr: WMCR,
}
#[doc = "Watchdog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wcr](wcr) module"]
pub type WCR = crate::Reg<u16, _WCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WCR;
#[doc = "`read()` method returns [wcr::R](wcr::R) reader structure"]
impl crate::Readable for WCR {}
#[doc = "`write(|w| ..)` method takes [wcr::W](wcr::W) writer structure"]
impl crate::Writable for WCR {}
#[doc = "Watchdog Control Register"]
pub mod wcr;
#[doc = "Watchdog Service Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wsr](wsr) module"]
pub type WSR = crate::Reg<u16, _WSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WSR;
#[doc = "`read()` method returns [wsr::R](wsr::R) reader structure"]
impl crate::Readable for WSR {}
#[doc = "`write(|w| ..)` method takes [wsr::W](wsr::W) writer structure"]
impl crate::Writable for WSR {}
#[doc = "Watchdog Service Register"]
pub mod wsr;
#[doc = "Watchdog Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrsr](wrsr) module"]
pub type WRSR = crate::Reg<u16, _WRSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRSR;
#[doc = "`read()` method returns [wrsr::R](wrsr::R) reader structure"]
impl crate::Readable for WRSR {}
#[doc = "Watchdog Reset Status Register"]
pub mod wrsr;
#[doc = "Watchdog Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wicr](wicr) module"]
pub type WICR = crate::Reg<u16, _WICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WICR;
#[doc = "`read()` method returns [wicr::R](wicr::R) reader structure"]
impl crate::Readable for WICR {}
#[doc = "`write(|w| ..)` method takes [wicr::W](wicr::W) writer structure"]
impl crate::Writable for WICR {}
#[doc = "Watchdog Interrupt Control Register"]
pub mod wicr;
#[doc = "Watchdog Miscellaneous Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmcr](wmcr) module"]
pub type WMCR = crate::Reg<u16, _WMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WMCR;
#[doc = "`read()` method returns [wmcr::R](wmcr::R) reader structure"]
impl crate::Readable for WMCR {}
#[doc = "`write(|w| ..)` method takes [wmcr::W](wmcr::W) writer structure"]
impl crate::Writable for WMCR {}
#[doc = "Watchdog Miscellaneous Control Register"]
pub mod wmcr;
