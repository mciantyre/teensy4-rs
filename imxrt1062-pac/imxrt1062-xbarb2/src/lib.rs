#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Crossbar B Select Register 0"]
    pub sel0: SEL0,
    #[doc = "0x02 - Crossbar B Select Register 1"]
    pub sel1: SEL1,
    #[doc = "0x04 - Crossbar B Select Register 2"]
    pub sel2: SEL2,
    #[doc = "0x06 - Crossbar B Select Register 3"]
    pub sel3: SEL3,
    #[doc = "0x08 - Crossbar B Select Register 4"]
    pub sel4: SEL4,
    #[doc = "0x0a - Crossbar B Select Register 5"]
    pub sel5: SEL5,
    #[doc = "0x0c - Crossbar B Select Register 6"]
    pub sel6: SEL6,
    #[doc = "0x0e - Crossbar B Select Register 7"]
    pub sel7: SEL7,
}
#[doc = "Crossbar B Select Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel0](sel0) module"]
pub type SEL0 = crate::Reg<u16, _SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL0;
#[doc = "`read()` method returns [sel0::R](sel0::R) reader structure"]
impl crate::Readable for SEL0 {}
#[doc = "`write(|w| ..)` method takes [sel0::W](sel0::W) writer structure"]
impl crate::Writable for SEL0 {}
#[doc = "Crossbar B Select Register 0"]
pub mod sel0;
#[doc = "Crossbar B Select Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel1](sel1) module"]
pub type SEL1 = crate::Reg<u16, _SEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL1;
#[doc = "`read()` method returns [sel1::R](sel1::R) reader structure"]
impl crate::Readable for SEL1 {}
#[doc = "`write(|w| ..)` method takes [sel1::W](sel1::W) writer structure"]
impl crate::Writable for SEL1 {}
#[doc = "Crossbar B Select Register 1"]
pub mod sel1;
#[doc = "Crossbar B Select Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel2](sel2) module"]
pub type SEL2 = crate::Reg<u16, _SEL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL2;
#[doc = "`read()` method returns [sel2::R](sel2::R) reader structure"]
impl crate::Readable for SEL2 {}
#[doc = "`write(|w| ..)` method takes [sel2::W](sel2::W) writer structure"]
impl crate::Writable for SEL2 {}
#[doc = "Crossbar B Select Register 2"]
pub mod sel2;
#[doc = "Crossbar B Select Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel3](sel3) module"]
pub type SEL3 = crate::Reg<u16, _SEL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL3;
#[doc = "`read()` method returns [sel3::R](sel3::R) reader structure"]
impl crate::Readable for SEL3 {}
#[doc = "`write(|w| ..)` method takes [sel3::W](sel3::W) writer structure"]
impl crate::Writable for SEL3 {}
#[doc = "Crossbar B Select Register 3"]
pub mod sel3;
#[doc = "Crossbar B Select Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel4](sel4) module"]
pub type SEL4 = crate::Reg<u16, _SEL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL4;
#[doc = "`read()` method returns [sel4::R](sel4::R) reader structure"]
impl crate::Readable for SEL4 {}
#[doc = "`write(|w| ..)` method takes [sel4::W](sel4::W) writer structure"]
impl crate::Writable for SEL4 {}
#[doc = "Crossbar B Select Register 4"]
pub mod sel4;
#[doc = "Crossbar B Select Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel5](sel5) module"]
pub type SEL5 = crate::Reg<u16, _SEL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL5;
#[doc = "`read()` method returns [sel5::R](sel5::R) reader structure"]
impl crate::Readable for SEL5 {}
#[doc = "`write(|w| ..)` method takes [sel5::W](sel5::W) writer structure"]
impl crate::Writable for SEL5 {}
#[doc = "Crossbar B Select Register 5"]
pub mod sel5;
#[doc = "Crossbar B Select Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel6](sel6) module"]
pub type SEL6 = crate::Reg<u16, _SEL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL6;
#[doc = "`read()` method returns [sel6::R](sel6::R) reader structure"]
impl crate::Readable for SEL6 {}
#[doc = "`write(|w| ..)` method takes [sel6::W](sel6::W) writer structure"]
impl crate::Writable for SEL6 {}
#[doc = "Crossbar B Select Register 6"]
pub mod sel6;
#[doc = "Crossbar B Select Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel7](sel7) module"]
pub type SEL7 = crate::Reg<u16, _SEL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL7;
#[doc = "`read()` method returns [sel7::R](sel7::R) reader structure"]
impl crate::Readable for SEL7 {}
#[doc = "`write(|w| ..)` method takes [sel7::W](sel7::W) writer structure"]
impl crate::Writable for SEL7 {}
#[doc = "Crossbar B Select Register 7"]
pub mod sel7;
