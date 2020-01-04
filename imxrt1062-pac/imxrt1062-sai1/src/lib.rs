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
    #[doc = "0x08 - SAI Transmit Control Register"]
    pub tcsr: TCSR,
    #[doc = "0x0c - SAI Transmit Configuration 1 Register"]
    pub tcr1: TCR1,
    #[doc = "0x10 - SAI Transmit Configuration 2 Register"]
    pub tcr2: TCR2,
    #[doc = "0x14 - SAI Transmit Configuration 3 Register"]
    pub tcr3: TCR3,
    #[doc = "0x18 - SAI Transmit Configuration 4 Register"]
    pub tcr4: TCR4,
    #[doc = "0x1c - SAI Transmit Configuration 5 Register"]
    pub tcr5: TCR5,
    #[doc = "0x20 - SAI Transmit Data Register"]
    pub tdr: [TDR; 4],
    _reserved9: [u8; 16usize],
    #[doc = "0x40 - SAI Transmit FIFO Register"]
    pub tfr: [TFR; 4],
    _reserved10: [u8; 16usize],
    #[doc = "0x60 - SAI Transmit Mask Register"]
    pub tmr: TMR,
    _reserved11: [u8; 36usize],
    #[doc = "0x88 - SAI Receive Control Register"]
    pub rcsr: RCSR,
    #[doc = "0x8c - SAI Receive Configuration 1 Register"]
    pub rcr1: RCR1,
    #[doc = "0x90 - SAI Receive Configuration 2 Register"]
    pub rcr2: RCR2,
    #[doc = "0x94 - SAI Receive Configuration 3 Register"]
    pub rcr3: RCR3,
    #[doc = "0x98 - SAI Receive Configuration 4 Register"]
    pub rcr4: RCR4,
    #[doc = "0x9c - SAI Receive Configuration 5 Register"]
    pub rcr5: RCR5,
    #[doc = "0xa0 - SAI Receive Data Register"]
    pub rdr: [RDR; 4],
    _reserved18: [u8; 16usize],
    #[doc = "0xc0 - SAI Receive FIFO Register"]
    pub rfr: [RFR; 4],
    _reserved19: [u8; 16usize],
    #[doc = "0xe0 - SAI Receive Mask Register"]
    pub rmr: RMR,
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
#[doc = "SAI Transmit Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcsr](tcsr) module"]
pub type TCSR = crate::Reg<u32, _TCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCSR;
#[doc = "`read()` method returns [tcsr::R](tcsr::R) reader structure"]
impl crate::Readable for TCSR {}
#[doc = "`write(|w| ..)` method takes [tcsr::W](tcsr::W) writer structure"]
impl crate::Writable for TCSR {}
#[doc = "SAI Transmit Control Register"]
pub mod tcsr;
#[doc = "SAI Transmit Configuration 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr1](tcr1) module"]
pub type TCR1 = crate::Reg<u32, _TCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCR1;
#[doc = "`read()` method returns [tcr1::R](tcr1::R) reader structure"]
impl crate::Readable for TCR1 {}
#[doc = "`write(|w| ..)` method takes [tcr1::W](tcr1::W) writer structure"]
impl crate::Writable for TCR1 {}
#[doc = "SAI Transmit Configuration 1 Register"]
pub mod tcr1;
#[doc = "SAI Transmit Configuration 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr2](tcr2) module"]
pub type TCR2 = crate::Reg<u32, _TCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCR2;
#[doc = "`read()` method returns [tcr2::R](tcr2::R) reader structure"]
impl crate::Readable for TCR2 {}
#[doc = "`write(|w| ..)` method takes [tcr2::W](tcr2::W) writer structure"]
impl crate::Writable for TCR2 {}
#[doc = "SAI Transmit Configuration 2 Register"]
pub mod tcr2;
#[doc = "SAI Transmit Configuration 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr3](tcr3) module"]
pub type TCR3 = crate::Reg<u32, _TCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCR3;
#[doc = "`read()` method returns [tcr3::R](tcr3::R) reader structure"]
impl crate::Readable for TCR3 {}
#[doc = "`write(|w| ..)` method takes [tcr3::W](tcr3::W) writer structure"]
impl crate::Writable for TCR3 {}
#[doc = "SAI Transmit Configuration 3 Register"]
pub mod tcr3;
#[doc = "SAI Transmit Configuration 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr4](tcr4) module"]
pub type TCR4 = crate::Reg<u32, _TCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCR4;
#[doc = "`read()` method returns [tcr4::R](tcr4::R) reader structure"]
impl crate::Readable for TCR4 {}
#[doc = "`write(|w| ..)` method takes [tcr4::W](tcr4::W) writer structure"]
impl crate::Writable for TCR4 {}
#[doc = "SAI Transmit Configuration 4 Register"]
pub mod tcr4;
#[doc = "SAI Transmit Configuration 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr5](tcr5) module"]
pub type TCR5 = crate::Reg<u32, _TCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCR5;
#[doc = "`read()` method returns [tcr5::R](tcr5::R) reader structure"]
impl crate::Readable for TCR5 {}
#[doc = "`write(|w| ..)` method takes [tcr5::W](tcr5::W) writer structure"]
impl crate::Writable for TCR5 {}
#[doc = "SAI Transmit Configuration 5 Register"]
pub mod tcr5;
#[doc = "SAI Transmit Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdr](tdr) module"]
pub type TDR = crate::Reg<u32, _TDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDR;
#[doc = "`read()` method returns [tdr::R](tdr::R) reader structure"]
impl crate::Readable for TDR {}
#[doc = "`write(|w| ..)` method takes [tdr::W](tdr::W) writer structure"]
impl crate::Writable for TDR {}
#[doc = "SAI Transmit Data Register"]
pub mod tdr;
#[doc = "SAI Transmit FIFO Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfr](tfr) module"]
pub type TFR = crate::Reg<u32, _TFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFR;
#[doc = "`read()` method returns [tfr::R](tfr::R) reader structure"]
impl crate::Readable for TFR {}
#[doc = "SAI Transmit FIFO Register"]
pub mod tfr;
#[doc = "SAI Transmit Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr](tmr) module"]
pub type TMR = crate::Reg<u32, _TMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR;
#[doc = "`read()` method returns [tmr::R](tmr::R) reader structure"]
impl crate::Readable for TMR {}
#[doc = "`write(|w| ..)` method takes [tmr::W](tmr::W) writer structure"]
impl crate::Writable for TMR {}
#[doc = "SAI Transmit Mask Register"]
pub mod tmr;
#[doc = "SAI Receive Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcsr](rcsr) module"]
pub type RCSR = crate::Reg<u32, _RCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCSR;
#[doc = "`read()` method returns [rcsr::R](rcsr::R) reader structure"]
impl crate::Readable for RCSR {}
#[doc = "`write(|w| ..)` method takes [rcsr::W](rcsr::W) writer structure"]
impl crate::Writable for RCSR {}
#[doc = "SAI Receive Control Register"]
pub mod rcsr;
#[doc = "SAI Receive Configuration 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr1](rcr1) module"]
pub type RCR1 = crate::Reg<u32, _RCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCR1;
#[doc = "`read()` method returns [rcr1::R](rcr1::R) reader structure"]
impl crate::Readable for RCR1 {}
#[doc = "`write(|w| ..)` method takes [rcr1::W](rcr1::W) writer structure"]
impl crate::Writable for RCR1 {}
#[doc = "SAI Receive Configuration 1 Register"]
pub mod rcr1;
#[doc = "SAI Receive Configuration 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr2](rcr2) module"]
pub type RCR2 = crate::Reg<u32, _RCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCR2;
#[doc = "`read()` method returns [rcr2::R](rcr2::R) reader structure"]
impl crate::Readable for RCR2 {}
#[doc = "`write(|w| ..)` method takes [rcr2::W](rcr2::W) writer structure"]
impl crate::Writable for RCR2 {}
#[doc = "SAI Receive Configuration 2 Register"]
pub mod rcr2;
#[doc = "SAI Receive Configuration 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr3](rcr3) module"]
pub type RCR3 = crate::Reg<u32, _RCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCR3;
#[doc = "`read()` method returns [rcr3::R](rcr3::R) reader structure"]
impl crate::Readable for RCR3 {}
#[doc = "`write(|w| ..)` method takes [rcr3::W](rcr3::W) writer structure"]
impl crate::Writable for RCR3 {}
#[doc = "SAI Receive Configuration 3 Register"]
pub mod rcr3;
#[doc = "SAI Receive Configuration 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr4](rcr4) module"]
pub type RCR4 = crate::Reg<u32, _RCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCR4;
#[doc = "`read()` method returns [rcr4::R](rcr4::R) reader structure"]
impl crate::Readable for RCR4 {}
#[doc = "`write(|w| ..)` method takes [rcr4::W](rcr4::W) writer structure"]
impl crate::Writable for RCR4 {}
#[doc = "SAI Receive Configuration 4 Register"]
pub mod rcr4;
#[doc = "SAI Receive Configuration 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr5](rcr5) module"]
pub type RCR5 = crate::Reg<u32, _RCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCR5;
#[doc = "`read()` method returns [rcr5::R](rcr5::R) reader structure"]
impl crate::Readable for RCR5 {}
#[doc = "`write(|w| ..)` method takes [rcr5::W](rcr5::W) writer structure"]
impl crate::Writable for RCR5 {}
#[doc = "SAI Receive Configuration 5 Register"]
pub mod rcr5;
#[doc = "SAI Receive Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdr](rdr) module"]
pub type RDR = crate::Reg<u32, _RDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDR;
#[doc = "`read()` method returns [rdr::R](rdr::R) reader structure"]
impl crate::Readable for RDR {}
#[doc = "SAI Receive Data Register"]
pub mod rdr;
#[doc = "SAI Receive FIFO Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfr](rfr) module"]
pub type RFR = crate::Reg<u32, _RFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFR;
#[doc = "`read()` method returns [rfr::R](rfr::R) reader structure"]
impl crate::Readable for RFR {}
#[doc = "SAI Receive FIFO Register"]
pub mod rfr;
#[doc = "SAI Receive Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmr](rmr) module"]
pub type RMR = crate::Reg<u32, _RMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMR;
#[doc = "`read()` method returns [rmr::R](rmr::R) reader structure"]
impl crate::Readable for RMR {}
#[doc = "`write(|w| ..)` method takes [rmr::W](rmr::W) writer structure"]
impl crate::Writable for RMR {}
#[doc = "SAI Receive Mask Register"]
pub mod rmr;
