#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPDIF Configuration Register"]
    pub scr: SCR,
    #[doc = "0x04 - CDText Control Register"]
    pub srcd: SRCD,
    #[doc = "0x08 - PhaseConfig Register"]
    pub srpc: SRPC,
    #[doc = "0x0c - InterruptEn Register"]
    pub sie: SIE,
    _reserved_4_sic: [u8; 4usize],
    #[doc = "0x14 - SPDIFRxLeft Register"]
    pub srl: SRL,
    #[doc = "0x18 - SPDIFRxRight Register"]
    pub srr: SRR,
    #[doc = "0x1c - SPDIFRxCChannel_h Register"]
    pub srcsh: SRCSH,
    #[doc = "0x20 - SPDIFRxCChannel_l Register"]
    pub srcsl: SRCSL,
    #[doc = "0x24 - UchannelRx Register"]
    pub sru: SRU,
    #[doc = "0x28 - QchannelRx Register"]
    pub srq: SRQ,
    #[doc = "0x2c - SPDIFTxLeft Register"]
    pub stl: STL,
    #[doc = "0x30 - SPDIFTxRight Register"]
    pub str: STR,
    #[doc = "0x34 - SPDIFTxCChannelCons_h Register"]
    pub stcsch: STCSCH,
    #[doc = "0x38 - SPDIFTxCChannelCons_l Register"]
    pub stcscl: STCSCL,
    _reserved15: [u8; 8usize],
    #[doc = "0x44 - FreqMeas Register"]
    pub srfm: SRFM,
    _reserved16: [u8; 8usize],
    #[doc = "0x50 - SPDIFTxClk Register"]
    pub stc: STC,
}
impl RegisterBlock {
    #[doc = "0x10 - InterruptStat Register"]
    #[inline(always)]
    pub fn sis(&self) -> &SIS {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const SIS) }
    }
    #[doc = "0x10 - InterruptStat Register"]
    #[inline(always)]
    pub fn sis_mut(&self) -> &mut SIS {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut SIS) }
    }
    #[doc = "0x10 - InterruptClear Register"]
    #[inline(always)]
    pub fn sic(&self) -> &SIC {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const SIC) }
    }
    #[doc = "0x10 - InterruptClear Register"]
    #[inline(always)]
    pub fn sic_mut(&self) -> &mut SIC {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut SIC) }
    }
}
#[doc = "SPDIF Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](scr) module"]
pub type SCR = crate::Reg<u32, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
#[doc = "`read()` method returns [scr::R](scr::R) reader structure"]
impl crate::Readable for SCR {}
#[doc = "`write(|w| ..)` method takes [scr::W](scr::W) writer structure"]
impl crate::Writable for SCR {}
#[doc = "SPDIF Configuration Register"]
pub mod scr;
#[doc = "CDText Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcd](srcd) module"]
pub type SRCD = crate::Reg<u32, _SRCD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCD;
#[doc = "`read()` method returns [srcd::R](srcd::R) reader structure"]
impl crate::Readable for SRCD {}
#[doc = "`write(|w| ..)` method takes [srcd::W](srcd::W) writer structure"]
impl crate::Writable for SRCD {}
#[doc = "CDText Control Register"]
pub mod srcd;
#[doc = "PhaseConfig Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srpc](srpc) module"]
pub type SRPC = crate::Reg<u32, _SRPC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRPC;
#[doc = "`read()` method returns [srpc::R](srpc::R) reader structure"]
impl crate::Readable for SRPC {}
#[doc = "`write(|w| ..)` method takes [srpc::W](srpc::W) writer structure"]
impl crate::Writable for SRPC {}
#[doc = "PhaseConfig Register"]
pub mod srpc;
#[doc = "InterruptEn Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie](sie) module"]
pub type SIE = crate::Reg<u32, _SIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIE;
#[doc = "`read()` method returns [sie::R](sie::R) reader structure"]
impl crate::Readable for SIE {}
#[doc = "`write(|w| ..)` method takes [sie::W](sie::W) writer structure"]
impl crate::Writable for SIE {}
#[doc = "InterruptEn Register"]
pub mod sie;
#[doc = "InterruptClear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sic](sic) module"]
pub type SIC = crate::Reg<u32, _SIC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIC;
#[doc = "`read()` method returns [sic::R](sic::R) reader structure"]
impl crate::Readable for SIC {}
#[doc = "`write(|w| ..)` method takes [sic::W](sic::W) writer structure"]
impl crate::Writable for SIC {}
#[doc = "InterruptClear Register"]
pub mod sic;
#[doc = "InterruptStat Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sis](sis) module"]
pub type SIS = crate::Reg<u32, _SIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIS;
#[doc = "`read()` method returns [sis::R](sis::R) reader structure"]
impl crate::Readable for SIS {}
#[doc = "InterruptStat Register"]
pub mod sis;
#[doc = "SPDIFRxLeft Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srl](srl) module"]
pub type SRL = crate::Reg<u32, _SRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRL;
#[doc = "`read()` method returns [srl::R](srl::R) reader structure"]
impl crate::Readable for SRL {}
#[doc = "SPDIFRxLeft Register"]
pub mod srl;
#[doc = "SPDIFRxRight Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srr](srr) module"]
pub type SRR = crate::Reg<u32, _SRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRR;
#[doc = "`read()` method returns [srr::R](srr::R) reader structure"]
impl crate::Readable for SRR {}
#[doc = "SPDIFRxRight Register"]
pub mod srr;
#[doc = "SPDIFRxCChannel_h Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcsh](srcsh) module"]
pub type SRCSH = crate::Reg<u32, _SRCSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCSH;
#[doc = "`read()` method returns [srcsh::R](srcsh::R) reader structure"]
impl crate::Readable for SRCSH {}
#[doc = "SPDIFRxCChannel_h Register"]
pub mod srcsh;
#[doc = "SPDIFRxCChannel_l Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcsl](srcsl) module"]
pub type SRCSL = crate::Reg<u32, _SRCSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCSL;
#[doc = "`read()` method returns [srcsl::R](srcsl::R) reader structure"]
impl crate::Readable for SRCSL {}
#[doc = "SPDIFRxCChannel_l Register"]
pub mod srcsl;
#[doc = "UchannelRx Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sru](sru) module"]
pub type SRU = crate::Reg<u32, _SRU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRU;
#[doc = "`read()` method returns [sru::R](sru::R) reader structure"]
impl crate::Readable for SRU {}
#[doc = "UchannelRx Register"]
pub mod sru;
#[doc = "QchannelRx Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srq](srq) module"]
pub type SRQ = crate::Reg<u32, _SRQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRQ;
#[doc = "`read()` method returns [srq::R](srq::R) reader structure"]
impl crate::Readable for SRQ {}
#[doc = "QchannelRx Register"]
pub mod srq;
#[doc = "SPDIFTxLeft Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stl](stl) module"]
pub type STL = crate::Reg<u32, _STL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STL;
#[doc = "`read()` method returns [stl::R](stl::R) reader structure"]
impl crate::Readable for STL {}
#[doc = "`write(|w| ..)` method takes [stl::W](stl::W) writer structure"]
impl crate::Writable for STL {}
#[doc = "SPDIFTxLeft Register"]
pub mod stl;
#[doc = "SPDIFTxRight Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [str](str) module"]
pub type STR = crate::Reg<u32, _STR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STR;
#[doc = "`read()` method returns [str::R](str::R) reader structure"]
impl crate::Readable for STR {}
#[doc = "`write(|w| ..)` method takes [str::W](str::W) writer structure"]
impl crate::Writable for STR {}
#[doc = "SPDIFTxRight Register"]
pub mod str;
#[doc = "SPDIFTxCChannelCons_h Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stcsch](stcsch) module"]
pub type STCSCH = crate::Reg<u32, _STCSCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STCSCH;
#[doc = "`read()` method returns [stcsch::R](stcsch::R) reader structure"]
impl crate::Readable for STCSCH {}
#[doc = "`write(|w| ..)` method takes [stcsch::W](stcsch::W) writer structure"]
impl crate::Writable for STCSCH {}
#[doc = "SPDIFTxCChannelCons_h Register"]
pub mod stcsch;
#[doc = "SPDIFTxCChannelCons_l Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stcscl](stcscl) module"]
pub type STCSCL = crate::Reg<u32, _STCSCL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STCSCL;
#[doc = "`read()` method returns [stcscl::R](stcscl::R) reader structure"]
impl crate::Readable for STCSCL {}
#[doc = "`write(|w| ..)` method takes [stcscl::W](stcscl::W) writer structure"]
impl crate::Writable for STCSCL {}
#[doc = "SPDIFTxCChannelCons_l Register"]
pub mod stcscl;
#[doc = "FreqMeas Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srfm](srfm) module"]
pub type SRFM = crate::Reg<u32, _SRFM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRFM;
#[doc = "`read()` method returns [srfm::R](srfm::R) reader structure"]
impl crate::Readable for SRFM {}
#[doc = "FreqMeas Register"]
pub mod srfm;
#[doc = "SPDIFTxClk Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stc](stc) module"]
pub type STC = crate::Reg<u32, _STC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STC;
#[doc = "`read()` method returns [stc::R](stc::R) reader structure"]
impl crate::Readable for STC {}
#[doc = "`write(|w| ..)` method takes [stc::W](stc::W) writer structure"]
impl crate::Writable for STC {}
#[doc = "SPDIFTxClk Register"]
pub mod stc;
