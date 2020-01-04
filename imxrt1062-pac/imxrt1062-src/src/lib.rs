#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRC Control Register"]
    pub scr: SCR,
    #[doc = "0x04 - SRC Boot Mode Register 1"]
    pub sbmr1: SBMR1,
    #[doc = "0x08 - SRC Reset Status Register"]
    pub srsr: SRSR,
    _reserved3: [u8; 16usize],
    #[doc = "0x1c - SRC Boot Mode Register 2"]
    pub sbmr2: SBMR2,
    #[doc = "0x20 - SRC General Purpose Register 1"]
    pub gpr1: GPR1,
    #[doc = "0x24 - SRC General Purpose Register 2"]
    pub gpr2: GPR2,
    #[doc = "0x28 - SRC General Purpose Register 3"]
    pub gpr3: GPR3,
    #[doc = "0x2c - SRC General Purpose Register 4"]
    pub gpr4: GPR4,
    #[doc = "0x30 - SRC General Purpose Register 5"]
    pub gpr5: GPR5,
    #[doc = "0x34 - SRC General Purpose Register 6"]
    pub gpr6: GPR6,
    #[doc = "0x38 - SRC General Purpose Register 7"]
    pub gpr7: GPR7,
    #[doc = "0x3c - SRC General Purpose Register 8"]
    pub gpr8: GPR8,
    #[doc = "0x40 - SRC General Purpose Register 9"]
    pub gpr9: GPR9,
    #[doc = "0x44 - SRC General Purpose Register 10"]
    pub gpr10: GPR10,
}
#[doc = "SRC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](scr) module"]
pub type SCR = crate::Reg<u32, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
#[doc = "`read()` method returns [scr::R](scr::R) reader structure"]
impl crate::Readable for SCR {}
#[doc = "`write(|w| ..)` method takes [scr::W](scr::W) writer structure"]
impl crate::Writable for SCR {}
#[doc = "SRC Control Register"]
pub mod scr;
#[doc = "SRC Boot Mode Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sbmr1](sbmr1) module"]
pub type SBMR1 = crate::Reg<u32, _SBMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBMR1;
#[doc = "`read()` method returns [sbmr1::R](sbmr1::R) reader structure"]
impl crate::Readable for SBMR1 {}
#[doc = "SRC Boot Mode Register 1"]
pub mod sbmr1;
#[doc = "SRC Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srsr](srsr) module"]
pub type SRSR = crate::Reg<u32, _SRSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRSR;
#[doc = "`read()` method returns [srsr::R](srsr::R) reader structure"]
impl crate::Readable for SRSR {}
#[doc = "`write(|w| ..)` method takes [srsr::W](srsr::W) writer structure"]
impl crate::Writable for SRSR {}
#[doc = "SRC Reset Status Register"]
pub mod srsr;
#[doc = "SRC Boot Mode Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sbmr2](sbmr2) module"]
pub type SBMR2 = crate::Reg<u32, _SBMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBMR2;
#[doc = "`read()` method returns [sbmr2::R](sbmr2::R) reader structure"]
impl crate::Readable for SBMR2 {}
#[doc = "SRC Boot Mode Register 2"]
pub mod sbmr2;
#[doc = "SRC General Purpose Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr1](gpr1) module"]
pub type GPR1 = crate::Reg<u32, _GPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR1;
#[doc = "`read()` method returns [gpr1::R](gpr1::R) reader structure"]
impl crate::Readable for GPR1 {}
#[doc = "`write(|w| ..)` method takes [gpr1::W](gpr1::W) writer structure"]
impl crate::Writable for GPR1 {}
#[doc = "SRC General Purpose Register 1"]
pub mod gpr1;
#[doc = "SRC General Purpose Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr2](gpr2) module"]
pub type GPR2 = crate::Reg<u32, _GPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR2;
#[doc = "`read()` method returns [gpr2::R](gpr2::R) reader structure"]
impl crate::Readable for GPR2 {}
#[doc = "`write(|w| ..)` method takes [gpr2::W](gpr2::W) writer structure"]
impl crate::Writable for GPR2 {}
#[doc = "SRC General Purpose Register 2"]
pub mod gpr2;
#[doc = "SRC General Purpose Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr3](gpr3) module"]
pub type GPR3 = crate::Reg<u32, _GPR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR3;
#[doc = "`read()` method returns [gpr3::R](gpr3::R) reader structure"]
impl crate::Readable for GPR3 {}
#[doc = "`write(|w| ..)` method takes [gpr3::W](gpr3::W) writer structure"]
impl crate::Writable for GPR3 {}
#[doc = "SRC General Purpose Register 3"]
pub mod gpr3;
#[doc = "SRC General Purpose Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr4](gpr4) module"]
pub type GPR4 = crate::Reg<u32, _GPR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR4;
#[doc = "`read()` method returns [gpr4::R](gpr4::R) reader structure"]
impl crate::Readable for GPR4 {}
#[doc = "`write(|w| ..)` method takes [gpr4::W](gpr4::W) writer structure"]
impl crate::Writable for GPR4 {}
#[doc = "SRC General Purpose Register 4"]
pub mod gpr4;
#[doc = "SRC General Purpose Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr5](gpr5) module"]
pub type GPR5 = crate::Reg<u32, _GPR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR5;
#[doc = "`read()` method returns [gpr5::R](gpr5::R) reader structure"]
impl crate::Readable for GPR5 {}
#[doc = "`write(|w| ..)` method takes [gpr5::W](gpr5::W) writer structure"]
impl crate::Writable for GPR5 {}
#[doc = "SRC General Purpose Register 5"]
pub mod gpr5;
#[doc = "SRC General Purpose Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr6](gpr6) module"]
pub type GPR6 = crate::Reg<u32, _GPR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR6;
#[doc = "`read()` method returns [gpr6::R](gpr6::R) reader structure"]
impl crate::Readable for GPR6 {}
#[doc = "`write(|w| ..)` method takes [gpr6::W](gpr6::W) writer structure"]
impl crate::Writable for GPR6 {}
#[doc = "SRC General Purpose Register 6"]
pub mod gpr6;
#[doc = "SRC General Purpose Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr7](gpr7) module"]
pub type GPR7 = crate::Reg<u32, _GPR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR7;
#[doc = "`read()` method returns [gpr7::R](gpr7::R) reader structure"]
impl crate::Readable for GPR7 {}
#[doc = "`write(|w| ..)` method takes [gpr7::W](gpr7::W) writer structure"]
impl crate::Writable for GPR7 {}
#[doc = "SRC General Purpose Register 7"]
pub mod gpr7;
#[doc = "SRC General Purpose Register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr8](gpr8) module"]
pub type GPR8 = crate::Reg<u32, _GPR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR8;
#[doc = "`read()` method returns [gpr8::R](gpr8::R) reader structure"]
impl crate::Readable for GPR8 {}
#[doc = "`write(|w| ..)` method takes [gpr8::W](gpr8::W) writer structure"]
impl crate::Writable for GPR8 {}
#[doc = "SRC General Purpose Register 8"]
pub mod gpr8;
#[doc = "SRC General Purpose Register 9\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr9](gpr9) module"]
pub type GPR9 = crate::Reg<u32, _GPR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR9;
#[doc = "`read()` method returns [gpr9::R](gpr9::R) reader structure"]
impl crate::Readable for GPR9 {}
#[doc = "SRC General Purpose Register 9"]
pub mod gpr9;
#[doc = "SRC General Purpose Register 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr10](gpr10) module"]
pub type GPR10 = crate::Reg<u32, _GPR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR10;
#[doc = "`read()` method returns [gpr10::R](gpr10::R) reader structure"]
impl crate::Readable for GPR10 {}
#[doc = "`write(|w| ..)` method takes [gpr10::W](gpr10::W) writer structure"]
impl crate::Writable for GPR10 {}
#[doc = "SRC General Purpose Register 10"]
pub mod gpr10;
