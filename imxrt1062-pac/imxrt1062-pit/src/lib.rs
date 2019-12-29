#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

include!("../../generic.rs");

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PIT Module Control Register"]
    pub mcr: MCR,
    _reserved1: [u8; 220usize],
    #[doc = "0xe0 - PIT Upper Lifetime Timer Register"]
    pub ltmr64h: LTMR64H,
    #[doc = "0xe4 - PIT Lower Lifetime Timer Register"]
    pub ltmr64l: LTMR64L,
    _reserved3: [u8; 24usize],
    #[doc = "0x100 - no description available"]
    pub timer: [TIMER; 4],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct TIMER {
    #[doc = "0x00 - Timer Load Value Register"]
    pub ldval: self::timer::LDVAL,
    #[doc = "0x04 - Current Timer Value Register"]
    pub cval: self::timer::CVAL,
    #[doc = "0x08 - Timer Control Register"]
    pub tctrl: self::timer::TCTRL,
    #[doc = "0x0c - Timer Flag Register"]
    pub tflg: self::timer::TFLG,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod timer;
#[doc = "PIT Module Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcr](mcr) module"]
pub type MCR = crate::Reg<u32, _MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR;
#[doc = "`read()` method returns [mcr::R](mcr::R) reader structure"]
impl crate::Readable for MCR {}
#[doc = "`write(|w| ..)` method takes [mcr::W](mcr::W) writer structure"]
impl crate::Writable for MCR {}
#[doc = "PIT Module Control Register"]
pub mod mcr;
#[doc = "PIT Upper Lifetime Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ltmr64h](ltmr64h) module"]
pub type LTMR64H = crate::Reg<u32, _LTMR64H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTMR64H;
#[doc = "`read()` method returns [ltmr64h::R](ltmr64h::R) reader structure"]
impl crate::Readable for LTMR64H {}
#[doc = "PIT Upper Lifetime Timer Register"]
pub mod ltmr64h;
#[doc = "PIT Lower Lifetime Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ltmr64l](ltmr64l) module"]
pub type LTMR64L = crate::Reg<u32, _LTMR64L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTMR64L;
#[doc = "`read()` method returns [ltmr64l::R](ltmr64l::R) reader structure"]
impl crate::Readable for LTMR64L {}
#[doc = "PIT Lower Lifetime Timer Register"]
pub mod ltmr64l;
