#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPC Interface control register"]
    pub cntr: CNTR,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - IRQ masking register 1"]
    pub imr1: IMR1,
    #[doc = "0x0c - IRQ masking register 2"]
    pub imr2: IMR2,
    #[doc = "0x10 - IRQ masking register 3"]
    pub imr3: IMR3,
    #[doc = "0x14 - IRQ masking register 4"]
    pub imr4: IMR4,
    #[doc = "0x18 - IRQ status resister 1"]
    pub isr1: ISR1,
    #[doc = "0x1c - IRQ status resister 2"]
    pub isr2: ISR2,
    #[doc = "0x20 - IRQ status resister 3"]
    pub isr3: ISR3,
    #[doc = "0x24 - IRQ status resister 4"]
    pub isr4: ISR4,
    _reserved9: [u8; 12usize],
    #[doc = "0x34 - IRQ masking register 5"]
    pub imr5: IMR5,
    #[doc = "0x38 - IRQ status resister 5"]
    pub isr5: ISR5,
}
#[doc = "GPC Interface control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntr](cntr) module"]
pub type CNTR = crate::Reg<u32, _CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTR;
#[doc = "`read()` method returns [cntr::R](cntr::R) reader structure"]
impl crate::Readable for CNTR {}
#[doc = "`write(|w| ..)` method takes [cntr::W](cntr::W) writer structure"]
impl crate::Writable for CNTR {}
#[doc = "GPC Interface control register"]
pub mod cntr;
#[doc = "IRQ masking register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr1](imr1) module"]
pub type IMR1 = crate::Reg<u32, _IMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR1;
#[doc = "`read()` method returns [imr1::R](imr1::R) reader structure"]
impl crate::Readable for IMR1 {}
#[doc = "`write(|w| ..)` method takes [imr1::W](imr1::W) writer structure"]
impl crate::Writable for IMR1 {}
#[doc = "IRQ masking register 1"]
pub mod imr1;
#[doc = "IRQ masking register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr2](imr2) module"]
pub type IMR2 = crate::Reg<u32, _IMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR2;
#[doc = "`read()` method returns [imr2::R](imr2::R) reader structure"]
impl crate::Readable for IMR2 {}
#[doc = "`write(|w| ..)` method takes [imr2::W](imr2::W) writer structure"]
impl crate::Writable for IMR2 {}
#[doc = "IRQ masking register 2"]
pub mod imr2;
#[doc = "IRQ masking register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr3](imr3) module"]
pub type IMR3 = crate::Reg<u32, _IMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR3;
#[doc = "`read()` method returns [imr3::R](imr3::R) reader structure"]
impl crate::Readable for IMR3 {}
#[doc = "`write(|w| ..)` method takes [imr3::W](imr3::W) writer structure"]
impl crate::Writable for IMR3 {}
#[doc = "IRQ masking register 3"]
pub mod imr3;
#[doc = "IRQ masking register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr4](imr4) module"]
pub type IMR4 = crate::Reg<u32, _IMR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR4;
#[doc = "`read()` method returns [imr4::R](imr4::R) reader structure"]
impl crate::Readable for IMR4 {}
#[doc = "`write(|w| ..)` method takes [imr4::W](imr4::W) writer structure"]
impl crate::Writable for IMR4 {}
#[doc = "IRQ masking register 4"]
pub mod imr4;
#[doc = "IRQ status resister 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr1](isr1) module"]
pub type ISR1 = crate::Reg<u32, _ISR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR1;
#[doc = "`read()` method returns [isr1::R](isr1::R) reader structure"]
impl crate::Readable for ISR1 {}
#[doc = "IRQ status resister 1"]
pub mod isr1;
#[doc = "IRQ status resister 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr2](isr2) module"]
pub type ISR2 = crate::Reg<u32, _ISR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR2;
#[doc = "`read()` method returns [isr2::R](isr2::R) reader structure"]
impl crate::Readable for ISR2 {}
#[doc = "IRQ status resister 2"]
pub mod isr2;
#[doc = "IRQ status resister 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr3](isr3) module"]
pub type ISR3 = crate::Reg<u32, _ISR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR3;
#[doc = "`read()` method returns [isr3::R](isr3::R) reader structure"]
impl crate::Readable for ISR3 {}
#[doc = "IRQ status resister 3"]
pub mod isr3;
#[doc = "IRQ status resister 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr4](isr4) module"]
pub type ISR4 = crate::Reg<u32, _ISR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR4;
#[doc = "`read()` method returns [isr4::R](isr4::R) reader structure"]
impl crate::Readable for ISR4 {}
#[doc = "IRQ status resister 4"]
pub mod isr4;
#[doc = "IRQ masking register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr5](imr5) module"]
pub type IMR5 = crate::Reg<u32, _IMR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR5;
#[doc = "`read()` method returns [imr5::R](imr5::R) reader structure"]
impl crate::Readable for IMR5 {}
#[doc = "`write(|w| ..)` method takes [imr5::W](imr5::W) writer structure"]
impl crate::Writable for IMR5 {}
#[doc = "IRQ masking register 5"]
pub mod imr5;
#[doc = "IRQ status resister 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr5](isr5) module"]
pub type ISR5 = crate::Reg<u32, _ISR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR5;
#[doc = "`read()` method returns [isr5::R](isr5::R) reader structure"]
impl crate::Readable for ISR5 {}
#[doc = "IRQ status resister 5"]
pub mod isr5;
