#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SNVS_HP Lock Register"]
    pub hplr: HPLR,
    #[doc = "0x04 - SNVS_HP Command Register"]
    pub hpcomr: HPCOMR,
    #[doc = "0x08 - SNVS_HP Control Register"]
    pub hpcr: HPCR,
    #[doc = "0x0c - SNVS_HP Security Interrupt Control Register"]
    pub hpsicr: HPSICR,
    #[doc = "0x10 - SNVS_HP Security Violation Control Register"]
    pub hpsvcr: HPSVCR,
    #[doc = "0x14 - SNVS_HP Status Register"]
    pub hpsr: HPSR,
    #[doc = "0x18 - SNVS_HP Security Violation Status Register"]
    pub hpsvsr: HPSVSR,
    #[doc = "0x1c - SNVS_HP High Assurance Counter IV Register"]
    pub hphacivr: HPHACIVR,
    #[doc = "0x20 - SNVS_HP High Assurance Counter Register"]
    pub hphacr: HPHACR,
    #[doc = "0x24 - SNVS_HP Real Time Counter MSB Register"]
    pub hprtcmr: HPRTCMR,
    #[doc = "0x28 - SNVS_HP Real Time Counter LSB Register"]
    pub hprtclr: HPRTCLR,
    #[doc = "0x2c - SNVS_HP Time Alarm MSB Register"]
    pub hptamr: HPTAMR,
    #[doc = "0x30 - SNVS_HP Time Alarm LSB Register"]
    pub hptalr: HPTALR,
    #[doc = "0x34 - SNVS_LP Lock Register"]
    pub lplr: LPLR,
    #[doc = "0x38 - SNVS_LP Control Register"]
    pub lpcr: LPCR,
    #[doc = "0x3c - SNVS_LP Master Key Control Register"]
    pub lpmkcr: LPMKCR,
    #[doc = "0x40 - SNVS_LP Security Violation Control Register"]
    pub lpsvcr: LPSVCR,
    _reserved17: [u8; 4usize],
    #[doc = "0x48 - SNVS_LP Tamper Detectors Configuration Register"]
    pub lptdcr: LPTDCR,
    #[doc = "0x4c - SNVS_LP Status Register"]
    pub lpsr: LPSR,
    #[doc = "0x50 - SNVS_LP Secure Real Time Counter MSB Register"]
    pub lpsrtcmr: LPSRTCMR,
    #[doc = "0x54 - SNVS_LP Secure Real Time Counter LSB Register"]
    pub lpsrtclr: LPSRTCLR,
    #[doc = "0x58 - SNVS_LP Time Alarm Register"]
    pub lptar: LPTAR,
    #[doc = "0x5c - SNVS_LP Secure Monotonic Counter MSB Register"]
    pub lpsmcmr: LPSMCMR,
    #[doc = "0x60 - SNVS_LP Secure Monotonic Counter LSB Register"]
    pub lpsmclr: LPSMCLR,
    #[doc = "0x64 - SNVS_LP Power Glitch Detector Register"]
    pub lppgdr: LPPGDR,
    #[doc = "0x68 - SNVS_LP General Purpose Register 0 (legacy alias)"]
    pub lpgpr0_legacy_alias: LPGPR0_LEGACY_ALIAS,
    #[doc = "0x6c - SNVS_LP Zeroizable Master Key Register"]
    pub lpzmkr: [LPZMKR; 8],
    _reserved27: [u8; 4usize],
    #[doc = "0x90 - SNVS_LP General Purpose Registers 0 .. 3"]
    pub lpgpr_alias: [LPGPR_ALIAS; 4],
    _reserved28: [u8; 96usize],
    #[doc = "0x100 - SNVS_LP General Purpose Registers 0 .. 7"]
    pub lpgpr: [LPGPR; 8],
    _reserved29: [u8; 2776usize],
    #[doc = "0xbf8 - SNVS_HP Version ID Register 1"]
    pub hpvidr1: HPVIDR1,
    #[doc = "0xbfc - SNVS_HP Version ID Register 2"]
    pub hpvidr2: HPVIDR2,
}
#[doc = "SNVS_HP Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hplr](hplr) module"]
pub type HPLR = crate::Reg<u32, _HPLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPLR;
#[doc = "`read()` method returns [hplr::R](hplr::R) reader structure"]
impl crate::Readable for HPLR {}
#[doc = "`write(|w| ..)` method takes [hplr::W](hplr::W) writer structure"]
impl crate::Writable for HPLR {}
#[doc = "SNVS_HP Lock Register"]
pub mod hplr;
#[doc = "SNVS_HP Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpcomr](hpcomr) module"]
pub type HPCOMR = crate::Reg<u32, _HPCOMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPCOMR;
#[doc = "`read()` method returns [hpcomr::R](hpcomr::R) reader structure"]
impl crate::Readable for HPCOMR {}
#[doc = "`write(|w| ..)` method takes [hpcomr::W](hpcomr::W) writer structure"]
impl crate::Writable for HPCOMR {}
#[doc = "SNVS_HP Command Register"]
pub mod hpcomr;
#[doc = "SNVS_HP Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpcr](hpcr) module"]
pub type HPCR = crate::Reg<u32, _HPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPCR;
#[doc = "`read()` method returns [hpcr::R](hpcr::R) reader structure"]
impl crate::Readable for HPCR {}
#[doc = "`write(|w| ..)` method takes [hpcr::W](hpcr::W) writer structure"]
impl crate::Writable for HPCR {}
#[doc = "SNVS_HP Control Register"]
pub mod hpcr;
#[doc = "SNVS_HP Security Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpsicr](hpsicr) module"]
pub type HPSICR = crate::Reg<u32, _HPSICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPSICR;
#[doc = "`read()` method returns [hpsicr::R](hpsicr::R) reader structure"]
impl crate::Readable for HPSICR {}
#[doc = "`write(|w| ..)` method takes [hpsicr::W](hpsicr::W) writer structure"]
impl crate::Writable for HPSICR {}
#[doc = "SNVS_HP Security Interrupt Control Register"]
pub mod hpsicr;
#[doc = "SNVS_HP Security Violation Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpsvcr](hpsvcr) module"]
pub type HPSVCR = crate::Reg<u32, _HPSVCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPSVCR;
#[doc = "`read()` method returns [hpsvcr::R](hpsvcr::R) reader structure"]
impl crate::Readable for HPSVCR {}
#[doc = "`write(|w| ..)` method takes [hpsvcr::W](hpsvcr::W) writer structure"]
impl crate::Writable for HPSVCR {}
#[doc = "SNVS_HP Security Violation Control Register"]
pub mod hpsvcr;
#[doc = "SNVS_HP Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpsr](hpsr) module"]
pub type HPSR = crate::Reg<u32, _HPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPSR;
#[doc = "`read()` method returns [hpsr::R](hpsr::R) reader structure"]
impl crate::Readable for HPSR {}
#[doc = "`write(|w| ..)` method takes [hpsr::W](hpsr::W) writer structure"]
impl crate::Writable for HPSR {}
#[doc = "SNVS_HP Status Register"]
pub mod hpsr;
#[doc = "SNVS_HP Security Violation Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpsvsr](hpsvsr) module"]
pub type HPSVSR = crate::Reg<u32, _HPSVSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPSVSR;
#[doc = "`read()` method returns [hpsvsr::R](hpsvsr::R) reader structure"]
impl crate::Readable for HPSVSR {}
#[doc = "`write(|w| ..)` method takes [hpsvsr::W](hpsvsr::W) writer structure"]
impl crate::Writable for HPSVSR {}
#[doc = "SNVS_HP Security Violation Status Register"]
pub mod hpsvsr;
#[doc = "SNVS_HP High Assurance Counter IV Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hphacivr](hphacivr) module"]
pub type HPHACIVR = crate::Reg<u32, _HPHACIVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPHACIVR;
#[doc = "`read()` method returns [hphacivr::R](hphacivr::R) reader structure"]
impl crate::Readable for HPHACIVR {}
#[doc = "`write(|w| ..)` method takes [hphacivr::W](hphacivr::W) writer structure"]
impl crate::Writable for HPHACIVR {}
#[doc = "SNVS_HP High Assurance Counter IV Register"]
pub mod hphacivr;
#[doc = "SNVS_HP High Assurance Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hphacr](hphacr) module"]
pub type HPHACR = crate::Reg<u32, _HPHACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPHACR;
#[doc = "`read()` method returns [hphacr::R](hphacr::R) reader structure"]
impl crate::Readable for HPHACR {}
#[doc = "SNVS_HP High Assurance Counter Register"]
pub mod hphacr;
#[doc = "SNVS_HP Real Time Counter MSB Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hprtcmr](hprtcmr) module"]
pub type HPRTCMR = crate::Reg<u32, _HPRTCMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPRTCMR;
#[doc = "`read()` method returns [hprtcmr::R](hprtcmr::R) reader structure"]
impl crate::Readable for HPRTCMR {}
#[doc = "`write(|w| ..)` method takes [hprtcmr::W](hprtcmr::W) writer structure"]
impl crate::Writable for HPRTCMR {}
#[doc = "SNVS_HP Real Time Counter MSB Register"]
pub mod hprtcmr;
#[doc = "SNVS_HP Real Time Counter LSB Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hprtclr](hprtclr) module"]
pub type HPRTCLR = crate::Reg<u32, _HPRTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPRTCLR;
#[doc = "`read()` method returns [hprtclr::R](hprtclr::R) reader structure"]
impl crate::Readable for HPRTCLR {}
#[doc = "`write(|w| ..)` method takes [hprtclr::W](hprtclr::W) writer structure"]
impl crate::Writable for HPRTCLR {}
#[doc = "SNVS_HP Real Time Counter LSB Register"]
pub mod hprtclr;
#[doc = "SNVS_HP Time Alarm MSB Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hptamr](hptamr) module"]
pub type HPTAMR = crate::Reg<u32, _HPTAMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPTAMR;
#[doc = "`read()` method returns [hptamr::R](hptamr::R) reader structure"]
impl crate::Readable for HPTAMR {}
#[doc = "`write(|w| ..)` method takes [hptamr::W](hptamr::W) writer structure"]
impl crate::Writable for HPTAMR {}
#[doc = "SNVS_HP Time Alarm MSB Register"]
pub mod hptamr;
#[doc = "SNVS_HP Time Alarm LSB Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hptalr](hptalr) module"]
pub type HPTALR = crate::Reg<u32, _HPTALR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPTALR;
#[doc = "`read()` method returns [hptalr::R](hptalr::R) reader structure"]
impl crate::Readable for HPTALR {}
#[doc = "`write(|w| ..)` method takes [hptalr::W](hptalr::W) writer structure"]
impl crate::Writable for HPTALR {}
#[doc = "SNVS_HP Time Alarm LSB Register"]
pub mod hptalr;
#[doc = "SNVS_LP Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lplr](lplr) module"]
pub type LPLR = crate::Reg<u32, _LPLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPLR;
#[doc = "`read()` method returns [lplr::R](lplr::R) reader structure"]
impl crate::Readable for LPLR {}
#[doc = "`write(|w| ..)` method takes [lplr::W](lplr::W) writer structure"]
impl crate::Writable for LPLR {}
#[doc = "SNVS_LP Lock Register"]
pub mod lplr;
#[doc = "SNVS_LP Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpcr](lpcr) module"]
pub type LPCR = crate::Reg<u32, _LPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPCR;
#[doc = "`read()` method returns [lpcr::R](lpcr::R) reader structure"]
impl crate::Readable for LPCR {}
#[doc = "`write(|w| ..)` method takes [lpcr::W](lpcr::W) writer structure"]
impl crate::Writable for LPCR {}
#[doc = "SNVS_LP Control Register"]
pub mod lpcr;
#[doc = "SNVS_LP Master Key Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmkcr](lpmkcr) module"]
pub type LPMKCR = crate::Reg<u32, _LPMKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPMKCR;
#[doc = "`read()` method returns [lpmkcr::R](lpmkcr::R) reader structure"]
impl crate::Readable for LPMKCR {}
#[doc = "`write(|w| ..)` method takes [lpmkcr::W](lpmkcr::W) writer structure"]
impl crate::Writable for LPMKCR {}
#[doc = "SNVS_LP Master Key Control Register"]
pub mod lpmkcr;
#[doc = "SNVS_LP Security Violation Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpsvcr](lpsvcr) module"]
pub type LPSVCR = crate::Reg<u32, _LPSVCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPSVCR;
#[doc = "`read()` method returns [lpsvcr::R](lpsvcr::R) reader structure"]
impl crate::Readable for LPSVCR {}
#[doc = "`write(|w| ..)` method takes [lpsvcr::W](lpsvcr::W) writer structure"]
impl crate::Writable for LPSVCR {}
#[doc = "SNVS_LP Security Violation Control Register"]
pub mod lpsvcr;
#[doc = "SNVS_LP Tamper Detectors Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptdcr](lptdcr) module"]
pub type LPTDCR = crate::Reg<u32, _LPTDCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTDCR;
#[doc = "`read()` method returns [lptdcr::R](lptdcr::R) reader structure"]
impl crate::Readable for LPTDCR {}
#[doc = "`write(|w| ..)` method takes [lptdcr::W](lptdcr::W) writer structure"]
impl crate::Writable for LPTDCR {}
#[doc = "SNVS_LP Tamper Detectors Configuration Register"]
pub mod lptdcr;
#[doc = "SNVS_LP Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpsr](lpsr) module"]
pub type LPSR = crate::Reg<u32, _LPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPSR;
#[doc = "`read()` method returns [lpsr::R](lpsr::R) reader structure"]
impl crate::Readable for LPSR {}
#[doc = "`write(|w| ..)` method takes [lpsr::W](lpsr::W) writer structure"]
impl crate::Writable for LPSR {}
#[doc = "SNVS_LP Status Register"]
pub mod lpsr;
#[doc = "SNVS_LP Secure Real Time Counter MSB Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpsrtcmr](lpsrtcmr) module"]
pub type LPSRTCMR = crate::Reg<u32, _LPSRTCMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPSRTCMR;
#[doc = "`read()` method returns [lpsrtcmr::R](lpsrtcmr::R) reader structure"]
impl crate::Readable for LPSRTCMR {}
#[doc = "`write(|w| ..)` method takes [lpsrtcmr::W](lpsrtcmr::W) writer structure"]
impl crate::Writable for LPSRTCMR {}
#[doc = "SNVS_LP Secure Real Time Counter MSB Register"]
pub mod lpsrtcmr;
#[doc = "SNVS_LP Secure Real Time Counter LSB Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpsrtclr](lpsrtclr) module"]
pub type LPSRTCLR = crate::Reg<u32, _LPSRTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPSRTCLR;
#[doc = "`read()` method returns [lpsrtclr::R](lpsrtclr::R) reader structure"]
impl crate::Readable for LPSRTCLR {}
#[doc = "`write(|w| ..)` method takes [lpsrtclr::W](lpsrtclr::W) writer structure"]
impl crate::Writable for LPSRTCLR {}
#[doc = "SNVS_LP Secure Real Time Counter LSB Register"]
pub mod lpsrtclr;
#[doc = "SNVS_LP Time Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptar](lptar) module"]
pub type LPTAR = crate::Reg<u32, _LPTAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTAR;
#[doc = "`read()` method returns [lptar::R](lptar::R) reader structure"]
impl crate::Readable for LPTAR {}
#[doc = "`write(|w| ..)` method takes [lptar::W](lptar::W) writer structure"]
impl crate::Writable for LPTAR {}
#[doc = "SNVS_LP Time Alarm Register"]
pub mod lptar;
#[doc = "SNVS_LP Secure Monotonic Counter MSB Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpsmcmr](lpsmcmr) module"]
pub type LPSMCMR = crate::Reg<u32, _LPSMCMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPSMCMR;
#[doc = "`read()` method returns [lpsmcmr::R](lpsmcmr::R) reader structure"]
impl crate::Readable for LPSMCMR {}
#[doc = "SNVS_LP Secure Monotonic Counter MSB Register"]
pub mod lpsmcmr;
#[doc = "SNVS_LP Secure Monotonic Counter LSB Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpsmclr](lpsmclr) module"]
pub type LPSMCLR = crate::Reg<u32, _LPSMCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPSMCLR;
#[doc = "`read()` method returns [lpsmclr::R](lpsmclr::R) reader structure"]
impl crate::Readable for LPSMCLR {}
#[doc = "SNVS_LP Secure Monotonic Counter LSB Register"]
pub mod lpsmclr;
#[doc = "SNVS_LP Power Glitch Detector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lppgdr](lppgdr) module"]
pub type LPPGDR = crate::Reg<u32, _LPPGDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPPGDR;
#[doc = "`read()` method returns [lppgdr::R](lppgdr::R) reader structure"]
impl crate::Readable for LPPGDR {}
#[doc = "`write(|w| ..)` method takes [lppgdr::W](lppgdr::W) writer structure"]
impl crate::Writable for LPPGDR {}
#[doc = "SNVS_LP Power Glitch Detector Register"]
pub mod lppgdr;
#[doc = "SNVS_LP General Purpose Register 0 (legacy alias)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpgpr0_legacy_alias](lpgpr0_legacy_alias) module"]
pub type LPGPR0_LEGACY_ALIAS = crate::Reg<u32, _LPGPR0_LEGACY_ALIAS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPGPR0_LEGACY_ALIAS;
#[doc = "`read()` method returns [lpgpr0_legacy_alias::R](lpgpr0_legacy_alias::R) reader structure"]
impl crate::Readable for LPGPR0_LEGACY_ALIAS {}
#[doc = "`write(|w| ..)` method takes [lpgpr0_legacy_alias::W](lpgpr0_legacy_alias::W) writer structure"]
impl crate::Writable for LPGPR0_LEGACY_ALIAS {}
#[doc = "SNVS_LP General Purpose Register 0 (legacy alias)"]
pub mod lpgpr0_legacy_alias;
#[doc = "SNVS_LP Zeroizable Master Key Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpzmkr](lpzmkr) module"]
pub type LPZMKR = crate::Reg<u32, _LPZMKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPZMKR;
#[doc = "`read()` method returns [lpzmkr::R](lpzmkr::R) reader structure"]
impl crate::Readable for LPZMKR {}
#[doc = "`write(|w| ..)` method takes [lpzmkr::W](lpzmkr::W) writer structure"]
impl crate::Writable for LPZMKR {}
#[doc = "SNVS_LP Zeroizable Master Key Register"]
pub mod lpzmkr;
#[doc = "SNVS_LP General Purpose Registers 0 .. 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpgpr_alias](lpgpr_alias) module"]
pub type LPGPR_ALIAS = crate::Reg<u32, _LPGPR_ALIAS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPGPR_ALIAS;
#[doc = "`read()` method returns [lpgpr_alias::R](lpgpr_alias::R) reader structure"]
impl crate::Readable for LPGPR_ALIAS {}
#[doc = "`write(|w| ..)` method takes [lpgpr_alias::W](lpgpr_alias::W) writer structure"]
impl crate::Writable for LPGPR_ALIAS {}
#[doc = "SNVS_LP General Purpose Registers 0 .. 3"]
pub mod lpgpr_alias;
#[doc = "SNVS_LP General Purpose Registers 0 .. 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpgpr](lpgpr) module"]
pub type LPGPR = crate::Reg<u32, _LPGPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPGPR;
#[doc = "`read()` method returns [lpgpr::R](lpgpr::R) reader structure"]
impl crate::Readable for LPGPR {}
#[doc = "`write(|w| ..)` method takes [lpgpr::W](lpgpr::W) writer structure"]
impl crate::Writable for LPGPR {}
#[doc = "SNVS_LP General Purpose Registers 0 .. 7"]
pub mod lpgpr;
#[doc = "SNVS_HP Version ID Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpvidr1](hpvidr1) module"]
pub type HPVIDR1 = crate::Reg<u32, _HPVIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPVIDR1;
#[doc = "`read()` method returns [hpvidr1::R](hpvidr1::R) reader structure"]
impl crate::Readable for HPVIDR1 {}
#[doc = "SNVS_HP Version ID Register 1"]
pub mod hpvidr1;
#[doc = "SNVS_HP Version ID Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpvidr2](hpvidr2) module"]
pub type HPVIDR2 = crate::Reg<u32, _HPVIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPVIDR2;
#[doc = "`read()` method returns [hpvidr2::R](hpvidr2::R) reader structure"]
impl crate::Readable for HPVIDR2 {}
#[doc = "SNVS_HP Version ID Register 2"]
pub mod hpvidr2;
