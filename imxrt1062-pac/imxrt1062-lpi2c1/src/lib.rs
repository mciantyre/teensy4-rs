#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

include!("../../generic.rs");

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub verid: VERID,
    #[doc = "0x04 - Parameter Register"]
    pub param: PARAM,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Master Control Register"]
    pub mcr: MCR,
    #[doc = "0x14 - Master Status Register"]
    pub msr: MSR,
    #[doc = "0x18 - Master Interrupt Enable Register"]
    pub mier: MIER,
    #[doc = "0x1c - Master DMA Enable Register"]
    pub mder: MDER,
    #[doc = "0x20 - Master Configuration Register 0"]
    pub mcfgr0: MCFGR0,
    #[doc = "0x24 - Master Configuration Register 1"]
    pub mcfgr1: MCFGR1,
    #[doc = "0x28 - Master Configuration Register 2"]
    pub mcfgr2: MCFGR2,
    #[doc = "0x2c - Master Configuration Register 3"]
    pub mcfgr3: MCFGR3,
    _reserved10: [u8; 16usize],
    #[doc = "0x40 - Master Data Match Register"]
    pub mdmr: MDMR,
    _reserved11: [u8; 4usize],
    #[doc = "0x48 - Master Clock Configuration Register 0"]
    pub mccr0: MCCR0,
    _reserved12: [u8; 4usize],
    #[doc = "0x50 - Master Clock Configuration Register 1"]
    pub mccr1: MCCR1,
    _reserved13: [u8; 4usize],
    #[doc = "0x58 - Master FIFO Control Register"]
    pub mfcr: MFCR,
    #[doc = "0x5c - Master FIFO Status Register"]
    pub mfsr: MFSR,
    #[doc = "0x60 - Master Transmit Data Register"]
    pub mtdr: MTDR,
    _reserved16: [u8; 12usize],
    #[doc = "0x70 - Master Receive Data Register"]
    pub mrdr: MRDR,
    _reserved17: [u8; 156usize],
    #[doc = "0x110 - Slave Control Register"]
    pub scr: SCR,
    #[doc = "0x114 - Slave Status Register"]
    pub ssr: SSR,
    #[doc = "0x118 - Slave Interrupt Enable Register"]
    pub sier: SIER,
    #[doc = "0x11c - Slave DMA Enable Register"]
    pub sder: SDER,
    _reserved21: [u8; 4usize],
    #[doc = "0x124 - Slave Configuration Register 1"]
    pub scfgr1: SCFGR1,
    #[doc = "0x128 - Slave Configuration Register 2"]
    pub scfgr2: SCFGR2,
    _reserved23: [u8; 20usize],
    #[doc = "0x140 - Slave Address Match Register"]
    pub samr: SAMR,
    _reserved24: [u8; 12usize],
    #[doc = "0x150 - Slave Address Status Register"]
    pub sasr: SASR,
    #[doc = "0x154 - Slave Transmit ACK Register"]
    pub star: STAR,
    _reserved26: [u8; 8usize],
    #[doc = "0x160 - Slave Transmit Data Register"]
    pub stdr: STDR,
    _reserved27: [u8; 12usize],
    #[doc = "0x170 - Slave Receive Data Register"]
    pub srdr: SRDR,
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
#[doc = "Master Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](mcr) module"]
pub type MCR = crate::Reg<u32, _MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR;
#[doc = "`read()` method returns [mcr::R](mcr::R) reader structure"]
impl crate::Readable for MCR {}
#[doc = "`write(|w| ..)` method takes [mcr::W](mcr::W) writer structure"]
impl crate::Writable for MCR {}
#[doc = "Master Control Register"]
pub mod mcr;
#[doc = "Master Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msr](msr) module"]
pub type MSR = crate::Reg<u32, _MSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSR;
#[doc = "`read()` method returns [msr::R](msr::R) reader structure"]
impl crate::Readable for MSR {}
#[doc = "`write(|w| ..)` method takes [msr::W](msr::W) writer structure"]
impl crate::Writable for MSR {}
#[doc = "Master Status Register"]
pub mod msr;
#[doc = "Master Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mier](mier) module"]
pub type MIER = crate::Reg<u32, _MIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIER;
#[doc = "`read()` method returns [mier::R](mier::R) reader structure"]
impl crate::Readable for MIER {}
#[doc = "`write(|w| ..)` method takes [mier::W](mier::W) writer structure"]
impl crate::Writable for MIER {}
#[doc = "Master Interrupt Enable Register"]
pub mod mier;
#[doc = "Master DMA Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mder](mder) module"]
pub type MDER = crate::Reg<u32, _MDER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDER;
#[doc = "`read()` method returns [mder::R](mder::R) reader structure"]
impl crate::Readable for MDER {}
#[doc = "`write(|w| ..)` method takes [mder::W](mder::W) writer structure"]
impl crate::Writable for MDER {}
#[doc = "Master DMA Enable Register"]
pub mod mder;
#[doc = "Master Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcfgr0](mcfgr0) module"]
pub type MCFGR0 = crate::Reg<u32, _MCFGR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCFGR0;
#[doc = "`read()` method returns [mcfgr0::R](mcfgr0::R) reader structure"]
impl crate::Readable for MCFGR0 {}
#[doc = "`write(|w| ..)` method takes [mcfgr0::W](mcfgr0::W) writer structure"]
impl crate::Writable for MCFGR0 {}
#[doc = "Master Configuration Register 0"]
pub mod mcfgr0;
#[doc = "Master Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcfgr1](mcfgr1) module"]
pub type MCFGR1 = crate::Reg<u32, _MCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCFGR1;
#[doc = "`read()` method returns [mcfgr1::R](mcfgr1::R) reader structure"]
impl crate::Readable for MCFGR1 {}
#[doc = "`write(|w| ..)` method takes [mcfgr1::W](mcfgr1::W) writer structure"]
impl crate::Writable for MCFGR1 {}
#[doc = "Master Configuration Register 1"]
pub mod mcfgr1;
#[doc = "Master Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcfgr2](mcfgr2) module"]
pub type MCFGR2 = crate::Reg<u32, _MCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCFGR2;
#[doc = "`read()` method returns [mcfgr2::R](mcfgr2::R) reader structure"]
impl crate::Readable for MCFGR2 {}
#[doc = "`write(|w| ..)` method takes [mcfgr2::W](mcfgr2::W) writer structure"]
impl crate::Writable for MCFGR2 {}
#[doc = "Master Configuration Register 2"]
pub mod mcfgr2;
#[doc = "Master Configuration Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcfgr3](mcfgr3) module"]
pub type MCFGR3 = crate::Reg<u32, _MCFGR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCFGR3;
#[doc = "`read()` method returns [mcfgr3::R](mcfgr3::R) reader structure"]
impl crate::Readable for MCFGR3 {}
#[doc = "`write(|w| ..)` method takes [mcfgr3::W](mcfgr3::W) writer structure"]
impl crate::Writable for MCFGR3 {}
#[doc = "Master Configuration Register 3"]
pub mod mcfgr3;
#[doc = "Master Data Match Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdmr](mdmr) module"]
pub type MDMR = crate::Reg<u32, _MDMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMR;
#[doc = "`read()` method returns [mdmr::R](mdmr::R) reader structure"]
impl crate::Readable for MDMR {}
#[doc = "`write(|w| ..)` method takes [mdmr::W](mdmr::W) writer structure"]
impl crate::Writable for MDMR {}
#[doc = "Master Data Match Register"]
pub mod mdmr;
#[doc = "Master Clock Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mccr0](mccr0) module"]
pub type MCCR0 = crate::Reg<u32, _MCCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCCR0;
#[doc = "`read()` method returns [mccr0::R](mccr0::R) reader structure"]
impl crate::Readable for MCCR0 {}
#[doc = "`write(|w| ..)` method takes [mccr0::W](mccr0::W) writer structure"]
impl crate::Writable for MCCR0 {}
#[doc = "Master Clock Configuration Register 0"]
pub mod mccr0;
#[doc = "Master Clock Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mccr1](mccr1) module"]
pub type MCCR1 = crate::Reg<u32, _MCCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCCR1;
#[doc = "`read()` method returns [mccr1::R](mccr1::R) reader structure"]
impl crate::Readable for MCCR1 {}
#[doc = "`write(|w| ..)` method takes [mccr1::W](mccr1::W) writer structure"]
impl crate::Writable for MCCR1 {}
#[doc = "Master Clock Configuration Register 1"]
pub mod mccr1;
#[doc = "Master FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mfcr](mfcr) module"]
pub type MFCR = crate::Reg<u32, _MFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MFCR;
#[doc = "`read()` method returns [mfcr::R](mfcr::R) reader structure"]
impl crate::Readable for MFCR {}
#[doc = "`write(|w| ..)` method takes [mfcr::W](mfcr::W) writer structure"]
impl crate::Writable for MFCR {}
#[doc = "Master FIFO Control Register"]
pub mod mfcr;
#[doc = "Master FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mfsr](mfsr) module"]
pub type MFSR = crate::Reg<u32, _MFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MFSR;
#[doc = "`read()` method returns [mfsr::R](mfsr::R) reader structure"]
impl crate::Readable for MFSR {}
#[doc = "Master FIFO Status Register"]
pub mod mfsr;
#[doc = "Master Transmit Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtdr](mtdr) module"]
pub type MTDR = crate::Reg<u32, _MTDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTDR;
#[doc = "`read()` method returns [mtdr::R](mtdr::R) reader structure"]
impl crate::Readable for MTDR {}
#[doc = "`write(|w| ..)` method takes [mtdr::W](mtdr::W) writer structure"]
impl crate::Writable for MTDR {}
#[doc = "Master Transmit Data Register"]
pub mod mtdr;
#[doc = "Master Receive Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mrdr](mrdr) module"]
pub type MRDR = crate::Reg<u32, _MRDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRDR;
#[doc = "`read()` method returns [mrdr::R](mrdr::R) reader structure"]
impl crate::Readable for MRDR {}
#[doc = "Master Receive Data Register"]
pub mod mrdr;
#[doc = "Slave Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](scr) module"]
pub type SCR = crate::Reg<u32, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
#[doc = "`read()` method returns [scr::R](scr::R) reader structure"]
impl crate::Readable for SCR {}
#[doc = "`write(|w| ..)` method takes [scr::W](scr::W) writer structure"]
impl crate::Writable for SCR {}
#[doc = "Slave Control Register"]
pub mod scr;
#[doc = "Slave Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssr](ssr) module"]
pub type SSR = crate::Reg<u32, _SSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSR;
#[doc = "`read()` method returns [ssr::R](ssr::R) reader structure"]
impl crate::Readable for SSR {}
#[doc = "`write(|w| ..)` method takes [ssr::W](ssr::W) writer structure"]
impl crate::Writable for SSR {}
#[doc = "Slave Status Register"]
pub mod ssr;
#[doc = "Slave Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sier](sier) module"]
pub type SIER = crate::Reg<u32, _SIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIER;
#[doc = "`read()` method returns [sier::R](sier::R) reader structure"]
impl crate::Readable for SIER {}
#[doc = "`write(|w| ..)` method takes [sier::W](sier::W) writer structure"]
impl crate::Writable for SIER {}
#[doc = "Slave Interrupt Enable Register"]
pub mod sier;
#[doc = "Slave DMA Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sder](sder) module"]
pub type SDER = crate::Reg<u32, _SDER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDER;
#[doc = "`read()` method returns [sder::R](sder::R) reader structure"]
impl crate::Readable for SDER {}
#[doc = "`write(|w| ..)` method takes [sder::W](sder::W) writer structure"]
impl crate::Writable for SDER {}
#[doc = "Slave DMA Enable Register"]
pub mod sder;
#[doc = "Slave Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scfgr1](scfgr1) module"]
pub type SCFGR1 = crate::Reg<u32, _SCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCFGR1;
#[doc = "`read()` method returns [scfgr1::R](scfgr1::R) reader structure"]
impl crate::Readable for SCFGR1 {}
#[doc = "`write(|w| ..)` method takes [scfgr1::W](scfgr1::W) writer structure"]
impl crate::Writable for SCFGR1 {}
#[doc = "Slave Configuration Register 1"]
pub mod scfgr1;
#[doc = "Slave Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scfgr2](scfgr2) module"]
pub type SCFGR2 = crate::Reg<u32, _SCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCFGR2;
#[doc = "`read()` method returns [scfgr2::R](scfgr2::R) reader structure"]
impl crate::Readable for SCFGR2 {}
#[doc = "`write(|w| ..)` method takes [scfgr2::W](scfgr2::W) writer structure"]
impl crate::Writable for SCFGR2 {}
#[doc = "Slave Configuration Register 2"]
pub mod scfgr2;
#[doc = "Slave Address Match Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [samr](samr) module"]
pub type SAMR = crate::Reg<u32, _SAMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAMR;
#[doc = "`read()` method returns [samr::R](samr::R) reader structure"]
impl crate::Readable for SAMR {}
#[doc = "`write(|w| ..)` method takes [samr::W](samr::W) writer structure"]
impl crate::Writable for SAMR {}
#[doc = "Slave Address Match Register"]
pub mod samr;
#[doc = "Slave Address Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sasr](sasr) module"]
pub type SASR = crate::Reg<u32, _SASR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SASR;
#[doc = "`read()` method returns [sasr::R](sasr::R) reader structure"]
impl crate::Readable for SASR {}
#[doc = "Slave Address Status Register"]
pub mod sasr;
#[doc = "Slave Transmit ACK Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [star](star) module"]
pub type STAR = crate::Reg<u32, _STAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAR;
#[doc = "`read()` method returns [star::R](star::R) reader structure"]
impl crate::Readable for STAR {}
#[doc = "`write(|w| ..)` method takes [star::W](star::W) writer structure"]
impl crate::Writable for STAR {}
#[doc = "Slave Transmit ACK Register"]
pub mod star;
#[doc = "Slave Transmit Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stdr](stdr) module"]
pub type STDR = crate::Reg<u32, _STDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STDR;
#[doc = "`read()` method returns [stdr::R](stdr::R) reader structure"]
impl crate::Readable for STDR {}
#[doc = "`write(|w| ..)` method takes [stdr::W](stdr::W) writer structure"]
impl crate::Writable for STDR {}
#[doc = "Slave Transmit Data Register"]
pub mod stdr;
#[doc = "Slave Receive Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srdr](srdr) module"]
pub type SRDR = crate::Reg<u32, _SRDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRDR;
#[doc = "`read()` method returns [srdr::R](srdr::R) reader structure"]
impl crate::Readable for SRDR {}
#[doc = "Slave Receive Data Register"]
pub mod srdr;
