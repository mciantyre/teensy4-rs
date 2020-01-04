#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Control Register"]
    pub mcr: MCR,
    #[doc = "0x04 - IO Mux Control Register"]
    pub iocr: IOCR,
    #[doc = "0x08 - Master Bus (AXI) Control Register 0"]
    pub bmcr0: BMCR0,
    #[doc = "0x0c - Master Bus (AXI) Control Register 1"]
    pub bmcr1: BMCR1,
    #[doc = "0x10 - Base Register 0 (For SDRAM CS0 device)"]
    pub br0: BR0,
    #[doc = "0x14 - Base Register 1 (For SDRAM CS1 device)"]
    pub br1: BR1,
    #[doc = "0x18 - Base Register 2 (For SDRAM CS2 device)"]
    pub br2: BR2,
    #[doc = "0x1c - Base Register 3 (For SDRAM CS3 device)"]
    pub br3: BR3,
    #[doc = "0x20 - Base Register 4 (For NAND device)"]
    pub br4: BR4,
    #[doc = "0x24 - Base Register 5 (For NOR device)"]
    pub br5: BR5,
    #[doc = "0x28 - Base Register 6 (For PSRAM device)"]
    pub br6: BR6,
    #[doc = "0x2c - Base Register 7 (For DBI-B (MIPI Display Bus Interface Type B) device)"]
    pub br7: BR7,
    #[doc = "0x30 - Base Register 8 (For NAND device)"]
    pub br8: BR8,
    #[doc = "0x34 - DLL Control Register"]
    pub dllcr: DLLCR,
    #[doc = "0x38 - Interrupt Enable Register"]
    pub inten: INTEN,
    #[doc = "0x3c - Interrupt Enable Register"]
    pub intr: INTR,
    #[doc = "0x40 - SDRAM control register 0"]
    pub sdramcr0: SDRAMCR0,
    #[doc = "0x44 - SDRAM control register 1"]
    pub sdramcr1: SDRAMCR1,
    #[doc = "0x48 - SDRAM control register 2"]
    pub sdramcr2: SDRAMCR2,
    #[doc = "0x4c - SDRAM control register 3"]
    pub sdramcr3: SDRAMCR3,
    #[doc = "0x50 - NAND control register 0"]
    pub nandcr0: NANDCR0,
    #[doc = "0x54 - NAND control register 1"]
    pub nandcr1: NANDCR1,
    #[doc = "0x58 - NAND control register 2"]
    pub nandcr2: NANDCR2,
    #[doc = "0x5c - NAND control register 3"]
    pub nandcr3: NANDCR3,
    #[doc = "0x60 - NOR control register 0"]
    pub norcr0: NORCR0,
    #[doc = "0x64 - NOR control register 1"]
    pub norcr1: NORCR1,
    #[doc = "0x68 - NOR control register 2"]
    pub norcr2: NORCR2,
    #[doc = "0x6c - NOR control register 3"]
    pub norcr3: NORCR3,
    #[doc = "0x70 - SRAM control register 0"]
    pub sramcr0: SRAMCR0,
    #[doc = "0x74 - SRAM control register 1"]
    pub sramcr1: SRAMCR1,
    #[doc = "0x78 - SRAM control register 2"]
    pub sramcr2: SRAMCR2,
    #[doc = "0x7c - SRAM control register 3"]
    pub sramcr3: SRAMCR3,
    #[doc = "0x80 - DBI-B control register 0"]
    pub dbicr0: DBICR0,
    #[doc = "0x84 - DBI-B control register 1"]
    pub dbicr1: DBICR1,
    _reserved34: [u8; 8usize],
    #[doc = "0x90 - IP Command control register 0"]
    pub ipcr0: IPCR0,
    #[doc = "0x94 - IP Command control register 1"]
    pub ipcr1: IPCR1,
    #[doc = "0x98 - IP Command control register 2"]
    pub ipcr2: IPCR2,
    #[doc = "0x9c - IP Command register"]
    pub ipcmd: IPCMD,
    #[doc = "0xa0 - TX DATA register (for IP Command)"]
    pub iptxdat: IPTXDAT,
    _reserved39: [u8; 12usize],
    #[doc = "0xb0 - RX DATA register (for IP Command)"]
    pub iprxdat: IPRXDAT,
    _reserved40: [u8; 12usize],
    #[doc = "0xc0 - Status register 0"]
    pub sts0: STS0,
    #[doc = "0xc4 - Status register 1"]
    pub sts1: STS1,
    #[doc = "0xc8 - Status register 2"]
    pub sts2: STS2,
    #[doc = "0xcc - Status register 3"]
    pub sts3: STS3,
    #[doc = "0xd0 - Status register 4"]
    pub sts4: STS4,
    #[doc = "0xd4 - Status register 5"]
    pub sts5: STS5,
    #[doc = "0xd8 - Status register 6"]
    pub sts6: STS6,
    #[doc = "0xdc - Status register 7"]
    pub sts7: STS7,
    #[doc = "0xe0 - Status register 8"]
    pub sts8: STS8,
    #[doc = "0xe4 - Status register 9"]
    pub sts9: STS9,
    #[doc = "0xe8 - Status register 10"]
    pub sts10: STS10,
    #[doc = "0xec - Status register 11"]
    pub sts11: STS11,
    #[doc = "0xf0 - Status register 12"]
    pub sts12: STS12,
    #[doc = "0xf4 - Status register 13"]
    pub sts13: STS13,
    #[doc = "0xf8 - Status register 14"]
    pub sts14: STS14,
    #[doc = "0xfc - Status register 15"]
    pub sts15: STS15,
}
#[doc = "Module Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](mcr) module"]
pub type MCR = crate::Reg<u32, _MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR;
#[doc = "`read()` method returns [mcr::R](mcr::R) reader structure"]
impl crate::Readable for MCR {}
#[doc = "`write(|w| ..)` method takes [mcr::W](mcr::W) writer structure"]
impl crate::Writable for MCR {}
#[doc = "Module Control Register"]
pub mod mcr;
#[doc = "IO Mux Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocr](iocr) module"]
pub type IOCR = crate::Reg<u32, _IOCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCR;
#[doc = "`read()` method returns [iocr::R](iocr::R) reader structure"]
impl crate::Readable for IOCR {}
#[doc = "`write(|w| ..)` method takes [iocr::W](iocr::W) writer structure"]
impl crate::Writable for IOCR {}
#[doc = "IO Mux Control Register"]
pub mod iocr;
#[doc = "Master Bus (AXI) Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmcr0](bmcr0) module"]
pub type BMCR0 = crate::Reg<u32, _BMCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMCR0;
#[doc = "`read()` method returns [bmcr0::R](bmcr0::R) reader structure"]
impl crate::Readable for BMCR0 {}
#[doc = "`write(|w| ..)` method takes [bmcr0::W](bmcr0::W) writer structure"]
impl crate::Writable for BMCR0 {}
#[doc = "Master Bus (AXI) Control Register 0"]
pub mod bmcr0;
#[doc = "Master Bus (AXI) Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmcr1](bmcr1) module"]
pub type BMCR1 = crate::Reg<u32, _BMCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMCR1;
#[doc = "`read()` method returns [bmcr1::R](bmcr1::R) reader structure"]
impl crate::Readable for BMCR1 {}
#[doc = "`write(|w| ..)` method takes [bmcr1::W](bmcr1::W) writer structure"]
impl crate::Writable for BMCR1 {}
#[doc = "Master Bus (AXI) Control Register 1"]
pub mod bmcr1;
#[doc = "Base Register 0 (For SDRAM CS0 device)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [br0](br0) module"]
pub type BR0 = crate::Reg<u32, _BR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BR0;
#[doc = "`read()` method returns [br0::R](br0::R) reader structure"]
impl crate::Readable for BR0 {}
#[doc = "`write(|w| ..)` method takes [br0::W](br0::W) writer structure"]
impl crate::Writable for BR0 {}
#[doc = "Base Register 0 (For SDRAM CS0 device)"]
pub mod br0;
#[doc = "Base Register 1 (For SDRAM CS1 device)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [br1](br1) module"]
pub type BR1 = crate::Reg<u32, _BR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BR1;
#[doc = "`read()` method returns [br1::R](br1::R) reader structure"]
impl crate::Readable for BR1 {}
#[doc = "`write(|w| ..)` method takes [br1::W](br1::W) writer structure"]
impl crate::Writable for BR1 {}
#[doc = "Base Register 1 (For SDRAM CS1 device)"]
pub mod br1;
#[doc = "Base Register 2 (For SDRAM CS2 device)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [br2](br2) module"]
pub type BR2 = crate::Reg<u32, _BR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BR2;
#[doc = "`read()` method returns [br2::R](br2::R) reader structure"]
impl crate::Readable for BR2 {}
#[doc = "`write(|w| ..)` method takes [br2::W](br2::W) writer structure"]
impl crate::Writable for BR2 {}
#[doc = "Base Register 2 (For SDRAM CS2 device)"]
pub mod br2;
#[doc = "Base Register 3 (For SDRAM CS3 device)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [br3](br3) module"]
pub type BR3 = crate::Reg<u32, _BR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BR3;
#[doc = "`read()` method returns [br3::R](br3::R) reader structure"]
impl crate::Readable for BR3 {}
#[doc = "`write(|w| ..)` method takes [br3::W](br3::W) writer structure"]
impl crate::Writable for BR3 {}
#[doc = "Base Register 3 (For SDRAM CS3 device)"]
pub mod br3;
#[doc = "Base Register 4 (For NAND device)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [br4](br4) module"]
pub type BR4 = crate::Reg<u32, _BR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BR4;
#[doc = "`read()` method returns [br4::R](br4::R) reader structure"]
impl crate::Readable for BR4 {}
#[doc = "`write(|w| ..)` method takes [br4::W](br4::W) writer structure"]
impl crate::Writable for BR4 {}
#[doc = "Base Register 4 (For NAND device)"]
pub mod br4;
#[doc = "Base Register 5 (For NOR device)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [br5](br5) module"]
pub type BR5 = crate::Reg<u32, _BR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BR5;
#[doc = "`read()` method returns [br5::R](br5::R) reader structure"]
impl crate::Readable for BR5 {}
#[doc = "`write(|w| ..)` method takes [br5::W](br5::W) writer structure"]
impl crate::Writable for BR5 {}
#[doc = "Base Register 5 (For NOR device)"]
pub mod br5;
#[doc = "Base Register 6 (For PSRAM device)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [br6](br6) module"]
pub type BR6 = crate::Reg<u32, _BR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BR6;
#[doc = "`read()` method returns [br6::R](br6::R) reader structure"]
impl crate::Readable for BR6 {}
#[doc = "`write(|w| ..)` method takes [br6::W](br6::W) writer structure"]
impl crate::Writable for BR6 {}
#[doc = "Base Register 6 (For PSRAM device)"]
pub mod br6;
#[doc = "Base Register 7 (For DBI-B (MIPI Display Bus Interface Type B) device)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [br7](br7) module"]
pub type BR7 = crate::Reg<u32, _BR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BR7;
#[doc = "`read()` method returns [br7::R](br7::R) reader structure"]
impl crate::Readable for BR7 {}
#[doc = "`write(|w| ..)` method takes [br7::W](br7::W) writer structure"]
impl crate::Writable for BR7 {}
#[doc = "Base Register 7 (For DBI-B (MIPI Display Bus Interface Type B) device)"]
pub mod br7;
#[doc = "Base Register 8 (For NAND device)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [br8](br8) module"]
pub type BR8 = crate::Reg<u32, _BR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BR8;
#[doc = "`read()` method returns [br8::R](br8::R) reader structure"]
impl crate::Readable for BR8 {}
#[doc = "`write(|w| ..)` method takes [br8::W](br8::W) writer structure"]
impl crate::Writable for BR8 {}
#[doc = "Base Register 8 (For NAND device)"]
pub mod br8;
#[doc = "DLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dllcr](dllcr) module"]
pub type DLLCR = crate::Reg<u32, _DLLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLLCR;
#[doc = "`read()` method returns [dllcr::R](dllcr::R) reader structure"]
impl crate::Readable for DLLCR {}
#[doc = "`write(|w| ..)` method takes [dllcr::W](dllcr::W) writer structure"]
impl crate::Writable for DLLCR {}
#[doc = "DLL Control Register"]
pub mod dllcr;
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
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](intr) module"]
pub type INTR = crate::Reg<u32, _INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR;
#[doc = "`read()` method returns [intr::R](intr::R) reader structure"]
impl crate::Readable for INTR {}
#[doc = "`write(|w| ..)` method takes [intr::W](intr::W) writer structure"]
impl crate::Writable for INTR {}
#[doc = "Interrupt Enable Register"]
pub mod intr;
#[doc = "SDRAM control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdramcr0](sdramcr0) module"]
pub type SDRAMCR0 = crate::Reg<u32, _SDRAMCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDRAMCR0;
#[doc = "`read()` method returns [sdramcr0::R](sdramcr0::R) reader structure"]
impl crate::Readable for SDRAMCR0 {}
#[doc = "`write(|w| ..)` method takes [sdramcr0::W](sdramcr0::W) writer structure"]
impl crate::Writable for SDRAMCR0 {}
#[doc = "SDRAM control register 0"]
pub mod sdramcr0;
#[doc = "SDRAM control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdramcr1](sdramcr1) module"]
pub type SDRAMCR1 = crate::Reg<u32, _SDRAMCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDRAMCR1;
#[doc = "`read()` method returns [sdramcr1::R](sdramcr1::R) reader structure"]
impl crate::Readable for SDRAMCR1 {}
#[doc = "`write(|w| ..)` method takes [sdramcr1::W](sdramcr1::W) writer structure"]
impl crate::Writable for SDRAMCR1 {}
#[doc = "SDRAM control register 1"]
pub mod sdramcr1;
#[doc = "SDRAM control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdramcr2](sdramcr2) module"]
pub type SDRAMCR2 = crate::Reg<u32, _SDRAMCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDRAMCR2;
#[doc = "`read()` method returns [sdramcr2::R](sdramcr2::R) reader structure"]
impl crate::Readable for SDRAMCR2 {}
#[doc = "`write(|w| ..)` method takes [sdramcr2::W](sdramcr2::W) writer structure"]
impl crate::Writable for SDRAMCR2 {}
#[doc = "SDRAM control register 2"]
pub mod sdramcr2;
#[doc = "SDRAM control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdramcr3](sdramcr3) module"]
pub type SDRAMCR3 = crate::Reg<u32, _SDRAMCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDRAMCR3;
#[doc = "`read()` method returns [sdramcr3::R](sdramcr3::R) reader structure"]
impl crate::Readable for SDRAMCR3 {}
#[doc = "`write(|w| ..)` method takes [sdramcr3::W](sdramcr3::W) writer structure"]
impl crate::Writable for SDRAMCR3 {}
#[doc = "SDRAM control register 3"]
pub mod sdramcr3;
#[doc = "NAND control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nandcr0](nandcr0) module"]
pub type NANDCR0 = crate::Reg<u32, _NANDCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NANDCR0;
#[doc = "`read()` method returns [nandcr0::R](nandcr0::R) reader structure"]
impl crate::Readable for NANDCR0 {}
#[doc = "`write(|w| ..)` method takes [nandcr0::W](nandcr0::W) writer structure"]
impl crate::Writable for NANDCR0 {}
#[doc = "NAND control register 0"]
pub mod nandcr0;
#[doc = "NAND control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nandcr1](nandcr1) module"]
pub type NANDCR1 = crate::Reg<u32, _NANDCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NANDCR1;
#[doc = "`read()` method returns [nandcr1::R](nandcr1::R) reader structure"]
impl crate::Readable for NANDCR1 {}
#[doc = "`write(|w| ..)` method takes [nandcr1::W](nandcr1::W) writer structure"]
impl crate::Writable for NANDCR1 {}
#[doc = "NAND control register 1"]
pub mod nandcr1;
#[doc = "NAND control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nandcr2](nandcr2) module"]
pub type NANDCR2 = crate::Reg<u32, _NANDCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NANDCR2;
#[doc = "`read()` method returns [nandcr2::R](nandcr2::R) reader structure"]
impl crate::Readable for NANDCR2 {}
#[doc = "`write(|w| ..)` method takes [nandcr2::W](nandcr2::W) writer structure"]
impl crate::Writable for NANDCR2 {}
#[doc = "NAND control register 2"]
pub mod nandcr2;
#[doc = "NAND control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nandcr3](nandcr3) module"]
pub type NANDCR3 = crate::Reg<u32, _NANDCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NANDCR3;
#[doc = "`read()` method returns [nandcr3::R](nandcr3::R) reader structure"]
impl crate::Readable for NANDCR3 {}
#[doc = "`write(|w| ..)` method takes [nandcr3::W](nandcr3::W) writer structure"]
impl crate::Writable for NANDCR3 {}
#[doc = "NAND control register 3"]
pub mod nandcr3;
#[doc = "NOR control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [norcr0](norcr0) module"]
pub type NORCR0 = crate::Reg<u32, _NORCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NORCR0;
#[doc = "`read()` method returns [norcr0::R](norcr0::R) reader structure"]
impl crate::Readable for NORCR0 {}
#[doc = "`write(|w| ..)` method takes [norcr0::W](norcr0::W) writer structure"]
impl crate::Writable for NORCR0 {}
#[doc = "NOR control register 0"]
pub mod norcr0;
#[doc = "NOR control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [norcr1](norcr1) module"]
pub type NORCR1 = crate::Reg<u32, _NORCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NORCR1;
#[doc = "`read()` method returns [norcr1::R](norcr1::R) reader structure"]
impl crate::Readable for NORCR1 {}
#[doc = "`write(|w| ..)` method takes [norcr1::W](norcr1::W) writer structure"]
impl crate::Writable for NORCR1 {}
#[doc = "NOR control register 1"]
pub mod norcr1;
#[doc = "NOR control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [norcr2](norcr2) module"]
pub type NORCR2 = crate::Reg<u32, _NORCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NORCR2;
#[doc = "`read()` method returns [norcr2::R](norcr2::R) reader structure"]
impl crate::Readable for NORCR2 {}
#[doc = "`write(|w| ..)` method takes [norcr2::W](norcr2::W) writer structure"]
impl crate::Writable for NORCR2 {}
#[doc = "NOR control register 2"]
pub mod norcr2;
#[doc = "NOR control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [norcr3](norcr3) module"]
pub type NORCR3 = crate::Reg<u32, _NORCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NORCR3;
#[doc = "`read()` method returns [norcr3::R](norcr3::R) reader structure"]
impl crate::Readable for NORCR3 {}
#[doc = "`write(|w| ..)` method takes [norcr3::W](norcr3::W) writer structure"]
impl crate::Writable for NORCR3 {}
#[doc = "NOR control register 3"]
pub mod norcr3;
#[doc = "SRAM control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sramcr0](sramcr0) module"]
pub type SRAMCR0 = crate::Reg<u32, _SRAMCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAMCR0;
#[doc = "`read()` method returns [sramcr0::R](sramcr0::R) reader structure"]
impl crate::Readable for SRAMCR0 {}
#[doc = "`write(|w| ..)` method takes [sramcr0::W](sramcr0::W) writer structure"]
impl crate::Writable for SRAMCR0 {}
#[doc = "SRAM control register 0"]
pub mod sramcr0;
#[doc = "SRAM control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sramcr1](sramcr1) module"]
pub type SRAMCR1 = crate::Reg<u32, _SRAMCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAMCR1;
#[doc = "`read()` method returns [sramcr1::R](sramcr1::R) reader structure"]
impl crate::Readable for SRAMCR1 {}
#[doc = "`write(|w| ..)` method takes [sramcr1::W](sramcr1::W) writer structure"]
impl crate::Writable for SRAMCR1 {}
#[doc = "SRAM control register 1"]
pub mod sramcr1;
#[doc = "SRAM control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sramcr2](sramcr2) module"]
pub type SRAMCR2 = crate::Reg<u32, _SRAMCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAMCR2;
#[doc = "`read()` method returns [sramcr2::R](sramcr2::R) reader structure"]
impl crate::Readable for SRAMCR2 {}
#[doc = "`write(|w| ..)` method takes [sramcr2::W](sramcr2::W) writer structure"]
impl crate::Writable for SRAMCR2 {}
#[doc = "SRAM control register 2"]
pub mod sramcr2;
#[doc = "SRAM control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sramcr3](sramcr3) module"]
pub type SRAMCR3 = crate::Reg<u32, _SRAMCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAMCR3;
#[doc = "`read()` method returns [sramcr3::R](sramcr3::R) reader structure"]
impl crate::Readable for SRAMCR3 {}
#[doc = "`write(|w| ..)` method takes [sramcr3::W](sramcr3::W) writer structure"]
impl crate::Writable for SRAMCR3 {}
#[doc = "SRAM control register 3"]
pub mod sramcr3;
#[doc = "DBI-B control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbicr0](dbicr0) module"]
pub type DBICR0 = crate::Reg<u32, _DBICR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBICR0;
#[doc = "`read()` method returns [dbicr0::R](dbicr0::R) reader structure"]
impl crate::Readable for DBICR0 {}
#[doc = "`write(|w| ..)` method takes [dbicr0::W](dbicr0::W) writer structure"]
impl crate::Writable for DBICR0 {}
#[doc = "DBI-B control register 0"]
pub mod dbicr0;
#[doc = "DBI-B control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbicr1](dbicr1) module"]
pub type DBICR1 = crate::Reg<u32, _DBICR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBICR1;
#[doc = "`read()` method returns [dbicr1::R](dbicr1::R) reader structure"]
impl crate::Readable for DBICR1 {}
#[doc = "`write(|w| ..)` method takes [dbicr1::W](dbicr1::W) writer structure"]
impl crate::Writable for DBICR1 {}
#[doc = "DBI-B control register 1"]
pub mod dbicr1;
#[doc = "IP Command control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcr0](ipcr0) module"]
pub type IPCR0 = crate::Reg<u32, _IPCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPCR0;
#[doc = "`read()` method returns [ipcr0::R](ipcr0::R) reader structure"]
impl crate::Readable for IPCR0 {}
#[doc = "`write(|w| ..)` method takes [ipcr0::W](ipcr0::W) writer structure"]
impl crate::Writable for IPCR0 {}
#[doc = "IP Command control register 0"]
pub mod ipcr0;
#[doc = "IP Command control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcr1](ipcr1) module"]
pub type IPCR1 = crate::Reg<u32, _IPCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPCR1;
#[doc = "`read()` method returns [ipcr1::R](ipcr1::R) reader structure"]
impl crate::Readable for IPCR1 {}
#[doc = "`write(|w| ..)` method takes [ipcr1::W](ipcr1::W) writer structure"]
impl crate::Writable for IPCR1 {}
#[doc = "IP Command control register 1"]
pub mod ipcr1;
#[doc = "IP Command control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcr2](ipcr2) module"]
pub type IPCR2 = crate::Reg<u32, _IPCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPCR2;
#[doc = "`read()` method returns [ipcr2::R](ipcr2::R) reader structure"]
impl crate::Readable for IPCR2 {}
#[doc = "`write(|w| ..)` method takes [ipcr2::W](ipcr2::W) writer structure"]
impl crate::Writable for IPCR2 {}
#[doc = "IP Command control register 2"]
pub mod ipcr2;
#[doc = "IP Command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcmd](ipcmd) module"]
pub type IPCMD = crate::Reg<u32, _IPCMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPCMD;
#[doc = "`read()` method returns [ipcmd::R](ipcmd::R) reader structure"]
impl crate::Readable for IPCMD {}
#[doc = "`write(|w| ..)` method takes [ipcmd::W](ipcmd::W) writer structure"]
impl crate::Writable for IPCMD {}
#[doc = "IP Command register"]
pub mod ipcmd;
#[doc = "TX DATA register (for IP Command)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iptxdat](iptxdat) module"]
pub type IPTXDAT = crate::Reg<u32, _IPTXDAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPTXDAT;
#[doc = "`read()` method returns [iptxdat::R](iptxdat::R) reader structure"]
impl crate::Readable for IPTXDAT {}
#[doc = "`write(|w| ..)` method takes [iptxdat::W](iptxdat::W) writer structure"]
impl crate::Writable for IPTXDAT {}
#[doc = "TX DATA register (for IP Command)"]
pub mod iptxdat;
#[doc = "RX DATA register (for IP Command)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iprxdat](iprxdat) module"]
pub type IPRXDAT = crate::Reg<u32, _IPRXDAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPRXDAT;
#[doc = "`read()` method returns [iprxdat::R](iprxdat::R) reader structure"]
impl crate::Readable for IPRXDAT {}
#[doc = "RX DATA register (for IP Command)"]
pub mod iprxdat;
#[doc = "Status register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts0](sts0) module"]
pub type STS0 = crate::Reg<u32, _STS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STS0;
#[doc = "`read()` method returns [sts0::R](sts0::R) reader structure"]
impl crate::Readable for STS0 {}
#[doc = "Status register 0"]
pub mod sts0;
#[doc = "Status register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts1](sts1) module"]
pub type STS1 = crate::Reg<u32, _STS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STS1;
#[doc = "`read()` method returns [sts1::R](sts1::R) reader structure"]
impl crate::Readable for STS1 {}
#[doc = "Status register 1"]
pub mod sts1;
#[doc = "Status register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts2](sts2) module"]
pub type STS2 = crate::Reg<u32, _STS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STS2;
#[doc = "`read()` method returns [sts2::R](sts2::R) reader structure"]
impl crate::Readable for STS2 {}
#[doc = "Status register 2"]
pub mod sts2;
#[doc = "Status register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts3](sts3) module"]
pub type STS3 = crate::Reg<u32, _STS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STS3;
#[doc = "`read()` method returns [sts3::R](sts3::R) reader structure"]
impl crate::Readable for STS3 {}
#[doc = "Status register 3"]
pub mod sts3;
#[doc = "Status register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts4](sts4) module"]
pub type STS4 = crate::Reg<u32, _STS4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STS4;
#[doc = "`read()` method returns [sts4::R](sts4::R) reader structure"]
impl crate::Readable for STS4 {}
#[doc = "Status register 4"]
pub mod sts4;
#[doc = "Status register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts5](sts5) module"]
pub type STS5 = crate::Reg<u32, _STS5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STS5;
#[doc = "`read()` method returns [sts5::R](sts5::R) reader structure"]
impl crate::Readable for STS5 {}
#[doc = "Status register 5"]
pub mod sts5;
#[doc = "Status register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts6](sts6) module"]
pub type STS6 = crate::Reg<u32, _STS6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STS6;
#[doc = "`read()` method returns [sts6::R](sts6::R) reader structure"]
impl crate::Readable for STS6 {}
#[doc = "Status register 6"]
pub mod sts6;
#[doc = "Status register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts7](sts7) module"]
pub type STS7 = crate::Reg<u32, _STS7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STS7;
#[doc = "`read()` method returns [sts7::R](sts7::R) reader structure"]
impl crate::Readable for STS7 {}
#[doc = "Status register 7"]
pub mod sts7;
#[doc = "Status register 8\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts8](sts8) module"]
pub type STS8 = crate::Reg<u32, _STS8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STS8;
#[doc = "`read()` method returns [sts8::R](sts8::R) reader structure"]
impl crate::Readable for STS8 {}
#[doc = "Status register 8"]
pub mod sts8;
#[doc = "Status register 9\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts9](sts9) module"]
pub type STS9 = crate::Reg<u32, _STS9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STS9;
#[doc = "`read()` method returns [sts9::R](sts9::R) reader structure"]
impl crate::Readable for STS9 {}
#[doc = "Status register 9"]
pub mod sts9;
#[doc = "Status register 10\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts10](sts10) module"]
pub type STS10 = crate::Reg<u32, _STS10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STS10;
#[doc = "`read()` method returns [sts10::R](sts10::R) reader structure"]
impl crate::Readable for STS10 {}
#[doc = "Status register 10"]
pub mod sts10;
#[doc = "Status register 11\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts11](sts11) module"]
pub type STS11 = crate::Reg<u32, _STS11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STS11;
#[doc = "`read()` method returns [sts11::R](sts11::R) reader structure"]
impl crate::Readable for STS11 {}
#[doc = "Status register 11"]
pub mod sts11;
#[doc = "Status register 12\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts12](sts12) module"]
pub type STS12 = crate::Reg<u32, _STS12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STS12;
#[doc = "`read()` method returns [sts12::R](sts12::R) reader structure"]
impl crate::Readable for STS12 {}
#[doc = "Status register 12"]
pub mod sts12;
#[doc = "Status register 13\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts13](sts13) module"]
pub type STS13 = crate::Reg<u32, _STS13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STS13;
#[doc = "`read()` method returns [sts13::R](sts13::R) reader structure"]
impl crate::Readable for STS13 {}
#[doc = "Status register 13"]
pub mod sts13;
#[doc = "Status register 14\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts14](sts14) module"]
pub type STS14 = crate::Reg<u32, _STS14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STS14;
#[doc = "`read()` method returns [sts14::R](sts14::R) reader structure"]
impl crate::Readable for STS14 {}
#[doc = "Status register 14"]
pub mod sts14;
#[doc = "Status register 15\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts15](sts15) module"]
pub type STS15 = crate::Reg<u32, _STS15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STS15;
#[doc = "`read()` method returns [sts15::R](sts15::R) reader structure"]
impl crate::Readable for STS15 {}
#[doc = "Status register 15"]
pub mod sts15;
