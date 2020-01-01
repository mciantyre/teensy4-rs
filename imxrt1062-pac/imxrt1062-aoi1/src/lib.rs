#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Boolean Function Term 0 and 1 Configuration Register for EVENTn"]
    pub bfcrt010: BFCRT01,
    #[doc = "0x02 - Boolean Function Term 2 and 3 Configuration Register for EVENTn"]
    pub bfcrt230: BFCRT23,
    #[doc = "0x04 - Boolean Function Term 0 and 1 Configuration Register for EVENTn"]
    pub bfcrt011: BFCRT01,
    #[doc = "0x06 - Boolean Function Term 2 and 3 Configuration Register for EVENTn"]
    pub bfcrt231: BFCRT23,
    #[doc = "0x08 - Boolean Function Term 0 and 1 Configuration Register for EVENTn"]
    pub bfcrt012: BFCRT01,
    #[doc = "0x0a - Boolean Function Term 2 and 3 Configuration Register for EVENTn"]
    pub bfcrt232: BFCRT23,
    #[doc = "0x0c - Boolean Function Term 0 and 1 Configuration Register for EVENTn"]
    pub bfcrt013: BFCRT01,
    #[doc = "0x0e - Boolean Function Term 2 and 3 Configuration Register for EVENTn"]
    pub bfcrt233: BFCRT23,
}
#[doc = "Boolean Function Term 0 and 1 Configuration Register for EVENTn\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bfcrt01](bfcrt01) module"]
pub type BFCRT01 = crate::Reg<u16, _BFCRT01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BFCRT01;
#[doc = "`read()` method returns [bfcrt01::R](bfcrt01::R) reader structure"]
impl crate::Readable for BFCRT01 {}
#[doc = "`write(|w| ..)` method takes [bfcrt01::W](bfcrt01::W) writer structure"]
impl crate::Writable for BFCRT01 {}
#[doc = "Boolean Function Term 0 and 1 Configuration Register for EVENTn"]
pub mod bfcrt01;
#[doc = "Boolean Function Term 2 and 3 Configuration Register for EVENTn\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bfcrt23](bfcrt23) module"]
pub type BFCRT23 = crate::Reg<u16, _BFCRT23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BFCRT23;
#[doc = "`read()` method returns [bfcrt23::R](bfcrt23::R) reader structure"]
impl crate::Readable for BFCRT23 {}
#[doc = "`write(|w| ..)` method takes [bfcrt23::W](bfcrt23::W) writer structure"]
impl crate::Writable for BFCRT23 {}
#[doc = "Boolean Function Term 2 and 3 Configuration Register for EVENTn"]
pub mod bfcrt23;
