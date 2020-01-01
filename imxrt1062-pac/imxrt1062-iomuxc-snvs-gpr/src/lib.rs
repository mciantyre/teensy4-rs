#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPR0 General Purpose Register"]
    pub gpr0: GPR0,
    #[doc = "0x04 - GPR1 General Purpose Register"]
    pub gpr1: GPR1,
    #[doc = "0x08 - GPR2 General Purpose Register"]
    pub gpr2: GPR2,
    #[doc = "0x0c - GPR3 General Purpose Register"]
    pub gpr3: GPR3,
}
#[doc = "GPR0 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr0](gpr0) module"]
pub type GPR0 = crate::Reg<u32, _GPR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR0;
#[doc = "`read()` method returns [gpr0::R](gpr0::R) reader structure"]
impl crate::Readable for GPR0 {}
#[doc = "GPR0 General Purpose Register"]
pub mod gpr0;
#[doc = "GPR1 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr1](gpr1) module"]
pub type GPR1 = crate::Reg<u32, _GPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR1;
#[doc = "`read()` method returns [gpr1::R](gpr1::R) reader structure"]
impl crate::Readable for GPR1 {}
#[doc = "GPR1 General Purpose Register"]
pub mod gpr1;
#[doc = "GPR2 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr2](gpr2) module"]
pub type GPR2 = crate::Reg<u32, _GPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR2;
#[doc = "`read()` method returns [gpr2::R](gpr2::R) reader structure"]
impl crate::Readable for GPR2 {}
#[doc = "GPR2 General Purpose Register"]
pub mod gpr2;
#[doc = "GPR3 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr3](gpr3) module"]
pub type GPR3 = crate::Reg<u32, _GPR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR3;
#[doc = "`read()` method returns [gpr3::R](gpr3::R) reader structure"]
impl crate::Readable for GPR3 {}
#[doc = "`write(|w| ..)` method takes [gpr3::W](gpr3::W) writer structure"]
impl crate::Writable for GPR3 {}
#[doc = "GPR3 General Purpose Register"]
pub mod gpr3;
