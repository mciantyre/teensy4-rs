#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register for hardware triggers"]
    pub hc0: HC0,
    #[doc = "0x04 - Control register for hardware triggers"]
    pub hc1: HC,
    #[doc = "0x08 - Control register for hardware triggers"]
    pub hc2: HC,
    #[doc = "0x0c - Control register for hardware triggers"]
    pub hc3: HC,
    #[doc = "0x10 - Control register for hardware triggers"]
    pub hc4: HC,
    #[doc = "0x14 - Control register for hardware triggers"]
    pub hc5: HC,
    #[doc = "0x18 - Control register for hardware triggers"]
    pub hc6: HC,
    #[doc = "0x1c - Control register for hardware triggers"]
    pub hc7: HC,
    #[doc = "0x20 - Status register for HW triggers"]
    pub hs: HS,
    #[doc = "0x24 - Data result register for HW triggers"]
    pub r0: RESULT0,
    #[doc = "0x28 - Data result register for HW triggers"]
    pub r1: RESULT,
    #[doc = "0x2c - Data result register for HW triggers"]
    pub r2: RESULT,
    #[doc = "0x30 - Data result register for HW triggers"]
    pub r3: RESULT,
    #[doc = "0x34 - Data result register for HW triggers"]
    pub r4: RESULT,
    #[doc = "0x38 - Data result register for HW triggers"]
    pub r5: RESULT,
    #[doc = "0x3c - Data result register for HW triggers"]
    pub r6: RESULT,
    #[doc = "0x40 - Data result register for HW triggers"]
    pub r7: RESULT,
    #[doc = "0x44 - Configuration register"]
    pub cfg: CFG,
    #[doc = "0x48 - General control register"]
    pub gc: GC,
    #[doc = "0x4c - General status register"]
    pub gs: GS,
    #[doc = "0x50 - Compare value register"]
    pub cv: CV,
    #[doc = "0x54 - Offset correction value register"]
    pub ofs: OFS,
    #[doc = "0x58 - Calibration value register"]
    pub cal: CAL,
}
#[doc = "Control register for hardware triggers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc0](hc0) module"]
pub type HC0 = crate::Reg<u32, _HC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC0;
#[doc = "`read()` method returns [hc0::R](hc0::R) reader structure"]
impl crate::Readable for HC0 {}
#[doc = "`write(|w| ..)` method takes [hc0::W](hc0::W) writer structure"]
impl crate::Writable for HC0 {}
#[doc = "Control register for hardware triggers"]
pub mod hc0;
#[doc = "Control register for hardware triggers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc](hc) module"]
pub type HC = crate::Reg<u32, _HC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC;
#[doc = "`read()` method returns [hc::R](hc::R) reader structure"]
impl crate::Readable for HC {}
#[doc = "`write(|w| ..)` method takes [hc::W](hc::W) writer structure"]
impl crate::Writable for HC {}
#[doc = "Control register for hardware triggers"]
pub mod hc;
#[doc = "Status register for HW triggers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hs](hs) module"]
pub type HS = crate::Reg<u32, _HS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HS;
#[doc = "`read()` method returns [hs::R](hs::R) reader structure"]
impl crate::Readable for HS {}
#[doc = "Status register for HW triggers"]
pub mod hs;
#[doc = "Data result register for HW triggers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r0](r0) module"]
pub type RESULT0 = crate::Reg<u32, _RESULT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESULT0;
#[doc = "`read()` method returns [r0::R](r0::R) reader structure"]
impl crate::Readable for RESULT0 {}
#[doc = "Data result register for HW triggers"]
pub mod result0;
#[doc = "Data result register for HW triggers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r](r) module"]
pub type RESULT = crate::Reg<u32, _RESULT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESULT;
#[doc = "`read()` method returns [r::R](r::R) reader structure"]
impl crate::Readable for RESULT {}
#[doc = "Data result register for HW triggers"]
pub mod result;
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "Configuration register"]
pub mod cfg;
#[doc = "General control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gc](gc) module"]
pub type GC = crate::Reg<u32, _GC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GC;
#[doc = "`read()` method returns [gc::R](gc::R) reader structure"]
impl crate::Readable for GC {}
#[doc = "`write(|w| ..)` method takes [gc::W](gc::W) writer structure"]
impl crate::Writable for GC {}
#[doc = "General control register"]
pub mod gc;
#[doc = "General status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gs](gs) module"]
pub type GS = crate::Reg<u32, _GS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GS;
#[doc = "`read()` method returns [gs::R](gs::R) reader structure"]
impl crate::Readable for GS {}
#[doc = "`write(|w| ..)` method takes [gs::W](gs::W) writer structure"]
impl crate::Writable for GS {}
#[doc = "General status register"]
pub mod gs;
#[doc = "Compare value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cv](cv) module"]
pub type CV = crate::Reg<u32, _CV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CV;
#[doc = "`read()` method returns [cv::R](cv::R) reader structure"]
impl crate::Readable for CV {}
#[doc = "`write(|w| ..)` method takes [cv::W](cv::W) writer structure"]
impl crate::Writable for CV {}
#[doc = "Compare value register"]
pub mod cv;
#[doc = "Offset correction value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofs](ofs) module"]
pub type OFS = crate::Reg<u32, _OFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OFS;
#[doc = "`read()` method returns [ofs::R](ofs::R) reader structure"]
impl crate::Readable for OFS {}
#[doc = "`write(|w| ..)` method takes [ofs::W](ofs::W) writer structure"]
impl crate::Writable for OFS {}
#[doc = "Offset correction value register"]
pub mod ofs;
#[doc = "Calibration value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal](cal) module"]
pub type CAL = crate::Reg<u32, _CAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAL;
#[doc = "`read()` method returns [cal::R](cal::R) reader structure"]
impl crate::Readable for CAL {}
#[doc = "`write(|w| ..)` method takes [cal::W](cal::W) writer structure"]
impl crate::Writable for CAL {}
#[doc = "Calibration value register"]
pub mod cal;
