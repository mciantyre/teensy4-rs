#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LCDIF General Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - LCDIF General Control Register"]
    pub ctrl_set: CTRL_SET,
    #[doc = "0x08 - LCDIF General Control Register"]
    pub ctrl_clr: CTRL_CLR,
    #[doc = "0x0c - LCDIF General Control Register"]
    pub ctrl_tog: CTRL_TOG,
    #[doc = "0x10 - LCDIF General Control1 Register"]
    pub ctrl1: CTRL1,
    #[doc = "0x14 - LCDIF General Control1 Register"]
    pub ctrl1_set: CTRL1_SET,
    #[doc = "0x18 - LCDIF General Control1 Register"]
    pub ctrl1_clr: CTRL1_CLR,
    #[doc = "0x1c - LCDIF General Control1 Register"]
    pub ctrl1_tog: CTRL1_TOG,
    #[doc = "0x20 - LCDIF General Control2 Register"]
    pub ctrl2: CTRL2,
    #[doc = "0x24 - LCDIF General Control2 Register"]
    pub ctrl2_set: CTRL2_SET,
    #[doc = "0x28 - LCDIF General Control2 Register"]
    pub ctrl2_clr: CTRL2_CLR,
    #[doc = "0x2c - LCDIF General Control2 Register"]
    pub ctrl2_tog: CTRL2_TOG,
    #[doc = "0x30 - LCDIF Horizontal and Vertical Valid Data Count Register"]
    pub transfer_count: TRANSFER_COUNT,
    _reserved13: [u8; 12usize],
    #[doc = "0x40 - LCD Interface Current Buffer Address Register"]
    pub cur_buf: CUR_BUF,
    _reserved14: [u8; 12usize],
    #[doc = "0x50 - LCD Interface Next Buffer Address Register"]
    pub next_buf: NEXT_BUF,
    _reserved15: [u8; 28usize],
    #[doc = "0x70 - LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
    pub vdctrl0: VDCTRL0,
    #[doc = "0x74 - LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
    pub vdctrl0_set: VDCTRL0_SET,
    #[doc = "0x78 - LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
    pub vdctrl0_clr: VDCTRL0_CLR,
    #[doc = "0x7c - LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
    pub vdctrl0_tog: VDCTRL0_TOG,
    #[doc = "0x80 - LCDIF VSYNC Mode and Dotclk Mode Control Register1"]
    pub vdctrl1: VDCTRL1,
    _reserved20: [u8; 12usize],
    #[doc = "0x90 - LCDIF VSYNC Mode and Dotclk Mode Control Register2"]
    pub vdctrl2: VDCTRL2,
    _reserved21: [u8; 12usize],
    #[doc = "0xa0 - LCDIF VSYNC Mode and Dotclk Mode Control Register3"]
    pub vdctrl3: VDCTRL3,
    _reserved22: [u8; 12usize],
    #[doc = "0xb0 - LCDIF VSYNC Mode and Dotclk Mode Control Register4"]
    pub vdctrl4: VDCTRL4,
    _reserved23: [u8; 220usize],
    #[doc = "0x190 - Bus Master Error Status Register"]
    pub bm_error_stat: BM_ERROR_STAT,
    _reserved24: [u8; 12usize],
    #[doc = "0x1a0 - CRC Status Register"]
    pub crc_stat: CRC_STAT,
    _reserved25: [u8; 12usize],
    #[doc = "0x1b0 - LCD Interface Status Register"]
    pub stat: STAT,
    _reserved26: [u8; 460usize],
    #[doc = "0x380 - LCDIF Pigeon Mode Control0 Register"]
    pub pigeonctrl0: PIGEONCTRL0,
    #[doc = "0x384 - LCDIF Pigeon Mode Control0 Register"]
    pub pigeonctrl0_set: PIGEONCTRL0_SET,
    #[doc = "0x388 - LCDIF Pigeon Mode Control0 Register"]
    pub pigeonctrl0_clr: PIGEONCTRL0_CLR,
    #[doc = "0x38c - LCDIF Pigeon Mode Control0 Register"]
    pub pigeonctrl0_tog: PIGEONCTRL0_TOG,
    #[doc = "0x390 - LCDIF Pigeon Mode Control1 Register"]
    pub pigeonctrl1: PIGEONCTRL1,
    #[doc = "0x394 - LCDIF Pigeon Mode Control1 Register"]
    pub pigeonctrl1_set: PIGEONCTRL1_SET,
    #[doc = "0x398 - LCDIF Pigeon Mode Control1 Register"]
    pub pigeonctrl1_clr: PIGEONCTRL1_CLR,
    #[doc = "0x39c - LCDIF Pigeon Mode Control1 Register"]
    pub pigeonctrl1_tog: PIGEONCTRL1_TOG,
    #[doc = "0x3a0 - LCDIF Pigeon Mode Control2 Register"]
    pub pigeonctrl2: PIGEONCTRL2,
    #[doc = "0x3a4 - LCDIF Pigeon Mode Control2 Register"]
    pub pigeonctrl2_set: PIGEONCTRL2_SET,
    #[doc = "0x3a8 - LCDIF Pigeon Mode Control2 Register"]
    pub pigeonctrl2_clr: PIGEONCTRL2_CLR,
    #[doc = "0x3ac - LCDIF Pigeon Mode Control2 Register"]
    pub pigeonctrl2_tog: PIGEONCTRL2_TOG,
    _reserved38: [u8; 1104usize],
    #[doc = "0x800 - Panel Interface Signal Generator Register"]
    pub pigeon_0_0: PIGEON_0_0,
    _reserved39: [u8; 12usize],
    #[doc = "0x810 - Panel Interface Signal Generator Register"]
    pub pigeon_0_1: PIGEON_0_1,
    _reserved40: [u8; 12usize],
    #[doc = "0x820 - Panel Interface Signal Generator Register"]
    pub pigeon_0_2: PIGEON_0_2,
    _reserved41: [u8; 28usize],
    #[doc = "0x840 - Panel Interface Signal Generator Register"]
    pub pigeon_1_0: PIGEON_1_0,
    _reserved42: [u8; 12usize],
    #[doc = "0x850 - Panel Interface Signal Generator Register"]
    pub pigeon_1_1: PIGEON_1_1,
    _reserved43: [u8; 12usize],
    #[doc = "0x860 - Panel Interface Signal Generator Register"]
    pub pigeon_1_2: PIGEON_1_2,
    _reserved44: [u8; 28usize],
    #[doc = "0x880 - Panel Interface Signal Generator Register"]
    pub pigeon_2_0: PIGEON_2_0,
    _reserved45: [u8; 12usize],
    #[doc = "0x890 - Panel Interface Signal Generator Register"]
    pub pigeon_2_1: PIGEON_2_1,
    _reserved46: [u8; 12usize],
    #[doc = "0x8a0 - Panel Interface Signal Generator Register"]
    pub pigeon_2_2: PIGEON_2_2,
    _reserved47: [u8; 28usize],
    #[doc = "0x8c0 - Panel Interface Signal Generator Register"]
    pub pigeon_3_0: PIGEON_3_0,
    _reserved48: [u8; 12usize],
    #[doc = "0x8d0 - Panel Interface Signal Generator Register"]
    pub pigeon_3_1: PIGEON_3_1,
    _reserved49: [u8; 12usize],
    #[doc = "0x8e0 - Panel Interface Signal Generator Register"]
    pub pigeon_3_2: PIGEON_3_2,
    _reserved50: [u8; 28usize],
    #[doc = "0x900 - Panel Interface Signal Generator Register"]
    pub pigeon_4_0: PIGEON_4_0,
    _reserved51: [u8; 12usize],
    #[doc = "0x910 - Panel Interface Signal Generator Register"]
    pub pigeon_4_1: PIGEON_4_1,
    _reserved52: [u8; 12usize],
    #[doc = "0x920 - Panel Interface Signal Generator Register"]
    pub pigeon_4_2: PIGEON_4_2,
    _reserved53: [u8; 28usize],
    #[doc = "0x940 - Panel Interface Signal Generator Register"]
    pub pigeon_5_0: PIGEON_5_0,
    _reserved54: [u8; 12usize],
    #[doc = "0x950 - Panel Interface Signal Generator Register"]
    pub pigeon_5_1: PIGEON_5_1,
    _reserved55: [u8; 12usize],
    #[doc = "0x960 - Panel Interface Signal Generator Register"]
    pub pigeon_5_2: PIGEON_5_2,
    _reserved56: [u8; 28usize],
    #[doc = "0x980 - Panel Interface Signal Generator Register"]
    pub pigeon_6_0: PIGEON_6_0,
    _reserved57: [u8; 12usize],
    #[doc = "0x990 - Panel Interface Signal Generator Register"]
    pub pigeon_6_1: PIGEON_6_1,
    _reserved58: [u8; 12usize],
    #[doc = "0x9a0 - Panel Interface Signal Generator Register"]
    pub pigeon_6_2: PIGEON_6_2,
    _reserved59: [u8; 28usize],
    #[doc = "0x9c0 - Panel Interface Signal Generator Register"]
    pub pigeon_7_0: PIGEON_7_0,
    _reserved60: [u8; 12usize],
    #[doc = "0x9d0 - Panel Interface Signal Generator Register"]
    pub pigeon_7_1: PIGEON_7_1,
    _reserved61: [u8; 12usize],
    #[doc = "0x9e0 - Panel Interface Signal Generator Register"]
    pub pigeon_7_2: PIGEON_7_2,
    _reserved62: [u8; 28usize],
    #[doc = "0xa00 - Panel Interface Signal Generator Register"]
    pub pigeon_8_0: PIGEON_8_0,
    _reserved63: [u8; 12usize],
    #[doc = "0xa10 - Panel Interface Signal Generator Register"]
    pub pigeon_8_1: PIGEON_8_1,
    _reserved64: [u8; 12usize],
    #[doc = "0xa20 - Panel Interface Signal Generator Register"]
    pub pigeon_8_2: PIGEON_8_2,
    _reserved65: [u8; 28usize],
    #[doc = "0xa40 - Panel Interface Signal Generator Register"]
    pub pigeon_9_0: PIGEON_9_0,
    _reserved66: [u8; 12usize],
    #[doc = "0xa50 - Panel Interface Signal Generator Register"]
    pub pigeon_9_1: PIGEON_9_1,
    _reserved67: [u8; 12usize],
    #[doc = "0xa60 - Panel Interface Signal Generator Register"]
    pub pigeon_9_2: PIGEON_9_2,
    _reserved68: [u8; 28usize],
    #[doc = "0xa80 - Panel Interface Signal Generator Register"]
    pub pigeon_10_0: PIGEON_10_0,
    _reserved69: [u8; 12usize],
    #[doc = "0xa90 - Panel Interface Signal Generator Register"]
    pub pigeon_10_1: PIGEON_10_1,
    _reserved70: [u8; 12usize],
    #[doc = "0xaa0 - Panel Interface Signal Generator Register"]
    pub pigeon_10_2: PIGEON_10_2,
    _reserved71: [u8; 28usize],
    #[doc = "0xac0 - Panel Interface Signal Generator Register"]
    pub pigeon_11_0: PIGEON_11_0,
    _reserved72: [u8; 12usize],
    #[doc = "0xad0 - Panel Interface Signal Generator Register"]
    pub pigeon_11_1: PIGEON_11_1,
    _reserved73: [u8; 12usize],
    #[doc = "0xae0 - Panel Interface Signal Generator Register"]
    pub pigeon_11_2: PIGEON_11_2,
    _reserved74: [u8; 28usize],
    #[doc = "0xb00 - Lookup Table Data Register."]
    pub lut_ctrl: LUT_CTRL,
    _reserved75: [u8; 12usize],
    #[doc = "0xb10 - Lookup Table Control Register."]
    pub lut0_addr: LUT0_ADDR,
    _reserved76: [u8; 12usize],
    #[doc = "0xb20 - Lookup Table Data Register."]
    pub lut0_data: LUT0_DATA,
    _reserved77: [u8; 12usize],
    #[doc = "0xb30 - Lookup Table Control Register."]
    pub lut1_addr: LUT1_ADDR,
    _reserved78: [u8; 12usize],
    #[doc = "0xb40 - Lookup Table Data Register."]
    pub lut1_data: LUT1_DATA,
}
#[doc = "LCDIF General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "LCDIF General Control Register"]
pub mod ctrl;
#[doc = "LCDIF General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_set](ctrl_set) module"]
pub type CTRL_SET = crate::Reg<u32, _CTRL_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_SET;
#[doc = "`read()` method returns [ctrl_set::R](ctrl_set::R) reader structure"]
impl crate::Readable for CTRL_SET {}
#[doc = "`write(|w| ..)` method takes [ctrl_set::W](ctrl_set::W) writer structure"]
impl crate::Writable for CTRL_SET {}
#[doc = "LCDIF General Control Register"]
pub mod ctrl_set;
#[doc = "LCDIF General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_clr](ctrl_clr) module"]
pub type CTRL_CLR = crate::Reg<u32, _CTRL_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_CLR;
#[doc = "`read()` method returns [ctrl_clr::R](ctrl_clr::R) reader structure"]
impl crate::Readable for CTRL_CLR {}
#[doc = "`write(|w| ..)` method takes [ctrl_clr::W](ctrl_clr::W) writer structure"]
impl crate::Writable for CTRL_CLR {}
#[doc = "LCDIF General Control Register"]
pub mod ctrl_clr;
#[doc = "LCDIF General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_tog](ctrl_tog) module"]
pub type CTRL_TOG = crate::Reg<u32, _CTRL_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_TOG;
#[doc = "`read()` method returns [ctrl_tog::R](ctrl_tog::R) reader structure"]
impl crate::Readable for CTRL_TOG {}
#[doc = "`write(|w| ..)` method takes [ctrl_tog::W](ctrl_tog::W) writer structure"]
impl crate::Writable for CTRL_TOG {}
#[doc = "LCDIF General Control Register"]
pub mod ctrl_tog;
#[doc = "LCDIF General Control1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1](ctrl1) module"]
pub type CTRL1 = crate::Reg<u32, _CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL1;
#[doc = "`read()` method returns [ctrl1::R](ctrl1::R) reader structure"]
impl crate::Readable for CTRL1 {}
#[doc = "`write(|w| ..)` method takes [ctrl1::W](ctrl1::W) writer structure"]
impl crate::Writable for CTRL1 {}
#[doc = "LCDIF General Control1 Register"]
pub mod ctrl1;
#[doc = "LCDIF General Control1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1_set](ctrl1_set) module"]
pub type CTRL1_SET = crate::Reg<u32, _CTRL1_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL1_SET;
#[doc = "`read()` method returns [ctrl1_set::R](ctrl1_set::R) reader structure"]
impl crate::Readable for CTRL1_SET {}
#[doc = "`write(|w| ..)` method takes [ctrl1_set::W](ctrl1_set::W) writer structure"]
impl crate::Writable for CTRL1_SET {}
#[doc = "LCDIF General Control1 Register"]
pub mod ctrl1_set;
#[doc = "LCDIF General Control1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1_clr](ctrl1_clr) module"]
pub type CTRL1_CLR = crate::Reg<u32, _CTRL1_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL1_CLR;
#[doc = "`read()` method returns [ctrl1_clr::R](ctrl1_clr::R) reader structure"]
impl crate::Readable for CTRL1_CLR {}
#[doc = "`write(|w| ..)` method takes [ctrl1_clr::W](ctrl1_clr::W) writer structure"]
impl crate::Writable for CTRL1_CLR {}
#[doc = "LCDIF General Control1 Register"]
pub mod ctrl1_clr;
#[doc = "LCDIF General Control1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1_tog](ctrl1_tog) module"]
pub type CTRL1_TOG = crate::Reg<u32, _CTRL1_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL1_TOG;
#[doc = "`read()` method returns [ctrl1_tog::R](ctrl1_tog::R) reader structure"]
impl crate::Readable for CTRL1_TOG {}
#[doc = "`write(|w| ..)` method takes [ctrl1_tog::W](ctrl1_tog::W) writer structure"]
impl crate::Writable for CTRL1_TOG {}
#[doc = "LCDIF General Control1 Register"]
pub mod ctrl1_tog;
#[doc = "LCDIF General Control2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](ctrl2) module"]
pub type CTRL2 = crate::Reg<u32, _CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL2;
#[doc = "`read()` method returns [ctrl2::R](ctrl2::R) reader structure"]
impl crate::Readable for CTRL2 {}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](ctrl2::W) writer structure"]
impl crate::Writable for CTRL2 {}
#[doc = "LCDIF General Control2 Register"]
pub mod ctrl2;
#[doc = "LCDIF General Control2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2_set](ctrl2_set) module"]
pub type CTRL2_SET = crate::Reg<u32, _CTRL2_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL2_SET;
#[doc = "`read()` method returns [ctrl2_set::R](ctrl2_set::R) reader structure"]
impl crate::Readable for CTRL2_SET {}
#[doc = "`write(|w| ..)` method takes [ctrl2_set::W](ctrl2_set::W) writer structure"]
impl crate::Writable for CTRL2_SET {}
#[doc = "LCDIF General Control2 Register"]
pub mod ctrl2_set;
#[doc = "LCDIF General Control2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2_clr](ctrl2_clr) module"]
pub type CTRL2_CLR = crate::Reg<u32, _CTRL2_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL2_CLR;
#[doc = "`read()` method returns [ctrl2_clr::R](ctrl2_clr::R) reader structure"]
impl crate::Readable for CTRL2_CLR {}
#[doc = "`write(|w| ..)` method takes [ctrl2_clr::W](ctrl2_clr::W) writer structure"]
impl crate::Writable for CTRL2_CLR {}
#[doc = "LCDIF General Control2 Register"]
pub mod ctrl2_clr;
#[doc = "LCDIF General Control2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2_tog](ctrl2_tog) module"]
pub type CTRL2_TOG = crate::Reg<u32, _CTRL2_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL2_TOG;
#[doc = "`read()` method returns [ctrl2_tog::R](ctrl2_tog::R) reader structure"]
impl crate::Readable for CTRL2_TOG {}
#[doc = "`write(|w| ..)` method takes [ctrl2_tog::W](ctrl2_tog::W) writer structure"]
impl crate::Writable for CTRL2_TOG {}
#[doc = "LCDIF General Control2 Register"]
pub mod ctrl2_tog;
#[doc = "LCDIF Horizontal and Vertical Valid Data Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [transfer_count](transfer_count) module"]
pub type TRANSFER_COUNT = crate::Reg<u32, _TRANSFER_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRANSFER_COUNT;
#[doc = "`read()` method returns [transfer_count::R](transfer_count::R) reader structure"]
impl crate::Readable for TRANSFER_COUNT {}
#[doc = "`write(|w| ..)` method takes [transfer_count::W](transfer_count::W) writer structure"]
impl crate::Writable for TRANSFER_COUNT {}
#[doc = "LCDIF Horizontal and Vertical Valid Data Count Register"]
pub mod transfer_count;
#[doc = "LCD Interface Current Buffer Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cur_buf](cur_buf) module"]
pub type CUR_BUF = crate::Reg<u32, _CUR_BUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUR_BUF;
#[doc = "`read()` method returns [cur_buf::R](cur_buf::R) reader structure"]
impl crate::Readable for CUR_BUF {}
#[doc = "`write(|w| ..)` method takes [cur_buf::W](cur_buf::W) writer structure"]
impl crate::Writable for CUR_BUF {}
#[doc = "LCD Interface Current Buffer Address Register"]
pub mod cur_buf;
#[doc = "LCD Interface Next Buffer Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [next_buf](next_buf) module"]
pub type NEXT_BUF = crate::Reg<u32, _NEXT_BUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NEXT_BUF;
#[doc = "`read()` method returns [next_buf::R](next_buf::R) reader structure"]
impl crate::Readable for NEXT_BUF {}
#[doc = "`write(|w| ..)` method takes [next_buf::W](next_buf::W) writer structure"]
impl crate::Writable for NEXT_BUF {}
#[doc = "LCD Interface Next Buffer Address Register"]
pub mod next_buf;
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdctrl0](vdctrl0) module"]
pub type VDCTRL0 = crate::Reg<u32, _VDCTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VDCTRL0;
#[doc = "`read()` method returns [vdctrl0::R](vdctrl0::R) reader structure"]
impl crate::Readable for VDCTRL0 {}
#[doc = "`write(|w| ..)` method takes [vdctrl0::W](vdctrl0::W) writer structure"]
impl crate::Writable for VDCTRL0 {}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
pub mod vdctrl0;
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdctrl0_set](vdctrl0_set) module"]
pub type VDCTRL0_SET = crate::Reg<u32, _VDCTRL0_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VDCTRL0_SET;
#[doc = "`read()` method returns [vdctrl0_set::R](vdctrl0_set::R) reader structure"]
impl crate::Readable for VDCTRL0_SET {}
#[doc = "`write(|w| ..)` method takes [vdctrl0_set::W](vdctrl0_set::W) writer structure"]
impl crate::Writable for VDCTRL0_SET {}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
pub mod vdctrl0_set;
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdctrl0_clr](vdctrl0_clr) module"]
pub type VDCTRL0_CLR = crate::Reg<u32, _VDCTRL0_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VDCTRL0_CLR;
#[doc = "`read()` method returns [vdctrl0_clr::R](vdctrl0_clr::R) reader structure"]
impl crate::Readable for VDCTRL0_CLR {}
#[doc = "`write(|w| ..)` method takes [vdctrl0_clr::W](vdctrl0_clr::W) writer structure"]
impl crate::Writable for VDCTRL0_CLR {}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
pub mod vdctrl0_clr;
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdctrl0_tog](vdctrl0_tog) module"]
pub type VDCTRL0_TOG = crate::Reg<u32, _VDCTRL0_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VDCTRL0_TOG;
#[doc = "`read()` method returns [vdctrl0_tog::R](vdctrl0_tog::R) reader structure"]
impl crate::Readable for VDCTRL0_TOG {}
#[doc = "`write(|w| ..)` method takes [vdctrl0_tog::W](vdctrl0_tog::W) writer structure"]
impl crate::Writable for VDCTRL0_TOG {}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
pub mod vdctrl0_tog;
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdctrl1](vdctrl1) module"]
pub type VDCTRL1 = crate::Reg<u32, _VDCTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VDCTRL1;
#[doc = "`read()` method returns [vdctrl1::R](vdctrl1::R) reader structure"]
impl crate::Readable for VDCTRL1 {}
#[doc = "`write(|w| ..)` method takes [vdctrl1::W](vdctrl1::W) writer structure"]
impl crate::Writable for VDCTRL1 {}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register1"]
pub mod vdctrl1;
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdctrl2](vdctrl2) module"]
pub type VDCTRL2 = crate::Reg<u32, _VDCTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VDCTRL2;
#[doc = "`read()` method returns [vdctrl2::R](vdctrl2::R) reader structure"]
impl crate::Readable for VDCTRL2 {}
#[doc = "`write(|w| ..)` method takes [vdctrl2::W](vdctrl2::W) writer structure"]
impl crate::Writable for VDCTRL2 {}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register2"]
pub mod vdctrl2;
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdctrl3](vdctrl3) module"]
pub type VDCTRL3 = crate::Reg<u32, _VDCTRL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VDCTRL3;
#[doc = "`read()` method returns [vdctrl3::R](vdctrl3::R) reader structure"]
impl crate::Readable for VDCTRL3 {}
#[doc = "`write(|w| ..)` method takes [vdctrl3::W](vdctrl3::W) writer structure"]
impl crate::Writable for VDCTRL3 {}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register3"]
pub mod vdctrl3;
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdctrl4](vdctrl4) module"]
pub type VDCTRL4 = crate::Reg<u32, _VDCTRL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VDCTRL4;
#[doc = "`read()` method returns [vdctrl4::R](vdctrl4::R) reader structure"]
impl crate::Readable for VDCTRL4 {}
#[doc = "`write(|w| ..)` method takes [vdctrl4::W](vdctrl4::W) writer structure"]
impl crate::Writable for VDCTRL4 {}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register4"]
pub mod vdctrl4;
#[doc = "Bus Master Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bm_error_stat](bm_error_stat) module"]
pub type BM_ERROR_STAT = crate::Reg<u32, _BM_ERROR_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BM_ERROR_STAT;
#[doc = "`read()` method returns [bm_error_stat::R](bm_error_stat::R) reader structure"]
impl crate::Readable for BM_ERROR_STAT {}
#[doc = "`write(|w| ..)` method takes [bm_error_stat::W](bm_error_stat::W) writer structure"]
impl crate::Writable for BM_ERROR_STAT {}
#[doc = "Bus Master Error Status Register"]
pub mod bm_error_stat;
#[doc = "CRC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_stat](crc_stat) module"]
pub type CRC_STAT = crate::Reg<u32, _CRC_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_STAT;
#[doc = "`read()` method returns [crc_stat::R](crc_stat::R) reader structure"]
impl crate::Readable for CRC_STAT {}
#[doc = "`write(|w| ..)` method takes [crc_stat::W](crc_stat::W) writer structure"]
impl crate::Writable for CRC_STAT {}
#[doc = "CRC Status Register"]
pub mod crc_stat;
#[doc = "LCD Interface Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "LCD Interface Status Register"]
pub mod stat;
#[doc = "LCDIF Pigeon Mode Control0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeonctrl0](pigeonctrl0) module"]
pub type PIGEONCTRL0 = crate::Reg<u32, _PIGEONCTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEONCTRL0;
#[doc = "`read()` method returns [pigeonctrl0::R](pigeonctrl0::R) reader structure"]
impl crate::Readable for PIGEONCTRL0 {}
#[doc = "`write(|w| ..)` method takes [pigeonctrl0::W](pigeonctrl0::W) writer structure"]
impl crate::Writable for PIGEONCTRL0 {}
#[doc = "LCDIF Pigeon Mode Control0 Register"]
pub mod pigeonctrl0;
#[doc = "LCDIF Pigeon Mode Control0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeonctrl0_set](pigeonctrl0_set) module"]
pub type PIGEONCTRL0_SET = crate::Reg<u32, _PIGEONCTRL0_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEONCTRL0_SET;
#[doc = "`read()` method returns [pigeonctrl0_set::R](pigeonctrl0_set::R) reader structure"]
impl crate::Readable for PIGEONCTRL0_SET {}
#[doc = "`write(|w| ..)` method takes [pigeonctrl0_set::W](pigeonctrl0_set::W) writer structure"]
impl crate::Writable for PIGEONCTRL0_SET {}
#[doc = "LCDIF Pigeon Mode Control0 Register"]
pub mod pigeonctrl0_set;
#[doc = "LCDIF Pigeon Mode Control0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeonctrl0_clr](pigeonctrl0_clr) module"]
pub type PIGEONCTRL0_CLR = crate::Reg<u32, _PIGEONCTRL0_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEONCTRL0_CLR;
#[doc = "`read()` method returns [pigeonctrl0_clr::R](pigeonctrl0_clr::R) reader structure"]
impl crate::Readable for PIGEONCTRL0_CLR {}
#[doc = "`write(|w| ..)` method takes [pigeonctrl0_clr::W](pigeonctrl0_clr::W) writer structure"]
impl crate::Writable for PIGEONCTRL0_CLR {}
#[doc = "LCDIF Pigeon Mode Control0 Register"]
pub mod pigeonctrl0_clr;
#[doc = "LCDIF Pigeon Mode Control0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeonctrl0_tog](pigeonctrl0_tog) module"]
pub type PIGEONCTRL0_TOG = crate::Reg<u32, _PIGEONCTRL0_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEONCTRL0_TOG;
#[doc = "`read()` method returns [pigeonctrl0_tog::R](pigeonctrl0_tog::R) reader structure"]
impl crate::Readable for PIGEONCTRL0_TOG {}
#[doc = "`write(|w| ..)` method takes [pigeonctrl0_tog::W](pigeonctrl0_tog::W) writer structure"]
impl crate::Writable for PIGEONCTRL0_TOG {}
#[doc = "LCDIF Pigeon Mode Control0 Register"]
pub mod pigeonctrl0_tog;
#[doc = "LCDIF Pigeon Mode Control1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeonctrl1](pigeonctrl1) module"]
pub type PIGEONCTRL1 = crate::Reg<u32, _PIGEONCTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEONCTRL1;
#[doc = "`read()` method returns [pigeonctrl1::R](pigeonctrl1::R) reader structure"]
impl crate::Readable for PIGEONCTRL1 {}
#[doc = "`write(|w| ..)` method takes [pigeonctrl1::W](pigeonctrl1::W) writer structure"]
impl crate::Writable for PIGEONCTRL1 {}
#[doc = "LCDIF Pigeon Mode Control1 Register"]
pub mod pigeonctrl1;
#[doc = "LCDIF Pigeon Mode Control1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeonctrl1_set](pigeonctrl1_set) module"]
pub type PIGEONCTRL1_SET = crate::Reg<u32, _PIGEONCTRL1_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEONCTRL1_SET;
#[doc = "`read()` method returns [pigeonctrl1_set::R](pigeonctrl1_set::R) reader structure"]
impl crate::Readable for PIGEONCTRL1_SET {}
#[doc = "`write(|w| ..)` method takes [pigeonctrl1_set::W](pigeonctrl1_set::W) writer structure"]
impl crate::Writable for PIGEONCTRL1_SET {}
#[doc = "LCDIF Pigeon Mode Control1 Register"]
pub mod pigeonctrl1_set;
#[doc = "LCDIF Pigeon Mode Control1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeonctrl1_clr](pigeonctrl1_clr) module"]
pub type PIGEONCTRL1_CLR = crate::Reg<u32, _PIGEONCTRL1_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEONCTRL1_CLR;
#[doc = "`read()` method returns [pigeonctrl1_clr::R](pigeonctrl1_clr::R) reader structure"]
impl crate::Readable for PIGEONCTRL1_CLR {}
#[doc = "`write(|w| ..)` method takes [pigeonctrl1_clr::W](pigeonctrl1_clr::W) writer structure"]
impl crate::Writable for PIGEONCTRL1_CLR {}
#[doc = "LCDIF Pigeon Mode Control1 Register"]
pub mod pigeonctrl1_clr;
#[doc = "LCDIF Pigeon Mode Control1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeonctrl1_tog](pigeonctrl1_tog) module"]
pub type PIGEONCTRL1_TOG = crate::Reg<u32, _PIGEONCTRL1_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEONCTRL1_TOG;
#[doc = "`read()` method returns [pigeonctrl1_tog::R](pigeonctrl1_tog::R) reader structure"]
impl crate::Readable for PIGEONCTRL1_TOG {}
#[doc = "`write(|w| ..)` method takes [pigeonctrl1_tog::W](pigeonctrl1_tog::W) writer structure"]
impl crate::Writable for PIGEONCTRL1_TOG {}
#[doc = "LCDIF Pigeon Mode Control1 Register"]
pub mod pigeonctrl1_tog;
#[doc = "LCDIF Pigeon Mode Control2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeonctrl2](pigeonctrl2) module"]
pub type PIGEONCTRL2 = crate::Reg<u32, _PIGEONCTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEONCTRL2;
#[doc = "`read()` method returns [pigeonctrl2::R](pigeonctrl2::R) reader structure"]
impl crate::Readable for PIGEONCTRL2 {}
#[doc = "`write(|w| ..)` method takes [pigeonctrl2::W](pigeonctrl2::W) writer structure"]
impl crate::Writable for PIGEONCTRL2 {}
#[doc = "LCDIF Pigeon Mode Control2 Register"]
pub mod pigeonctrl2;
#[doc = "LCDIF Pigeon Mode Control2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeonctrl2_set](pigeonctrl2_set) module"]
pub type PIGEONCTRL2_SET = crate::Reg<u32, _PIGEONCTRL2_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEONCTRL2_SET;
#[doc = "`read()` method returns [pigeonctrl2_set::R](pigeonctrl2_set::R) reader structure"]
impl crate::Readable for PIGEONCTRL2_SET {}
#[doc = "`write(|w| ..)` method takes [pigeonctrl2_set::W](pigeonctrl2_set::W) writer structure"]
impl crate::Writable for PIGEONCTRL2_SET {}
#[doc = "LCDIF Pigeon Mode Control2 Register"]
pub mod pigeonctrl2_set;
#[doc = "LCDIF Pigeon Mode Control2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeonctrl2_clr](pigeonctrl2_clr) module"]
pub type PIGEONCTRL2_CLR = crate::Reg<u32, _PIGEONCTRL2_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEONCTRL2_CLR;
#[doc = "`read()` method returns [pigeonctrl2_clr::R](pigeonctrl2_clr::R) reader structure"]
impl crate::Readable for PIGEONCTRL2_CLR {}
#[doc = "`write(|w| ..)` method takes [pigeonctrl2_clr::W](pigeonctrl2_clr::W) writer structure"]
impl crate::Writable for PIGEONCTRL2_CLR {}
#[doc = "LCDIF Pigeon Mode Control2 Register"]
pub mod pigeonctrl2_clr;
#[doc = "LCDIF Pigeon Mode Control2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeonctrl2_tog](pigeonctrl2_tog) module"]
pub type PIGEONCTRL2_TOG = crate::Reg<u32, _PIGEONCTRL2_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEONCTRL2_TOG;
#[doc = "`read()` method returns [pigeonctrl2_tog::R](pigeonctrl2_tog::R) reader structure"]
impl crate::Readable for PIGEONCTRL2_TOG {}
#[doc = "`write(|w| ..)` method takes [pigeonctrl2_tog::W](pigeonctrl2_tog::W) writer structure"]
impl crate::Writable for PIGEONCTRL2_TOG {}
#[doc = "LCDIF Pigeon Mode Control2 Register"]
pub mod pigeonctrl2_tog;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_0_0](pigeon_0_0) module"]
pub type PIGEON_0_0 = crate::Reg<u32, _PIGEON_0_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_0_0;
#[doc = "`read()` method returns [pigeon_0_0::R](pigeon_0_0::R) reader structure"]
impl crate::Readable for PIGEON_0_0 {}
#[doc = "`write(|w| ..)` method takes [pigeon_0_0::W](pigeon_0_0::W) writer structure"]
impl crate::Writable for PIGEON_0_0 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_0_0;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_0_1](pigeon_0_1) module"]
pub type PIGEON_0_1 = crate::Reg<u32, _PIGEON_0_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_0_1;
#[doc = "`read()` method returns [pigeon_0_1::R](pigeon_0_1::R) reader structure"]
impl crate::Readable for PIGEON_0_1 {}
#[doc = "`write(|w| ..)` method takes [pigeon_0_1::W](pigeon_0_1::W) writer structure"]
impl crate::Writable for PIGEON_0_1 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_0_1;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_0_2](pigeon_0_2) module"]
pub type PIGEON_0_2 = crate::Reg<u32, _PIGEON_0_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_0_2;
#[doc = "`read()` method returns [pigeon_0_2::R](pigeon_0_2::R) reader structure"]
impl crate::Readable for PIGEON_0_2 {}
#[doc = "`write(|w| ..)` method takes [pigeon_0_2::W](pigeon_0_2::W) writer structure"]
impl crate::Writable for PIGEON_0_2 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_0_2;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_1_0](pigeon_1_0) module"]
pub type PIGEON_1_0 = crate::Reg<u32, _PIGEON_1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_1_0;
#[doc = "`read()` method returns [pigeon_1_0::R](pigeon_1_0::R) reader structure"]
impl crate::Readable for PIGEON_1_0 {}
#[doc = "`write(|w| ..)` method takes [pigeon_1_0::W](pigeon_1_0::W) writer structure"]
impl crate::Writable for PIGEON_1_0 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_1_0;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_1_1](pigeon_1_1) module"]
pub type PIGEON_1_1 = crate::Reg<u32, _PIGEON_1_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_1_1;
#[doc = "`read()` method returns [pigeon_1_1::R](pigeon_1_1::R) reader structure"]
impl crate::Readable for PIGEON_1_1 {}
#[doc = "`write(|w| ..)` method takes [pigeon_1_1::W](pigeon_1_1::W) writer structure"]
impl crate::Writable for PIGEON_1_1 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_1_1;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_1_2](pigeon_1_2) module"]
pub type PIGEON_1_2 = crate::Reg<u32, _PIGEON_1_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_1_2;
#[doc = "`read()` method returns [pigeon_1_2::R](pigeon_1_2::R) reader structure"]
impl crate::Readable for PIGEON_1_2 {}
#[doc = "`write(|w| ..)` method takes [pigeon_1_2::W](pigeon_1_2::W) writer structure"]
impl crate::Writable for PIGEON_1_2 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_1_2;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_2_0](pigeon_2_0) module"]
pub type PIGEON_2_0 = crate::Reg<u32, _PIGEON_2_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_2_0;
#[doc = "`read()` method returns [pigeon_2_0::R](pigeon_2_0::R) reader structure"]
impl crate::Readable for PIGEON_2_0 {}
#[doc = "`write(|w| ..)` method takes [pigeon_2_0::W](pigeon_2_0::W) writer structure"]
impl crate::Writable for PIGEON_2_0 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_2_0;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_2_1](pigeon_2_1) module"]
pub type PIGEON_2_1 = crate::Reg<u32, _PIGEON_2_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_2_1;
#[doc = "`read()` method returns [pigeon_2_1::R](pigeon_2_1::R) reader structure"]
impl crate::Readable for PIGEON_2_1 {}
#[doc = "`write(|w| ..)` method takes [pigeon_2_1::W](pigeon_2_1::W) writer structure"]
impl crate::Writable for PIGEON_2_1 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_2_1;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_2_2](pigeon_2_2) module"]
pub type PIGEON_2_2 = crate::Reg<u32, _PIGEON_2_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_2_2;
#[doc = "`read()` method returns [pigeon_2_2::R](pigeon_2_2::R) reader structure"]
impl crate::Readable for PIGEON_2_2 {}
#[doc = "`write(|w| ..)` method takes [pigeon_2_2::W](pigeon_2_2::W) writer structure"]
impl crate::Writable for PIGEON_2_2 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_2_2;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_3_0](pigeon_3_0) module"]
pub type PIGEON_3_0 = crate::Reg<u32, _PIGEON_3_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_3_0;
#[doc = "`read()` method returns [pigeon_3_0::R](pigeon_3_0::R) reader structure"]
impl crate::Readable for PIGEON_3_0 {}
#[doc = "`write(|w| ..)` method takes [pigeon_3_0::W](pigeon_3_0::W) writer structure"]
impl crate::Writable for PIGEON_3_0 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_3_0;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_3_1](pigeon_3_1) module"]
pub type PIGEON_3_1 = crate::Reg<u32, _PIGEON_3_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_3_1;
#[doc = "`read()` method returns [pigeon_3_1::R](pigeon_3_1::R) reader structure"]
impl crate::Readable for PIGEON_3_1 {}
#[doc = "`write(|w| ..)` method takes [pigeon_3_1::W](pigeon_3_1::W) writer structure"]
impl crate::Writable for PIGEON_3_1 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_3_1;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_3_2](pigeon_3_2) module"]
pub type PIGEON_3_2 = crate::Reg<u32, _PIGEON_3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_3_2;
#[doc = "`read()` method returns [pigeon_3_2::R](pigeon_3_2::R) reader structure"]
impl crate::Readable for PIGEON_3_2 {}
#[doc = "`write(|w| ..)` method takes [pigeon_3_2::W](pigeon_3_2::W) writer structure"]
impl crate::Writable for PIGEON_3_2 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_3_2;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_4_0](pigeon_4_0) module"]
pub type PIGEON_4_0 = crate::Reg<u32, _PIGEON_4_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_4_0;
#[doc = "`read()` method returns [pigeon_4_0::R](pigeon_4_0::R) reader structure"]
impl crate::Readable for PIGEON_4_0 {}
#[doc = "`write(|w| ..)` method takes [pigeon_4_0::W](pigeon_4_0::W) writer structure"]
impl crate::Writable for PIGEON_4_0 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_4_0;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_4_1](pigeon_4_1) module"]
pub type PIGEON_4_1 = crate::Reg<u32, _PIGEON_4_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_4_1;
#[doc = "`read()` method returns [pigeon_4_1::R](pigeon_4_1::R) reader structure"]
impl crate::Readable for PIGEON_4_1 {}
#[doc = "`write(|w| ..)` method takes [pigeon_4_1::W](pigeon_4_1::W) writer structure"]
impl crate::Writable for PIGEON_4_1 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_4_1;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_4_2](pigeon_4_2) module"]
pub type PIGEON_4_2 = crate::Reg<u32, _PIGEON_4_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_4_2;
#[doc = "`read()` method returns [pigeon_4_2::R](pigeon_4_2::R) reader structure"]
impl crate::Readable for PIGEON_4_2 {}
#[doc = "`write(|w| ..)` method takes [pigeon_4_2::W](pigeon_4_2::W) writer structure"]
impl crate::Writable for PIGEON_4_2 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_4_2;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_5_0](pigeon_5_0) module"]
pub type PIGEON_5_0 = crate::Reg<u32, _PIGEON_5_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_5_0;
#[doc = "`read()` method returns [pigeon_5_0::R](pigeon_5_0::R) reader structure"]
impl crate::Readable for PIGEON_5_0 {}
#[doc = "`write(|w| ..)` method takes [pigeon_5_0::W](pigeon_5_0::W) writer structure"]
impl crate::Writable for PIGEON_5_0 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_5_0;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_5_1](pigeon_5_1) module"]
pub type PIGEON_5_1 = crate::Reg<u32, _PIGEON_5_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_5_1;
#[doc = "`read()` method returns [pigeon_5_1::R](pigeon_5_1::R) reader structure"]
impl crate::Readable for PIGEON_5_1 {}
#[doc = "`write(|w| ..)` method takes [pigeon_5_1::W](pigeon_5_1::W) writer structure"]
impl crate::Writable for PIGEON_5_1 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_5_1;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_5_2](pigeon_5_2) module"]
pub type PIGEON_5_2 = crate::Reg<u32, _PIGEON_5_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_5_2;
#[doc = "`read()` method returns [pigeon_5_2::R](pigeon_5_2::R) reader structure"]
impl crate::Readable for PIGEON_5_2 {}
#[doc = "`write(|w| ..)` method takes [pigeon_5_2::W](pigeon_5_2::W) writer structure"]
impl crate::Writable for PIGEON_5_2 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_5_2;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_6_0](pigeon_6_0) module"]
pub type PIGEON_6_0 = crate::Reg<u32, _PIGEON_6_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_6_0;
#[doc = "`read()` method returns [pigeon_6_0::R](pigeon_6_0::R) reader structure"]
impl crate::Readable for PIGEON_6_0 {}
#[doc = "`write(|w| ..)` method takes [pigeon_6_0::W](pigeon_6_0::W) writer structure"]
impl crate::Writable for PIGEON_6_0 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_6_0;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_6_1](pigeon_6_1) module"]
pub type PIGEON_6_1 = crate::Reg<u32, _PIGEON_6_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_6_1;
#[doc = "`read()` method returns [pigeon_6_1::R](pigeon_6_1::R) reader structure"]
impl crate::Readable for PIGEON_6_1 {}
#[doc = "`write(|w| ..)` method takes [pigeon_6_1::W](pigeon_6_1::W) writer structure"]
impl crate::Writable for PIGEON_6_1 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_6_1;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_6_2](pigeon_6_2) module"]
pub type PIGEON_6_2 = crate::Reg<u32, _PIGEON_6_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_6_2;
#[doc = "`read()` method returns [pigeon_6_2::R](pigeon_6_2::R) reader structure"]
impl crate::Readable for PIGEON_6_2 {}
#[doc = "`write(|w| ..)` method takes [pigeon_6_2::W](pigeon_6_2::W) writer structure"]
impl crate::Writable for PIGEON_6_2 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_6_2;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_7_0](pigeon_7_0) module"]
pub type PIGEON_7_0 = crate::Reg<u32, _PIGEON_7_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_7_0;
#[doc = "`read()` method returns [pigeon_7_0::R](pigeon_7_0::R) reader structure"]
impl crate::Readable for PIGEON_7_0 {}
#[doc = "`write(|w| ..)` method takes [pigeon_7_0::W](pigeon_7_0::W) writer structure"]
impl crate::Writable for PIGEON_7_0 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_7_0;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_7_1](pigeon_7_1) module"]
pub type PIGEON_7_1 = crate::Reg<u32, _PIGEON_7_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_7_1;
#[doc = "`read()` method returns [pigeon_7_1::R](pigeon_7_1::R) reader structure"]
impl crate::Readable for PIGEON_7_1 {}
#[doc = "`write(|w| ..)` method takes [pigeon_7_1::W](pigeon_7_1::W) writer structure"]
impl crate::Writable for PIGEON_7_1 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_7_1;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_7_2](pigeon_7_2) module"]
pub type PIGEON_7_2 = crate::Reg<u32, _PIGEON_7_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_7_2;
#[doc = "`read()` method returns [pigeon_7_2::R](pigeon_7_2::R) reader structure"]
impl crate::Readable for PIGEON_7_2 {}
#[doc = "`write(|w| ..)` method takes [pigeon_7_2::W](pigeon_7_2::W) writer structure"]
impl crate::Writable for PIGEON_7_2 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_7_2;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_8_0](pigeon_8_0) module"]
pub type PIGEON_8_0 = crate::Reg<u32, _PIGEON_8_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_8_0;
#[doc = "`read()` method returns [pigeon_8_0::R](pigeon_8_0::R) reader structure"]
impl crate::Readable for PIGEON_8_0 {}
#[doc = "`write(|w| ..)` method takes [pigeon_8_0::W](pigeon_8_0::W) writer structure"]
impl crate::Writable for PIGEON_8_0 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_8_0;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_8_1](pigeon_8_1) module"]
pub type PIGEON_8_1 = crate::Reg<u32, _PIGEON_8_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_8_1;
#[doc = "`read()` method returns [pigeon_8_1::R](pigeon_8_1::R) reader structure"]
impl crate::Readable for PIGEON_8_1 {}
#[doc = "`write(|w| ..)` method takes [pigeon_8_1::W](pigeon_8_1::W) writer structure"]
impl crate::Writable for PIGEON_8_1 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_8_1;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_8_2](pigeon_8_2) module"]
pub type PIGEON_8_2 = crate::Reg<u32, _PIGEON_8_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_8_2;
#[doc = "`read()` method returns [pigeon_8_2::R](pigeon_8_2::R) reader structure"]
impl crate::Readable for PIGEON_8_2 {}
#[doc = "`write(|w| ..)` method takes [pigeon_8_2::W](pigeon_8_2::W) writer structure"]
impl crate::Writable for PIGEON_8_2 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_8_2;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_9_0](pigeon_9_0) module"]
pub type PIGEON_9_0 = crate::Reg<u32, _PIGEON_9_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_9_0;
#[doc = "`read()` method returns [pigeon_9_0::R](pigeon_9_0::R) reader structure"]
impl crate::Readable for PIGEON_9_0 {}
#[doc = "`write(|w| ..)` method takes [pigeon_9_0::W](pigeon_9_0::W) writer structure"]
impl crate::Writable for PIGEON_9_0 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_9_0;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_9_1](pigeon_9_1) module"]
pub type PIGEON_9_1 = crate::Reg<u32, _PIGEON_9_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_9_1;
#[doc = "`read()` method returns [pigeon_9_1::R](pigeon_9_1::R) reader structure"]
impl crate::Readable for PIGEON_9_1 {}
#[doc = "`write(|w| ..)` method takes [pigeon_9_1::W](pigeon_9_1::W) writer structure"]
impl crate::Writable for PIGEON_9_1 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_9_1;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_9_2](pigeon_9_2) module"]
pub type PIGEON_9_2 = crate::Reg<u32, _PIGEON_9_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_9_2;
#[doc = "`read()` method returns [pigeon_9_2::R](pigeon_9_2::R) reader structure"]
impl crate::Readable for PIGEON_9_2 {}
#[doc = "`write(|w| ..)` method takes [pigeon_9_2::W](pigeon_9_2::W) writer structure"]
impl crate::Writable for PIGEON_9_2 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_9_2;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_10_0](pigeon_10_0) module"]
pub type PIGEON_10_0 = crate::Reg<u32, _PIGEON_10_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_10_0;
#[doc = "`read()` method returns [pigeon_10_0::R](pigeon_10_0::R) reader structure"]
impl crate::Readable for PIGEON_10_0 {}
#[doc = "`write(|w| ..)` method takes [pigeon_10_0::W](pigeon_10_0::W) writer structure"]
impl crate::Writable for PIGEON_10_0 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_10_0;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_10_1](pigeon_10_1) module"]
pub type PIGEON_10_1 = crate::Reg<u32, _PIGEON_10_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_10_1;
#[doc = "`read()` method returns [pigeon_10_1::R](pigeon_10_1::R) reader structure"]
impl crate::Readable for PIGEON_10_1 {}
#[doc = "`write(|w| ..)` method takes [pigeon_10_1::W](pigeon_10_1::W) writer structure"]
impl crate::Writable for PIGEON_10_1 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_10_1;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_10_2](pigeon_10_2) module"]
pub type PIGEON_10_2 = crate::Reg<u32, _PIGEON_10_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_10_2;
#[doc = "`read()` method returns [pigeon_10_2::R](pigeon_10_2::R) reader structure"]
impl crate::Readable for PIGEON_10_2 {}
#[doc = "`write(|w| ..)` method takes [pigeon_10_2::W](pigeon_10_2::W) writer structure"]
impl crate::Writable for PIGEON_10_2 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_10_2;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_11_0](pigeon_11_0) module"]
pub type PIGEON_11_0 = crate::Reg<u32, _PIGEON_11_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_11_0;
#[doc = "`read()` method returns [pigeon_11_0::R](pigeon_11_0::R) reader structure"]
impl crate::Readable for PIGEON_11_0 {}
#[doc = "`write(|w| ..)` method takes [pigeon_11_0::W](pigeon_11_0::W) writer structure"]
impl crate::Writable for PIGEON_11_0 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_11_0;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_11_1](pigeon_11_1) module"]
pub type PIGEON_11_1 = crate::Reg<u32, _PIGEON_11_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_11_1;
#[doc = "`read()` method returns [pigeon_11_1::R](pigeon_11_1::R) reader structure"]
impl crate::Readable for PIGEON_11_1 {}
#[doc = "`write(|w| ..)` method takes [pigeon_11_1::W](pigeon_11_1::W) writer structure"]
impl crate::Writable for PIGEON_11_1 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_11_1;
#[doc = "Panel Interface Signal Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pigeon_11_2](pigeon_11_2) module"]
pub type PIGEON_11_2 = crate::Reg<u32, _PIGEON_11_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIGEON_11_2;
#[doc = "`read()` method returns [pigeon_11_2::R](pigeon_11_2::R) reader structure"]
impl crate::Readable for PIGEON_11_2 {}
#[doc = "`write(|w| ..)` method takes [pigeon_11_2::W](pigeon_11_2::W) writer structure"]
impl crate::Writable for PIGEON_11_2 {}
#[doc = "Panel Interface Signal Generator Register"]
pub mod pigeon_11_2;
#[doc = "Lookup Table Data Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lut_ctrl](lut_ctrl) module"]
pub type LUT_CTRL = crate::Reg<u32, _LUT_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LUT_CTRL;
#[doc = "`read()` method returns [lut_ctrl::R](lut_ctrl::R) reader structure"]
impl crate::Readable for LUT_CTRL {}
#[doc = "`write(|w| ..)` method takes [lut_ctrl::W](lut_ctrl::W) writer structure"]
impl crate::Writable for LUT_CTRL {}
#[doc = "Lookup Table Data Register."]
pub mod lut_ctrl;
#[doc = "Lookup Table Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lut0_addr](lut0_addr) module"]
pub type LUT0_ADDR = crate::Reg<u32, _LUT0_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LUT0_ADDR;
#[doc = "`read()` method returns [lut0_addr::R](lut0_addr::R) reader structure"]
impl crate::Readable for LUT0_ADDR {}
#[doc = "`write(|w| ..)` method takes [lut0_addr::W](lut0_addr::W) writer structure"]
impl crate::Writable for LUT0_ADDR {}
#[doc = "Lookup Table Control Register."]
pub mod lut0_addr;
#[doc = "Lookup Table Data Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lut0_data](lut0_data) module"]
pub type LUT0_DATA = crate::Reg<u32, _LUT0_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LUT0_DATA;
#[doc = "`read()` method returns [lut0_data::R](lut0_data::R) reader structure"]
impl crate::Readable for LUT0_DATA {}
#[doc = "`write(|w| ..)` method takes [lut0_data::W](lut0_data::W) writer structure"]
impl crate::Writable for LUT0_DATA {}
#[doc = "Lookup Table Data Register."]
pub mod lut0_data;
#[doc = "Lookup Table Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lut1_addr](lut1_addr) module"]
pub type LUT1_ADDR = crate::Reg<u32, _LUT1_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LUT1_ADDR;
#[doc = "`read()` method returns [lut1_addr::R](lut1_addr::R) reader structure"]
impl crate::Readable for LUT1_ADDR {}
#[doc = "`write(|w| ..)` method takes [lut1_addr::W](lut1_addr::W) writer structure"]
impl crate::Writable for LUT1_ADDR {}
#[doc = "Lookup Table Control Register."]
pub mod lut1_addr;
#[doc = "Lookup Table Data Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lut1_data](lut1_data) module"]
pub type LUT1_DATA = crate::Reg<u32, _LUT1_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LUT1_DATA;
#[doc = "`read()` method returns [lut1_data::R](lut1_data::R) reader structure"]
impl crate::Readable for LUT1_DATA {}
#[doc = "`write(|w| ..)` method takes [lut1_data::W](lut1_data::W) writer structure"]
impl crate::Writable for LUT1_DATA {}
#[doc = "Lookup Table Data Register."]
pub mod lut1_data;
