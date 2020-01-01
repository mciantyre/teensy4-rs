#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Control Register 0"]
    pub mcr0: MCR0,
    #[doc = "0x04 - Module Control Register 1"]
    pub mcr1: MCR1,
    #[doc = "0x08 - Module Control Register 2"]
    pub mcr2: MCR2,
    #[doc = "0x0c - AHB Bus Control Register"]
    pub ahbcr: AHBCR,
    #[doc = "0x10 - Interrupt Enable Register"]
    pub inten: INTEN,
    #[doc = "0x14 - Interrupt Register"]
    pub intr: INTR,
    #[doc = "0x18 - LUT Key Register"]
    pub lutkey: LUTKEY,
    #[doc = "0x1c - LUT Control Register"]
    pub lutcr: LUTCR,
    #[doc = "0x20 - AHB RX Buffer 0 Control Register 0"]
    pub ahbrxbuf0cr0: AHBRXBUF0CR0,
    #[doc = "0x24 - AHB RX Buffer 1 Control Register 0"]
    pub ahbrxbuf1cr0: AHBRXBUF1CR0,
    #[doc = "0x28 - AHB RX Buffer 2 Control Register 0"]
    pub ahbrxbuf2cr0: AHBRXBUF2CR0,
    #[doc = "0x2c - AHB RX Buffer 3 Control Register 0"]
    pub ahbrxbuf3cr0: AHBRXBUF3CR0,
    _reserved12: [u8; 48usize],
    #[doc = "0x60 - Flash A1 Control Register 0"]
    pub flsha1cr0: FLSHA1CR0,
    #[doc = "0x64 - Flash A2 Control Register 0"]
    pub flsha2cr0: FLSHA2CR0,
    #[doc = "0x68 - Flash B1 Control Register 0"]
    pub flshb1cr0: FLSHB1CR0,
    #[doc = "0x6c - Flash B2 Control Register 0"]
    pub flshb2cr0: FLSHB2CR0,
    #[doc = "0x70 - Flash A1 Control Register 1"]
    pub flshcr1a1: FLSHCR1,
    #[doc = "0x74 - Flash A1 Control Register 1"]
    pub flshcr1a2: FLSHCR1,
    #[doc = "0x78 - Flash A1 Control Register 1"]
    pub flshcr1b1: FLSHCR1,
    #[doc = "0x7c - Flash A1 Control Register 1"]
    pub flshcr1b2: FLSHCR1,
    #[doc = "0x80 - Flash A1 Control Register 2"]
    pub flshcr2a1: FLSHCR2,
    #[doc = "0x84 - Flash A1 Control Register 2"]
    pub flshcr2a2: FLSHCR2,
    #[doc = "0x88 - Flash A1 Control Register 2"]
    pub flshcr2b1: FLSHCR2,
    #[doc = "0x8c - Flash A1 Control Register 2"]
    pub flshcr2b2: FLSHCR2,
    _reserved24: [u8; 4usize],
    #[doc = "0x94 - Flash Control Register 4"]
    pub flshcr4: FLSHCR4,
    _reserved25: [u8; 8usize],
    #[doc = "0xa0 - IP Control Register 0"]
    pub ipcr0: IPCR0,
    #[doc = "0xa4 - IP Control Register 1"]
    pub ipcr1: IPCR1,
    _reserved27: [u8; 8usize],
    #[doc = "0xb0 - IP Command Register"]
    pub ipcmd: IPCMD,
    _reserved28: [u8; 4usize],
    #[doc = "0xb8 - IP RX FIFO Control Register"]
    pub iprxfcr: IPRXFCR,
    #[doc = "0xbc - IP TX FIFO Control Register"]
    pub iptxfcr: IPTXFCR,
    #[doc = "0xc0 - DLL Control Register 0"]
    pub dllcra: DLLCR,
    #[doc = "0xc4 - DLL Control Register 0"]
    pub dllcrb: DLLCR,
    _reserved32: [u8; 24usize],
    #[doc = "0xe0 - Status Register 0"]
    pub sts0: STS0,
    #[doc = "0xe4 - Status Register 1"]
    pub sts1: STS1,
    #[doc = "0xe8 - Status Register 2"]
    pub sts2: STS2,
    #[doc = "0xec - AHB Suspend Status Register"]
    pub ahbspndsts: AHBSPNDSTS,
    #[doc = "0xf0 - IP RX FIFO Status Register"]
    pub iprxfsts: IPRXFSTS,
    #[doc = "0xf4 - IP TX FIFO Status Register"]
    pub iptxfsts: IPTXFSTS,
    _reserved38: [u8; 8usize],
    #[doc = "0x100 - IP RX FIFO Data Register 0"]
    pub rfdr: [RFDR; 32],
    #[doc = "0x180 - IP TX FIFO Data Register 0"]
    pub tfdr: [TFDR; 32],
    #[doc = "0x200 - LUT 0"]
    pub lut: [LUT; 64],
}
#[doc = "Module Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr0](mcr0) module"]
pub type MCR0 = crate::Reg<u32, _MCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR0;
#[doc = "`read()` method returns [mcr0::R](mcr0::R) reader structure"]
impl crate::Readable for MCR0 {}
#[doc = "`write(|w| ..)` method takes [mcr0::W](mcr0::W) writer structure"]
impl crate::Writable for MCR0 {}
#[doc = "Module Control Register 0"]
pub mod mcr0;
#[doc = "Module Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr1](mcr1) module"]
pub type MCR1 = crate::Reg<u32, _MCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR1;
#[doc = "`read()` method returns [mcr1::R](mcr1::R) reader structure"]
impl crate::Readable for MCR1 {}
#[doc = "`write(|w| ..)` method takes [mcr1::W](mcr1::W) writer structure"]
impl crate::Writable for MCR1 {}
#[doc = "Module Control Register 1"]
pub mod mcr1;
#[doc = "Module Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr2](mcr2) module"]
pub type MCR2 = crate::Reg<u32, _MCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR2;
#[doc = "`read()` method returns [mcr2::R](mcr2::R) reader structure"]
impl crate::Readable for MCR2 {}
#[doc = "`write(|w| ..)` method takes [mcr2::W](mcr2::W) writer structure"]
impl crate::Writable for MCR2 {}
#[doc = "Module Control Register 2"]
pub mod mcr2;
#[doc = "AHB Bus Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbcr](ahbcr) module"]
pub type AHBCR = crate::Reg<u32, _AHBCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBCR;
#[doc = "`read()` method returns [ahbcr::R](ahbcr::R) reader structure"]
impl crate::Readable for AHBCR {}
#[doc = "`write(|w| ..)` method takes [ahbcr::W](ahbcr::W) writer structure"]
impl crate::Writable for AHBCR {}
#[doc = "AHB Bus Control Register"]
pub mod ahbcr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "Interrupt Enable Register"]
pub mod inten;
#[doc = "Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](intr) module"]
pub type INTR = crate::Reg<u32, _INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR;
#[doc = "`read()` method returns [intr::R](intr::R) reader structure"]
impl crate::Readable for INTR {}
#[doc = "`write(|w| ..)` method takes [intr::W](intr::W) writer structure"]
impl crate::Writable for INTR {}
#[doc = "Interrupt Register"]
pub mod intr;
#[doc = "LUT Key Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lutkey](lutkey) module"]
pub type LUTKEY = crate::Reg<u32, _LUTKEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LUTKEY;
#[doc = "`read()` method returns [lutkey::R](lutkey::R) reader structure"]
impl crate::Readable for LUTKEY {}
#[doc = "`write(|w| ..)` method takes [lutkey::W](lutkey::W) writer structure"]
impl crate::Writable for LUTKEY {}
#[doc = "LUT Key Register"]
pub mod lutkey;
#[doc = "LUT Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lutcr](lutcr) module"]
pub type LUTCR = crate::Reg<u32, _LUTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LUTCR;
#[doc = "`read()` method returns [lutcr::R](lutcr::R) reader structure"]
impl crate::Readable for LUTCR {}
#[doc = "`write(|w| ..)` method takes [lutcr::W](lutcr::W) writer structure"]
impl crate::Writable for LUTCR {}
#[doc = "LUT Control Register"]
pub mod lutcr;
#[doc = "AHB RX Buffer 0 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbrxbuf0cr0](ahbrxbuf0cr0) module"]
pub type AHBRXBUF0CR0 = crate::Reg<u32, _AHBRXBUF0CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBRXBUF0CR0;
#[doc = "`read()` method returns [ahbrxbuf0cr0::R](ahbrxbuf0cr0::R) reader structure"]
impl crate::Readable for AHBRXBUF0CR0 {}
#[doc = "`write(|w| ..)` method takes [ahbrxbuf0cr0::W](ahbrxbuf0cr0::W) writer structure"]
impl crate::Writable for AHBRXBUF0CR0 {}
#[doc = "AHB RX Buffer 0 Control Register 0"]
pub mod ahbrxbuf0cr0;
#[doc = "AHB RX Buffer 1 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbrxbuf1cr0](ahbrxbuf1cr0) module"]
pub type AHBRXBUF1CR0 = crate::Reg<u32, _AHBRXBUF1CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBRXBUF1CR0;
#[doc = "`read()` method returns [ahbrxbuf1cr0::R](ahbrxbuf1cr0::R) reader structure"]
impl crate::Readable for AHBRXBUF1CR0 {}
#[doc = "`write(|w| ..)` method takes [ahbrxbuf1cr0::W](ahbrxbuf1cr0::W) writer structure"]
impl crate::Writable for AHBRXBUF1CR0 {}
#[doc = "AHB RX Buffer 1 Control Register 0"]
pub mod ahbrxbuf1cr0;
#[doc = "AHB RX Buffer 2 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbrxbuf2cr0](ahbrxbuf2cr0) module"]
pub type AHBRXBUF2CR0 = crate::Reg<u32, _AHBRXBUF2CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBRXBUF2CR0;
#[doc = "`read()` method returns [ahbrxbuf2cr0::R](ahbrxbuf2cr0::R) reader structure"]
impl crate::Readable for AHBRXBUF2CR0 {}
#[doc = "`write(|w| ..)` method takes [ahbrxbuf2cr0::W](ahbrxbuf2cr0::W) writer structure"]
impl crate::Writable for AHBRXBUF2CR0 {}
#[doc = "AHB RX Buffer 2 Control Register 0"]
pub mod ahbrxbuf2cr0;
#[doc = "AHB RX Buffer 3 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbrxbuf3cr0](ahbrxbuf3cr0) module"]
pub type AHBRXBUF3CR0 = crate::Reg<u32, _AHBRXBUF3CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBRXBUF3CR0;
#[doc = "`read()` method returns [ahbrxbuf3cr0::R](ahbrxbuf3cr0::R) reader structure"]
impl crate::Readable for AHBRXBUF3CR0 {}
#[doc = "`write(|w| ..)` method takes [ahbrxbuf3cr0::W](ahbrxbuf3cr0::W) writer structure"]
impl crate::Writable for AHBRXBUF3CR0 {}
#[doc = "AHB RX Buffer 3 Control Register 0"]
pub mod ahbrxbuf3cr0;
#[doc = "Flash A1 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flsha1cr0](flsha1cr0) module"]
pub type FLSHA1CR0 = crate::Reg<u32, _FLSHA1CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLSHA1CR0;
#[doc = "`read()` method returns [flsha1cr0::R](flsha1cr0::R) reader structure"]
impl crate::Readable for FLSHA1CR0 {}
#[doc = "`write(|w| ..)` method takes [flsha1cr0::W](flsha1cr0::W) writer structure"]
impl crate::Writable for FLSHA1CR0 {}
#[doc = "Flash A1 Control Register 0"]
pub mod flsha1cr0;
#[doc = "Flash A2 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flsha2cr0](flsha2cr0) module"]
pub type FLSHA2CR0 = crate::Reg<u32, _FLSHA2CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLSHA2CR0;
#[doc = "`read()` method returns [flsha2cr0::R](flsha2cr0::R) reader structure"]
impl crate::Readable for FLSHA2CR0 {}
#[doc = "`write(|w| ..)` method takes [flsha2cr0::W](flsha2cr0::W) writer structure"]
impl crate::Writable for FLSHA2CR0 {}
#[doc = "Flash A2 Control Register 0"]
pub mod flsha2cr0;
#[doc = "Flash B1 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flshb1cr0](flshb1cr0) module"]
pub type FLSHB1CR0 = crate::Reg<u32, _FLSHB1CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLSHB1CR0;
#[doc = "`read()` method returns [flshb1cr0::R](flshb1cr0::R) reader structure"]
impl crate::Readable for FLSHB1CR0 {}
#[doc = "`write(|w| ..)` method takes [flshb1cr0::W](flshb1cr0::W) writer structure"]
impl crate::Writable for FLSHB1CR0 {}
#[doc = "Flash B1 Control Register 0"]
pub mod flshb1cr0;
#[doc = "Flash B2 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flshb2cr0](flshb2cr0) module"]
pub type FLSHB2CR0 = crate::Reg<u32, _FLSHB2CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLSHB2CR0;
#[doc = "`read()` method returns [flshb2cr0::R](flshb2cr0::R) reader structure"]
impl crate::Readable for FLSHB2CR0 {}
#[doc = "`write(|w| ..)` method takes [flshb2cr0::W](flshb2cr0::W) writer structure"]
impl crate::Writable for FLSHB2CR0 {}
#[doc = "Flash B2 Control Register 0"]
pub mod flshb2cr0;
#[doc = "Flash A1 Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flshcr1](flshcr1) module"]
pub type FLSHCR1 = crate::Reg<u32, _FLSHCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLSHCR1;
#[doc = "`read()` method returns [flshcr1::R](flshcr1::R) reader structure"]
impl crate::Readable for FLSHCR1 {}
#[doc = "`write(|w| ..)` method takes [flshcr1::W](flshcr1::W) writer structure"]
impl crate::Writable for FLSHCR1 {}
#[doc = "Flash A1 Control Register 1"]
pub mod flshcr1;
#[doc = "Flash A1 Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flshcr2](flshcr2) module"]
pub type FLSHCR2 = crate::Reg<u32, _FLSHCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLSHCR2;
#[doc = "`read()` method returns [flshcr2::R](flshcr2::R) reader structure"]
impl crate::Readable for FLSHCR2 {}
#[doc = "`write(|w| ..)` method takes [flshcr2::W](flshcr2::W) writer structure"]
impl crate::Writable for FLSHCR2 {}
#[doc = "Flash A1 Control Register 2"]
pub mod flshcr2;
#[doc = "Flash Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flshcr4](flshcr4) module"]
pub type FLSHCR4 = crate::Reg<u32, _FLSHCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLSHCR4;
#[doc = "`read()` method returns [flshcr4::R](flshcr4::R) reader structure"]
impl crate::Readable for FLSHCR4 {}
#[doc = "`write(|w| ..)` method takes [flshcr4::W](flshcr4::W) writer structure"]
impl crate::Writable for FLSHCR4 {}
#[doc = "Flash Control Register 4"]
pub mod flshcr4;
#[doc = "IP Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcr0](ipcr0) module"]
pub type IPCR0 = crate::Reg<u32, _IPCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPCR0;
#[doc = "`read()` method returns [ipcr0::R](ipcr0::R) reader structure"]
impl crate::Readable for IPCR0 {}
#[doc = "`write(|w| ..)` method takes [ipcr0::W](ipcr0::W) writer structure"]
impl crate::Writable for IPCR0 {}
#[doc = "IP Control Register 0"]
pub mod ipcr0;
#[doc = "IP Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcr1](ipcr1) module"]
pub type IPCR1 = crate::Reg<u32, _IPCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPCR1;
#[doc = "`read()` method returns [ipcr1::R](ipcr1::R) reader structure"]
impl crate::Readable for IPCR1 {}
#[doc = "`write(|w| ..)` method takes [ipcr1::W](ipcr1::W) writer structure"]
impl crate::Writable for IPCR1 {}
#[doc = "IP Control Register 1"]
pub mod ipcr1;
#[doc = "IP Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcmd](ipcmd) module"]
pub type IPCMD = crate::Reg<u32, _IPCMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPCMD;
#[doc = "`read()` method returns [ipcmd::R](ipcmd::R) reader structure"]
impl crate::Readable for IPCMD {}
#[doc = "`write(|w| ..)` method takes [ipcmd::W](ipcmd::W) writer structure"]
impl crate::Writable for IPCMD {}
#[doc = "IP Command Register"]
pub mod ipcmd;
#[doc = "IP RX FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iprxfcr](iprxfcr) module"]
pub type IPRXFCR = crate::Reg<u32, _IPRXFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRXFCR;
#[doc = "`read()` method returns [iprxfcr::R](iprxfcr::R) reader structure"]
impl crate::Readable for IPRXFCR {}
#[doc = "`write(|w| ..)` method takes [iprxfcr::W](iprxfcr::W) writer structure"]
impl crate::Writable for IPRXFCR {}
#[doc = "IP RX FIFO Control Register"]
pub mod iprxfcr;
#[doc = "IP TX FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iptxfcr](iptxfcr) module"]
pub type IPTXFCR = crate::Reg<u32, _IPTXFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPTXFCR;
#[doc = "`read()` method returns [iptxfcr::R](iptxfcr::R) reader structure"]
impl crate::Readable for IPTXFCR {}
#[doc = "`write(|w| ..)` method takes [iptxfcr::W](iptxfcr::W) writer structure"]
impl crate::Writable for IPTXFCR {}
#[doc = "IP TX FIFO Control Register"]
pub mod iptxfcr;
#[doc = "DLL Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dllcr](dllcr) module"]
pub type DLLCR = crate::Reg<u32, _DLLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLLCR;
#[doc = "`read()` method returns [dllcr::R](dllcr::R) reader structure"]
impl crate::Readable for DLLCR {}
#[doc = "`write(|w| ..)` method takes [dllcr::W](dllcr::W) writer structure"]
impl crate::Writable for DLLCR {}
#[doc = "DLL Control Register 0"]
pub mod dllcr;
#[doc = "Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts0](sts0) module"]
pub type STS0 = crate::Reg<u32, _STS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STS0;
#[doc = "`read()` method returns [sts0::R](sts0::R) reader structure"]
impl crate::Readable for STS0 {}
#[doc = "Status Register 0"]
pub mod sts0;
#[doc = "Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts1](sts1) module"]
pub type STS1 = crate::Reg<u32, _STS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STS1;
#[doc = "`read()` method returns [sts1::R](sts1::R) reader structure"]
impl crate::Readable for STS1 {}
#[doc = "Status Register 1"]
pub mod sts1;
#[doc = "Status Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts2](sts2) module"]
pub type STS2 = crate::Reg<u32, _STS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STS2;
#[doc = "`read()` method returns [sts2::R](sts2::R) reader structure"]
impl crate::Readable for STS2 {}
#[doc = "Status Register 2"]
pub mod sts2;
#[doc = "AHB Suspend Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbspndsts](ahbspndsts) module"]
pub type AHBSPNDSTS = crate::Reg<u32, _AHBSPNDSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBSPNDSTS;
#[doc = "`read()` method returns [ahbspndsts::R](ahbspndsts::R) reader structure"]
impl crate::Readable for AHBSPNDSTS {}
#[doc = "AHB Suspend Status Register"]
pub mod ahbspndsts;
#[doc = "IP RX FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iprxfsts](iprxfsts) module"]
pub type IPRXFSTS = crate::Reg<u32, _IPRXFSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRXFSTS;
#[doc = "`read()` method returns [iprxfsts::R](iprxfsts::R) reader structure"]
impl crate::Readable for IPRXFSTS {}
#[doc = "IP RX FIFO Status Register"]
pub mod iprxfsts;
#[doc = "IP TX FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iptxfsts](iptxfsts) module"]
pub type IPTXFSTS = crate::Reg<u32, _IPTXFSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPTXFSTS;
#[doc = "`read()` method returns [iptxfsts::R](iptxfsts::R) reader structure"]
impl crate::Readable for IPTXFSTS {}
#[doc = "IP TX FIFO Status Register"]
pub mod iptxfsts;
#[doc = "IP RX FIFO Data Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfdr](rfdr) module"]
pub type RFDR = crate::Reg<u32, _RFDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFDR;
#[doc = "`read()` method returns [rfdr::R](rfdr::R) reader structure"]
impl crate::Readable for RFDR {}
#[doc = "IP RX FIFO Data Register 0"]
pub mod rfdr;
#[doc = "IP TX FIFO Data Register 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfdr](tfdr) module"]
pub type TFDR = crate::Reg<u32, _TFDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFDR;
#[doc = "`write(|w| ..)` method takes [tfdr::W](tfdr::W) writer structure"]
impl crate::Writable for TFDR {}
#[doc = "IP TX FIFO Data Register 0"]
pub mod tfdr;
#[doc = "LUT 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lut](lut) module"]
pub type LUT = crate::Reg<u32, _LUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LUT;
#[doc = "`read()` method returns [lut::R](lut::R) reader structure"]
impl crate::Readable for LUT {}
#[doc = "`write(|w| ..)` method takes [lut::W](lut::W) writer structure"]
impl crate::Writable for LUT {}
#[doc = "LUT 0"]
pub mod lut;
