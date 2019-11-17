#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

include!("../../generic.rs");

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CCM Control Register"]
    pub ccr: CCR,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - CCM Status Register"]
    pub csr: CSR,
    #[doc = "0x0c - CCM Clock Switcher Register"]
    pub ccsr: CCSR,
    #[doc = "0x10 - CCM Arm Clock Root Register"]
    pub cacrr: CACRR,
    #[doc = "0x14 - CCM Bus Clock Divider Register"]
    pub cbcdr: CBCDR,
    #[doc = "0x18 - CCM Bus Clock Multiplexer Register"]
    pub cbcmr: CBCMR,
    #[doc = "0x1c - CCM Serial Clock Multiplexer Register 1"]
    pub cscmr1: CSCMR1,
    #[doc = "0x20 - CCM Serial Clock Multiplexer Register 2"]
    pub cscmr2: CSCMR2,
    #[doc = "0x24 - CCM Serial Clock Divider Register 1"]
    pub cscdr1: CSCDR1,
    #[doc = "0x28 - CCM Clock Divider Register"]
    pub cs1cdr: CS1CDR,
    #[doc = "0x2c - CCM Clock Divider Register"]
    pub cs2cdr: CS2CDR,
    #[doc = "0x30 - CCM D1 Clock Divider Register"]
    pub cdcdr: CDCDR,
    _reserved12: [u8; 4usize],
    #[doc = "0x38 - CCM Serial Clock Divider Register 2"]
    pub cscdr2: CSCDR2,
    #[doc = "0x3c - CCM Serial Clock Divider Register 3"]
    pub cscdr3: CSCDR3,
    _reserved14: [u8; 8usize],
    #[doc = "0x48 - CCM Divider Handshake In-Process Register"]
    pub cdhipr: CDHIPR,
    _reserved15: [u8; 8usize],
    #[doc = "0x54 - CCM Low Power Control Register"]
    pub clpcr: CLPCR,
    #[doc = "0x58 - CCM Interrupt Status Register"]
    pub cisr: CISR,
    #[doc = "0x5c - CCM Interrupt Mask Register"]
    pub cimr: CIMR,
    #[doc = "0x60 - CCM Clock Output Source Register"]
    pub ccosr: CCOSR,
    #[doc = "0x64 - CCM General Purpose Register"]
    pub cgpr: CGPR,
    #[doc = "0x68 - CCM Clock Gating Register 0"]
    pub ccgr0: CCGR0,
    #[doc = "0x6c - CCM Clock Gating Register 1"]
    pub ccgr1: CCGR1,
    #[doc = "0x70 - CCM Clock Gating Register 2"]
    pub ccgr2: CCGR2,
    #[doc = "0x74 - CCM Clock Gating Register 3"]
    pub ccgr3: CCGR3,
    #[doc = "0x78 - CCM Clock Gating Register 4"]
    pub ccgr4: CCGR4,
    #[doc = "0x7c - CCM Clock Gating Register 5"]
    pub ccgr5: CCGR5,
    #[doc = "0x80 - CCM Clock Gating Register 6"]
    pub ccgr6: CCGR6,
    #[doc = "0x84 - CCM Clock Gating Register 7"]
    pub ccgr7: CCGR7,
    #[doc = "0x88 - CCM Module Enable Overide Register"]
    pub cmeor: CMEOR,
}
#[doc = "CCM Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ccr](ccr) module"]
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
#[doc = "`read()` method returns [ccr::R](ccr::R) reader structure"]
impl crate::Readable for CCR {}
#[doc = "`write(|w| ..)` method takes [ccr::W](ccr::W) writer structure"]
impl crate::Writable for CCR {}
#[doc = "CCM Control Register"]
pub mod ccr;
#[doc = "CCM Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csr](csr) module"]
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
#[doc = "`read()` method returns [csr::R](csr::R) reader structure"]
impl crate::Readable for CSR {}
#[doc = "CCM Status Register"]
pub mod csr;
#[doc = "CCM Clock Switcher Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ccsr](ccsr) module"]
pub type CCSR = crate::Reg<u32, _CCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCSR;
#[doc = "`read()` method returns [ccsr::R](ccsr::R) reader structure"]
impl crate::Readable for CCSR {}
#[doc = "`write(|w| ..)` method takes [ccsr::W](ccsr::W) writer structure"]
impl crate::Writable for CCSR {}
#[doc = "CCM Clock Switcher Register"]
pub mod ccsr;
#[doc = "CCM Arm Clock Root Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cacrr](cacrr) module"]
pub type CACRR = crate::Reg<u32, _CACRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CACRR;
#[doc = "`read()` method returns [cacrr::R](cacrr::R) reader structure"]
impl crate::Readable for CACRR {}
#[doc = "`write(|w| ..)` method takes [cacrr::W](cacrr::W) writer structure"]
impl crate::Writable for CACRR {}
#[doc = "CCM Arm Clock Root Register"]
pub mod cacrr;
#[doc = "CCM Bus Clock Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cbcdr](cbcdr) module"]
pub type CBCDR = crate::Reg<u32, _CBCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBCDR;
#[doc = "`read()` method returns [cbcdr::R](cbcdr::R) reader structure"]
impl crate::Readable for CBCDR {}
#[doc = "`write(|w| ..)` method takes [cbcdr::W](cbcdr::W) writer structure"]
impl crate::Writable for CBCDR {}
#[doc = "CCM Bus Clock Divider Register"]
pub mod cbcdr;
#[doc = "CCM Bus Clock Multiplexer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cbcmr](cbcmr) module"]
pub type CBCMR = crate::Reg<u32, _CBCMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBCMR;
#[doc = "`read()` method returns [cbcmr::R](cbcmr::R) reader structure"]
impl crate::Readable for CBCMR {}
#[doc = "`write(|w| ..)` method takes [cbcmr::W](cbcmr::W) writer structure"]
impl crate::Writable for CBCMR {}
#[doc = "CCM Bus Clock Multiplexer Register"]
pub mod cbcmr;
#[doc = "CCM Serial Clock Multiplexer Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cscmr1](cscmr1) module"]
pub type CSCMR1 = crate::Reg<u32, _CSCMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCMR1;
#[doc = "`read()` method returns [cscmr1::R](cscmr1::R) reader structure"]
impl crate::Readable for CSCMR1 {}
#[doc = "`write(|w| ..)` method takes [cscmr1::W](cscmr1::W) writer structure"]
impl crate::Writable for CSCMR1 {}
#[doc = "CCM Serial Clock Multiplexer Register 1"]
pub mod cscmr1;
#[doc = "CCM Serial Clock Multiplexer Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cscmr2](cscmr2) module"]
pub type CSCMR2 = crate::Reg<u32, _CSCMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCMR2;
#[doc = "`read()` method returns [cscmr2::R](cscmr2::R) reader structure"]
impl crate::Readable for CSCMR2 {}
#[doc = "`write(|w| ..)` method takes [cscmr2::W](cscmr2::W) writer structure"]
impl crate::Writable for CSCMR2 {}
#[doc = "CCM Serial Clock Multiplexer Register 2"]
pub mod cscmr2;
#[doc = "CCM Serial Clock Divider Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cscdr1](cscdr1) module"]
pub type CSCDR1 = crate::Reg<u32, _CSCDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCDR1;
#[doc = "`read()` method returns [cscdr1::R](cscdr1::R) reader structure"]
impl crate::Readable for CSCDR1 {}
#[doc = "`write(|w| ..)` method takes [cscdr1::W](cscdr1::W) writer structure"]
impl crate::Writable for CSCDR1 {}
#[doc = "CCM Serial Clock Divider Register 1"]
pub mod cscdr1;
#[doc = "CCM Clock Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cs1cdr](cs1cdr) module"]
pub type CS1CDR = crate::Reg<u32, _CS1CDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS1CDR;
#[doc = "`read()` method returns [cs1cdr::R](cs1cdr::R) reader structure"]
impl crate::Readable for CS1CDR {}
#[doc = "`write(|w| ..)` method takes [cs1cdr::W](cs1cdr::W) writer structure"]
impl crate::Writable for CS1CDR {}
#[doc = "CCM Clock Divider Register"]
pub mod cs1cdr;
#[doc = "CCM Clock Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cs2cdr](cs2cdr) module"]
pub type CS2CDR = crate::Reg<u32, _CS2CDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS2CDR;
#[doc = "`read()` method returns [cs2cdr::R](cs2cdr::R) reader structure"]
impl crate::Readable for CS2CDR {}
#[doc = "`write(|w| ..)` method takes [cs2cdr::W](cs2cdr::W) writer structure"]
impl crate::Writable for CS2CDR {}
#[doc = "CCM Clock Divider Register"]
pub mod cs2cdr;
#[doc = "CCM D1 Clock Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cdcdr](cdcdr) module"]
pub type CDCDR = crate::Reg<u32, _CDCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDCDR;
#[doc = "`read()` method returns [cdcdr::R](cdcdr::R) reader structure"]
impl crate::Readable for CDCDR {}
#[doc = "`write(|w| ..)` method takes [cdcdr::W](cdcdr::W) writer structure"]
impl crate::Writable for CDCDR {}
#[doc = "CCM D1 Clock Divider Register"]
pub mod cdcdr;
#[doc = "CCM Serial Clock Divider Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cscdr2](cscdr2) module"]
pub type CSCDR2 = crate::Reg<u32, _CSCDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCDR2;
#[doc = "`read()` method returns [cscdr2::R](cscdr2::R) reader structure"]
impl crate::Readable for CSCDR2 {}
#[doc = "`write(|w| ..)` method takes [cscdr2::W](cscdr2::W) writer structure"]
impl crate::Writable for CSCDR2 {}
#[doc = "CCM Serial Clock Divider Register 2"]
pub mod cscdr2;
#[doc = "CCM Serial Clock Divider Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cscdr3](cscdr3) module"]
pub type CSCDR3 = crate::Reg<u32, _CSCDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCDR3;
#[doc = "`read()` method returns [cscdr3::R](cscdr3::R) reader structure"]
impl crate::Readable for CSCDR3 {}
#[doc = "`write(|w| ..)` method takes [cscdr3::W](cscdr3::W) writer structure"]
impl crate::Writable for CSCDR3 {}
#[doc = "CCM Serial Clock Divider Register 3"]
pub mod cscdr3;
#[doc = "CCM Divider Handshake In-Process Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cdhipr](cdhipr) module"]
pub type CDHIPR = crate::Reg<u32, _CDHIPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDHIPR;
#[doc = "`read()` method returns [cdhipr::R](cdhipr::R) reader structure"]
impl crate::Readable for CDHIPR {}
#[doc = "CCM Divider Handshake In-Process Register"]
pub mod cdhipr;
#[doc = "CCM Low Power Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clpcr](clpcr) module"]
pub type CLPCR = crate::Reg<u32, _CLPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLPCR;
#[doc = "`read()` method returns [clpcr::R](clpcr::R) reader structure"]
impl crate::Readable for CLPCR {}
#[doc = "`write(|w| ..)` method takes [clpcr::W](clpcr::W) writer structure"]
impl crate::Writable for CLPCR {}
#[doc = "CCM Low Power Control Register"]
pub mod clpcr;
#[doc = "CCM Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cisr](cisr) module"]
pub type CISR = crate::Reg<u32, _CISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CISR;
#[doc = "`read()` method returns [cisr::R](cisr::R) reader structure"]
impl crate::Readable for CISR {}
#[doc = "`write(|w| ..)` method takes [cisr::W](cisr::W) writer structure"]
impl crate::Writable for CISR {}
#[doc = "CCM Interrupt Status Register"]
pub mod cisr;
#[doc = "CCM Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cimr](cimr) module"]
pub type CIMR = crate::Reg<u32, _CIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIMR;
#[doc = "`read()` method returns [cimr::R](cimr::R) reader structure"]
impl crate::Readable for CIMR {}
#[doc = "`write(|w| ..)` method takes [cimr::W](cimr::W) writer structure"]
impl crate::Writable for CIMR {}
#[doc = "CCM Interrupt Mask Register"]
pub mod cimr;
#[doc = "CCM Clock Output Source Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ccosr](ccosr) module"]
pub type CCOSR = crate::Reg<u32, _CCOSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCOSR;
#[doc = "`read()` method returns [ccosr::R](ccosr::R) reader structure"]
impl crate::Readable for CCOSR {}
#[doc = "`write(|w| ..)` method takes [ccosr::W](ccosr::W) writer structure"]
impl crate::Writable for CCOSR {}
#[doc = "CCM Clock Output Source Register"]
pub mod ccosr;
#[doc = "CCM General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cgpr](cgpr) module"]
pub type CGPR = crate::Reg<u32, _CGPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CGPR;
#[doc = "`read()` method returns [cgpr::R](cgpr::R) reader structure"]
impl crate::Readable for CGPR {}
#[doc = "`write(|w| ..)` method takes [cgpr::W](cgpr::W) writer structure"]
impl crate::Writable for CGPR {}
#[doc = "CCM General Purpose Register"]
pub mod cgpr;
#[doc = "CCM Clock Gating Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ccgr0](ccgr0) module"]
pub type CCGR0 = crate::Reg<u32, _CCGR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCGR0;
#[doc = "`read()` method returns [ccgr0::R](ccgr0::R) reader structure"]
impl crate::Readable for CCGR0 {}
#[doc = "`write(|w| ..)` method takes [ccgr0::W](ccgr0::W) writer structure"]
impl crate::Writable for CCGR0 {}
#[doc = "CCM Clock Gating Register 0"]
pub mod ccgr0;
#[doc = "CCM Clock Gating Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ccgr1](ccgr1) module"]
pub type CCGR1 = crate::Reg<u32, _CCGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCGR1;
#[doc = "`read()` method returns [ccgr1::R](ccgr1::R) reader structure"]
impl crate::Readable for CCGR1 {}
#[doc = "`write(|w| ..)` method takes [ccgr1::W](ccgr1::W) writer structure"]
impl crate::Writable for CCGR1 {}
#[doc = "CCM Clock Gating Register 1"]
pub mod ccgr1;
#[doc = "CCM Clock Gating Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ccgr2](ccgr2) module"]
pub type CCGR2 = crate::Reg<u32, _CCGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCGR2;
#[doc = "`read()` method returns [ccgr2::R](ccgr2::R) reader structure"]
impl crate::Readable for CCGR2 {}
#[doc = "`write(|w| ..)` method takes [ccgr2::W](ccgr2::W) writer structure"]
impl crate::Writable for CCGR2 {}
#[doc = "CCM Clock Gating Register 2"]
pub mod ccgr2;
#[doc = "CCM Clock Gating Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ccgr3](ccgr3) module"]
pub type CCGR3 = crate::Reg<u32, _CCGR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCGR3;
#[doc = "`read()` method returns [ccgr3::R](ccgr3::R) reader structure"]
impl crate::Readable for CCGR3 {}
#[doc = "`write(|w| ..)` method takes [ccgr3::W](ccgr3::W) writer structure"]
impl crate::Writable for CCGR3 {}
#[doc = "CCM Clock Gating Register 3"]
pub mod ccgr3;
#[doc = "CCM Clock Gating Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ccgr4](ccgr4) module"]
pub type CCGR4 = crate::Reg<u32, _CCGR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCGR4;
#[doc = "`read()` method returns [ccgr4::R](ccgr4::R) reader structure"]
impl crate::Readable for CCGR4 {}
#[doc = "`write(|w| ..)` method takes [ccgr4::W](ccgr4::W) writer structure"]
impl crate::Writable for CCGR4 {}
#[doc = "CCM Clock Gating Register 4"]
pub mod ccgr4;
#[doc = "CCM Clock Gating Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ccgr5](ccgr5) module"]
pub type CCGR5 = crate::Reg<u32, _CCGR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCGR5;
#[doc = "`read()` method returns [ccgr5::R](ccgr5::R) reader structure"]
impl crate::Readable for CCGR5 {}
#[doc = "`write(|w| ..)` method takes [ccgr5::W](ccgr5::W) writer structure"]
impl crate::Writable for CCGR5 {}
#[doc = "CCM Clock Gating Register 5"]
pub mod ccgr5;
#[doc = "CCM Clock Gating Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ccgr6](ccgr6) module"]
pub type CCGR6 = crate::Reg<u32, _CCGR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCGR6;
#[doc = "`read()` method returns [ccgr6::R](ccgr6::R) reader structure"]
impl crate::Readable for CCGR6 {}
#[doc = "`write(|w| ..)` method takes [ccgr6::W](ccgr6::W) writer structure"]
impl crate::Writable for CCGR6 {}
#[doc = "CCM Clock Gating Register 6"]
pub mod ccgr6;
#[doc = "CCM Clock Gating Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ccgr7](ccgr7) module"]
pub type CCGR7 = crate::Reg<u32, _CCGR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCGR7;
#[doc = "`read()` method returns [ccgr7::R](ccgr7::R) reader structure"]
impl crate::Readable for CCGR7 {}
#[doc = "`write(|w| ..)` method takes [ccgr7::W](ccgr7::W) writer structure"]
impl crate::Writable for CCGR7 {}
#[doc = "CCM Clock Gating Register 7"]
pub mod ccgr7;
#[doc = "CCM Module Enable Overide Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmeor](cmeor) module"]
pub type CMEOR = crate::Reg<u32, _CMEOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMEOR;
#[doc = "`read()` method returns [cmeor::R](cmeor::R) reader structure"]
impl crate::Readable for CMEOR {}
#[doc = "`write(|w| ..)` method takes [cmeor::W](cmeor::W) writer structure"]
impl crate::Writable for CMEOR {}
#[doc = "CCM Module Enable Overide Register"]
pub mod cmeor;
