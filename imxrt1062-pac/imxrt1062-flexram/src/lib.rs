#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TCM CRTL Register"]
    pub tcm_ctrl: TCM_CTRL,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Interrupt Status Register"]
    pub int_status: INT_STATUS,
    #[doc = "0x14 - Interrupt Status Enable Register"]
    pub int_stat_en: INT_STAT_EN,
    #[doc = "0x18 - Interrupt Enable Register"]
    pub int_sig_en: INT_SIG_EN,
}
#[doc = "TCM CRTL Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcm_ctrl](tcm_ctrl) module"]
pub type TCM_CTRL = crate::Reg<u32, _TCM_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCM_CTRL;
#[doc = "`read()` method returns [tcm_ctrl::R](tcm_ctrl::R) reader structure"]
impl crate::Readable for TCM_CTRL {}
#[doc = "`write(|w| ..)` method takes [tcm_ctrl::W](tcm_ctrl::W) writer structure"]
impl crate::Writable for TCM_CTRL {}
#[doc = "TCM CRTL Register"]
pub mod tcm_ctrl;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_status](int_status) module"]
pub type INT_STATUS = crate::Reg<u32, _INT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_STATUS;
#[doc = "`read()` method returns [int_status::R](int_status::R) reader structure"]
impl crate::Readable for INT_STATUS {}
#[doc = "`write(|w| ..)` method takes [int_status::W](int_status::W) writer structure"]
impl crate::Writable for INT_STATUS {}
#[doc = "Interrupt Status Register"]
pub mod int_status;
#[doc = "Interrupt Status Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_stat_en](int_stat_en) module"]
pub type INT_STAT_EN = crate::Reg<u32, _INT_STAT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_STAT_EN;
#[doc = "`read()` method returns [int_stat_en::R](int_stat_en::R) reader structure"]
impl crate::Readable for INT_STAT_EN {}
#[doc = "`write(|w| ..)` method takes [int_stat_en::W](int_stat_en::W) writer structure"]
impl crate::Writable for INT_STAT_EN {}
#[doc = "Interrupt Status Enable Register"]
pub mod int_stat_en;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_sig_en](int_sig_en) module"]
pub type INT_SIG_EN = crate::Reg<u32, _INT_SIG_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_SIG_EN;
#[doc = "`read()` method returns [int_sig_en::R](int_sig_en::R) reader structure"]
impl crate::Readable for INT_SIG_EN {}
#[doc = "`write(|w| ..)` method takes [int_sig_en::W](int_sig_en::W) writer structure"]
impl crate::Writable for INT_SIG_EN {}
#[doc = "Interrupt Enable Register"]
pub mod int_sig_en;
