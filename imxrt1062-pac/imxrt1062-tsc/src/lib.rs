#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - no description available"]
    pub basic_setting: BASIC_SETTING,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - no description available"]
    pub pre_charge_time: PRE_CHARGE_TIME,
    _reserved2: [u8; 12usize],
    #[doc = "0x20 - Flow Control"]
    pub flow_control: FLOW_CONTROL,
    _reserved3: [u8; 12usize],
    #[doc = "0x30 - Measure Value"]
    pub measeure_value: MEASEURE_VALUE,
    _reserved4: [u8; 12usize],
    #[doc = "0x40 - Interrupt Enable"]
    pub int_en: INT_EN,
    _reserved5: [u8; 12usize],
    #[doc = "0x50 - Interrupt Signal Enable"]
    pub int_sig_en: INT_SIG_EN,
    _reserved6: [u8; 12usize],
    #[doc = "0x60 - Intterrupt Status"]
    pub int_status: INT_STATUS,
    _reserved7: [u8; 12usize],
    #[doc = "0x70 - no description available"]
    pub debug_mode: DEBUG_MODE,
    _reserved8: [u8; 12usize],
    #[doc = "0x80 - no description available"]
    pub debug_mode2: DEBUG_MODE2,
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [basic_setting](basic_setting) module"]
pub type BASIC_SETTING = crate::Reg<u32, _BASIC_SETTING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BASIC_SETTING;
#[doc = "`read()` method returns [basic_setting::R](basic_setting::R) reader structure"]
impl crate::Readable for BASIC_SETTING {}
#[doc = "`write(|w| ..)` method takes [basic_setting::W](basic_setting::W) writer structure"]
impl crate::Writable for BASIC_SETTING {}
#[doc = "no description available"]
pub mod basic_setting;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pre_charge_time](pre_charge_time) module"]
pub type PRE_CHARGE_TIME = crate::Reg<u32, _PRE_CHARGE_TIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRE_CHARGE_TIME;
#[doc = "`read()` method returns [pre_charge_time::R](pre_charge_time::R) reader structure"]
impl crate::Readable for PRE_CHARGE_TIME {}
#[doc = "`write(|w| ..)` method takes [pre_charge_time::W](pre_charge_time::W) writer structure"]
impl crate::Writable for PRE_CHARGE_TIME {}
#[doc = "no description available"]
pub mod pre_charge_time;
#[doc = "Flow Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flow_control](flow_control) module"]
pub type FLOW_CONTROL = crate::Reg<u32, _FLOW_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLOW_CONTROL;
#[doc = "`read()` method returns [flow_control::R](flow_control::R) reader structure"]
impl crate::Readable for FLOW_CONTROL {}
#[doc = "`write(|w| ..)` method takes [flow_control::W](flow_control::W) writer structure"]
impl crate::Writable for FLOW_CONTROL {}
#[doc = "Flow Control"]
pub mod flow_control;
#[doc = "Measure Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [measeure_value](measeure_value) module"]
pub type MEASEURE_VALUE = crate::Reg<u32, _MEASEURE_VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEASEURE_VALUE;
#[doc = "`read()` method returns [measeure_value::R](measeure_value::R) reader structure"]
impl crate::Readable for MEASEURE_VALUE {}
#[doc = "Measure Value"]
pub mod measeure_value;
#[doc = "Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_en](int_en) module"]
pub type INT_EN = crate::Reg<u32, _INT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_EN;
#[doc = "`read()` method returns [int_en::R](int_en::R) reader structure"]
impl crate::Readable for INT_EN {}
#[doc = "`write(|w| ..)` method takes [int_en::W](int_en::W) writer structure"]
impl crate::Writable for INT_EN {}
#[doc = "Interrupt Enable"]
pub mod int_en;
#[doc = "Interrupt Signal Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_sig_en](int_sig_en) module"]
pub type INT_SIG_EN = crate::Reg<u32, _INT_SIG_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_SIG_EN;
#[doc = "`read()` method returns [int_sig_en::R](int_sig_en::R) reader structure"]
impl crate::Readable for INT_SIG_EN {}
#[doc = "`write(|w| ..)` method takes [int_sig_en::W](int_sig_en::W) writer structure"]
impl crate::Writable for INT_SIG_EN {}
#[doc = "Interrupt Signal Enable"]
pub mod int_sig_en;
#[doc = "Intterrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_status](int_status) module"]
pub type INT_STATUS = crate::Reg<u32, _INT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_STATUS;
#[doc = "`read()` method returns [int_status::R](int_status::R) reader structure"]
impl crate::Readable for INT_STATUS {}
#[doc = "`write(|w| ..)` method takes [int_status::W](int_status::W) writer structure"]
impl crate::Writable for INT_STATUS {}
#[doc = "Intterrupt Status"]
pub mod int_status;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_mode](debug_mode) module"]
pub type DEBUG_MODE = crate::Reg<u32, _DEBUG_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUG_MODE;
#[doc = "`read()` method returns [debug_mode::R](debug_mode::R) reader structure"]
impl crate::Readable for DEBUG_MODE {}
#[doc = "`write(|w| ..)` method takes [debug_mode::W](debug_mode::W) writer structure"]
impl crate::Writable for DEBUG_MODE {}
#[doc = "no description available"]
pub mod debug_mode;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_mode2](debug_mode2) module"]
pub type DEBUG_MODE2 = crate::Reg<u32, _DEBUG_MODE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUG_MODE2;
#[doc = "`read()` method returns [debug_mode2::R](debug_mode2::R) reader structure"]
impl crate::Readable for DEBUG_MODE2 {}
#[doc = "`write(|w| ..)` method takes [debug_mode2::W](debug_mode2::W) writer structure"]
impl crate::Writable for DEBUG_MODE2 {}
#[doc = "no description available"]
pub mod debug_mode2;
