#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Configuration Register"]
    pub mcr: MCR,
    #[doc = "0x04 - Control 1 Register"]
    pub ctrl1: CTRL1,
    #[doc = "0x08 - Free Running Timer Register"]
    pub timer: TIMER,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Rx Mailboxes Global Mask Register"]
    pub rxmgmask: RXMGMASK,
    #[doc = "0x14 - Rx Buffer 14 Mask Register"]
    pub rx14mask: RX14MASK,
    #[doc = "0x18 - Rx Buffer 15 Mask Register"]
    pub rx15mask: RX15MASK,
    #[doc = "0x1c - Error Counter Register"]
    pub ecr: ECR,
    #[doc = "0x20 - Error and Status 1 Register"]
    pub esr1: ESR1,
    #[doc = "0x24 - Interrupt Masks 2 Register"]
    pub imask2: IMASK2,
    #[doc = "0x28 - Interrupt Masks 1 Register"]
    pub imask1: IMASK1,
    #[doc = "0x2c - Interrupt Flags 2 Register"]
    pub iflag2: IFLAG2,
    #[doc = "0x30 - Interrupt Flags 1 Register"]
    pub iflag1: IFLAG1,
    #[doc = "0x34 - Control 2 Register"]
    pub ctrl2: CTRL2,
    #[doc = "0x38 - Error and Status 2 Register"]
    pub esr2: ESR2,
    _reserved14: [u8; 8usize],
    #[doc = "0x44 - CRC Register"]
    pub crcr: CRCR,
    #[doc = "0x48 - Rx FIFO Global Mask Register"]
    pub rxfgmask: RXFGMASK,
    #[doc = "0x4c - Rx FIFO Information Register"]
    pub rxfir: RXFIR,
    _reserved17: [u8; 8usize],
    #[doc = "0x58 - Debug 1 register"]
    pub dbg1: DBG1,
    #[doc = "0x5c - Debug 2 register"]
    pub dbg2: DBG2,
    _reserved19: [u8; 2080usize],
    #[doc = "0x880 - Rx Individual Mask Registers"]
    pub rximr: [RXIMR; 64],
    _reserved20: [u8; 96usize],
    #[doc = "0x9e0 - Glitch Filter Width Registers"]
    pub gfwr: GFWR,
}
#[doc = "Module Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](mcr) module"]
pub type MCR = crate::Reg<u32, _MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR;
#[doc = "`read()` method returns [mcr::R](mcr::R) reader structure"]
impl crate::Readable for MCR {}
#[doc = "`write(|w| ..)` method takes [mcr::W](mcr::W) writer structure"]
impl crate::Writable for MCR {}
#[doc = "Module Configuration Register"]
pub mod mcr;
#[doc = "Control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1](ctrl1) module"]
pub type CTRL1 = crate::Reg<u32, _CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL1;
#[doc = "`read()` method returns [ctrl1::R](ctrl1::R) reader structure"]
impl crate::Readable for CTRL1 {}
#[doc = "`write(|w| ..)` method takes [ctrl1::W](ctrl1::W) writer structure"]
impl crate::Writable for CTRL1 {}
#[doc = "Control 1 Register"]
pub mod ctrl1;
#[doc = "Free Running Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer](timer) module"]
pub type TIMER = crate::Reg<u32, _TIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER;
#[doc = "`read()` method returns [timer::R](timer::R) reader structure"]
impl crate::Readable for TIMER {}
#[doc = "`write(|w| ..)` method takes [timer::W](timer::W) writer structure"]
impl crate::Writable for TIMER {}
#[doc = "Free Running Timer Register"]
pub mod timer;
#[doc = "Rx Mailboxes Global Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmgmask](rxmgmask) module"]
pub type RXMGMASK = crate::Reg<u32, _RXMGMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXMGMASK;
#[doc = "`read()` method returns [rxmgmask::R](rxmgmask::R) reader structure"]
impl crate::Readable for RXMGMASK {}
#[doc = "`write(|w| ..)` method takes [rxmgmask::W](rxmgmask::W) writer structure"]
impl crate::Writable for RXMGMASK {}
#[doc = "Rx Mailboxes Global Mask Register"]
pub mod rxmgmask;
#[doc = "Rx Buffer 14 Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx14mask](rx14mask) module"]
pub type RX14MASK = crate::Reg<u32, _RX14MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX14MASK;
#[doc = "`read()` method returns [rx14mask::R](rx14mask::R) reader structure"]
impl crate::Readable for RX14MASK {}
#[doc = "`write(|w| ..)` method takes [rx14mask::W](rx14mask::W) writer structure"]
impl crate::Writable for RX14MASK {}
#[doc = "Rx Buffer 14 Mask Register"]
pub mod rx14mask;
#[doc = "Rx Buffer 15 Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx15mask](rx15mask) module"]
pub type RX15MASK = crate::Reg<u32, _RX15MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX15MASK;
#[doc = "`read()` method returns [rx15mask::R](rx15mask::R) reader structure"]
impl crate::Readable for RX15MASK {}
#[doc = "`write(|w| ..)` method takes [rx15mask::W](rx15mask::W) writer structure"]
impl crate::Writable for RX15MASK {}
#[doc = "Rx Buffer 15 Mask Register"]
pub mod rx15mask;
#[doc = "Error Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecr](ecr) module"]
pub type ECR = crate::Reg<u32, _ECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECR;
#[doc = "`read()` method returns [ecr::R](ecr::R) reader structure"]
impl crate::Readable for ECR {}
#[doc = "`write(|w| ..)` method takes [ecr::W](ecr::W) writer structure"]
impl crate::Writable for ECR {}
#[doc = "Error Counter Register"]
pub mod ecr;
#[doc = "Error and Status 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esr1](esr1) module"]
pub type ESR1 = crate::Reg<u32, _ESR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESR1;
#[doc = "`read()` method returns [esr1::R](esr1::R) reader structure"]
impl crate::Readable for ESR1 {}
#[doc = "`write(|w| ..)` method takes [esr1::W](esr1::W) writer structure"]
impl crate::Writable for ESR1 {}
#[doc = "Error and Status 1 Register"]
pub mod esr1;
#[doc = "Interrupt Masks 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imask2](imask2) module"]
pub type IMASK2 = crate::Reg<u32, _IMASK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMASK2;
#[doc = "`read()` method returns [imask2::R](imask2::R) reader structure"]
impl crate::Readable for IMASK2 {}
#[doc = "`write(|w| ..)` method takes [imask2::W](imask2::W) writer structure"]
impl crate::Writable for IMASK2 {}
#[doc = "Interrupt Masks 2 Register"]
pub mod imask2;
#[doc = "Interrupt Masks 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imask1](imask1) module"]
pub type IMASK1 = crate::Reg<u32, _IMASK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMASK1;
#[doc = "`read()` method returns [imask1::R](imask1::R) reader structure"]
impl crate::Readable for IMASK1 {}
#[doc = "`write(|w| ..)` method takes [imask1::W](imask1::W) writer structure"]
impl crate::Writable for IMASK1 {}
#[doc = "Interrupt Masks 1 Register"]
pub mod imask1;
#[doc = "Interrupt Flags 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iflag2](iflag2) module"]
pub type IFLAG2 = crate::Reg<u32, _IFLAG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFLAG2;
#[doc = "`read()` method returns [iflag2::R](iflag2::R) reader structure"]
impl crate::Readable for IFLAG2 {}
#[doc = "`write(|w| ..)` method takes [iflag2::W](iflag2::W) writer structure"]
impl crate::Writable for IFLAG2 {}
#[doc = "Interrupt Flags 2 Register"]
pub mod iflag2;
#[doc = "Interrupt Flags 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iflag1](iflag1) module"]
pub type IFLAG1 = crate::Reg<u32, _IFLAG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFLAG1;
#[doc = "`read()` method returns [iflag1::R](iflag1::R) reader structure"]
impl crate::Readable for IFLAG1 {}
#[doc = "`write(|w| ..)` method takes [iflag1::W](iflag1::W) writer structure"]
impl crate::Writable for IFLAG1 {}
#[doc = "Interrupt Flags 1 Register"]
pub mod iflag1;
#[doc = "Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](ctrl2) module"]
pub type CTRL2 = crate::Reg<u32, _CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL2;
#[doc = "`read()` method returns [ctrl2::R](ctrl2::R) reader structure"]
impl crate::Readable for CTRL2 {}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](ctrl2::W) writer structure"]
impl crate::Writable for CTRL2 {}
#[doc = "Control 2 Register"]
pub mod ctrl2;
#[doc = "Error and Status 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esr2](esr2) module"]
pub type ESR2 = crate::Reg<u32, _ESR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESR2;
#[doc = "`read()` method returns [esr2::R](esr2::R) reader structure"]
impl crate::Readable for ESR2 {}
#[doc = "Error and Status 2 Register"]
pub mod esr2;
#[doc = "CRC Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcr](crcr) module"]
pub type CRCR = crate::Reg<u32, _CRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCR;
#[doc = "`read()` method returns [crcr::R](crcr::R) reader structure"]
impl crate::Readable for CRCR {}
#[doc = "CRC Register"]
pub mod crcr;
#[doc = "Rx FIFO Global Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfgmask](rxfgmask) module"]
pub type RXFGMASK = crate::Reg<u32, _RXFGMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFGMASK;
#[doc = "`read()` method returns [rxfgmask::R](rxfgmask::R) reader structure"]
impl crate::Readable for RXFGMASK {}
#[doc = "`write(|w| ..)` method takes [rxfgmask::W](rxfgmask::W) writer structure"]
impl crate::Writable for RXFGMASK {}
#[doc = "Rx FIFO Global Mask Register"]
pub mod rxfgmask;
#[doc = "Rx FIFO Information Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfir](rxfir) module"]
pub type RXFIR = crate::Reg<u32, _RXFIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFIR;
#[doc = "`read()` method returns [rxfir::R](rxfir::R) reader structure"]
impl crate::Readable for RXFIR {}
#[doc = "Rx FIFO Information Register"]
pub mod rxfir;
#[doc = "Debug 1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg1](dbg1) module"]
pub type DBG1 = crate::Reg<u32, _DBG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBG1;
#[doc = "`read()` method returns [dbg1::R](dbg1::R) reader structure"]
impl crate::Readable for DBG1 {}
#[doc = "Debug 1 register"]
pub mod dbg1;
#[doc = "Debug 2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg2](dbg2) module"]
pub type DBG2 = crate::Reg<u32, _DBG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBG2;
#[doc = "`read()` method returns [dbg2::R](dbg2::R) reader structure"]
impl crate::Readable for DBG2 {}
#[doc = "Debug 2 register"]
pub mod dbg2;
#[doc = "Rx Individual Mask Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rximr](rximr) module"]
pub type RXIMR = crate::Reg<u32, _RXIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIMR;
#[doc = "`read()` method returns [rximr::R](rximr::R) reader structure"]
impl crate::Readable for RXIMR {}
#[doc = "`write(|w| ..)` method takes [rximr::W](rximr::W) writer structure"]
impl crate::Writable for RXIMR {}
#[doc = "Rx Individual Mask Registers"]
pub mod rximr;
#[doc = "Glitch Filter Width Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gfwr](gfwr) module"]
pub type GFWR = crate::Reg<u32, _GFWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GFWR;
#[doc = "`read()` method returns [gfwr::R](gfwr::R) reader structure"]
impl crate::Readable for GFWR {}
#[doc = "`write(|w| ..)` method takes [gfwr::W](gfwr::W) writer structure"]
impl crate::Writable for GFWR {}
#[doc = "Glitch Filter Width Registers"]
pub mod gfwr;
