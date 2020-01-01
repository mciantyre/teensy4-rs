#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 384usize],
    #[doc = "0x180 - Tempsensor Control Register 0"]
    pub tempsense0: TEMPSENSE0,
    #[doc = "0x184 - Tempsensor Control Register 0"]
    pub tempsense0_set: TEMPSENSE0_SET,
    #[doc = "0x188 - Tempsensor Control Register 0"]
    pub tempsense0_clr: TEMPSENSE0_CLR,
    #[doc = "0x18c - Tempsensor Control Register 0"]
    pub tempsense0_tog: TEMPSENSE0_TOG,
    #[doc = "0x190 - Tempsensor Control Register 1"]
    pub tempsense1: TEMPSENSE1,
    #[doc = "0x194 - Tempsensor Control Register 1"]
    pub tempsense1_set: TEMPSENSE1_SET,
    #[doc = "0x198 - Tempsensor Control Register 1"]
    pub tempsense1_clr: TEMPSENSE1_CLR,
    #[doc = "0x19c - Tempsensor Control Register 1"]
    pub tempsense1_tog: TEMPSENSE1_TOG,
    _reserved8: [u8; 240usize],
    #[doc = "0x290 - Tempsensor Control Register 2"]
    pub tempsense2: TEMPSENSE2,
    #[doc = "0x294 - Tempsensor Control Register 2"]
    pub tempsense2_set: TEMPSENSE2_SET,
    #[doc = "0x298 - Tempsensor Control Register 2"]
    pub tempsense2_clr: TEMPSENSE2_CLR,
    #[doc = "0x29c - Tempsensor Control Register 2"]
    pub tempsense2_tog: TEMPSENSE2_TOG,
}
#[doc = "Tempsensor Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempsense0](tempsense0) module"]
pub type TEMPSENSE0 = crate::Reg<u32, _TEMPSENSE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMPSENSE0;
#[doc = "`read()` method returns [tempsense0::R](tempsense0::R) reader structure"]
impl crate::Readable for TEMPSENSE0 {}
#[doc = "`write(|w| ..)` method takes [tempsense0::W](tempsense0::W) writer structure"]
impl crate::Writable for TEMPSENSE0 {}
#[doc = "Tempsensor Control Register 0"]
pub mod tempsense0;
#[doc = "Tempsensor Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempsense0_set](tempsense0_set) module"]
pub type TEMPSENSE0_SET = crate::Reg<u32, _TEMPSENSE0_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMPSENSE0_SET;
#[doc = "`read()` method returns [tempsense0_set::R](tempsense0_set::R) reader structure"]
impl crate::Readable for TEMPSENSE0_SET {}
#[doc = "`write(|w| ..)` method takes [tempsense0_set::W](tempsense0_set::W) writer structure"]
impl crate::Writable for TEMPSENSE0_SET {}
#[doc = "Tempsensor Control Register 0"]
pub mod tempsense0_set;
#[doc = "Tempsensor Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempsense0_clr](tempsense0_clr) module"]
pub type TEMPSENSE0_CLR = crate::Reg<u32, _TEMPSENSE0_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMPSENSE0_CLR;
#[doc = "`read()` method returns [tempsense0_clr::R](tempsense0_clr::R) reader structure"]
impl crate::Readable for TEMPSENSE0_CLR {}
#[doc = "`write(|w| ..)` method takes [tempsense0_clr::W](tempsense0_clr::W) writer structure"]
impl crate::Writable for TEMPSENSE0_CLR {}
#[doc = "Tempsensor Control Register 0"]
pub mod tempsense0_clr;
#[doc = "Tempsensor Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempsense0_tog](tempsense0_tog) module"]
pub type TEMPSENSE0_TOG = crate::Reg<u32, _TEMPSENSE0_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMPSENSE0_TOG;
#[doc = "`read()` method returns [tempsense0_tog::R](tempsense0_tog::R) reader structure"]
impl crate::Readable for TEMPSENSE0_TOG {}
#[doc = "`write(|w| ..)` method takes [tempsense0_tog::W](tempsense0_tog::W) writer structure"]
impl crate::Writable for TEMPSENSE0_TOG {}
#[doc = "Tempsensor Control Register 0"]
pub mod tempsense0_tog;
#[doc = "Tempsensor Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempsense1](tempsense1) module"]
pub type TEMPSENSE1 = crate::Reg<u32, _TEMPSENSE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMPSENSE1;
#[doc = "`read()` method returns [tempsense1::R](tempsense1::R) reader structure"]
impl crate::Readable for TEMPSENSE1 {}
#[doc = "`write(|w| ..)` method takes [tempsense1::W](tempsense1::W) writer structure"]
impl crate::Writable for TEMPSENSE1 {}
#[doc = "Tempsensor Control Register 1"]
pub mod tempsense1;
#[doc = "Tempsensor Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempsense1_set](tempsense1_set) module"]
pub type TEMPSENSE1_SET = crate::Reg<u32, _TEMPSENSE1_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMPSENSE1_SET;
#[doc = "`read()` method returns [tempsense1_set::R](tempsense1_set::R) reader structure"]
impl crate::Readable for TEMPSENSE1_SET {}
#[doc = "`write(|w| ..)` method takes [tempsense1_set::W](tempsense1_set::W) writer structure"]
impl crate::Writable for TEMPSENSE1_SET {}
#[doc = "Tempsensor Control Register 1"]
pub mod tempsense1_set;
#[doc = "Tempsensor Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempsense1_clr](tempsense1_clr) module"]
pub type TEMPSENSE1_CLR = crate::Reg<u32, _TEMPSENSE1_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMPSENSE1_CLR;
#[doc = "`read()` method returns [tempsense1_clr::R](tempsense1_clr::R) reader structure"]
impl crate::Readable for TEMPSENSE1_CLR {}
#[doc = "`write(|w| ..)` method takes [tempsense1_clr::W](tempsense1_clr::W) writer structure"]
impl crate::Writable for TEMPSENSE1_CLR {}
#[doc = "Tempsensor Control Register 1"]
pub mod tempsense1_clr;
#[doc = "Tempsensor Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempsense1_tog](tempsense1_tog) module"]
pub type TEMPSENSE1_TOG = crate::Reg<u32, _TEMPSENSE1_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMPSENSE1_TOG;
#[doc = "`read()` method returns [tempsense1_tog::R](tempsense1_tog::R) reader structure"]
impl crate::Readable for TEMPSENSE1_TOG {}
#[doc = "`write(|w| ..)` method takes [tempsense1_tog::W](tempsense1_tog::W) writer structure"]
impl crate::Writable for TEMPSENSE1_TOG {}
#[doc = "Tempsensor Control Register 1"]
pub mod tempsense1_tog;
#[doc = "Tempsensor Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempsense2](tempsense2) module"]
pub type TEMPSENSE2 = crate::Reg<u32, _TEMPSENSE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMPSENSE2;
#[doc = "`read()` method returns [tempsense2::R](tempsense2::R) reader structure"]
impl crate::Readable for TEMPSENSE2 {}
#[doc = "`write(|w| ..)` method takes [tempsense2::W](tempsense2::W) writer structure"]
impl crate::Writable for TEMPSENSE2 {}
#[doc = "Tempsensor Control Register 2"]
pub mod tempsense2;
#[doc = "Tempsensor Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempsense2_set](tempsense2_set) module"]
pub type TEMPSENSE2_SET = crate::Reg<u32, _TEMPSENSE2_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMPSENSE2_SET;
#[doc = "`read()` method returns [tempsense2_set::R](tempsense2_set::R) reader structure"]
impl crate::Readable for TEMPSENSE2_SET {}
#[doc = "`write(|w| ..)` method takes [tempsense2_set::W](tempsense2_set::W) writer structure"]
impl crate::Writable for TEMPSENSE2_SET {}
#[doc = "Tempsensor Control Register 2"]
pub mod tempsense2_set;
#[doc = "Tempsensor Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempsense2_clr](tempsense2_clr) module"]
pub type TEMPSENSE2_CLR = crate::Reg<u32, _TEMPSENSE2_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMPSENSE2_CLR;
#[doc = "`read()` method returns [tempsense2_clr::R](tempsense2_clr::R) reader structure"]
impl crate::Readable for TEMPSENSE2_CLR {}
#[doc = "`write(|w| ..)` method takes [tempsense2_clr::W](tempsense2_clr::W) writer structure"]
impl crate::Writable for TEMPSENSE2_CLR {}
#[doc = "Tempsensor Control Register 2"]
pub mod tempsense2_clr;
#[doc = "Tempsensor Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempsense2_tog](tempsense2_tog) module"]
pub type TEMPSENSE2_TOG = crate::Reg<u32, _TEMPSENSE2_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMPSENSE2_TOG;
#[doc = "`read()` method returns [tempsense2_tog::R](tempsense2_tog::R) reader structure"]
impl crate::Readable for TEMPSENSE2_TOG {}
#[doc = "`write(|w| ..)` method takes [tempsense2_tog::W](tempsense2_tog::W) writer structure"]
impl crate::Writable for TEMPSENSE2_TOG {}
#[doc = "Tempsensor Control Register 2"]
pub mod tempsense2_tog;
