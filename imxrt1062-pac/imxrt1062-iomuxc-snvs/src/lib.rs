#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SW_MUX_CTL_PAD_WAKEUP SW MUX Control Register"]
    pub sw_mux_ctl_pad_wakeup: SW_MUX_CTL_PAD_WAKEUP,
    #[doc = "0x04 - SW_MUX_CTL_PAD_PMIC_ON_REQ SW MUX Control Register"]
    pub sw_mux_ctl_pad_pmic_on_req: SW_MUX_CTL_PAD_PMIC_ON_REQ,
    #[doc = "0x08 - SW_MUX_CTL_PAD_PMIC_STBY_REQ SW MUX Control Register"]
    pub sw_mux_ctl_pad_pmic_stby_req: SW_MUX_CTL_PAD_PMIC_STBY_REQ,
    #[doc = "0x0c - SW_PAD_CTL_PAD_TEST_MODE SW PAD Control Register"]
    pub sw_pad_ctl_pad_test_mode: SW_PAD_CTL_PAD_TEST_MODE,
    #[doc = "0x10 - SW_PAD_CTL_PAD_POR_B SW PAD Control Register"]
    pub sw_pad_ctl_pad_por_b: SW_PAD_CTL_PAD_POR_B,
    #[doc = "0x14 - SW_PAD_CTL_PAD_ONOFF SW PAD Control Register"]
    pub sw_pad_ctl_pad_onoff: SW_PAD_CTL_PAD_ONOFF,
    #[doc = "0x18 - SW_PAD_CTL_PAD_WAKEUP SW PAD Control Register"]
    pub sw_pad_ctl_pad_wakeup: SW_PAD_CTL_PAD_WAKEUP,
    #[doc = "0x1c - SW_PAD_CTL_PAD_PMIC_ON_REQ SW PAD Control Register"]
    pub sw_pad_ctl_pad_pmic_on_req: SW_PAD_CTL_PAD_PMIC_ON_REQ,
    #[doc = "0x20 - SW_PAD_CTL_PAD_PMIC_STBY_REQ SW PAD Control Register"]
    pub sw_pad_ctl_pad_pmic_stby_req: SW_PAD_CTL_PAD_PMIC_STBY_REQ,
}
#[doc = "SW_MUX_CTL_PAD_WAKEUP SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_mux_ctl_pad_wakeup](sw_mux_ctl_pad_wakeup) module"]
pub type SW_MUX_CTL_PAD_WAKEUP = crate::Reg<u32, _SW_MUX_CTL_PAD_WAKEUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_WAKEUP;
#[doc = "`read()` method returns [sw_mux_ctl_pad_wakeup::R](sw_mux_ctl_pad_wakeup::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_WAKEUP {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_wakeup::W](sw_mux_ctl_pad_wakeup::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_WAKEUP {}
#[doc = "SW_MUX_CTL_PAD_WAKEUP SW MUX Control Register"]
pub mod sw_mux_ctl_pad_wakeup;
#[doc = "SW_MUX_CTL_PAD_PMIC_ON_REQ SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_mux_ctl_pad_pmic_on_req](sw_mux_ctl_pad_pmic_on_req) module"]
pub type SW_MUX_CTL_PAD_PMIC_ON_REQ = crate::Reg<u32, _SW_MUX_CTL_PAD_PMIC_ON_REQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_PMIC_ON_REQ;
#[doc = "`read()` method returns [sw_mux_ctl_pad_pmic_on_req::R](sw_mux_ctl_pad_pmic_on_req::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_PMIC_ON_REQ {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_pmic_on_req::W](sw_mux_ctl_pad_pmic_on_req::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_PMIC_ON_REQ {}
#[doc = "SW_MUX_CTL_PAD_PMIC_ON_REQ SW MUX Control Register"]
pub mod sw_mux_ctl_pad_pmic_on_req;
#[doc = "SW_MUX_CTL_PAD_PMIC_STBY_REQ SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_mux_ctl_pad_pmic_stby_req](sw_mux_ctl_pad_pmic_stby_req) module"]
pub type SW_MUX_CTL_PAD_PMIC_STBY_REQ = crate::Reg<u32, _SW_MUX_CTL_PAD_PMIC_STBY_REQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_MUX_CTL_PAD_PMIC_STBY_REQ;
#[doc = "`read()` method returns [sw_mux_ctl_pad_pmic_stby_req::R](sw_mux_ctl_pad_pmic_stby_req::R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_PMIC_STBY_REQ {}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_pmic_stby_req::W](sw_mux_ctl_pad_pmic_stby_req::W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_PMIC_STBY_REQ {}
#[doc = "SW_MUX_CTL_PAD_PMIC_STBY_REQ SW MUX Control Register"]
pub mod sw_mux_ctl_pad_pmic_stby_req;
#[doc = "SW_PAD_CTL_PAD_TEST_MODE SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_pad_ctl_pad_test_mode](sw_pad_ctl_pad_test_mode) module"]
pub type SW_PAD_CTL_PAD_TEST_MODE = crate::Reg<u32, _SW_PAD_CTL_PAD_TEST_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_TEST_MODE;
#[doc = "`read()` method returns [sw_pad_ctl_pad_test_mode::R](sw_pad_ctl_pad_test_mode::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_TEST_MODE {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_test_mode::W](sw_pad_ctl_pad_test_mode::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_TEST_MODE {}
#[doc = "SW_PAD_CTL_PAD_TEST_MODE SW PAD Control Register"]
pub mod sw_pad_ctl_pad_test_mode;
#[doc = "SW_PAD_CTL_PAD_POR_B SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_pad_ctl_pad_por_b](sw_pad_ctl_pad_por_b) module"]
pub type SW_PAD_CTL_PAD_POR_B = crate::Reg<u32, _SW_PAD_CTL_PAD_POR_B>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_POR_B;
#[doc = "`read()` method returns [sw_pad_ctl_pad_por_b::R](sw_pad_ctl_pad_por_b::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_POR_B {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_por_b::W](sw_pad_ctl_pad_por_b::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_POR_B {}
#[doc = "SW_PAD_CTL_PAD_POR_B SW PAD Control Register"]
pub mod sw_pad_ctl_pad_por_b;
#[doc = "SW_PAD_CTL_PAD_ONOFF SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_pad_ctl_pad_onoff](sw_pad_ctl_pad_onoff) module"]
pub type SW_PAD_CTL_PAD_ONOFF = crate::Reg<u32, _SW_PAD_CTL_PAD_ONOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_ONOFF;
#[doc = "`read()` method returns [sw_pad_ctl_pad_onoff::R](sw_pad_ctl_pad_onoff::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_ONOFF {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_onoff::W](sw_pad_ctl_pad_onoff::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_ONOFF {}
#[doc = "SW_PAD_CTL_PAD_ONOFF SW PAD Control Register"]
pub mod sw_pad_ctl_pad_onoff;
#[doc = "SW_PAD_CTL_PAD_WAKEUP SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_pad_ctl_pad_wakeup](sw_pad_ctl_pad_wakeup) module"]
pub type SW_PAD_CTL_PAD_WAKEUP = crate::Reg<u32, _SW_PAD_CTL_PAD_WAKEUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_WAKEUP;
#[doc = "`read()` method returns [sw_pad_ctl_pad_wakeup::R](sw_pad_ctl_pad_wakeup::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_WAKEUP {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_wakeup::W](sw_pad_ctl_pad_wakeup::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_WAKEUP {}
#[doc = "SW_PAD_CTL_PAD_WAKEUP SW PAD Control Register"]
pub mod sw_pad_ctl_pad_wakeup;
#[doc = "SW_PAD_CTL_PAD_PMIC_ON_REQ SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_pad_ctl_pad_pmic_on_req](sw_pad_ctl_pad_pmic_on_req) module"]
pub type SW_PAD_CTL_PAD_PMIC_ON_REQ = crate::Reg<u32, _SW_PAD_CTL_PAD_PMIC_ON_REQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_PMIC_ON_REQ;
#[doc = "`read()` method returns [sw_pad_ctl_pad_pmic_on_req::R](sw_pad_ctl_pad_pmic_on_req::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_PMIC_ON_REQ {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_pmic_on_req::W](sw_pad_ctl_pad_pmic_on_req::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_PMIC_ON_REQ {}
#[doc = "SW_PAD_CTL_PAD_PMIC_ON_REQ SW PAD Control Register"]
pub mod sw_pad_ctl_pad_pmic_on_req;
#[doc = "SW_PAD_CTL_PAD_PMIC_STBY_REQ SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_pad_ctl_pad_pmic_stby_req](sw_pad_ctl_pad_pmic_stby_req) module"]
pub type SW_PAD_CTL_PAD_PMIC_STBY_REQ = crate::Reg<u32, _SW_PAD_CTL_PAD_PMIC_STBY_REQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_PAD_CTL_PAD_PMIC_STBY_REQ;
#[doc = "`read()` method returns [sw_pad_ctl_pad_pmic_stby_req::R](sw_pad_ctl_pad_pmic_stby_req::R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_PMIC_STBY_REQ {}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_pmic_stby_req::W](sw_pad_ctl_pad_pmic_stby_req::W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_PMIC_STBY_REQ {}
#[doc = "SW_PAD_CTL_PAD_PMIC_STBY_REQ SW PAD Control Register"]
pub mod sw_pad_ctl_pad_pmic_stby_req;
