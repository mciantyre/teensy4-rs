#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DCDC Register 0"]
    pub reg0: REG0,
    #[doc = "0x04 - DCDC Register 1"]
    pub reg1: REG1,
    #[doc = "0x08 - DCDC Register 2"]
    pub reg2: REG2,
    #[doc = "0x0c - DCDC Register 3"]
    pub reg3: REG3,
}
#[doc = "DCDC Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg0](reg0) module"]
pub type REG0 = crate::Reg<u32, _REG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG0;
#[doc = "`read()` method returns [reg0::R](reg0::R) reader structure"]
impl crate::Readable for REG0 {}
#[doc = "`write(|w| ..)` method takes [reg0::W](reg0::W) writer structure"]
impl crate::Writable for REG0 {}
#[doc = "DCDC Register 0"]
pub mod reg0;
#[doc = "DCDC Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg1](reg1) module"]
pub type REG1 = crate::Reg<u32, _REG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG1;
#[doc = "`read()` method returns [reg1::R](reg1::R) reader structure"]
impl crate::Readable for REG1 {}
#[doc = "`write(|w| ..)` method takes [reg1::W](reg1::W) writer structure"]
impl crate::Writable for REG1 {}
#[doc = "DCDC Register 1"]
pub mod reg1;
#[doc = "DCDC Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg2](reg2) module"]
pub type REG2 = crate::Reg<u32, _REG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG2;
#[doc = "`read()` method returns [reg2::R](reg2::R) reader structure"]
impl crate::Readable for REG2 {}
#[doc = "`write(|w| ..)` method takes [reg2::W](reg2::W) writer structure"]
impl crate::Writable for REG2 {}
#[doc = "DCDC Register 2"]
pub mod reg2;
#[doc = "DCDC Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg3](reg3) module"]
pub type REG3 = crate::Reg<u32, _REG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG3;
#[doc = "`read()` method returns [reg3::R](reg3::R) reader structure"]
impl crate::Readable for REG3 {}
#[doc = "`write(|w| ..)` method takes [reg3::W](reg3::W) writer structure"]
impl crate::Writable for REG3 {}
#[doc = "DCDC Register 3"]
pub mod reg3;
