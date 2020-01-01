#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register 0"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Control Register 0"]
    pub ctrl_set: CTRL_SET,
    #[doc = "0x08 - Control Register 0"]
    pub ctrl_clr: CTRL_CLR,
    #[doc = "0x0c - Control Register 0"]
    pub ctrl_tog: CTRL_TOG,
    #[doc = "0x10 - Status Register"]
    pub stat: STAT,
    #[doc = "0x14 - Status Register"]
    pub stat_set: STAT_SET,
    #[doc = "0x18 - Status Register"]
    pub stat_clr: STAT_CLR,
    #[doc = "0x1c - Status Register"]
    pub stat_tog: STAT_TOG,
    #[doc = "0x20 - Output Buffer Control Register"]
    pub out_ctrl: OUT_CTRL,
    #[doc = "0x24 - Output Buffer Control Register"]
    pub out_ctrl_set: OUT_CTRL_SET,
    #[doc = "0x28 - Output Buffer Control Register"]
    pub out_ctrl_clr: OUT_CTRL_CLR,
    #[doc = "0x2c - Output Buffer Control Register"]
    pub out_ctrl_tog: OUT_CTRL_TOG,
    #[doc = "0x30 - Output Frame Buffer Pointer"]
    pub out_buf: OUT_BUF,
    _reserved13: [u8; 12usize],
    #[doc = "0x40 - Output Frame Buffer Pointer #2"]
    pub out_buf2: OUT_BUF2,
    _reserved14: [u8; 12usize],
    #[doc = "0x50 - Output Buffer Pitch"]
    pub out_pitch: OUT_PITCH,
    _reserved15: [u8; 12usize],
    #[doc = "0x60 - Output Surface Lower Right Coordinate"]
    pub out_lrc: OUT_LRC,
    _reserved16: [u8; 12usize],
    #[doc = "0x70 - Processed Surface Upper Left Coordinate"]
    pub out_ps_ulc: OUT_PS_ULC,
    _reserved17: [u8; 12usize],
    #[doc = "0x80 - Processed Surface Lower Right Coordinate"]
    pub out_ps_lrc: OUT_PS_LRC,
    _reserved18: [u8; 12usize],
    #[doc = "0x90 - Alpha Surface Upper Left Coordinate"]
    pub out_as_ulc: OUT_AS_ULC,
    _reserved19: [u8; 12usize],
    #[doc = "0xa0 - Alpha Surface Lower Right Coordinate"]
    pub out_as_lrc: OUT_AS_LRC,
    _reserved20: [u8; 12usize],
    #[doc = "0xb0 - Processed Surface (PS) Control Register"]
    pub ps_ctrl: PS_CTRL,
    #[doc = "0xb4 - Processed Surface (PS) Control Register"]
    pub ps_ctrl_set: PS_CTRL_SET,
    #[doc = "0xb8 - Processed Surface (PS) Control Register"]
    pub ps_ctrl_clr: PS_CTRL_CLR,
    #[doc = "0xbc - Processed Surface (PS) Control Register"]
    pub ps_ctrl_tog: PS_CTRL_TOG,
    #[doc = "0xc0 - PS Input Buffer Address"]
    pub ps_buf: PS_BUF,
    _reserved25: [u8; 12usize],
    #[doc = "0xd0 - PS U/Cb or 2 Plane UV Input Buffer Address"]
    pub ps_ubuf: PS_UBUF,
    _reserved26: [u8; 12usize],
    #[doc = "0xe0 - PS V/Cr Input Buffer Address"]
    pub ps_vbuf: PS_VBUF,
    _reserved27: [u8; 12usize],
    #[doc = "0xf0 - Processed Surface Pitch"]
    pub ps_pitch: PS_PITCH,
    _reserved28: [u8; 12usize],
    #[doc = "0x100 - PS Background Color"]
    pub ps_background: PS_BACKGROUND,
    _reserved29: [u8; 12usize],
    #[doc = "0x110 - PS Scale Factor Register"]
    pub ps_scale: PS_SCALE,
    _reserved30: [u8; 12usize],
    #[doc = "0x120 - PS Scale Offset Register"]
    pub ps_offset: PS_OFFSET,
    _reserved31: [u8; 12usize],
    #[doc = "0x130 - PS Color Key Low"]
    pub ps_clrkeylow: PS_CLRKEYLOW,
    _reserved32: [u8; 12usize],
    #[doc = "0x140 - PS Color Key High"]
    pub ps_clrkeyhigh: PS_CLRKEYHIGH,
    _reserved33: [u8; 12usize],
    #[doc = "0x150 - Alpha Surface Control"]
    pub as_ctrl: AS_CTRL,
    _reserved34: [u8; 12usize],
    #[doc = "0x160 - Alpha Surface Buffer Pointer"]
    pub as_buf: AS_BUF,
    _reserved35: [u8; 12usize],
    #[doc = "0x170 - Alpha Surface Pitch"]
    pub as_pitch: AS_PITCH,
    _reserved36: [u8; 12usize],
    #[doc = "0x180 - Overlay Color Key Low"]
    pub as_clrkeylow: AS_CLRKEYLOW,
    _reserved37: [u8; 12usize],
    #[doc = "0x190 - Overlay Color Key High"]
    pub as_clrkeyhigh: AS_CLRKEYHIGH,
    _reserved38: [u8; 12usize],
    #[doc = "0x1a0 - Color Space Conversion Coefficient Register 0"]
    pub csc1_coef0: CSC1_COEF0,
    _reserved39: [u8; 12usize],
    #[doc = "0x1b0 - Color Space Conversion Coefficient Register 1"]
    pub csc1_coef1: CSC1_COEF1,
    _reserved40: [u8; 12usize],
    #[doc = "0x1c0 - Color Space Conversion Coefficient Register 2"]
    pub csc1_coef2: CSC1_COEF2,
    _reserved41: [u8; 348usize],
    #[doc = "0x320 - PXP Power Control Register"]
    pub power: POWER,
    _reserved42: [u8; 220usize],
    #[doc = "0x400 - Next Frame Pointer"]
    pub next: NEXT,
    _reserved43: [u8; 60usize],
    #[doc = "0x440 - PXP Alpha Engine A Control Register."]
    pub porter_duff_ctrl: PORTER_DUFF_CTRL,
}
#[doc = "Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control Register 0"]
pub mod ctrl;
#[doc = "Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_set](ctrl_set) module"]
pub type CTRL_SET = crate::Reg<u32, _CTRL_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_SET;
#[doc = "`read()` method returns [ctrl_set::R](ctrl_set::R) reader structure"]
impl crate::Readable for CTRL_SET {}
#[doc = "`write(|w| ..)` method takes [ctrl_set::W](ctrl_set::W) writer structure"]
impl crate::Writable for CTRL_SET {}
#[doc = "Control Register 0"]
pub mod ctrl_set;
#[doc = "Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_clr](ctrl_clr) module"]
pub type CTRL_CLR = crate::Reg<u32, _CTRL_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_CLR;
#[doc = "`read()` method returns [ctrl_clr::R](ctrl_clr::R) reader structure"]
impl crate::Readable for CTRL_CLR {}
#[doc = "`write(|w| ..)` method takes [ctrl_clr::W](ctrl_clr::W) writer structure"]
impl crate::Writable for CTRL_CLR {}
#[doc = "Control Register 0"]
pub mod ctrl_clr;
#[doc = "Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_tog](ctrl_tog) module"]
pub type CTRL_TOG = crate::Reg<u32, _CTRL_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_TOG;
#[doc = "`read()` method returns [ctrl_tog::R](ctrl_tog::R) reader structure"]
impl crate::Readable for CTRL_TOG {}
#[doc = "`write(|w| ..)` method takes [ctrl_tog::W](ctrl_tog::W) writer structure"]
impl crate::Writable for CTRL_TOG {}
#[doc = "Control Register 0"]
pub mod ctrl_tog;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "Status Register"]
pub mod stat;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat_set](stat_set) module"]
pub type STAT_SET = crate::Reg<u32, _STAT_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT_SET;
#[doc = "`read()` method returns [stat_set::R](stat_set::R) reader structure"]
impl crate::Readable for STAT_SET {}
#[doc = "`write(|w| ..)` method takes [stat_set::W](stat_set::W) writer structure"]
impl crate::Writable for STAT_SET {}
#[doc = "Status Register"]
pub mod stat_set;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat_clr](stat_clr) module"]
pub type STAT_CLR = crate::Reg<u32, _STAT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT_CLR;
#[doc = "`read()` method returns [stat_clr::R](stat_clr::R) reader structure"]
impl crate::Readable for STAT_CLR {}
#[doc = "`write(|w| ..)` method takes [stat_clr::W](stat_clr::W) writer structure"]
impl crate::Writable for STAT_CLR {}
#[doc = "Status Register"]
pub mod stat_clr;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat_tog](stat_tog) module"]
pub type STAT_TOG = crate::Reg<u32, _STAT_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT_TOG;
#[doc = "`read()` method returns [stat_tog::R](stat_tog::R) reader structure"]
impl crate::Readable for STAT_TOG {}
#[doc = "`write(|w| ..)` method takes [stat_tog::W](stat_tog::W) writer structure"]
impl crate::Writable for STAT_TOG {}
#[doc = "Status Register"]
pub mod stat_tog;
#[doc = "Output Buffer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_ctrl](out_ctrl) module"]
pub type OUT_CTRL = crate::Reg<u32, _OUT_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_CTRL;
#[doc = "`read()` method returns [out_ctrl::R](out_ctrl::R) reader structure"]
impl crate::Readable for OUT_CTRL {}
#[doc = "`write(|w| ..)` method takes [out_ctrl::W](out_ctrl::W) writer structure"]
impl crate::Writable for OUT_CTRL {}
#[doc = "Output Buffer Control Register"]
pub mod out_ctrl;
#[doc = "Output Buffer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_ctrl_set](out_ctrl_set) module"]
pub type OUT_CTRL_SET = crate::Reg<u32, _OUT_CTRL_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_CTRL_SET;
#[doc = "`read()` method returns [out_ctrl_set::R](out_ctrl_set::R) reader structure"]
impl crate::Readable for OUT_CTRL_SET {}
#[doc = "`write(|w| ..)` method takes [out_ctrl_set::W](out_ctrl_set::W) writer structure"]
impl crate::Writable for OUT_CTRL_SET {}
#[doc = "Output Buffer Control Register"]
pub mod out_ctrl_set;
#[doc = "Output Buffer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_ctrl_clr](out_ctrl_clr) module"]
pub type OUT_CTRL_CLR = crate::Reg<u32, _OUT_CTRL_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_CTRL_CLR;
#[doc = "`read()` method returns [out_ctrl_clr::R](out_ctrl_clr::R) reader structure"]
impl crate::Readable for OUT_CTRL_CLR {}
#[doc = "`write(|w| ..)` method takes [out_ctrl_clr::W](out_ctrl_clr::W) writer structure"]
impl crate::Writable for OUT_CTRL_CLR {}
#[doc = "Output Buffer Control Register"]
pub mod out_ctrl_clr;
#[doc = "Output Buffer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_ctrl_tog](out_ctrl_tog) module"]
pub type OUT_CTRL_TOG = crate::Reg<u32, _OUT_CTRL_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_CTRL_TOG;
#[doc = "`read()` method returns [out_ctrl_tog::R](out_ctrl_tog::R) reader structure"]
impl crate::Readable for OUT_CTRL_TOG {}
#[doc = "`write(|w| ..)` method takes [out_ctrl_tog::W](out_ctrl_tog::W) writer structure"]
impl crate::Writable for OUT_CTRL_TOG {}
#[doc = "Output Buffer Control Register"]
pub mod out_ctrl_tog;
#[doc = "Output Frame Buffer Pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_buf](out_buf) module"]
pub type OUT_BUF = crate::Reg<u32, _OUT_BUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_BUF;
#[doc = "`read()` method returns [out_buf::R](out_buf::R) reader structure"]
impl crate::Readable for OUT_BUF {}
#[doc = "`write(|w| ..)` method takes [out_buf::W](out_buf::W) writer structure"]
impl crate::Writable for OUT_BUF {}
#[doc = "Output Frame Buffer Pointer"]
pub mod out_buf;
#[doc = "Output Frame Buffer Pointer #2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_buf2](out_buf2) module"]
pub type OUT_BUF2 = crate::Reg<u32, _OUT_BUF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_BUF2;
#[doc = "`read()` method returns [out_buf2::R](out_buf2::R) reader structure"]
impl crate::Readable for OUT_BUF2 {}
#[doc = "`write(|w| ..)` method takes [out_buf2::W](out_buf2::W) writer structure"]
impl crate::Writable for OUT_BUF2 {}
#[doc = "Output Frame Buffer Pointer #2"]
pub mod out_buf2;
#[doc = "Output Buffer Pitch\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_pitch](out_pitch) module"]
pub type OUT_PITCH = crate::Reg<u32, _OUT_PITCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_PITCH;
#[doc = "`read()` method returns [out_pitch::R](out_pitch::R) reader structure"]
impl crate::Readable for OUT_PITCH {}
#[doc = "`write(|w| ..)` method takes [out_pitch::W](out_pitch::W) writer structure"]
impl crate::Writable for OUT_PITCH {}
#[doc = "Output Buffer Pitch"]
pub mod out_pitch;
#[doc = "Output Surface Lower Right Coordinate\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_lrc](out_lrc) module"]
pub type OUT_LRC = crate::Reg<u32, _OUT_LRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_LRC;
#[doc = "`read()` method returns [out_lrc::R](out_lrc::R) reader structure"]
impl crate::Readable for OUT_LRC {}
#[doc = "`write(|w| ..)` method takes [out_lrc::W](out_lrc::W) writer structure"]
impl crate::Writable for OUT_LRC {}
#[doc = "Output Surface Lower Right Coordinate"]
pub mod out_lrc;
#[doc = "Processed Surface Upper Left Coordinate\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_ps_ulc](out_ps_ulc) module"]
pub type OUT_PS_ULC = crate::Reg<u32, _OUT_PS_ULC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_PS_ULC;
#[doc = "`read()` method returns [out_ps_ulc::R](out_ps_ulc::R) reader structure"]
impl crate::Readable for OUT_PS_ULC {}
#[doc = "`write(|w| ..)` method takes [out_ps_ulc::W](out_ps_ulc::W) writer structure"]
impl crate::Writable for OUT_PS_ULC {}
#[doc = "Processed Surface Upper Left Coordinate"]
pub mod out_ps_ulc;
#[doc = "Processed Surface Lower Right Coordinate\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_ps_lrc](out_ps_lrc) module"]
pub type OUT_PS_LRC = crate::Reg<u32, _OUT_PS_LRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_PS_LRC;
#[doc = "`read()` method returns [out_ps_lrc::R](out_ps_lrc::R) reader structure"]
impl crate::Readable for OUT_PS_LRC {}
#[doc = "`write(|w| ..)` method takes [out_ps_lrc::W](out_ps_lrc::W) writer structure"]
impl crate::Writable for OUT_PS_LRC {}
#[doc = "Processed Surface Lower Right Coordinate"]
pub mod out_ps_lrc;
#[doc = "Alpha Surface Upper Left Coordinate\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_as_ulc](out_as_ulc) module"]
pub type OUT_AS_ULC = crate::Reg<u32, _OUT_AS_ULC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_AS_ULC;
#[doc = "`read()` method returns [out_as_ulc::R](out_as_ulc::R) reader structure"]
impl crate::Readable for OUT_AS_ULC {}
#[doc = "`write(|w| ..)` method takes [out_as_ulc::W](out_as_ulc::W) writer structure"]
impl crate::Writable for OUT_AS_ULC {}
#[doc = "Alpha Surface Upper Left Coordinate"]
pub mod out_as_ulc;
#[doc = "Alpha Surface Lower Right Coordinate\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_as_lrc](out_as_lrc) module"]
pub type OUT_AS_LRC = crate::Reg<u32, _OUT_AS_LRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_AS_LRC;
#[doc = "`read()` method returns [out_as_lrc::R](out_as_lrc::R) reader structure"]
impl crate::Readable for OUT_AS_LRC {}
#[doc = "`write(|w| ..)` method takes [out_as_lrc::W](out_as_lrc::W) writer structure"]
impl crate::Writable for OUT_AS_LRC {}
#[doc = "Alpha Surface Lower Right Coordinate"]
pub mod out_as_lrc;
#[doc = "Processed Surface (PS) Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps_ctrl](ps_ctrl) module"]
pub type PS_CTRL = crate::Reg<u32, _PS_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PS_CTRL;
#[doc = "`read()` method returns [ps_ctrl::R](ps_ctrl::R) reader structure"]
impl crate::Readable for PS_CTRL {}
#[doc = "`write(|w| ..)` method takes [ps_ctrl::W](ps_ctrl::W) writer structure"]
impl crate::Writable for PS_CTRL {}
#[doc = "Processed Surface (PS) Control Register"]
pub mod ps_ctrl;
#[doc = "Processed Surface (PS) Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps_ctrl_set](ps_ctrl_set) module"]
pub type PS_CTRL_SET = crate::Reg<u32, _PS_CTRL_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PS_CTRL_SET;
#[doc = "`read()` method returns [ps_ctrl_set::R](ps_ctrl_set::R) reader structure"]
impl crate::Readable for PS_CTRL_SET {}
#[doc = "`write(|w| ..)` method takes [ps_ctrl_set::W](ps_ctrl_set::W) writer structure"]
impl crate::Writable for PS_CTRL_SET {}
#[doc = "Processed Surface (PS) Control Register"]
pub mod ps_ctrl_set;
#[doc = "Processed Surface (PS) Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps_ctrl_clr](ps_ctrl_clr) module"]
pub type PS_CTRL_CLR = crate::Reg<u32, _PS_CTRL_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PS_CTRL_CLR;
#[doc = "`read()` method returns [ps_ctrl_clr::R](ps_ctrl_clr::R) reader structure"]
impl crate::Readable for PS_CTRL_CLR {}
#[doc = "`write(|w| ..)` method takes [ps_ctrl_clr::W](ps_ctrl_clr::W) writer structure"]
impl crate::Writable for PS_CTRL_CLR {}
#[doc = "Processed Surface (PS) Control Register"]
pub mod ps_ctrl_clr;
#[doc = "Processed Surface (PS) Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps_ctrl_tog](ps_ctrl_tog) module"]
pub type PS_CTRL_TOG = crate::Reg<u32, _PS_CTRL_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PS_CTRL_TOG;
#[doc = "`read()` method returns [ps_ctrl_tog::R](ps_ctrl_tog::R) reader structure"]
impl crate::Readable for PS_CTRL_TOG {}
#[doc = "`write(|w| ..)` method takes [ps_ctrl_tog::W](ps_ctrl_tog::W) writer structure"]
impl crate::Writable for PS_CTRL_TOG {}
#[doc = "Processed Surface (PS) Control Register"]
pub mod ps_ctrl_tog;
#[doc = "PS Input Buffer Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps_buf](ps_buf) module"]
pub type PS_BUF = crate::Reg<u32, _PS_BUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PS_BUF;
#[doc = "`read()` method returns [ps_buf::R](ps_buf::R) reader structure"]
impl crate::Readable for PS_BUF {}
#[doc = "`write(|w| ..)` method takes [ps_buf::W](ps_buf::W) writer structure"]
impl crate::Writable for PS_BUF {}
#[doc = "PS Input Buffer Address"]
pub mod ps_buf;
#[doc = "PS U/Cb or 2 Plane UV Input Buffer Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps_ubuf](ps_ubuf) module"]
pub type PS_UBUF = crate::Reg<u32, _PS_UBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PS_UBUF;
#[doc = "`read()` method returns [ps_ubuf::R](ps_ubuf::R) reader structure"]
impl crate::Readable for PS_UBUF {}
#[doc = "`write(|w| ..)` method takes [ps_ubuf::W](ps_ubuf::W) writer structure"]
impl crate::Writable for PS_UBUF {}
#[doc = "PS U/Cb or 2 Plane UV Input Buffer Address"]
pub mod ps_ubuf;
#[doc = "PS V/Cr Input Buffer Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps_vbuf](ps_vbuf) module"]
pub type PS_VBUF = crate::Reg<u32, _PS_VBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PS_VBUF;
#[doc = "`read()` method returns [ps_vbuf::R](ps_vbuf::R) reader structure"]
impl crate::Readable for PS_VBUF {}
#[doc = "`write(|w| ..)` method takes [ps_vbuf::W](ps_vbuf::W) writer structure"]
impl crate::Writable for PS_VBUF {}
#[doc = "PS V/Cr Input Buffer Address"]
pub mod ps_vbuf;
#[doc = "Processed Surface Pitch\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps_pitch](ps_pitch) module"]
pub type PS_PITCH = crate::Reg<u32, _PS_PITCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PS_PITCH;
#[doc = "`read()` method returns [ps_pitch::R](ps_pitch::R) reader structure"]
impl crate::Readable for PS_PITCH {}
#[doc = "`write(|w| ..)` method takes [ps_pitch::W](ps_pitch::W) writer structure"]
impl crate::Writable for PS_PITCH {}
#[doc = "Processed Surface Pitch"]
pub mod ps_pitch;
#[doc = "PS Background Color\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps_background](ps_background) module"]
pub type PS_BACKGROUND = crate::Reg<u32, _PS_BACKGROUND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PS_BACKGROUND;
#[doc = "`read()` method returns [ps_background::R](ps_background::R) reader structure"]
impl crate::Readable for PS_BACKGROUND {}
#[doc = "`write(|w| ..)` method takes [ps_background::W](ps_background::W) writer structure"]
impl crate::Writable for PS_BACKGROUND {}
#[doc = "PS Background Color"]
pub mod ps_background;
#[doc = "PS Scale Factor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps_scale](ps_scale) module"]
pub type PS_SCALE = crate::Reg<u32, _PS_SCALE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PS_SCALE;
#[doc = "`read()` method returns [ps_scale::R](ps_scale::R) reader structure"]
impl crate::Readable for PS_SCALE {}
#[doc = "`write(|w| ..)` method takes [ps_scale::W](ps_scale::W) writer structure"]
impl crate::Writable for PS_SCALE {}
#[doc = "PS Scale Factor Register"]
pub mod ps_scale;
#[doc = "PS Scale Offset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps_offset](ps_offset) module"]
pub type PS_OFFSET = crate::Reg<u32, _PS_OFFSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PS_OFFSET;
#[doc = "`read()` method returns [ps_offset::R](ps_offset::R) reader structure"]
impl crate::Readable for PS_OFFSET {}
#[doc = "`write(|w| ..)` method takes [ps_offset::W](ps_offset::W) writer structure"]
impl crate::Writable for PS_OFFSET {}
#[doc = "PS Scale Offset Register"]
pub mod ps_offset;
#[doc = "PS Color Key Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps_clrkeylow](ps_clrkeylow) module"]
pub type PS_CLRKEYLOW = crate::Reg<u32, _PS_CLRKEYLOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PS_CLRKEYLOW;
#[doc = "`read()` method returns [ps_clrkeylow::R](ps_clrkeylow::R) reader structure"]
impl crate::Readable for PS_CLRKEYLOW {}
#[doc = "`write(|w| ..)` method takes [ps_clrkeylow::W](ps_clrkeylow::W) writer structure"]
impl crate::Writable for PS_CLRKEYLOW {}
#[doc = "PS Color Key Low"]
pub mod ps_clrkeylow;
#[doc = "PS Color Key High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps_clrkeyhigh](ps_clrkeyhigh) module"]
pub type PS_CLRKEYHIGH = crate::Reg<u32, _PS_CLRKEYHIGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PS_CLRKEYHIGH;
#[doc = "`read()` method returns [ps_clrkeyhigh::R](ps_clrkeyhigh::R) reader structure"]
impl crate::Readable for PS_CLRKEYHIGH {}
#[doc = "`write(|w| ..)` method takes [ps_clrkeyhigh::W](ps_clrkeyhigh::W) writer structure"]
impl crate::Writable for PS_CLRKEYHIGH {}
#[doc = "PS Color Key High"]
pub mod ps_clrkeyhigh;
#[doc = "Alpha Surface Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [as_ctrl](as_ctrl) module"]
pub type AS_CTRL = crate::Reg<u32, _AS_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AS_CTRL;
#[doc = "`read()` method returns [as_ctrl::R](as_ctrl::R) reader structure"]
impl crate::Readable for AS_CTRL {}
#[doc = "`write(|w| ..)` method takes [as_ctrl::W](as_ctrl::W) writer structure"]
impl crate::Writable for AS_CTRL {}
#[doc = "Alpha Surface Control"]
pub mod as_ctrl;
#[doc = "Alpha Surface Buffer Pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [as_buf](as_buf) module"]
pub type AS_BUF = crate::Reg<u32, _AS_BUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AS_BUF;
#[doc = "`read()` method returns [as_buf::R](as_buf::R) reader structure"]
impl crate::Readable for AS_BUF {}
#[doc = "`write(|w| ..)` method takes [as_buf::W](as_buf::W) writer structure"]
impl crate::Writable for AS_BUF {}
#[doc = "Alpha Surface Buffer Pointer"]
pub mod as_buf;
#[doc = "Alpha Surface Pitch\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [as_pitch](as_pitch) module"]
pub type AS_PITCH = crate::Reg<u32, _AS_PITCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AS_PITCH;
#[doc = "`read()` method returns [as_pitch::R](as_pitch::R) reader structure"]
impl crate::Readable for AS_PITCH {}
#[doc = "`write(|w| ..)` method takes [as_pitch::W](as_pitch::W) writer structure"]
impl crate::Writable for AS_PITCH {}
#[doc = "Alpha Surface Pitch"]
pub mod as_pitch;
#[doc = "Overlay Color Key Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [as_clrkeylow](as_clrkeylow) module"]
pub type AS_CLRKEYLOW = crate::Reg<u32, _AS_CLRKEYLOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AS_CLRKEYLOW;
#[doc = "`read()` method returns [as_clrkeylow::R](as_clrkeylow::R) reader structure"]
impl crate::Readable for AS_CLRKEYLOW {}
#[doc = "`write(|w| ..)` method takes [as_clrkeylow::W](as_clrkeylow::W) writer structure"]
impl crate::Writable for AS_CLRKEYLOW {}
#[doc = "Overlay Color Key Low"]
pub mod as_clrkeylow;
#[doc = "Overlay Color Key High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [as_clrkeyhigh](as_clrkeyhigh) module"]
pub type AS_CLRKEYHIGH = crate::Reg<u32, _AS_CLRKEYHIGH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AS_CLRKEYHIGH;
#[doc = "`read()` method returns [as_clrkeyhigh::R](as_clrkeyhigh::R) reader structure"]
impl crate::Readable for AS_CLRKEYHIGH {}
#[doc = "`write(|w| ..)` method takes [as_clrkeyhigh::W](as_clrkeyhigh::W) writer structure"]
impl crate::Writable for AS_CLRKEYHIGH {}
#[doc = "Overlay Color Key High"]
pub mod as_clrkeyhigh;
#[doc = "Color Space Conversion Coefficient Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csc1_coef0](csc1_coef0) module"]
pub type CSC1_COEF0 = crate::Reg<u32, _CSC1_COEF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSC1_COEF0;
#[doc = "`read()` method returns [csc1_coef0::R](csc1_coef0::R) reader structure"]
impl crate::Readable for CSC1_COEF0 {}
#[doc = "`write(|w| ..)` method takes [csc1_coef0::W](csc1_coef0::W) writer structure"]
impl crate::Writable for CSC1_COEF0 {}
#[doc = "Color Space Conversion Coefficient Register 0"]
pub mod csc1_coef0;
#[doc = "Color Space Conversion Coefficient Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csc1_coef1](csc1_coef1) module"]
pub type CSC1_COEF1 = crate::Reg<u32, _CSC1_COEF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSC1_COEF1;
#[doc = "`read()` method returns [csc1_coef1::R](csc1_coef1::R) reader structure"]
impl crate::Readable for CSC1_COEF1 {}
#[doc = "`write(|w| ..)` method takes [csc1_coef1::W](csc1_coef1::W) writer structure"]
impl crate::Writable for CSC1_COEF1 {}
#[doc = "Color Space Conversion Coefficient Register 1"]
pub mod csc1_coef1;
#[doc = "Color Space Conversion Coefficient Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csc1_coef2](csc1_coef2) module"]
pub type CSC1_COEF2 = crate::Reg<u32, _CSC1_COEF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSC1_COEF2;
#[doc = "`read()` method returns [csc1_coef2::R](csc1_coef2::R) reader structure"]
impl crate::Readable for CSC1_COEF2 {}
#[doc = "`write(|w| ..)` method takes [csc1_coef2::W](csc1_coef2::W) writer structure"]
impl crate::Writable for CSC1_COEF2 {}
#[doc = "Color Space Conversion Coefficient Register 2"]
pub mod csc1_coef2;
#[doc = "PXP Power Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power](power) module"]
pub type POWER = crate::Reg<u32, _POWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POWER;
#[doc = "`read()` method returns [power::R](power::R) reader structure"]
impl crate::Readable for POWER {}
#[doc = "`write(|w| ..)` method takes [power::W](power::W) writer structure"]
impl crate::Writable for POWER {}
#[doc = "PXP Power Control Register"]
pub mod power;
#[doc = "Next Frame Pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [next](next) module"]
pub type NEXT = crate::Reg<u32, _NEXT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NEXT;
#[doc = "`read()` method returns [next::R](next::R) reader structure"]
impl crate::Readable for NEXT {}
#[doc = "`write(|w| ..)` method takes [next::W](next::W) writer structure"]
impl crate::Writable for NEXT {}
#[doc = "Next Frame Pointer"]
pub mod next;
#[doc = "PXP Alpha Engine A Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [porter_duff_ctrl](porter_duff_ctrl) module"]
pub type PORTER_DUFF_CTRL = crate::Reg<u32, _PORTER_DUFF_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PORTER_DUFF_CTRL;
#[doc = "`read()` method returns [porter_duff_ctrl::R](porter_duff_ctrl::R) reader structure"]
impl crate::Readable for PORTER_DUFF_CTRL {}
#[doc = "`write(|w| ..)` method takes [porter_duff_ctrl::W](porter_duff_ctrl::W) writer structure"]
impl crate::Writable for PORTER_DUFF_CTRL {}
#[doc = "PXP Alpha Engine A Control Register."]
pub mod porter_duff_ctrl;
