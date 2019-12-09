#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

include!("../../generic.rs");

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
    #[doc = "0x10 - GPR4 General Purpose Register"]
    pub gpr4: GPR4,
    #[doc = "0x14 - GPR5 General Purpose Register"]
    pub gpr5: GPR5,
    #[doc = "0x18 - GPR6 General Purpose Register"]
    pub gpr6: GPR6,
    #[doc = "0x1c - GPR7 General Purpose Register"]
    pub gpr7: GPR7,
    #[doc = "0x20 - GPR8 General Purpose Register"]
    pub gpr8: GPR8,
    #[doc = "0x24 - GPR9 General Purpose Register"]
    pub gpr9: GPR9,
    #[doc = "0x28 - GPR10 General Purpose Register"]
    pub gpr10: GPR10,
    #[doc = "0x2c - GPR11 General Purpose Register"]
    pub gpr11: GPR11,
    #[doc = "0x30 - GPR12 General Purpose Register"]
    pub gpr12: GPR12,
    #[doc = "0x34 - GPR13 General Purpose Register"]
    pub gpr13: GPR13,
    #[doc = "0x38 - GPR14 General Purpose Register"]
    pub gpr14: GPR14,
    #[doc = "0x3c - GPR15 General Purpose Register"]
    pub gpr15: GPR15,
    #[doc = "0x40 - GPR16 General Purpose Register"]
    pub gpr16: GPR16,
    #[doc = "0x44 - GPR17 General Purpose Register"]
    pub gpr17: GPR17,
    #[doc = "0x48 - GPR18 General Purpose Register"]
    pub gpr18: GPR18,
    #[doc = "0x4c - GPR19 General Purpose Register"]
    pub gpr19: GPR19,
    #[doc = "0x50 - GPR20 General Purpose Register"]
    pub gpr20: GPR20,
    #[doc = "0x54 - GPR21 General Purpose Register"]
    pub gpr21: GPR21,
    #[doc = "0x58 - GPR22 General Purpose Register"]
    pub gpr22: GPR22,
    #[doc = "0x5c - GPR23 General Purpose Register"]
    pub gpr23: GPR23,
    #[doc = "0x60 - GPR24 General Purpose Register"]
    pub gpr24: GPR24,
    #[doc = "0x64 - GPR25 General Purpose Register"]
    pub gpr25: GPR25,
    #[doc = "0x68 - GPR26 General Purpose Register"]
    pub gpr26: GPR26,
    #[doc = "0x6c - GPR27 General Purpose Register"]
    pub gpr27: GPR27,
    #[doc = "0x70 - GPR28 General Purpose Register"]
    pub gpr28: GPR28,
    #[doc = "0x74 - GPR29 General Purpose Register"]
    pub gpr29: GPR29,
    #[doc = "0x78 - GPR30 General Purpose Register"]
    pub gpr30: GPR30,
    #[doc = "0x7c - GPR31 General Purpose Register"]
    pub gpr31: GPR31,
    #[doc = "0x80 - GPR32 General Purpose Register"]
    pub gpr32: GPR32,
    #[doc = "0x84 - GPR33 General Purpose Register"]
    pub gpr33: GPR33,
    #[doc = "0x88 - GPR34 General Purpose Register"]
    pub gpr34: GPR34,
}
#[doc = "GPR0 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr0](gpr0) module"]
pub type GPR0 = crate::Reg<u32, _GPR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR0;
#[doc = "`read()` method returns [gpr0::R](gpr0::R) reader structure"]
impl crate::Readable for GPR0 {}
#[doc = "GPR0 General Purpose Register"]
pub mod gpr0;
#[doc = "GPR1 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr1](gpr1) module"]
pub type GPR1 = crate::Reg<u32, _GPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR1;
#[doc = "`read()` method returns [gpr1::R](gpr1::R) reader structure"]
impl crate::Readable for GPR1 {}
#[doc = "`write(|w| ..)` method takes [gpr1::W](gpr1::W) writer structure"]
impl crate::Writable for GPR1 {}
#[doc = "GPR1 General Purpose Register"]
pub mod gpr1;
#[doc = "GPR2 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr2](gpr2) module"]
pub type GPR2 = crate::Reg<u32, _GPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR2;
#[doc = "`read()` method returns [gpr2::R](gpr2::R) reader structure"]
impl crate::Readable for GPR2 {}
#[doc = "`write(|w| ..)` method takes [gpr2::W](gpr2::W) writer structure"]
impl crate::Writable for GPR2 {}
#[doc = "GPR2 General Purpose Register"]
pub mod gpr2;
#[doc = "GPR3 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr3](gpr3) module"]
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
#[doc = "GPR4 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr4](gpr4) module"]
pub type GPR4 = crate::Reg<u32, _GPR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR4;
#[doc = "`read()` method returns [gpr4::R](gpr4::R) reader structure"]
impl crate::Readable for GPR4 {}
#[doc = "`write(|w| ..)` method takes [gpr4::W](gpr4::W) writer structure"]
impl crate::Writable for GPR4 {}
#[doc = "GPR4 General Purpose Register"]
pub mod gpr4;
#[doc = "GPR5 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr5](gpr5) module"]
pub type GPR5 = crate::Reg<u32, _GPR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR5;
#[doc = "`read()` method returns [gpr5::R](gpr5::R) reader structure"]
impl crate::Readable for GPR5 {}
#[doc = "`write(|w| ..)` method takes [gpr5::W](gpr5::W) writer structure"]
impl crate::Writable for GPR5 {}
#[doc = "GPR5 General Purpose Register"]
pub mod gpr5;
#[doc = "GPR6 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr6](gpr6) module"]
pub type GPR6 = crate::Reg<u32, _GPR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR6;
#[doc = "`read()` method returns [gpr6::R](gpr6::R) reader structure"]
impl crate::Readable for GPR6 {}
#[doc = "`write(|w| ..)` method takes [gpr6::W](gpr6::W) writer structure"]
impl crate::Writable for GPR6 {}
#[doc = "GPR6 General Purpose Register"]
pub mod gpr6;
#[doc = "GPR7 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr7](gpr7) module"]
pub type GPR7 = crate::Reg<u32, _GPR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR7;
#[doc = "`read()` method returns [gpr7::R](gpr7::R) reader structure"]
impl crate::Readable for GPR7 {}
#[doc = "`write(|w| ..)` method takes [gpr7::W](gpr7::W) writer structure"]
impl crate::Writable for GPR7 {}
#[doc = "GPR7 General Purpose Register"]
pub mod gpr7;
#[doc = "GPR8 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr8](gpr8) module"]
pub type GPR8 = crate::Reg<u32, _GPR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR8;
#[doc = "`read()` method returns [gpr8::R](gpr8::R) reader structure"]
impl crate::Readable for GPR8 {}
#[doc = "`write(|w| ..)` method takes [gpr8::W](gpr8::W) writer structure"]
impl crate::Writable for GPR8 {}
#[doc = "GPR8 General Purpose Register"]
pub mod gpr8;
#[doc = "GPR9 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr9](gpr9) module"]
pub type GPR9 = crate::Reg<u32, _GPR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR9;
#[doc = "`read()` method returns [gpr9::R](gpr9::R) reader structure"]
impl crate::Readable for GPR9 {}
#[doc = "GPR9 General Purpose Register"]
pub mod gpr9;
#[doc = "GPR10 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr10](gpr10) module"]
pub type GPR10 = crate::Reg<u32, _GPR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR10;
#[doc = "`read()` method returns [gpr10::R](gpr10::R) reader structure"]
impl crate::Readable for GPR10 {}
#[doc = "`write(|w| ..)` method takes [gpr10::W](gpr10::W) writer structure"]
impl crate::Writable for GPR10 {}
#[doc = "GPR10 General Purpose Register"]
pub mod gpr10;
#[doc = "GPR11 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr11](gpr11) module"]
pub type GPR11 = crate::Reg<u32, _GPR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR11;
#[doc = "`read()` method returns [gpr11::R](gpr11::R) reader structure"]
impl crate::Readable for GPR11 {}
#[doc = "`write(|w| ..)` method takes [gpr11::W](gpr11::W) writer structure"]
impl crate::Writable for GPR11 {}
#[doc = "GPR11 General Purpose Register"]
pub mod gpr11;
#[doc = "GPR12 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr12](gpr12) module"]
pub type GPR12 = crate::Reg<u32, _GPR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR12;
#[doc = "`read()` method returns [gpr12::R](gpr12::R) reader structure"]
impl crate::Readable for GPR12 {}
#[doc = "`write(|w| ..)` method takes [gpr12::W](gpr12::W) writer structure"]
impl crate::Writable for GPR12 {}
#[doc = "GPR12 General Purpose Register"]
pub mod gpr12;
#[doc = "GPR13 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr13](gpr13) module"]
pub type GPR13 = crate::Reg<u32, _GPR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR13;
#[doc = "`read()` method returns [gpr13::R](gpr13::R) reader structure"]
impl crate::Readable for GPR13 {}
#[doc = "`write(|w| ..)` method takes [gpr13::W](gpr13::W) writer structure"]
impl crate::Writable for GPR13 {}
#[doc = "GPR13 General Purpose Register"]
pub mod gpr13;
#[doc = "GPR14 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr14](gpr14) module"]
pub type GPR14 = crate::Reg<u32, _GPR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR14;
#[doc = "`read()` method returns [gpr14::R](gpr14::R) reader structure"]
impl crate::Readable for GPR14 {}
#[doc = "`write(|w| ..)` method takes [gpr14::W](gpr14::W) writer structure"]
impl crate::Writable for GPR14 {}
#[doc = "GPR14 General Purpose Register"]
pub mod gpr14;
#[doc = "GPR15 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr15](gpr15) module"]
pub type GPR15 = crate::Reg<u32, _GPR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR15;
#[doc = "`read()` method returns [gpr15::R](gpr15::R) reader structure"]
impl crate::Readable for GPR15 {}
#[doc = "GPR15 General Purpose Register"]
pub mod gpr15;
#[doc = "GPR16 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr16](gpr16) module"]
pub type GPR16 = crate::Reg<u32, _GPR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR16;
#[doc = "`read()` method returns [gpr16::R](gpr16::R) reader structure"]
impl crate::Readable for GPR16 {}
#[doc = "`write(|w| ..)` method takes [gpr16::W](gpr16::W) writer structure"]
impl crate::Writable for GPR16 {}
#[doc = "GPR16 General Purpose Register"]
pub mod gpr16;
#[doc = "GPR17 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr17](gpr17) module"]
pub type GPR17 = crate::Reg<u32, _GPR17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR17;
#[doc = "`read()` method returns [gpr17::R](gpr17::R) reader structure"]
impl crate::Readable for GPR17 {}
#[doc = "`write(|w| ..)` method takes [gpr17::W](gpr17::W) writer structure"]
impl crate::Writable for GPR17 {}
#[doc = "GPR17 General Purpose Register"]
pub mod gpr17;
#[doc = "GPR18 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr18](gpr18) module"]
pub type GPR18 = crate::Reg<u32, _GPR18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR18;
#[doc = "`read()` method returns [gpr18::R](gpr18::R) reader structure"]
impl crate::Readable for GPR18 {}
#[doc = "`write(|w| ..)` method takes [gpr18::W](gpr18::W) writer structure"]
impl crate::Writable for GPR18 {}
#[doc = "GPR18 General Purpose Register"]
pub mod gpr18;
#[doc = "GPR19 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr19](gpr19) module"]
pub type GPR19 = crate::Reg<u32, _GPR19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR19;
#[doc = "`read()` method returns [gpr19::R](gpr19::R) reader structure"]
impl crate::Readable for GPR19 {}
#[doc = "`write(|w| ..)` method takes [gpr19::W](gpr19::W) writer structure"]
impl crate::Writable for GPR19 {}
#[doc = "GPR19 General Purpose Register"]
pub mod gpr19;
#[doc = "GPR20 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr20](gpr20) module"]
pub type GPR20 = crate::Reg<u32, _GPR20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR20;
#[doc = "`read()` method returns [gpr20::R](gpr20::R) reader structure"]
impl crate::Readable for GPR20 {}
#[doc = "`write(|w| ..)` method takes [gpr20::W](gpr20::W) writer structure"]
impl crate::Writable for GPR20 {}
#[doc = "GPR20 General Purpose Register"]
pub mod gpr20;
#[doc = "GPR21 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr21](gpr21) module"]
pub type GPR21 = crate::Reg<u32, _GPR21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR21;
#[doc = "`read()` method returns [gpr21::R](gpr21::R) reader structure"]
impl crate::Readable for GPR21 {}
#[doc = "`write(|w| ..)` method takes [gpr21::W](gpr21::W) writer structure"]
impl crate::Writable for GPR21 {}
#[doc = "GPR21 General Purpose Register"]
pub mod gpr21;
#[doc = "GPR22 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr22](gpr22) module"]
pub type GPR22 = crate::Reg<u32, _GPR22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR22;
#[doc = "`read()` method returns [gpr22::R](gpr22::R) reader structure"]
impl crate::Readable for GPR22 {}
#[doc = "`write(|w| ..)` method takes [gpr22::W](gpr22::W) writer structure"]
impl crate::Writable for GPR22 {}
#[doc = "GPR22 General Purpose Register"]
pub mod gpr22;
#[doc = "GPR23 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr23](gpr23) module"]
pub type GPR23 = crate::Reg<u32, _GPR23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR23;
#[doc = "`read()` method returns [gpr23::R](gpr23::R) reader structure"]
impl crate::Readable for GPR23 {}
#[doc = "`write(|w| ..)` method takes [gpr23::W](gpr23::W) writer structure"]
impl crate::Writable for GPR23 {}
#[doc = "GPR23 General Purpose Register"]
pub mod gpr23;
#[doc = "GPR24 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr24](gpr24) module"]
pub type GPR24 = crate::Reg<u32, _GPR24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR24;
#[doc = "`read()` method returns [gpr24::R](gpr24::R) reader structure"]
impl crate::Readable for GPR24 {}
#[doc = "`write(|w| ..)` method takes [gpr24::W](gpr24::W) writer structure"]
impl crate::Writable for GPR24 {}
#[doc = "GPR24 General Purpose Register"]
pub mod gpr24;
#[doc = "GPR25 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr25](gpr25) module"]
pub type GPR25 = crate::Reg<u32, _GPR25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR25;
#[doc = "`read()` method returns [gpr25::R](gpr25::R) reader structure"]
impl crate::Readable for GPR25 {}
#[doc = "`write(|w| ..)` method takes [gpr25::W](gpr25::W) writer structure"]
impl crate::Writable for GPR25 {}
#[doc = "GPR25 General Purpose Register"]
pub mod gpr25;
#[doc = "GPR26 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr26](gpr26) module"]
pub type GPR26 = crate::Reg<u32, _GPR26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR26;
#[doc = "`read()` method returns [gpr26::R](gpr26::R) reader structure"]
impl crate::Readable for GPR26 {}
#[doc = "`write(|w| ..)` method takes [gpr26::W](gpr26::W) writer structure"]
impl crate::Writable for GPR26 {}
#[doc = "GPR26 General Purpose Register"]
pub mod gpr26;
#[doc = "GPR27 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr27](gpr27) module"]
pub type GPR27 = crate::Reg<u32, _GPR27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR27;
#[doc = "`read()` method returns [gpr27::R](gpr27::R) reader structure"]
impl crate::Readable for GPR27 {}
#[doc = "`write(|w| ..)` method takes [gpr27::W](gpr27::W) writer structure"]
impl crate::Writable for GPR27 {}
#[doc = "GPR27 General Purpose Register"]
pub mod gpr27;
#[doc = "GPR28 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr28](gpr28) module"]
pub type GPR28 = crate::Reg<u32, _GPR28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR28;
#[doc = "`read()` method returns [gpr28::R](gpr28::R) reader structure"]
impl crate::Readable for GPR28 {}
#[doc = "`write(|w| ..)` method takes [gpr28::W](gpr28::W) writer structure"]
impl crate::Writable for GPR28 {}
#[doc = "GPR28 General Purpose Register"]
pub mod gpr28;
#[doc = "GPR29 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr29](gpr29) module"]
pub type GPR29 = crate::Reg<u32, _GPR29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR29;
#[doc = "`read()` method returns [gpr29::R](gpr29::R) reader structure"]
impl crate::Readable for GPR29 {}
#[doc = "`write(|w| ..)` method takes [gpr29::W](gpr29::W) writer structure"]
impl crate::Writable for GPR29 {}
#[doc = "GPR29 General Purpose Register"]
pub mod gpr29;
#[doc = "GPR30 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr30](gpr30) module"]
pub type GPR30 = crate::Reg<u32, _GPR30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR30;
#[doc = "`read()` method returns [gpr30::R](gpr30::R) reader structure"]
impl crate::Readable for GPR30 {}
#[doc = "`write(|w| ..)` method takes [gpr30::W](gpr30::W) writer structure"]
impl crate::Writable for GPR30 {}
#[doc = "GPR30 General Purpose Register"]
pub mod gpr30;
#[doc = "GPR31 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr31](gpr31) module"]
pub type GPR31 = crate::Reg<u32, _GPR31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR31;
#[doc = "`read()` method returns [gpr31::R](gpr31::R) reader structure"]
impl crate::Readable for GPR31 {}
#[doc = "`write(|w| ..)` method takes [gpr31::W](gpr31::W) writer structure"]
impl crate::Writable for GPR31 {}
#[doc = "GPR31 General Purpose Register"]
pub mod gpr31;
#[doc = "GPR32 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr32](gpr32) module"]
pub type GPR32 = crate::Reg<u32, _GPR32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR32;
#[doc = "`read()` method returns [gpr32::R](gpr32::R) reader structure"]
impl crate::Readable for GPR32 {}
#[doc = "`write(|w| ..)` method takes [gpr32::W](gpr32::W) writer structure"]
impl crate::Writable for GPR32 {}
#[doc = "GPR32 General Purpose Register"]
pub mod gpr32;
#[doc = "GPR33 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr33](gpr33) module"]
pub type GPR33 = crate::Reg<u32, _GPR33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR33;
#[doc = "`read()` method returns [gpr33::R](gpr33::R) reader structure"]
impl crate::Readable for GPR33 {}
#[doc = "`write(|w| ..)` method takes [gpr33::W](gpr33::W) writer structure"]
impl crate::Writable for GPR33 {}
#[doc = "GPR33 General Purpose Register"]
pub mod gpr33;
#[doc = "GPR34 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpr34](gpr34) module"]
pub type GPR34 = crate::Reg<u32, _GPR34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPR34;
#[doc = "`read()` method returns [gpr34::R](gpr34::R) reader structure"]
impl crate::Readable for GPR34 {}
#[doc = "`write(|w| ..)` method takes [gpr34::W](gpr34::W) writer structure"]
impl crate::Writable for GPR34 {}
#[doc = "GPR34 General Purpose Register"]
pub mod gpr34;
