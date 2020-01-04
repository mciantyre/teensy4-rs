#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 272usize],
    #[doc = "0x110 - Regulator 1P1 Register"]
    pub reg_1p1: REG_1P1,
    #[doc = "0x114 - Regulator 1P1 Register"]
    pub reg_1p1_set: REG_1P1_SET,
    #[doc = "0x118 - Regulator 1P1 Register"]
    pub reg_1p1_clr: REG_1P1_CLR,
    #[doc = "0x11c - Regulator 1P1 Register"]
    pub reg_1p1_tog: REG_1P1_TOG,
    #[doc = "0x120 - Regulator 3P0 Register"]
    pub reg_3p0: REG_3P0,
    #[doc = "0x124 - Regulator 3P0 Register"]
    pub reg_3p0_set: REG_3P0_SET,
    #[doc = "0x128 - Regulator 3P0 Register"]
    pub reg_3p0_clr: REG_3P0_CLR,
    #[doc = "0x12c - Regulator 3P0 Register"]
    pub reg_3p0_tog: REG_3P0_TOG,
    #[doc = "0x130 - Regulator 2P5 Register"]
    pub reg_2p5: REG_2P5,
    #[doc = "0x134 - Regulator 2P5 Register"]
    pub reg_2p5_set: REG_2P5_SET,
    #[doc = "0x138 - Regulator 2P5 Register"]
    pub reg_2p5_clr: REG_2P5_CLR,
    #[doc = "0x13c - Regulator 2P5 Register"]
    pub reg_2p5_tog: REG_2P5_TOG,
    #[doc = "0x140 - Digital Regulator Core Register"]
    pub reg_core: REG_CORE,
    #[doc = "0x144 - Digital Regulator Core Register"]
    pub reg_core_set: REG_CORE_SET,
    #[doc = "0x148 - Digital Regulator Core Register"]
    pub reg_core_clr: REG_CORE_CLR,
    #[doc = "0x14c - Digital Regulator Core Register"]
    pub reg_core_tog: REG_CORE_TOG,
    #[doc = "0x150 - Miscellaneous Register 0"]
    pub misc0: MISC0,
    #[doc = "0x154 - Miscellaneous Register 0"]
    pub misc0_set: MISC0_SET,
    #[doc = "0x158 - Miscellaneous Register 0"]
    pub misc0_clr: MISC0_CLR,
    #[doc = "0x15c - Miscellaneous Register 0"]
    pub misc0_tog: MISC0_TOG,
    #[doc = "0x160 - Miscellaneous Register 1"]
    pub misc1: MISC1,
    #[doc = "0x164 - Miscellaneous Register 1"]
    pub misc1_set: MISC1_SET,
    #[doc = "0x168 - Miscellaneous Register 1"]
    pub misc1_clr: MISC1_CLR,
    #[doc = "0x16c - Miscellaneous Register 1"]
    pub misc1_tog: MISC1_TOG,
    #[doc = "0x170 - Miscellaneous Control Register"]
    pub misc2: MISC2,
    #[doc = "0x174 - Miscellaneous Control Register"]
    pub misc2_set: MISC2_SET,
    #[doc = "0x178 - Miscellaneous Control Register"]
    pub misc2_clr: MISC2_CLR,
    #[doc = "0x17c - Miscellaneous Control Register"]
    pub misc2_tog: MISC2_TOG,
}
#[doc = "Regulator 1P1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_1p1](reg_1p1) module"]
pub type REG_1P1 = crate::Reg<u32, _REG_1P1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_1P1;
#[doc = "`read()` method returns [reg_1p1::R](reg_1p1::R) reader structure"]
impl crate::Readable for REG_1P1 {}
#[doc = "`write(|w| ..)` method takes [reg_1p1::W](reg_1p1::W) writer structure"]
impl crate::Writable for REG_1P1 {}
#[doc = "Regulator 1P1 Register"]
pub mod reg_1p1;
#[doc = "Regulator 1P1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_1p1_set](reg_1p1_set) module"]
pub type REG_1P1_SET = crate::Reg<u32, _REG_1P1_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_1P1_SET;
#[doc = "`read()` method returns [reg_1p1_set::R](reg_1p1_set::R) reader structure"]
impl crate::Readable for REG_1P1_SET {}
#[doc = "`write(|w| ..)` method takes [reg_1p1_set::W](reg_1p1_set::W) writer structure"]
impl crate::Writable for REG_1P1_SET {}
#[doc = "Regulator 1P1 Register"]
pub mod reg_1p1_set;
#[doc = "Regulator 1P1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_1p1_clr](reg_1p1_clr) module"]
pub type REG_1P1_CLR = crate::Reg<u32, _REG_1P1_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_1P1_CLR;
#[doc = "`read()` method returns [reg_1p1_clr::R](reg_1p1_clr::R) reader structure"]
impl crate::Readable for REG_1P1_CLR {}
#[doc = "`write(|w| ..)` method takes [reg_1p1_clr::W](reg_1p1_clr::W) writer structure"]
impl crate::Writable for REG_1P1_CLR {}
#[doc = "Regulator 1P1 Register"]
pub mod reg_1p1_clr;
#[doc = "Regulator 1P1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_1p1_tog](reg_1p1_tog) module"]
pub type REG_1P1_TOG = crate::Reg<u32, _REG_1P1_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_1P1_TOG;
#[doc = "`read()` method returns [reg_1p1_tog::R](reg_1p1_tog::R) reader structure"]
impl crate::Readable for REG_1P1_TOG {}
#[doc = "`write(|w| ..)` method takes [reg_1p1_tog::W](reg_1p1_tog::W) writer structure"]
impl crate::Writable for REG_1P1_TOG {}
#[doc = "Regulator 1P1 Register"]
pub mod reg_1p1_tog;
#[doc = "Regulator 3P0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_3p0](reg_3p0) module"]
pub type REG_3P0 = crate::Reg<u32, _REG_3P0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_3P0;
#[doc = "`read()` method returns [reg_3p0::R](reg_3p0::R) reader structure"]
impl crate::Readable for REG_3P0 {}
#[doc = "`write(|w| ..)` method takes [reg_3p0::W](reg_3p0::W) writer structure"]
impl crate::Writable for REG_3P0 {}
#[doc = "Regulator 3P0 Register"]
pub mod reg_3p0;
#[doc = "Regulator 3P0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_3p0_set](reg_3p0_set) module"]
pub type REG_3P0_SET = crate::Reg<u32, _REG_3P0_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_3P0_SET;
#[doc = "`read()` method returns [reg_3p0_set::R](reg_3p0_set::R) reader structure"]
impl crate::Readable for REG_3P0_SET {}
#[doc = "`write(|w| ..)` method takes [reg_3p0_set::W](reg_3p0_set::W) writer structure"]
impl crate::Writable for REG_3P0_SET {}
#[doc = "Regulator 3P0 Register"]
pub mod reg_3p0_set;
#[doc = "Regulator 3P0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_3p0_clr](reg_3p0_clr) module"]
pub type REG_3P0_CLR = crate::Reg<u32, _REG_3P0_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_3P0_CLR;
#[doc = "`read()` method returns [reg_3p0_clr::R](reg_3p0_clr::R) reader structure"]
impl crate::Readable for REG_3P0_CLR {}
#[doc = "`write(|w| ..)` method takes [reg_3p0_clr::W](reg_3p0_clr::W) writer structure"]
impl crate::Writable for REG_3P0_CLR {}
#[doc = "Regulator 3P0 Register"]
pub mod reg_3p0_clr;
#[doc = "Regulator 3P0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_3p0_tog](reg_3p0_tog) module"]
pub type REG_3P0_TOG = crate::Reg<u32, _REG_3P0_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_3P0_TOG;
#[doc = "`read()` method returns [reg_3p0_tog::R](reg_3p0_tog::R) reader structure"]
impl crate::Readable for REG_3P0_TOG {}
#[doc = "`write(|w| ..)` method takes [reg_3p0_tog::W](reg_3p0_tog::W) writer structure"]
impl crate::Writable for REG_3P0_TOG {}
#[doc = "Regulator 3P0 Register"]
pub mod reg_3p0_tog;
#[doc = "Regulator 2P5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_2p5](reg_2p5) module"]
pub type REG_2P5 = crate::Reg<u32, _REG_2P5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_2P5;
#[doc = "`read()` method returns [reg_2p5::R](reg_2p5::R) reader structure"]
impl crate::Readable for REG_2P5 {}
#[doc = "`write(|w| ..)` method takes [reg_2p5::W](reg_2p5::W) writer structure"]
impl crate::Writable for REG_2P5 {}
#[doc = "Regulator 2P5 Register"]
pub mod reg_2p5;
#[doc = "Regulator 2P5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_2p5_set](reg_2p5_set) module"]
pub type REG_2P5_SET = crate::Reg<u32, _REG_2P5_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_2P5_SET;
#[doc = "`read()` method returns [reg_2p5_set::R](reg_2p5_set::R) reader structure"]
impl crate::Readable for REG_2P5_SET {}
#[doc = "`write(|w| ..)` method takes [reg_2p5_set::W](reg_2p5_set::W) writer structure"]
impl crate::Writable for REG_2P5_SET {}
#[doc = "Regulator 2P5 Register"]
pub mod reg_2p5_set;
#[doc = "Regulator 2P5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_2p5_clr](reg_2p5_clr) module"]
pub type REG_2P5_CLR = crate::Reg<u32, _REG_2P5_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_2P5_CLR;
#[doc = "`read()` method returns [reg_2p5_clr::R](reg_2p5_clr::R) reader structure"]
impl crate::Readable for REG_2P5_CLR {}
#[doc = "`write(|w| ..)` method takes [reg_2p5_clr::W](reg_2p5_clr::W) writer structure"]
impl crate::Writable for REG_2P5_CLR {}
#[doc = "Regulator 2P5 Register"]
pub mod reg_2p5_clr;
#[doc = "Regulator 2P5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_2p5_tog](reg_2p5_tog) module"]
pub type REG_2P5_TOG = crate::Reg<u32, _REG_2P5_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_2P5_TOG;
#[doc = "`read()` method returns [reg_2p5_tog::R](reg_2p5_tog::R) reader structure"]
impl crate::Readable for REG_2P5_TOG {}
#[doc = "`write(|w| ..)` method takes [reg_2p5_tog::W](reg_2p5_tog::W) writer structure"]
impl crate::Writable for REG_2P5_TOG {}
#[doc = "Regulator 2P5 Register"]
pub mod reg_2p5_tog;
#[doc = "Digital Regulator Core Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_core](reg_core) module"]
pub type REG_CORE = crate::Reg<u32, _REG_CORE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_CORE;
#[doc = "`read()` method returns [reg_core::R](reg_core::R) reader structure"]
impl crate::Readable for REG_CORE {}
#[doc = "`write(|w| ..)` method takes [reg_core::W](reg_core::W) writer structure"]
impl crate::Writable for REG_CORE {}
#[doc = "Digital Regulator Core Register"]
pub mod reg_core;
#[doc = "Digital Regulator Core Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_core_set](reg_core_set) module"]
pub type REG_CORE_SET = crate::Reg<u32, _REG_CORE_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_CORE_SET;
#[doc = "`read()` method returns [reg_core_set::R](reg_core_set::R) reader structure"]
impl crate::Readable for REG_CORE_SET {}
#[doc = "`write(|w| ..)` method takes [reg_core_set::W](reg_core_set::W) writer structure"]
impl crate::Writable for REG_CORE_SET {}
#[doc = "Digital Regulator Core Register"]
pub mod reg_core_set;
#[doc = "Digital Regulator Core Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_core_clr](reg_core_clr) module"]
pub type REG_CORE_CLR = crate::Reg<u32, _REG_CORE_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_CORE_CLR;
#[doc = "`read()` method returns [reg_core_clr::R](reg_core_clr::R) reader structure"]
impl crate::Readable for REG_CORE_CLR {}
#[doc = "`write(|w| ..)` method takes [reg_core_clr::W](reg_core_clr::W) writer structure"]
impl crate::Writable for REG_CORE_CLR {}
#[doc = "Digital Regulator Core Register"]
pub mod reg_core_clr;
#[doc = "Digital Regulator Core Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_core_tog](reg_core_tog) module"]
pub type REG_CORE_TOG = crate::Reg<u32, _REG_CORE_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG_CORE_TOG;
#[doc = "`read()` method returns [reg_core_tog::R](reg_core_tog::R) reader structure"]
impl crate::Readable for REG_CORE_TOG {}
#[doc = "`write(|w| ..)` method takes [reg_core_tog::W](reg_core_tog::W) writer structure"]
impl crate::Writable for REG_CORE_TOG {}
#[doc = "Digital Regulator Core Register"]
pub mod reg_core_tog;
#[doc = "Miscellaneous Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc0](misc0) module"]
pub type MISC0 = crate::Reg<u32, _MISC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC0;
#[doc = "`read()` method returns [misc0::R](misc0::R) reader structure"]
impl crate::Readable for MISC0 {}
#[doc = "`write(|w| ..)` method takes [misc0::W](misc0::W) writer structure"]
impl crate::Writable for MISC0 {}
#[doc = "Miscellaneous Register 0"]
pub mod misc0;
#[doc = "Miscellaneous Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc0_set](misc0_set) module"]
pub type MISC0_SET = crate::Reg<u32, _MISC0_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC0_SET;
#[doc = "`read()` method returns [misc0_set::R](misc0_set::R) reader structure"]
impl crate::Readable for MISC0_SET {}
#[doc = "`write(|w| ..)` method takes [misc0_set::W](misc0_set::W) writer structure"]
impl crate::Writable for MISC0_SET {}
#[doc = "Miscellaneous Register 0"]
pub mod misc0_set;
#[doc = "Miscellaneous Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc0_clr](misc0_clr) module"]
pub type MISC0_CLR = crate::Reg<u32, _MISC0_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC0_CLR;
#[doc = "`read()` method returns [misc0_clr::R](misc0_clr::R) reader structure"]
impl crate::Readable for MISC0_CLR {}
#[doc = "`write(|w| ..)` method takes [misc0_clr::W](misc0_clr::W) writer structure"]
impl crate::Writable for MISC0_CLR {}
#[doc = "Miscellaneous Register 0"]
pub mod misc0_clr;
#[doc = "Miscellaneous Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc0_tog](misc0_tog) module"]
pub type MISC0_TOG = crate::Reg<u32, _MISC0_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC0_TOG;
#[doc = "`read()` method returns [misc0_tog::R](misc0_tog::R) reader structure"]
impl crate::Readable for MISC0_TOG {}
#[doc = "`write(|w| ..)` method takes [misc0_tog::W](misc0_tog::W) writer structure"]
impl crate::Writable for MISC0_TOG {}
#[doc = "Miscellaneous Register 0"]
pub mod misc0_tog;
#[doc = "Miscellaneous Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc1](misc1) module"]
pub type MISC1 = crate::Reg<u32, _MISC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC1;
#[doc = "`read()` method returns [misc1::R](misc1::R) reader structure"]
impl crate::Readable for MISC1 {}
#[doc = "`write(|w| ..)` method takes [misc1::W](misc1::W) writer structure"]
impl crate::Writable for MISC1 {}
#[doc = "Miscellaneous Register 1"]
pub mod misc1;
#[doc = "Miscellaneous Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc1_set](misc1_set) module"]
pub type MISC1_SET = crate::Reg<u32, _MISC1_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC1_SET;
#[doc = "`read()` method returns [misc1_set::R](misc1_set::R) reader structure"]
impl crate::Readable for MISC1_SET {}
#[doc = "`write(|w| ..)` method takes [misc1_set::W](misc1_set::W) writer structure"]
impl crate::Writable for MISC1_SET {}
#[doc = "Miscellaneous Register 1"]
pub mod misc1_set;
#[doc = "Miscellaneous Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc1_clr](misc1_clr) module"]
pub type MISC1_CLR = crate::Reg<u32, _MISC1_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC1_CLR;
#[doc = "`read()` method returns [misc1_clr::R](misc1_clr::R) reader structure"]
impl crate::Readable for MISC1_CLR {}
#[doc = "`write(|w| ..)` method takes [misc1_clr::W](misc1_clr::W) writer structure"]
impl crate::Writable for MISC1_CLR {}
#[doc = "Miscellaneous Register 1"]
pub mod misc1_clr;
#[doc = "Miscellaneous Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc1_tog](misc1_tog) module"]
pub type MISC1_TOG = crate::Reg<u32, _MISC1_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC1_TOG;
#[doc = "`read()` method returns [misc1_tog::R](misc1_tog::R) reader structure"]
impl crate::Readable for MISC1_TOG {}
#[doc = "`write(|w| ..)` method takes [misc1_tog::W](misc1_tog::W) writer structure"]
impl crate::Writable for MISC1_TOG {}
#[doc = "Miscellaneous Register 1"]
pub mod misc1_tog;
#[doc = "Miscellaneous Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc2](misc2) module"]
pub type MISC2 = crate::Reg<u32, _MISC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC2;
#[doc = "`read()` method returns [misc2::R](misc2::R) reader structure"]
impl crate::Readable for MISC2 {}
#[doc = "`write(|w| ..)` method takes [misc2::W](misc2::W) writer structure"]
impl crate::Writable for MISC2 {}
#[doc = "Miscellaneous Control Register"]
pub mod misc2;
#[doc = "Miscellaneous Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc2_set](misc2_set) module"]
pub type MISC2_SET = crate::Reg<u32, _MISC2_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC2_SET;
#[doc = "`read()` method returns [misc2_set::R](misc2_set::R) reader structure"]
impl crate::Readable for MISC2_SET {}
#[doc = "`write(|w| ..)` method takes [misc2_set::W](misc2_set::W) writer structure"]
impl crate::Writable for MISC2_SET {}
#[doc = "Miscellaneous Control Register"]
pub mod misc2_set;
#[doc = "Miscellaneous Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc2_clr](misc2_clr) module"]
pub type MISC2_CLR = crate::Reg<u32, _MISC2_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC2_CLR;
#[doc = "`read()` method returns [misc2_clr::R](misc2_clr::R) reader structure"]
impl crate::Readable for MISC2_CLR {}
#[doc = "`write(|w| ..)` method takes [misc2_clr::W](misc2_clr::W) writer structure"]
impl crate::Writable for MISC2_CLR {}
#[doc = "Miscellaneous Control Register"]
pub mod misc2_clr;
#[doc = "Miscellaneous Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc2_tog](misc2_tog) module"]
pub type MISC2_TOG = crate::Reg<u32, _MISC2_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC2_TOG;
#[doc = "`read()` method returns [misc2_tog::R](misc2_tog::R) reader structure"]
impl crate::Readable for MISC2_TOG {}
#[doc = "`write(|w| ..)` method takes [misc2_tog::W](misc2_tog::W) writer structure"]
impl crate::Writable for MISC2_TOG {}
#[doc = "Miscellaneous Control Register"]
pub mod misc2_tog;
