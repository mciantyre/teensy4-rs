#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 336usize],
    #[doc = "0x150 - Miscellaneous Register 0"]
    pub misc0: MISC0,
    #[doc = "0x154 - Miscellaneous Register 0"]
    pub misc0_set: MISC0_SET,
    #[doc = "0x158 - Miscellaneous Register 0"]
    pub misc0_clr: MISC0_CLR,
    #[doc = "0x15c - Miscellaneous Register 0"]
    pub misc0_tog: MISC0_TOG,
    _reserved4: [u8; 272usize],
    #[doc = "0x270 - XTAL OSC (LP) Control Register"]
    pub lowpwr_ctrl: LOWPWR_CTRL,
    #[doc = "0x274 - XTAL OSC (LP) Control Register"]
    pub lowpwr_ctrl_set: LOWPWR_CTRL_SET,
    #[doc = "0x278 - XTAL OSC (LP) Control Register"]
    pub lowpwr_ctrl_clr: LOWPWR_CTRL_CLR,
    #[doc = "0x27c - XTAL OSC (LP) Control Register"]
    pub lowpwr_ctrl_tog: LOWPWR_CTRL_TOG,
    _reserved8: [u8; 32usize],
    #[doc = "0x2a0 - XTAL OSC Configuration 0 Register"]
    pub osc_config0: OSC_CONFIG0,
    #[doc = "0x2a4 - XTAL OSC Configuration 0 Register"]
    pub osc_config0_set: OSC_CONFIG0_SET,
    #[doc = "0x2a8 - XTAL OSC Configuration 0 Register"]
    pub osc_config0_clr: OSC_CONFIG0_CLR,
    #[doc = "0x2ac - XTAL OSC Configuration 0 Register"]
    pub osc_config0_tog: OSC_CONFIG0_TOG,
    #[doc = "0x2b0 - XTAL OSC Configuration 1 Register"]
    pub osc_config1: OSC_CONFIG1,
    #[doc = "0x2b4 - XTAL OSC Configuration 1 Register"]
    pub osc_config1_set: OSC_CONFIG1_SET,
    #[doc = "0x2b8 - XTAL OSC Configuration 1 Register"]
    pub osc_config1_clr: OSC_CONFIG1_CLR,
    #[doc = "0x2bc - XTAL OSC Configuration 1 Register"]
    pub osc_config1_tog: OSC_CONFIG1_TOG,
    #[doc = "0x2c0 - XTAL OSC Configuration 2 Register"]
    pub osc_config2: OSC_CONFIG2,
    #[doc = "0x2c4 - XTAL OSC Configuration 2 Register"]
    pub osc_config2_set: OSC_CONFIG2_SET,
    #[doc = "0x2c8 - XTAL OSC Configuration 2 Register"]
    pub osc_config2_clr: OSC_CONFIG2_CLR,
    #[doc = "0x2cc - XTAL OSC Configuration 2 Register"]
    pub osc_config2_tog: OSC_CONFIG2_TOG,
}
#[doc = "Miscellaneous Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc0](misc0) module"]
pub type MISC0 = crate::Reg<u32, _MISC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC0;
#[doc = "`read()` method returns [misc0::R](misc0::R) reader structure"]
impl crate::Readable for MISC0 {}
#[doc = "`write(|w| ..)` method takes [misc0::W](misc0::W) writer structure"]
impl crate::Writable for MISC0 {}
#[doc = "Miscellaneous Register 0"]
pub mod misc0;
#[doc = "Miscellaneous Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc0_set](misc0_set) module"]
pub type MISC0_SET = crate::Reg<u32, _MISC0_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC0_SET;
#[doc = "`read()` method returns [misc0_set::R](misc0_set::R) reader structure"]
impl crate::Readable for MISC0_SET {}
#[doc = "`write(|w| ..)` method takes [misc0_set::W](misc0_set::W) writer structure"]
impl crate::Writable for MISC0_SET {}
#[doc = "Miscellaneous Register 0"]
pub mod misc0_set;
#[doc = "Miscellaneous Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc0_clr](misc0_clr) module"]
pub type MISC0_CLR = crate::Reg<u32, _MISC0_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC0_CLR;
#[doc = "`read()` method returns [misc0_clr::R](misc0_clr::R) reader structure"]
impl crate::Readable for MISC0_CLR {}
#[doc = "`write(|w| ..)` method takes [misc0_clr::W](misc0_clr::W) writer structure"]
impl crate::Writable for MISC0_CLR {}
#[doc = "Miscellaneous Register 0"]
pub mod misc0_clr;
#[doc = "Miscellaneous Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc0_tog](misc0_tog) module"]
pub type MISC0_TOG = crate::Reg<u32, _MISC0_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC0_TOG;
#[doc = "`read()` method returns [misc0_tog::R](misc0_tog::R) reader structure"]
impl crate::Readable for MISC0_TOG {}
#[doc = "`write(|w| ..)` method takes [misc0_tog::W](misc0_tog::W) writer structure"]
impl crate::Writable for MISC0_TOG {}
#[doc = "Miscellaneous Register 0"]
pub mod misc0_tog;
#[doc = "XTAL OSC (LP) Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lowpwr_ctrl](lowpwr_ctrl) module"]
pub type LOWPWR_CTRL = crate::Reg<u32, _LOWPWR_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOWPWR_CTRL;
#[doc = "`read()` method returns [lowpwr_ctrl::R](lowpwr_ctrl::R) reader structure"]
impl crate::Readable for LOWPWR_CTRL {}
#[doc = "`write(|w| ..)` method takes [lowpwr_ctrl::W](lowpwr_ctrl::W) writer structure"]
impl crate::Writable for LOWPWR_CTRL {}
#[doc = "XTAL OSC (LP) Control Register"]
pub mod lowpwr_ctrl;
#[doc = "XTAL OSC (LP) Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lowpwr_ctrl_set](lowpwr_ctrl_set) module"]
pub type LOWPWR_CTRL_SET = crate::Reg<u32, _LOWPWR_CTRL_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOWPWR_CTRL_SET;
#[doc = "`read()` method returns [lowpwr_ctrl_set::R](lowpwr_ctrl_set::R) reader structure"]
impl crate::Readable for LOWPWR_CTRL_SET {}
#[doc = "`write(|w| ..)` method takes [lowpwr_ctrl_set::W](lowpwr_ctrl_set::W) writer structure"]
impl crate::Writable for LOWPWR_CTRL_SET {}
#[doc = "XTAL OSC (LP) Control Register"]
pub mod lowpwr_ctrl_set;
#[doc = "XTAL OSC (LP) Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lowpwr_ctrl_clr](lowpwr_ctrl_clr) module"]
pub type LOWPWR_CTRL_CLR = crate::Reg<u32, _LOWPWR_CTRL_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOWPWR_CTRL_CLR;
#[doc = "`read()` method returns [lowpwr_ctrl_clr::R](lowpwr_ctrl_clr::R) reader structure"]
impl crate::Readable for LOWPWR_CTRL_CLR {}
#[doc = "`write(|w| ..)` method takes [lowpwr_ctrl_clr::W](lowpwr_ctrl_clr::W) writer structure"]
impl crate::Writable for LOWPWR_CTRL_CLR {}
#[doc = "XTAL OSC (LP) Control Register"]
pub mod lowpwr_ctrl_clr;
#[doc = "XTAL OSC (LP) Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lowpwr_ctrl_tog](lowpwr_ctrl_tog) module"]
pub type LOWPWR_CTRL_TOG = crate::Reg<u32, _LOWPWR_CTRL_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOWPWR_CTRL_TOG;
#[doc = "`read()` method returns [lowpwr_ctrl_tog::R](lowpwr_ctrl_tog::R) reader structure"]
impl crate::Readable for LOWPWR_CTRL_TOG {}
#[doc = "`write(|w| ..)` method takes [lowpwr_ctrl_tog::W](lowpwr_ctrl_tog::W) writer structure"]
impl crate::Writable for LOWPWR_CTRL_TOG {}
#[doc = "XTAL OSC (LP) Control Register"]
pub mod lowpwr_ctrl_tog;
#[doc = "XTAL OSC Configuration 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_config0](osc_config0) module"]
pub type OSC_CONFIG0 = crate::Reg<u32, _OSC_CONFIG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC_CONFIG0;
#[doc = "`read()` method returns [osc_config0::R](osc_config0::R) reader structure"]
impl crate::Readable for OSC_CONFIG0 {}
#[doc = "`write(|w| ..)` method takes [osc_config0::W](osc_config0::W) writer structure"]
impl crate::Writable for OSC_CONFIG0 {}
#[doc = "XTAL OSC Configuration 0 Register"]
pub mod osc_config0;
#[doc = "XTAL OSC Configuration 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_config0_set](osc_config0_set) module"]
pub type OSC_CONFIG0_SET = crate::Reg<u32, _OSC_CONFIG0_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC_CONFIG0_SET;
#[doc = "`read()` method returns [osc_config0_set::R](osc_config0_set::R) reader structure"]
impl crate::Readable for OSC_CONFIG0_SET {}
#[doc = "`write(|w| ..)` method takes [osc_config0_set::W](osc_config0_set::W) writer structure"]
impl crate::Writable for OSC_CONFIG0_SET {}
#[doc = "XTAL OSC Configuration 0 Register"]
pub mod osc_config0_set;
#[doc = "XTAL OSC Configuration 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_config0_clr](osc_config0_clr) module"]
pub type OSC_CONFIG0_CLR = crate::Reg<u32, _OSC_CONFIG0_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC_CONFIG0_CLR;
#[doc = "`read()` method returns [osc_config0_clr::R](osc_config0_clr::R) reader structure"]
impl crate::Readable for OSC_CONFIG0_CLR {}
#[doc = "`write(|w| ..)` method takes [osc_config0_clr::W](osc_config0_clr::W) writer structure"]
impl crate::Writable for OSC_CONFIG0_CLR {}
#[doc = "XTAL OSC Configuration 0 Register"]
pub mod osc_config0_clr;
#[doc = "XTAL OSC Configuration 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_config0_tog](osc_config0_tog) module"]
pub type OSC_CONFIG0_TOG = crate::Reg<u32, _OSC_CONFIG0_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC_CONFIG0_TOG;
#[doc = "`read()` method returns [osc_config0_tog::R](osc_config0_tog::R) reader structure"]
impl crate::Readable for OSC_CONFIG0_TOG {}
#[doc = "`write(|w| ..)` method takes [osc_config0_tog::W](osc_config0_tog::W) writer structure"]
impl crate::Writable for OSC_CONFIG0_TOG {}
#[doc = "XTAL OSC Configuration 0 Register"]
pub mod osc_config0_tog;
#[doc = "XTAL OSC Configuration 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_config1](osc_config1) module"]
pub type OSC_CONFIG1 = crate::Reg<u32, _OSC_CONFIG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC_CONFIG1;
#[doc = "`read()` method returns [osc_config1::R](osc_config1::R) reader structure"]
impl crate::Readable for OSC_CONFIG1 {}
#[doc = "`write(|w| ..)` method takes [osc_config1::W](osc_config1::W) writer structure"]
impl crate::Writable for OSC_CONFIG1 {}
#[doc = "XTAL OSC Configuration 1 Register"]
pub mod osc_config1;
#[doc = "XTAL OSC Configuration 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_config1_set](osc_config1_set) module"]
pub type OSC_CONFIG1_SET = crate::Reg<u32, _OSC_CONFIG1_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC_CONFIG1_SET;
#[doc = "`read()` method returns [osc_config1_set::R](osc_config1_set::R) reader structure"]
impl crate::Readable for OSC_CONFIG1_SET {}
#[doc = "`write(|w| ..)` method takes [osc_config1_set::W](osc_config1_set::W) writer structure"]
impl crate::Writable for OSC_CONFIG1_SET {}
#[doc = "XTAL OSC Configuration 1 Register"]
pub mod osc_config1_set;
#[doc = "XTAL OSC Configuration 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_config1_clr](osc_config1_clr) module"]
pub type OSC_CONFIG1_CLR = crate::Reg<u32, _OSC_CONFIG1_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC_CONFIG1_CLR;
#[doc = "`read()` method returns [osc_config1_clr::R](osc_config1_clr::R) reader structure"]
impl crate::Readable for OSC_CONFIG1_CLR {}
#[doc = "`write(|w| ..)` method takes [osc_config1_clr::W](osc_config1_clr::W) writer structure"]
impl crate::Writable for OSC_CONFIG1_CLR {}
#[doc = "XTAL OSC Configuration 1 Register"]
pub mod osc_config1_clr;
#[doc = "XTAL OSC Configuration 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_config1_tog](osc_config1_tog) module"]
pub type OSC_CONFIG1_TOG = crate::Reg<u32, _OSC_CONFIG1_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC_CONFIG1_TOG;
#[doc = "`read()` method returns [osc_config1_tog::R](osc_config1_tog::R) reader structure"]
impl crate::Readable for OSC_CONFIG1_TOG {}
#[doc = "`write(|w| ..)` method takes [osc_config1_tog::W](osc_config1_tog::W) writer structure"]
impl crate::Writable for OSC_CONFIG1_TOG {}
#[doc = "XTAL OSC Configuration 1 Register"]
pub mod osc_config1_tog;
#[doc = "XTAL OSC Configuration 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_config2](osc_config2) module"]
pub type OSC_CONFIG2 = crate::Reg<u32, _OSC_CONFIG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC_CONFIG2;
#[doc = "`read()` method returns [osc_config2::R](osc_config2::R) reader structure"]
impl crate::Readable for OSC_CONFIG2 {}
#[doc = "`write(|w| ..)` method takes [osc_config2::W](osc_config2::W) writer structure"]
impl crate::Writable for OSC_CONFIG2 {}
#[doc = "XTAL OSC Configuration 2 Register"]
pub mod osc_config2;
#[doc = "XTAL OSC Configuration 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_config2_set](osc_config2_set) module"]
pub type OSC_CONFIG2_SET = crate::Reg<u32, _OSC_CONFIG2_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC_CONFIG2_SET;
#[doc = "`read()` method returns [osc_config2_set::R](osc_config2_set::R) reader structure"]
impl crate::Readable for OSC_CONFIG2_SET {}
#[doc = "`write(|w| ..)` method takes [osc_config2_set::W](osc_config2_set::W) writer structure"]
impl crate::Writable for OSC_CONFIG2_SET {}
#[doc = "XTAL OSC Configuration 2 Register"]
pub mod osc_config2_set;
#[doc = "XTAL OSC Configuration 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_config2_clr](osc_config2_clr) module"]
pub type OSC_CONFIG2_CLR = crate::Reg<u32, _OSC_CONFIG2_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC_CONFIG2_CLR;
#[doc = "`read()` method returns [osc_config2_clr::R](osc_config2_clr::R) reader structure"]
impl crate::Readable for OSC_CONFIG2_CLR {}
#[doc = "`write(|w| ..)` method takes [osc_config2_clr::W](osc_config2_clr::W) writer structure"]
impl crate::Writable for OSC_CONFIG2_CLR {}
#[doc = "XTAL OSC Configuration 2 Register"]
pub mod osc_config2_clr;
#[doc = "XTAL OSC Configuration 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_config2_tog](osc_config2_tog) module"]
pub type OSC_CONFIG2_TOG = crate::Reg<u32, _OSC_CONFIG2_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC_CONFIG2_TOG;
#[doc = "`read()` method returns [osc_config2_tog::R](osc_config2_tog::R) reader structure"]
impl crate::Readable for OSC_CONFIG2_TOG {}
#[doc = "`write(|w| ..)` method takes [osc_config2_tog::W](osc_config2_tog::W) writer structure"]
impl crate::Writable for OSC_CONFIG2_TOG {}
#[doc = "XTAL OSC Configuration 2 Register"]
pub mod osc_config2_tog;
