#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel 0 Configuration Register"]
    pub chcfg: [CHCFG; 32],
}
#[doc = "Channel 0 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chcfg](chcfg) module"]
pub type CHCFG = crate::Reg<u32, _CHCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCFG;
#[doc = "`read()` method returns [chcfg::R](chcfg::R) reader structure"]
impl crate::Readable for CHCFG {}
#[doc = "`write(|w| ..)` method takes [chcfg::W](chcfg::W) writer structure"]
impl crate::Writable for CHCFG {}
#[doc = "Channel 0 Configuration Register"]
pub mod chcfg;
