#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Crossbar A Select Register 0"]
    pub sel0: SEL0,
    #[doc = "0x02 - Crossbar A Select Register 1"]
    pub sel1: SEL1,
    #[doc = "0x04 - Crossbar A Select Register 2"]
    pub sel2: SEL2,
    #[doc = "0x06 - Crossbar A Select Register 3"]
    pub sel3: SEL3,
    #[doc = "0x08 - Crossbar A Select Register 4"]
    pub sel4: SEL4,
    #[doc = "0x0a - Crossbar A Select Register 5"]
    pub sel5: SEL5,
    #[doc = "0x0c - Crossbar A Select Register 6"]
    pub sel6: SEL6,
    #[doc = "0x0e - Crossbar A Select Register 7"]
    pub sel7: SEL7,
    #[doc = "0x10 - Crossbar A Select Register 8"]
    pub sel8: SEL8,
    #[doc = "0x12 - Crossbar A Select Register 9"]
    pub sel9: SEL9,
    #[doc = "0x14 - Crossbar A Select Register 10"]
    pub sel10: SEL10,
    #[doc = "0x16 - Crossbar A Select Register 11"]
    pub sel11: SEL11,
    #[doc = "0x18 - Crossbar A Select Register 12"]
    pub sel12: SEL12,
    #[doc = "0x1a - Crossbar A Select Register 13"]
    pub sel13: SEL13,
    #[doc = "0x1c - Crossbar A Select Register 14"]
    pub sel14: SEL14,
    #[doc = "0x1e - Crossbar A Select Register 15"]
    pub sel15: SEL15,
    #[doc = "0x20 - Crossbar A Select Register 16"]
    pub sel16: SEL16,
    #[doc = "0x22 - Crossbar A Select Register 17"]
    pub sel17: SEL17,
    #[doc = "0x24 - Crossbar A Select Register 18"]
    pub sel18: SEL18,
    #[doc = "0x26 - Crossbar A Select Register 19"]
    pub sel19: SEL19,
    #[doc = "0x28 - Crossbar A Select Register 20"]
    pub sel20: SEL20,
    #[doc = "0x2a - Crossbar A Select Register 21"]
    pub sel21: SEL21,
    #[doc = "0x2c - Crossbar A Select Register 22"]
    pub sel22: SEL22,
    #[doc = "0x2e - Crossbar A Select Register 23"]
    pub sel23: SEL23,
    #[doc = "0x30 - Crossbar A Select Register 24"]
    pub sel24: SEL24,
    #[doc = "0x32 - Crossbar A Select Register 25"]
    pub sel25: SEL25,
    #[doc = "0x34 - Crossbar A Select Register 26"]
    pub sel26: SEL26,
    #[doc = "0x36 - Crossbar A Select Register 27"]
    pub sel27: SEL27,
    #[doc = "0x38 - Crossbar A Select Register 28"]
    pub sel28: SEL28,
    #[doc = "0x3a - Crossbar A Select Register 29"]
    pub sel29: SEL29,
    #[doc = "0x3c - Crossbar A Select Register 30"]
    pub sel30: SEL30,
    #[doc = "0x3e - Crossbar A Select Register 31"]
    pub sel31: SEL31,
    #[doc = "0x40 - Crossbar A Select Register 32"]
    pub sel32: SEL32,
    #[doc = "0x42 - Crossbar A Select Register 33"]
    pub sel33: SEL33,
    #[doc = "0x44 - Crossbar A Select Register 34"]
    pub sel34: SEL34,
    #[doc = "0x46 - Crossbar A Select Register 35"]
    pub sel35: SEL35,
    #[doc = "0x48 - Crossbar A Select Register 36"]
    pub sel36: SEL36,
    #[doc = "0x4a - Crossbar A Select Register 37"]
    pub sel37: SEL37,
    #[doc = "0x4c - Crossbar A Select Register 38"]
    pub sel38: SEL38,
    #[doc = "0x4e - Crossbar A Select Register 39"]
    pub sel39: SEL39,
    #[doc = "0x50 - Crossbar A Select Register 40"]
    pub sel40: SEL40,
    #[doc = "0x52 - Crossbar A Select Register 41"]
    pub sel41: SEL41,
    #[doc = "0x54 - Crossbar A Select Register 42"]
    pub sel42: SEL42,
    #[doc = "0x56 - Crossbar A Select Register 43"]
    pub sel43: SEL43,
    #[doc = "0x58 - Crossbar A Select Register 44"]
    pub sel44: SEL44,
    #[doc = "0x5a - Crossbar A Select Register 45"]
    pub sel45: SEL45,
    #[doc = "0x5c - Crossbar A Select Register 46"]
    pub sel46: SEL46,
    #[doc = "0x5e - Crossbar A Select Register 47"]
    pub sel47: SEL47,
    #[doc = "0x60 - Crossbar A Select Register 48"]
    pub sel48: SEL48,
    #[doc = "0x62 - Crossbar A Select Register 49"]
    pub sel49: SEL49,
    #[doc = "0x64 - Crossbar A Select Register 50"]
    pub sel50: SEL50,
    #[doc = "0x66 - Crossbar A Select Register 51"]
    pub sel51: SEL51,
    #[doc = "0x68 - Crossbar A Select Register 52"]
    pub sel52: SEL52,
    #[doc = "0x6a - Crossbar A Select Register 53"]
    pub sel53: SEL53,
    #[doc = "0x6c - Crossbar A Select Register 54"]
    pub sel54: SEL54,
    #[doc = "0x6e - Crossbar A Select Register 55"]
    pub sel55: SEL55,
    #[doc = "0x70 - Crossbar A Select Register 56"]
    pub sel56: SEL56,
    #[doc = "0x72 - Crossbar A Select Register 57"]
    pub sel57: SEL57,
    #[doc = "0x74 - Crossbar A Select Register 58"]
    pub sel58: SEL58,
    #[doc = "0x76 - Crossbar A Select Register 59"]
    pub sel59: SEL59,
    #[doc = "0x78 - Crossbar A Select Register 60"]
    pub sel60: SEL60,
    #[doc = "0x7a - Crossbar A Select Register 61"]
    pub sel61: SEL61,
    #[doc = "0x7c - Crossbar A Select Register 62"]
    pub sel62: SEL62,
    #[doc = "0x7e - Crossbar A Select Register 63"]
    pub sel63: SEL63,
    #[doc = "0x80 - Crossbar A Select Register 64"]
    pub sel64: SEL64,
    #[doc = "0x82 - Crossbar A Select Register 65"]
    pub sel65: SEL65,
    #[doc = "0x84 - Crossbar A Control Register 0"]
    pub ctrl0: CTRL0,
    #[doc = "0x86 - Crossbar A Control Register 1"]
    pub ctrl1: CTRL1,
}
#[doc = "Crossbar A Select Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel0](sel0) module"]
pub type SEL0 = crate::Reg<u16, _SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL0;
#[doc = "`read()` method returns [sel0::R](sel0::R) reader structure"]
impl crate::Readable for SEL0 {}
#[doc = "`write(|w| ..)` method takes [sel0::W](sel0::W) writer structure"]
impl crate::Writable for SEL0 {}
#[doc = "Crossbar A Select Register 0"]
pub mod sel0;
#[doc = "Crossbar A Select Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel1](sel1) module"]
pub type SEL1 = crate::Reg<u16, _SEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL1;
#[doc = "`read()` method returns [sel1::R](sel1::R) reader structure"]
impl crate::Readable for SEL1 {}
#[doc = "`write(|w| ..)` method takes [sel1::W](sel1::W) writer structure"]
impl crate::Writable for SEL1 {}
#[doc = "Crossbar A Select Register 1"]
pub mod sel1;
#[doc = "Crossbar A Select Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel2](sel2) module"]
pub type SEL2 = crate::Reg<u16, _SEL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL2;
#[doc = "`read()` method returns [sel2::R](sel2::R) reader structure"]
impl crate::Readable for SEL2 {}
#[doc = "`write(|w| ..)` method takes [sel2::W](sel2::W) writer structure"]
impl crate::Writable for SEL2 {}
#[doc = "Crossbar A Select Register 2"]
pub mod sel2;
#[doc = "Crossbar A Select Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel3](sel3) module"]
pub type SEL3 = crate::Reg<u16, _SEL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL3;
#[doc = "`read()` method returns [sel3::R](sel3::R) reader structure"]
impl crate::Readable for SEL3 {}
#[doc = "`write(|w| ..)` method takes [sel3::W](sel3::W) writer structure"]
impl crate::Writable for SEL3 {}
#[doc = "Crossbar A Select Register 3"]
pub mod sel3;
#[doc = "Crossbar A Select Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel4](sel4) module"]
pub type SEL4 = crate::Reg<u16, _SEL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL4;
#[doc = "`read()` method returns [sel4::R](sel4::R) reader structure"]
impl crate::Readable for SEL4 {}
#[doc = "`write(|w| ..)` method takes [sel4::W](sel4::W) writer structure"]
impl crate::Writable for SEL4 {}
#[doc = "Crossbar A Select Register 4"]
pub mod sel4;
#[doc = "Crossbar A Select Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel5](sel5) module"]
pub type SEL5 = crate::Reg<u16, _SEL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL5;
#[doc = "`read()` method returns [sel5::R](sel5::R) reader structure"]
impl crate::Readable for SEL5 {}
#[doc = "`write(|w| ..)` method takes [sel5::W](sel5::W) writer structure"]
impl crate::Writable for SEL5 {}
#[doc = "Crossbar A Select Register 5"]
pub mod sel5;
#[doc = "Crossbar A Select Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel6](sel6) module"]
pub type SEL6 = crate::Reg<u16, _SEL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL6;
#[doc = "`read()` method returns [sel6::R](sel6::R) reader structure"]
impl crate::Readable for SEL6 {}
#[doc = "`write(|w| ..)` method takes [sel6::W](sel6::W) writer structure"]
impl crate::Writable for SEL6 {}
#[doc = "Crossbar A Select Register 6"]
pub mod sel6;
#[doc = "Crossbar A Select Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel7](sel7) module"]
pub type SEL7 = crate::Reg<u16, _SEL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL7;
#[doc = "`read()` method returns [sel7::R](sel7::R) reader structure"]
impl crate::Readable for SEL7 {}
#[doc = "`write(|w| ..)` method takes [sel7::W](sel7::W) writer structure"]
impl crate::Writable for SEL7 {}
#[doc = "Crossbar A Select Register 7"]
pub mod sel7;
#[doc = "Crossbar A Select Register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel8](sel8) module"]
pub type SEL8 = crate::Reg<u16, _SEL8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL8;
#[doc = "`read()` method returns [sel8::R](sel8::R) reader structure"]
impl crate::Readable for SEL8 {}
#[doc = "`write(|w| ..)` method takes [sel8::W](sel8::W) writer structure"]
impl crate::Writable for SEL8 {}
#[doc = "Crossbar A Select Register 8"]
pub mod sel8;
#[doc = "Crossbar A Select Register 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel9](sel9) module"]
pub type SEL9 = crate::Reg<u16, _SEL9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL9;
#[doc = "`read()` method returns [sel9::R](sel9::R) reader structure"]
impl crate::Readable for SEL9 {}
#[doc = "`write(|w| ..)` method takes [sel9::W](sel9::W) writer structure"]
impl crate::Writable for SEL9 {}
#[doc = "Crossbar A Select Register 9"]
pub mod sel9;
#[doc = "Crossbar A Select Register 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel10](sel10) module"]
pub type SEL10 = crate::Reg<u16, _SEL10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL10;
#[doc = "`read()` method returns [sel10::R](sel10::R) reader structure"]
impl crate::Readable for SEL10 {}
#[doc = "`write(|w| ..)` method takes [sel10::W](sel10::W) writer structure"]
impl crate::Writable for SEL10 {}
#[doc = "Crossbar A Select Register 10"]
pub mod sel10;
#[doc = "Crossbar A Select Register 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel11](sel11) module"]
pub type SEL11 = crate::Reg<u16, _SEL11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL11;
#[doc = "`read()` method returns [sel11::R](sel11::R) reader structure"]
impl crate::Readable for SEL11 {}
#[doc = "`write(|w| ..)` method takes [sel11::W](sel11::W) writer structure"]
impl crate::Writable for SEL11 {}
#[doc = "Crossbar A Select Register 11"]
pub mod sel11;
#[doc = "Crossbar A Select Register 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel12](sel12) module"]
pub type SEL12 = crate::Reg<u16, _SEL12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL12;
#[doc = "`read()` method returns [sel12::R](sel12::R) reader structure"]
impl crate::Readable for SEL12 {}
#[doc = "`write(|w| ..)` method takes [sel12::W](sel12::W) writer structure"]
impl crate::Writable for SEL12 {}
#[doc = "Crossbar A Select Register 12"]
pub mod sel12;
#[doc = "Crossbar A Select Register 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel13](sel13) module"]
pub type SEL13 = crate::Reg<u16, _SEL13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL13;
#[doc = "`read()` method returns [sel13::R](sel13::R) reader structure"]
impl crate::Readable for SEL13 {}
#[doc = "`write(|w| ..)` method takes [sel13::W](sel13::W) writer structure"]
impl crate::Writable for SEL13 {}
#[doc = "Crossbar A Select Register 13"]
pub mod sel13;
#[doc = "Crossbar A Select Register 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel14](sel14) module"]
pub type SEL14 = crate::Reg<u16, _SEL14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL14;
#[doc = "`read()` method returns [sel14::R](sel14::R) reader structure"]
impl crate::Readable for SEL14 {}
#[doc = "`write(|w| ..)` method takes [sel14::W](sel14::W) writer structure"]
impl crate::Writable for SEL14 {}
#[doc = "Crossbar A Select Register 14"]
pub mod sel14;
#[doc = "Crossbar A Select Register 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel15](sel15) module"]
pub type SEL15 = crate::Reg<u16, _SEL15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL15;
#[doc = "`read()` method returns [sel15::R](sel15::R) reader structure"]
impl crate::Readable for SEL15 {}
#[doc = "`write(|w| ..)` method takes [sel15::W](sel15::W) writer structure"]
impl crate::Writable for SEL15 {}
#[doc = "Crossbar A Select Register 15"]
pub mod sel15;
#[doc = "Crossbar A Select Register 16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel16](sel16) module"]
pub type SEL16 = crate::Reg<u16, _SEL16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL16;
#[doc = "`read()` method returns [sel16::R](sel16::R) reader structure"]
impl crate::Readable for SEL16 {}
#[doc = "`write(|w| ..)` method takes [sel16::W](sel16::W) writer structure"]
impl crate::Writable for SEL16 {}
#[doc = "Crossbar A Select Register 16"]
pub mod sel16;
#[doc = "Crossbar A Select Register 17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel17](sel17) module"]
pub type SEL17 = crate::Reg<u16, _SEL17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL17;
#[doc = "`read()` method returns [sel17::R](sel17::R) reader structure"]
impl crate::Readable for SEL17 {}
#[doc = "`write(|w| ..)` method takes [sel17::W](sel17::W) writer structure"]
impl crate::Writable for SEL17 {}
#[doc = "Crossbar A Select Register 17"]
pub mod sel17;
#[doc = "Crossbar A Select Register 18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel18](sel18) module"]
pub type SEL18 = crate::Reg<u16, _SEL18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL18;
#[doc = "`read()` method returns [sel18::R](sel18::R) reader structure"]
impl crate::Readable for SEL18 {}
#[doc = "`write(|w| ..)` method takes [sel18::W](sel18::W) writer structure"]
impl crate::Writable for SEL18 {}
#[doc = "Crossbar A Select Register 18"]
pub mod sel18;
#[doc = "Crossbar A Select Register 19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel19](sel19) module"]
pub type SEL19 = crate::Reg<u16, _SEL19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL19;
#[doc = "`read()` method returns [sel19::R](sel19::R) reader structure"]
impl crate::Readable for SEL19 {}
#[doc = "`write(|w| ..)` method takes [sel19::W](sel19::W) writer structure"]
impl crate::Writable for SEL19 {}
#[doc = "Crossbar A Select Register 19"]
pub mod sel19;
#[doc = "Crossbar A Select Register 20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel20](sel20) module"]
pub type SEL20 = crate::Reg<u16, _SEL20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL20;
#[doc = "`read()` method returns [sel20::R](sel20::R) reader structure"]
impl crate::Readable for SEL20 {}
#[doc = "`write(|w| ..)` method takes [sel20::W](sel20::W) writer structure"]
impl crate::Writable for SEL20 {}
#[doc = "Crossbar A Select Register 20"]
pub mod sel20;
#[doc = "Crossbar A Select Register 21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel21](sel21) module"]
pub type SEL21 = crate::Reg<u16, _SEL21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL21;
#[doc = "`read()` method returns [sel21::R](sel21::R) reader structure"]
impl crate::Readable for SEL21 {}
#[doc = "`write(|w| ..)` method takes [sel21::W](sel21::W) writer structure"]
impl crate::Writable for SEL21 {}
#[doc = "Crossbar A Select Register 21"]
pub mod sel21;
#[doc = "Crossbar A Select Register 22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel22](sel22) module"]
pub type SEL22 = crate::Reg<u16, _SEL22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL22;
#[doc = "`read()` method returns [sel22::R](sel22::R) reader structure"]
impl crate::Readable for SEL22 {}
#[doc = "`write(|w| ..)` method takes [sel22::W](sel22::W) writer structure"]
impl crate::Writable for SEL22 {}
#[doc = "Crossbar A Select Register 22"]
pub mod sel22;
#[doc = "Crossbar A Select Register 23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel23](sel23) module"]
pub type SEL23 = crate::Reg<u16, _SEL23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL23;
#[doc = "`read()` method returns [sel23::R](sel23::R) reader structure"]
impl crate::Readable for SEL23 {}
#[doc = "`write(|w| ..)` method takes [sel23::W](sel23::W) writer structure"]
impl crate::Writable for SEL23 {}
#[doc = "Crossbar A Select Register 23"]
pub mod sel23;
#[doc = "Crossbar A Select Register 24\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel24](sel24) module"]
pub type SEL24 = crate::Reg<u16, _SEL24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL24;
#[doc = "`read()` method returns [sel24::R](sel24::R) reader structure"]
impl crate::Readable for SEL24 {}
#[doc = "`write(|w| ..)` method takes [sel24::W](sel24::W) writer structure"]
impl crate::Writable for SEL24 {}
#[doc = "Crossbar A Select Register 24"]
pub mod sel24;
#[doc = "Crossbar A Select Register 25\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel25](sel25) module"]
pub type SEL25 = crate::Reg<u16, _SEL25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL25;
#[doc = "`read()` method returns [sel25::R](sel25::R) reader structure"]
impl crate::Readable for SEL25 {}
#[doc = "`write(|w| ..)` method takes [sel25::W](sel25::W) writer structure"]
impl crate::Writable for SEL25 {}
#[doc = "Crossbar A Select Register 25"]
pub mod sel25;
#[doc = "Crossbar A Select Register 26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel26](sel26) module"]
pub type SEL26 = crate::Reg<u16, _SEL26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL26;
#[doc = "`read()` method returns [sel26::R](sel26::R) reader structure"]
impl crate::Readable for SEL26 {}
#[doc = "`write(|w| ..)` method takes [sel26::W](sel26::W) writer structure"]
impl crate::Writable for SEL26 {}
#[doc = "Crossbar A Select Register 26"]
pub mod sel26;
#[doc = "Crossbar A Select Register 27\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel27](sel27) module"]
pub type SEL27 = crate::Reg<u16, _SEL27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL27;
#[doc = "`read()` method returns [sel27::R](sel27::R) reader structure"]
impl crate::Readable for SEL27 {}
#[doc = "`write(|w| ..)` method takes [sel27::W](sel27::W) writer structure"]
impl crate::Writable for SEL27 {}
#[doc = "Crossbar A Select Register 27"]
pub mod sel27;
#[doc = "Crossbar A Select Register 28\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel28](sel28) module"]
pub type SEL28 = crate::Reg<u16, _SEL28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL28;
#[doc = "`read()` method returns [sel28::R](sel28::R) reader structure"]
impl crate::Readable for SEL28 {}
#[doc = "`write(|w| ..)` method takes [sel28::W](sel28::W) writer structure"]
impl crate::Writable for SEL28 {}
#[doc = "Crossbar A Select Register 28"]
pub mod sel28;
#[doc = "Crossbar A Select Register 29\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel29](sel29) module"]
pub type SEL29 = crate::Reg<u16, _SEL29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL29;
#[doc = "`read()` method returns [sel29::R](sel29::R) reader structure"]
impl crate::Readable for SEL29 {}
#[doc = "`write(|w| ..)` method takes [sel29::W](sel29::W) writer structure"]
impl crate::Writable for SEL29 {}
#[doc = "Crossbar A Select Register 29"]
pub mod sel29;
#[doc = "Crossbar A Select Register 30\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel30](sel30) module"]
pub type SEL30 = crate::Reg<u16, _SEL30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL30;
#[doc = "`read()` method returns [sel30::R](sel30::R) reader structure"]
impl crate::Readable for SEL30 {}
#[doc = "`write(|w| ..)` method takes [sel30::W](sel30::W) writer structure"]
impl crate::Writable for SEL30 {}
#[doc = "Crossbar A Select Register 30"]
pub mod sel30;
#[doc = "Crossbar A Select Register 31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel31](sel31) module"]
pub type SEL31 = crate::Reg<u16, _SEL31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL31;
#[doc = "`read()` method returns [sel31::R](sel31::R) reader structure"]
impl crate::Readable for SEL31 {}
#[doc = "`write(|w| ..)` method takes [sel31::W](sel31::W) writer structure"]
impl crate::Writable for SEL31 {}
#[doc = "Crossbar A Select Register 31"]
pub mod sel31;
#[doc = "Crossbar A Select Register 32\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel32](sel32) module"]
pub type SEL32 = crate::Reg<u16, _SEL32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL32;
#[doc = "`read()` method returns [sel32::R](sel32::R) reader structure"]
impl crate::Readable for SEL32 {}
#[doc = "`write(|w| ..)` method takes [sel32::W](sel32::W) writer structure"]
impl crate::Writable for SEL32 {}
#[doc = "Crossbar A Select Register 32"]
pub mod sel32;
#[doc = "Crossbar A Select Register 33\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel33](sel33) module"]
pub type SEL33 = crate::Reg<u16, _SEL33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL33;
#[doc = "`read()` method returns [sel33::R](sel33::R) reader structure"]
impl crate::Readable for SEL33 {}
#[doc = "`write(|w| ..)` method takes [sel33::W](sel33::W) writer structure"]
impl crate::Writable for SEL33 {}
#[doc = "Crossbar A Select Register 33"]
pub mod sel33;
#[doc = "Crossbar A Select Register 34\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel34](sel34) module"]
pub type SEL34 = crate::Reg<u16, _SEL34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL34;
#[doc = "`read()` method returns [sel34::R](sel34::R) reader structure"]
impl crate::Readable for SEL34 {}
#[doc = "`write(|w| ..)` method takes [sel34::W](sel34::W) writer structure"]
impl crate::Writable for SEL34 {}
#[doc = "Crossbar A Select Register 34"]
pub mod sel34;
#[doc = "Crossbar A Select Register 35\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel35](sel35) module"]
pub type SEL35 = crate::Reg<u16, _SEL35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL35;
#[doc = "`read()` method returns [sel35::R](sel35::R) reader structure"]
impl crate::Readable for SEL35 {}
#[doc = "`write(|w| ..)` method takes [sel35::W](sel35::W) writer structure"]
impl crate::Writable for SEL35 {}
#[doc = "Crossbar A Select Register 35"]
pub mod sel35;
#[doc = "Crossbar A Select Register 36\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel36](sel36) module"]
pub type SEL36 = crate::Reg<u16, _SEL36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL36;
#[doc = "`read()` method returns [sel36::R](sel36::R) reader structure"]
impl crate::Readable for SEL36 {}
#[doc = "`write(|w| ..)` method takes [sel36::W](sel36::W) writer structure"]
impl crate::Writable for SEL36 {}
#[doc = "Crossbar A Select Register 36"]
pub mod sel36;
#[doc = "Crossbar A Select Register 37\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel37](sel37) module"]
pub type SEL37 = crate::Reg<u16, _SEL37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL37;
#[doc = "`read()` method returns [sel37::R](sel37::R) reader structure"]
impl crate::Readable for SEL37 {}
#[doc = "`write(|w| ..)` method takes [sel37::W](sel37::W) writer structure"]
impl crate::Writable for SEL37 {}
#[doc = "Crossbar A Select Register 37"]
pub mod sel37;
#[doc = "Crossbar A Select Register 38\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel38](sel38) module"]
pub type SEL38 = crate::Reg<u16, _SEL38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL38;
#[doc = "`read()` method returns [sel38::R](sel38::R) reader structure"]
impl crate::Readable for SEL38 {}
#[doc = "`write(|w| ..)` method takes [sel38::W](sel38::W) writer structure"]
impl crate::Writable for SEL38 {}
#[doc = "Crossbar A Select Register 38"]
pub mod sel38;
#[doc = "Crossbar A Select Register 39\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel39](sel39) module"]
pub type SEL39 = crate::Reg<u16, _SEL39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL39;
#[doc = "`read()` method returns [sel39::R](sel39::R) reader structure"]
impl crate::Readable for SEL39 {}
#[doc = "`write(|w| ..)` method takes [sel39::W](sel39::W) writer structure"]
impl crate::Writable for SEL39 {}
#[doc = "Crossbar A Select Register 39"]
pub mod sel39;
#[doc = "Crossbar A Select Register 40\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel40](sel40) module"]
pub type SEL40 = crate::Reg<u16, _SEL40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL40;
#[doc = "`read()` method returns [sel40::R](sel40::R) reader structure"]
impl crate::Readable for SEL40 {}
#[doc = "`write(|w| ..)` method takes [sel40::W](sel40::W) writer structure"]
impl crate::Writable for SEL40 {}
#[doc = "Crossbar A Select Register 40"]
pub mod sel40;
#[doc = "Crossbar A Select Register 41\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel41](sel41) module"]
pub type SEL41 = crate::Reg<u16, _SEL41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL41;
#[doc = "`read()` method returns [sel41::R](sel41::R) reader structure"]
impl crate::Readable for SEL41 {}
#[doc = "`write(|w| ..)` method takes [sel41::W](sel41::W) writer structure"]
impl crate::Writable for SEL41 {}
#[doc = "Crossbar A Select Register 41"]
pub mod sel41;
#[doc = "Crossbar A Select Register 42\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel42](sel42) module"]
pub type SEL42 = crate::Reg<u16, _SEL42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL42;
#[doc = "`read()` method returns [sel42::R](sel42::R) reader structure"]
impl crate::Readable for SEL42 {}
#[doc = "`write(|w| ..)` method takes [sel42::W](sel42::W) writer structure"]
impl crate::Writable for SEL42 {}
#[doc = "Crossbar A Select Register 42"]
pub mod sel42;
#[doc = "Crossbar A Select Register 43\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel43](sel43) module"]
pub type SEL43 = crate::Reg<u16, _SEL43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL43;
#[doc = "`read()` method returns [sel43::R](sel43::R) reader structure"]
impl crate::Readable for SEL43 {}
#[doc = "`write(|w| ..)` method takes [sel43::W](sel43::W) writer structure"]
impl crate::Writable for SEL43 {}
#[doc = "Crossbar A Select Register 43"]
pub mod sel43;
#[doc = "Crossbar A Select Register 44\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel44](sel44) module"]
pub type SEL44 = crate::Reg<u16, _SEL44>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL44;
#[doc = "`read()` method returns [sel44::R](sel44::R) reader structure"]
impl crate::Readable for SEL44 {}
#[doc = "`write(|w| ..)` method takes [sel44::W](sel44::W) writer structure"]
impl crate::Writable for SEL44 {}
#[doc = "Crossbar A Select Register 44"]
pub mod sel44;
#[doc = "Crossbar A Select Register 45\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel45](sel45) module"]
pub type SEL45 = crate::Reg<u16, _SEL45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL45;
#[doc = "`read()` method returns [sel45::R](sel45::R) reader structure"]
impl crate::Readable for SEL45 {}
#[doc = "`write(|w| ..)` method takes [sel45::W](sel45::W) writer structure"]
impl crate::Writable for SEL45 {}
#[doc = "Crossbar A Select Register 45"]
pub mod sel45;
#[doc = "Crossbar A Select Register 46\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel46](sel46) module"]
pub type SEL46 = crate::Reg<u16, _SEL46>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL46;
#[doc = "`read()` method returns [sel46::R](sel46::R) reader structure"]
impl crate::Readable for SEL46 {}
#[doc = "`write(|w| ..)` method takes [sel46::W](sel46::W) writer structure"]
impl crate::Writable for SEL46 {}
#[doc = "Crossbar A Select Register 46"]
pub mod sel46;
#[doc = "Crossbar A Select Register 47\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel47](sel47) module"]
pub type SEL47 = crate::Reg<u16, _SEL47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL47;
#[doc = "`read()` method returns [sel47::R](sel47::R) reader structure"]
impl crate::Readable for SEL47 {}
#[doc = "`write(|w| ..)` method takes [sel47::W](sel47::W) writer structure"]
impl crate::Writable for SEL47 {}
#[doc = "Crossbar A Select Register 47"]
pub mod sel47;
#[doc = "Crossbar A Select Register 48\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel48](sel48) module"]
pub type SEL48 = crate::Reg<u16, _SEL48>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL48;
#[doc = "`read()` method returns [sel48::R](sel48::R) reader structure"]
impl crate::Readable for SEL48 {}
#[doc = "`write(|w| ..)` method takes [sel48::W](sel48::W) writer structure"]
impl crate::Writable for SEL48 {}
#[doc = "Crossbar A Select Register 48"]
pub mod sel48;
#[doc = "Crossbar A Select Register 49\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel49](sel49) module"]
pub type SEL49 = crate::Reg<u16, _SEL49>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL49;
#[doc = "`read()` method returns [sel49::R](sel49::R) reader structure"]
impl crate::Readable for SEL49 {}
#[doc = "`write(|w| ..)` method takes [sel49::W](sel49::W) writer structure"]
impl crate::Writable for SEL49 {}
#[doc = "Crossbar A Select Register 49"]
pub mod sel49;
#[doc = "Crossbar A Select Register 50\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel50](sel50) module"]
pub type SEL50 = crate::Reg<u16, _SEL50>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL50;
#[doc = "`read()` method returns [sel50::R](sel50::R) reader structure"]
impl crate::Readable for SEL50 {}
#[doc = "`write(|w| ..)` method takes [sel50::W](sel50::W) writer structure"]
impl crate::Writable for SEL50 {}
#[doc = "Crossbar A Select Register 50"]
pub mod sel50;
#[doc = "Crossbar A Select Register 51\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel51](sel51) module"]
pub type SEL51 = crate::Reg<u16, _SEL51>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL51;
#[doc = "`read()` method returns [sel51::R](sel51::R) reader structure"]
impl crate::Readable for SEL51 {}
#[doc = "`write(|w| ..)` method takes [sel51::W](sel51::W) writer structure"]
impl crate::Writable for SEL51 {}
#[doc = "Crossbar A Select Register 51"]
pub mod sel51;
#[doc = "Crossbar A Select Register 52\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel52](sel52) module"]
pub type SEL52 = crate::Reg<u16, _SEL52>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL52;
#[doc = "`read()` method returns [sel52::R](sel52::R) reader structure"]
impl crate::Readable for SEL52 {}
#[doc = "`write(|w| ..)` method takes [sel52::W](sel52::W) writer structure"]
impl crate::Writable for SEL52 {}
#[doc = "Crossbar A Select Register 52"]
pub mod sel52;
#[doc = "Crossbar A Select Register 53\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel53](sel53) module"]
pub type SEL53 = crate::Reg<u16, _SEL53>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL53;
#[doc = "`read()` method returns [sel53::R](sel53::R) reader structure"]
impl crate::Readable for SEL53 {}
#[doc = "`write(|w| ..)` method takes [sel53::W](sel53::W) writer structure"]
impl crate::Writable for SEL53 {}
#[doc = "Crossbar A Select Register 53"]
pub mod sel53;
#[doc = "Crossbar A Select Register 54\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel54](sel54) module"]
pub type SEL54 = crate::Reg<u16, _SEL54>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL54;
#[doc = "`read()` method returns [sel54::R](sel54::R) reader structure"]
impl crate::Readable for SEL54 {}
#[doc = "`write(|w| ..)` method takes [sel54::W](sel54::W) writer structure"]
impl crate::Writable for SEL54 {}
#[doc = "Crossbar A Select Register 54"]
pub mod sel54;
#[doc = "Crossbar A Select Register 55\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel55](sel55) module"]
pub type SEL55 = crate::Reg<u16, _SEL55>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL55;
#[doc = "`read()` method returns [sel55::R](sel55::R) reader structure"]
impl crate::Readable for SEL55 {}
#[doc = "`write(|w| ..)` method takes [sel55::W](sel55::W) writer structure"]
impl crate::Writable for SEL55 {}
#[doc = "Crossbar A Select Register 55"]
pub mod sel55;
#[doc = "Crossbar A Select Register 56\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel56](sel56) module"]
pub type SEL56 = crate::Reg<u16, _SEL56>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL56;
#[doc = "`read()` method returns [sel56::R](sel56::R) reader structure"]
impl crate::Readable for SEL56 {}
#[doc = "`write(|w| ..)` method takes [sel56::W](sel56::W) writer structure"]
impl crate::Writable for SEL56 {}
#[doc = "Crossbar A Select Register 56"]
pub mod sel56;
#[doc = "Crossbar A Select Register 57\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel57](sel57) module"]
pub type SEL57 = crate::Reg<u16, _SEL57>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL57;
#[doc = "`read()` method returns [sel57::R](sel57::R) reader structure"]
impl crate::Readable for SEL57 {}
#[doc = "`write(|w| ..)` method takes [sel57::W](sel57::W) writer structure"]
impl crate::Writable for SEL57 {}
#[doc = "Crossbar A Select Register 57"]
pub mod sel57;
#[doc = "Crossbar A Select Register 58\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel58](sel58) module"]
pub type SEL58 = crate::Reg<u16, _SEL58>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL58;
#[doc = "`read()` method returns [sel58::R](sel58::R) reader structure"]
impl crate::Readable for SEL58 {}
#[doc = "`write(|w| ..)` method takes [sel58::W](sel58::W) writer structure"]
impl crate::Writable for SEL58 {}
#[doc = "Crossbar A Select Register 58"]
pub mod sel58;
#[doc = "Crossbar A Select Register 59\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel59](sel59) module"]
pub type SEL59 = crate::Reg<u16, _SEL59>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL59;
#[doc = "`read()` method returns [sel59::R](sel59::R) reader structure"]
impl crate::Readable for SEL59 {}
#[doc = "`write(|w| ..)` method takes [sel59::W](sel59::W) writer structure"]
impl crate::Writable for SEL59 {}
#[doc = "Crossbar A Select Register 59"]
pub mod sel59;
#[doc = "Crossbar A Select Register 60\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel60](sel60) module"]
pub type SEL60 = crate::Reg<u16, _SEL60>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL60;
#[doc = "`read()` method returns [sel60::R](sel60::R) reader structure"]
impl crate::Readable for SEL60 {}
#[doc = "`write(|w| ..)` method takes [sel60::W](sel60::W) writer structure"]
impl crate::Writable for SEL60 {}
#[doc = "Crossbar A Select Register 60"]
pub mod sel60;
#[doc = "Crossbar A Select Register 61\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel61](sel61) module"]
pub type SEL61 = crate::Reg<u16, _SEL61>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL61;
#[doc = "`read()` method returns [sel61::R](sel61::R) reader structure"]
impl crate::Readable for SEL61 {}
#[doc = "`write(|w| ..)` method takes [sel61::W](sel61::W) writer structure"]
impl crate::Writable for SEL61 {}
#[doc = "Crossbar A Select Register 61"]
pub mod sel61;
#[doc = "Crossbar A Select Register 62\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel62](sel62) module"]
pub type SEL62 = crate::Reg<u16, _SEL62>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL62;
#[doc = "`read()` method returns [sel62::R](sel62::R) reader structure"]
impl crate::Readable for SEL62 {}
#[doc = "`write(|w| ..)` method takes [sel62::W](sel62::W) writer structure"]
impl crate::Writable for SEL62 {}
#[doc = "Crossbar A Select Register 62"]
pub mod sel62;
#[doc = "Crossbar A Select Register 63\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel63](sel63) module"]
pub type SEL63 = crate::Reg<u16, _SEL63>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL63;
#[doc = "`read()` method returns [sel63::R](sel63::R) reader structure"]
impl crate::Readable for SEL63 {}
#[doc = "`write(|w| ..)` method takes [sel63::W](sel63::W) writer structure"]
impl crate::Writable for SEL63 {}
#[doc = "Crossbar A Select Register 63"]
pub mod sel63;
#[doc = "Crossbar A Select Register 64\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel64](sel64) module"]
pub type SEL64 = crate::Reg<u16, _SEL64>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL64;
#[doc = "`read()` method returns [sel64::R](sel64::R) reader structure"]
impl crate::Readable for SEL64 {}
#[doc = "`write(|w| ..)` method takes [sel64::W](sel64::W) writer structure"]
impl crate::Writable for SEL64 {}
#[doc = "Crossbar A Select Register 64"]
pub mod sel64;
#[doc = "Crossbar A Select Register 65\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel65](sel65) module"]
pub type SEL65 = crate::Reg<u16, _SEL65>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL65;
#[doc = "`read()` method returns [sel65::R](sel65::R) reader structure"]
impl crate::Readable for SEL65 {}
#[doc = "`write(|w| ..)` method takes [sel65::W](sel65::W) writer structure"]
impl crate::Writable for SEL65 {}
#[doc = "Crossbar A Select Register 65"]
pub mod sel65;
#[doc = "Crossbar A Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl0](ctrl0) module"]
pub type CTRL0 = crate::Reg<u16, _CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL0;
#[doc = "`read()` method returns [ctrl0::R](ctrl0::R) reader structure"]
impl crate::Readable for CTRL0 {}
#[doc = "`write(|w| ..)` method takes [ctrl0::W](ctrl0::W) writer structure"]
impl crate::Writable for CTRL0 {}
#[doc = "Crossbar A Control Register 0"]
pub mod ctrl0;
#[doc = "Crossbar A Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1](ctrl1) module"]
pub type CTRL1 = crate::Reg<u16, _CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL1;
#[doc = "`read()` method returns [ctrl1::R](ctrl1::R) reader structure"]
impl crate::Readable for CTRL1 {}
#[doc = "`write(|w| ..)` method takes [ctrl1::W](ctrl1::W) writer structure"]
impl crate::Writable for CTRL1 {}
#[doc = "Crossbar A Control Register 1"]
pub mod ctrl1;
