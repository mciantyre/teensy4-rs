#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]

include!("../../generic.rs");

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPT Control Register"]
    pub cr: CR,
    #[doc = "0x04 - GPT Prescaler Register"]
    pub pr: PR,
    #[doc = "0x08 - GPT Status Register"]
    pub sr: SR,
    #[doc = "0x0c - GPT Interrupt Register"]
    pub ir: IR,
    #[doc = "0x10 - GPT Output Compare Register 1"]
    pub ocr1: OCR1,
    #[doc = "0x14 - GPT Output Compare Register 2"]
    pub ocr2: OCR2,
    #[doc = "0x18 - GPT Output Compare Register 3"]
    pub ocr3: OCR3,
    #[doc = "0x1c - GPT Input Capture Register 1"]
    pub icr1: ICR1,
    #[doc = "0x20 - GPT Input Capture Register 2"]
    pub icr2: ICR2,
    #[doc = "0x24 - GPT Counter Register"]
    pub cnt: CNT,
}
#[doc = "GPT Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "GPT Control Register"]
pub mod cr;
#[doc = "GPT Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pr](pr) module"]
pub type PR = crate::Reg<u32, _PR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR;
#[doc = "`read()` method returns [pr::R](pr::R) reader structure"]
impl crate::Readable for PR {}
#[doc = "`write(|w| ..)` method takes [pr::W](pr::W) writer structure"]
impl crate::Writable for PR {}
#[doc = "GPT Prescaler Register"]
pub mod pr;
#[doc = "GPT Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "GPT Status Register"]
pub mod sr;
#[doc = "GPT Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ir](ir) module"]
pub type IR = crate::Reg<u32, _IR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IR;
#[doc = "`read()` method returns [ir::R](ir::R) reader structure"]
impl crate::Readable for IR {}
#[doc = "`write(|w| ..)` method takes [ir::W](ir::W) writer structure"]
impl crate::Writable for IR {}
#[doc = "GPT Interrupt Register"]
pub mod ir;
#[doc = "GPT Output Compare Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ocr1](ocr1) module"]
pub type OCR1 = crate::Reg<u32, _OCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCR1;
#[doc = "`read()` method returns [ocr1::R](ocr1::R) reader structure"]
impl crate::Readable for OCR1 {}
#[doc = "`write(|w| ..)` method takes [ocr1::W](ocr1::W) writer structure"]
impl crate::Writable for OCR1 {}
#[doc = "GPT Output Compare Register 1"]
pub mod ocr1;
#[doc = "GPT Output Compare Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ocr2](ocr2) module"]
pub type OCR2 = crate::Reg<u32, _OCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCR2;
#[doc = "`read()` method returns [ocr2::R](ocr2::R) reader structure"]
impl crate::Readable for OCR2 {}
#[doc = "`write(|w| ..)` method takes [ocr2::W](ocr2::W) writer structure"]
impl crate::Writable for OCR2 {}
#[doc = "GPT Output Compare Register 2"]
pub mod ocr2;
#[doc = "GPT Output Compare Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ocr3](ocr3) module"]
pub type OCR3 = crate::Reg<u32, _OCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCR3;
#[doc = "`read()` method returns [ocr3::R](ocr3::R) reader structure"]
impl crate::Readable for OCR3 {}
#[doc = "`write(|w| ..)` method takes [ocr3::W](ocr3::W) writer structure"]
impl crate::Writable for OCR3 {}
#[doc = "GPT Output Compare Register 3"]
pub mod ocr3;
#[doc = "GPT Input Capture Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [icr1](icr1) module"]
pub type ICR1 = crate::Reg<u32, _ICR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR1;
#[doc = "`read()` method returns [icr1::R](icr1::R) reader structure"]
impl crate::Readable for ICR1 {}
#[doc = "GPT Input Capture Register 1"]
pub mod icr1;
#[doc = "GPT Input Capture Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [icr2](icr2) module"]
pub type ICR2 = crate::Reg<u32, _ICR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR2;
#[doc = "`read()` method returns [icr2::R](icr2::R) reader structure"]
impl crate::Readable for ICR2 {}
#[doc = "GPT Input Capture Register 2"]
pub mod icr2;
#[doc = "GPT Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cnt](cnt) module"]
pub type CNT = crate::Reg<u32, _CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT;
#[doc = "`read()` method returns [cnt::R](cnt::R) reader structure"]
impl crate::Readable for CNT {}
#[doc = "GPT Counter Register"]
pub mod cnt;
