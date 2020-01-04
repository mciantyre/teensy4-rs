#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DCP control register 0"]
    pub ctrl: CTRL,
    #[doc = "0x04 - DCP control register 0"]
    pub ctrl_set: CTRL_SET,
    #[doc = "0x08 - DCP control register 0"]
    pub ctrl_clr: CTRL_CLR,
    #[doc = "0x0c - DCP control register 0"]
    pub ctrl_tog: CTRL_TOG,
    #[doc = "0x10 - DCP status register"]
    pub stat: STAT,
    #[doc = "0x14 - DCP status register"]
    pub stat_set: STAT_SET,
    #[doc = "0x18 - DCP status register"]
    pub stat_clr: STAT_CLR,
    #[doc = "0x1c - DCP status register"]
    pub stat_tog: STAT_TOG,
    #[doc = "0x20 - DCP channel control register"]
    pub channelctrl: CHANNELCTRL,
    #[doc = "0x24 - DCP channel control register"]
    pub channelctrl_set: CHANNELCTRL_SET,
    #[doc = "0x28 - DCP channel control register"]
    pub channelctrl_clr: CHANNELCTRL_CLR,
    #[doc = "0x2c - DCP channel control register"]
    pub channelctrl_tog: CHANNELCTRL_TOG,
    #[doc = "0x30 - DCP capability 0 register"]
    pub capability0: CAPABILITY0,
    _reserved13: [u8; 12usize],
    #[doc = "0x40 - DCP capability 1 register"]
    pub capability1: CAPABILITY1,
    _reserved14: [u8; 12usize],
    #[doc = "0x50 - DCP context buffer pointer"]
    pub context: CONTEXT,
    _reserved15: [u8; 12usize],
    #[doc = "0x60 - DCP key index"]
    pub key: KEY,
    _reserved16: [u8; 12usize],
    #[doc = "0x70 - DCP key data"]
    pub keydata: KEYDATA,
    _reserved17: [u8; 12usize],
    #[doc = "0x80 - DCP work packet 0 status register"]
    pub packet0: PACKET0,
    _reserved18: [u8; 12usize],
    #[doc = "0x90 - DCP work packet 1 status register"]
    pub packet1: PACKET1,
    _reserved19: [u8; 12usize],
    #[doc = "0xa0 - DCP work packet 2 status register"]
    pub packet2: PACKET2,
    _reserved20: [u8; 12usize],
    #[doc = "0xb0 - DCP work packet 3 status register"]
    pub packet3: PACKET3,
    _reserved21: [u8; 12usize],
    #[doc = "0xc0 - DCP work packet 4 status register"]
    pub packet4: PACKET4,
    _reserved22: [u8; 12usize],
    #[doc = "0xd0 - DCP work packet 5 status register"]
    pub packet5: PACKET5,
    _reserved23: [u8; 12usize],
    #[doc = "0xe0 - DCP work packet 6 status register"]
    pub packet6: PACKET6,
    _reserved24: [u8; 28usize],
    #[doc = "0x100 - DCP channel 0 command pointer address register"]
    pub ch0cmdptr: CH0CMDPTR,
    _reserved25: [u8; 12usize],
    #[doc = "0x110 - DCP channel 0 semaphore register"]
    pub ch0sema: CH0SEMA,
    _reserved26: [u8; 12usize],
    #[doc = "0x120 - DCP channel 0 status register"]
    pub ch0stat: CH0STAT,
    #[doc = "0x124 - DCP channel 0 status register"]
    pub ch0stat_set: CH0STAT_SET,
    #[doc = "0x128 - DCP channel 0 status register"]
    pub ch0stat_clr: CH0STAT_CLR,
    #[doc = "0x12c - DCP channel 0 status register"]
    pub ch0stat_tog: CH0STAT_TOG,
    #[doc = "0x130 - DCP channel 0 options register"]
    pub ch0opts: CH0OPTS,
    #[doc = "0x134 - DCP channel 0 options register"]
    pub ch0opts_set: CH0OPTS_SET,
    #[doc = "0x138 - DCP channel 0 options register"]
    pub ch0opts_clr: CH0OPTS_CLR,
    #[doc = "0x13c - DCP channel 0 options register"]
    pub ch0opts_tog: CH0OPTS_TOG,
    #[doc = "0x140 - DCP channel 1 command pointer address register"]
    pub ch1cmdptr: CH1CMDPTR,
    _reserved35: [u8; 12usize],
    #[doc = "0x150 - DCP channel 1 semaphore register"]
    pub ch1sema: CH1SEMA,
    _reserved36: [u8; 12usize],
    #[doc = "0x160 - DCP channel 1 status register"]
    pub ch1stat: CH1STAT,
    #[doc = "0x164 - DCP channel 1 status register"]
    pub ch1stat_set: CH1STAT_SET,
    #[doc = "0x168 - DCP channel 1 status register"]
    pub ch1stat_clr: CH1STAT_CLR,
    #[doc = "0x16c - DCP channel 1 status register"]
    pub ch1stat_tog: CH1STAT_TOG,
    #[doc = "0x170 - DCP channel 1 options register"]
    pub ch1opts: CH1OPTS,
    #[doc = "0x174 - DCP channel 1 options register"]
    pub ch1opts_set: CH1OPTS_SET,
    #[doc = "0x178 - DCP channel 1 options register"]
    pub ch1opts_clr: CH1OPTS_CLR,
    #[doc = "0x17c - DCP channel 1 options register"]
    pub ch1opts_tog: CH1OPTS_TOG,
    #[doc = "0x180 - DCP channel 2 command pointer address register"]
    pub ch2cmdptr: CH2CMDPTR,
    _reserved45: [u8; 12usize],
    #[doc = "0x190 - DCP channel 2 semaphore register"]
    pub ch2sema: CH2SEMA,
    _reserved46: [u8; 12usize],
    #[doc = "0x1a0 - DCP channel 2 status register"]
    pub ch2stat: CH2STAT,
    #[doc = "0x1a4 - DCP channel 2 status register"]
    pub ch2stat_set: CH2STAT_SET,
    #[doc = "0x1a8 - DCP channel 2 status register"]
    pub ch2stat_clr: CH2STAT_CLR,
    #[doc = "0x1ac - DCP channel 2 status register"]
    pub ch2stat_tog: CH2STAT_TOG,
    #[doc = "0x1b0 - DCP channel 2 options register"]
    pub ch2opts: CH2OPTS,
    #[doc = "0x1b4 - DCP channel 2 options register"]
    pub ch2opts_set: CH2OPTS_SET,
    #[doc = "0x1b8 - DCP channel 2 options register"]
    pub ch2opts_clr: CH2OPTS_CLR,
    #[doc = "0x1bc - DCP channel 2 options register"]
    pub ch2opts_tog: CH2OPTS_TOG,
    #[doc = "0x1c0 - DCP channel 3 command pointer address register"]
    pub ch3cmdptr: CH3CMDPTR,
    _reserved55: [u8; 12usize],
    #[doc = "0x1d0 - DCP channel 3 semaphore register"]
    pub ch3sema: CH3SEMA,
    _reserved56: [u8; 12usize],
    #[doc = "0x1e0 - DCP channel 3 status register"]
    pub ch3stat: CH3STAT,
    #[doc = "0x1e4 - DCP channel 3 status register"]
    pub ch3stat_set: CH3STAT_SET,
    #[doc = "0x1e8 - DCP channel 3 status register"]
    pub ch3stat_clr: CH3STAT_CLR,
    #[doc = "0x1ec - DCP channel 3 status register"]
    pub ch3stat_tog: CH3STAT_TOG,
    #[doc = "0x1f0 - DCP channel 3 options register"]
    pub ch3opts: CH3OPTS,
    #[doc = "0x1f4 - DCP channel 3 options register"]
    pub ch3opts_set: CH3OPTS_SET,
    #[doc = "0x1f8 - DCP channel 3 options register"]
    pub ch3opts_clr: CH3OPTS_CLR,
    #[doc = "0x1fc - DCP channel 3 options register"]
    pub ch3opts_tog: CH3OPTS_TOG,
    _reserved64: [u8; 512usize],
    #[doc = "0x400 - DCP debug select register"]
    pub dbgselect: DBGSELECT,
    _reserved65: [u8; 12usize],
    #[doc = "0x410 - DCP debug data register"]
    pub dbgdata: DBGDATA,
    _reserved66: [u8; 12usize],
    #[doc = "0x420 - DCP page table register"]
    pub pagetable: PAGETABLE,
    _reserved67: [u8; 12usize],
    #[doc = "0x430 - DCP version register"]
    pub version: VERSION,
}
#[doc = "DCP control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "DCP control register 0"]
pub mod ctrl;
#[doc = "DCP control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_set](ctrl_set) module"]
pub type CTRL_SET = crate::Reg<u32, _CTRL_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_SET;
#[doc = "`read()` method returns [ctrl_set::R](ctrl_set::R) reader structure"]
impl crate::Readable for CTRL_SET {}
#[doc = "`write(|w| ..)` method takes [ctrl_set::W](ctrl_set::W) writer structure"]
impl crate::Writable for CTRL_SET {}
#[doc = "DCP control register 0"]
pub mod ctrl_set;
#[doc = "DCP control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_clr](ctrl_clr) module"]
pub type CTRL_CLR = crate::Reg<u32, _CTRL_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_CLR;
#[doc = "`read()` method returns [ctrl_clr::R](ctrl_clr::R) reader structure"]
impl crate::Readable for CTRL_CLR {}
#[doc = "`write(|w| ..)` method takes [ctrl_clr::W](ctrl_clr::W) writer structure"]
impl crate::Writable for CTRL_CLR {}
#[doc = "DCP control register 0"]
pub mod ctrl_clr;
#[doc = "DCP control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_tog](ctrl_tog) module"]
pub type CTRL_TOG = crate::Reg<u32, _CTRL_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_TOG;
#[doc = "`read()` method returns [ctrl_tog::R](ctrl_tog::R) reader structure"]
impl crate::Readable for CTRL_TOG {}
#[doc = "`write(|w| ..)` method takes [ctrl_tog::W](ctrl_tog::W) writer structure"]
impl crate::Writable for CTRL_TOG {}
#[doc = "DCP control register 0"]
pub mod ctrl_tog;
#[doc = "DCP status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "DCP status register"]
pub mod stat;
#[doc = "DCP status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat_set](stat_set) module"]
pub type STAT_SET = crate::Reg<u32, _STAT_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT_SET;
#[doc = "`read()` method returns [stat_set::R](stat_set::R) reader structure"]
impl crate::Readable for STAT_SET {}
#[doc = "`write(|w| ..)` method takes [stat_set::W](stat_set::W) writer structure"]
impl crate::Writable for STAT_SET {}
#[doc = "DCP status register"]
pub mod stat_set;
#[doc = "DCP status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat_clr](stat_clr) module"]
pub type STAT_CLR = crate::Reg<u32, _STAT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT_CLR;
#[doc = "`read()` method returns [stat_clr::R](stat_clr::R) reader structure"]
impl crate::Readable for STAT_CLR {}
#[doc = "`write(|w| ..)` method takes [stat_clr::W](stat_clr::W) writer structure"]
impl crate::Writable for STAT_CLR {}
#[doc = "DCP status register"]
pub mod stat_clr;
#[doc = "DCP status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat_tog](stat_tog) module"]
pub type STAT_TOG = crate::Reg<u32, _STAT_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT_TOG;
#[doc = "`read()` method returns [stat_tog::R](stat_tog::R) reader structure"]
impl crate::Readable for STAT_TOG {}
#[doc = "`write(|w| ..)` method takes [stat_tog::W](stat_tog::W) writer structure"]
impl crate::Writable for STAT_TOG {}
#[doc = "DCP status register"]
pub mod stat_tog;
#[doc = "DCP channel control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channelctrl](channelctrl) module"]
pub type CHANNELCTRL = crate::Reg<u32, _CHANNELCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHANNELCTRL;
#[doc = "`read()` method returns [channelctrl::R](channelctrl::R) reader structure"]
impl crate::Readable for CHANNELCTRL {}
#[doc = "`write(|w| ..)` method takes [channelctrl::W](channelctrl::W) writer structure"]
impl crate::Writable for CHANNELCTRL {}
#[doc = "DCP channel control register"]
pub mod channelctrl;
#[doc = "DCP channel control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channelctrl_set](channelctrl_set) module"]
pub type CHANNELCTRL_SET = crate::Reg<u32, _CHANNELCTRL_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHANNELCTRL_SET;
#[doc = "`read()` method returns [channelctrl_set::R](channelctrl_set::R) reader structure"]
impl crate::Readable for CHANNELCTRL_SET {}
#[doc = "`write(|w| ..)` method takes [channelctrl_set::W](channelctrl_set::W) writer structure"]
impl crate::Writable for CHANNELCTRL_SET {}
#[doc = "DCP channel control register"]
pub mod channelctrl_set;
#[doc = "DCP channel control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channelctrl_clr](channelctrl_clr) module"]
pub type CHANNELCTRL_CLR = crate::Reg<u32, _CHANNELCTRL_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHANNELCTRL_CLR;
#[doc = "`read()` method returns [channelctrl_clr::R](channelctrl_clr::R) reader structure"]
impl crate::Readable for CHANNELCTRL_CLR {}
#[doc = "`write(|w| ..)` method takes [channelctrl_clr::W](channelctrl_clr::W) writer structure"]
impl crate::Writable for CHANNELCTRL_CLR {}
#[doc = "DCP channel control register"]
pub mod channelctrl_clr;
#[doc = "DCP channel control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channelctrl_tog](channelctrl_tog) module"]
pub type CHANNELCTRL_TOG = crate::Reg<u32, _CHANNELCTRL_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHANNELCTRL_TOG;
#[doc = "`read()` method returns [channelctrl_tog::R](channelctrl_tog::R) reader structure"]
impl crate::Readable for CHANNELCTRL_TOG {}
#[doc = "`write(|w| ..)` method takes [channelctrl_tog::W](channelctrl_tog::W) writer structure"]
impl crate::Writable for CHANNELCTRL_TOG {}
#[doc = "DCP channel control register"]
pub mod channelctrl_tog;
#[doc = "DCP capability 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capability0](capability0) module"]
pub type CAPABILITY0 = crate::Reg<u32, _CAPABILITY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPABILITY0;
#[doc = "`read()` method returns [capability0::R](capability0::R) reader structure"]
impl crate::Readable for CAPABILITY0 {}
#[doc = "`write(|w| ..)` method takes [capability0::W](capability0::W) writer structure"]
impl crate::Writable for CAPABILITY0 {}
#[doc = "DCP capability 0 register"]
pub mod capability0;
#[doc = "DCP capability 1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capability1](capability1) module"]
pub type CAPABILITY1 = crate::Reg<u32, _CAPABILITY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPABILITY1;
#[doc = "`read()` method returns [capability1::R](capability1::R) reader structure"]
impl crate::Readable for CAPABILITY1 {}
#[doc = "DCP capability 1 register"]
pub mod capability1;
#[doc = "DCP context buffer pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [context](context) module"]
pub type CONTEXT = crate::Reg<u32, _CONTEXT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONTEXT;
#[doc = "`read()` method returns [context::R](context::R) reader structure"]
impl crate::Readable for CONTEXT {}
#[doc = "`write(|w| ..)` method takes [context::W](context::W) writer structure"]
impl crate::Writable for CONTEXT {}
#[doc = "DCP context buffer pointer"]
pub mod context;
#[doc = "DCP key index\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key](key) module"]
pub type KEY = crate::Reg<u32, _KEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY;
#[doc = "`read()` method returns [key::R](key::R) reader structure"]
impl crate::Readable for KEY {}
#[doc = "`write(|w| ..)` method takes [key::W](key::W) writer structure"]
impl crate::Writable for KEY {}
#[doc = "DCP key index"]
pub mod key;
#[doc = "DCP key data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keydata](keydata) module"]
pub type KEYDATA = crate::Reg<u32, _KEYDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYDATA;
#[doc = "`read()` method returns [keydata::R](keydata::R) reader structure"]
impl crate::Readable for KEYDATA {}
#[doc = "`write(|w| ..)` method takes [keydata::W](keydata::W) writer structure"]
impl crate::Writable for KEYDATA {}
#[doc = "DCP key data"]
pub mod keydata;
#[doc = "DCP work packet 0 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [packet0](packet0) module"]
pub type PACKET0 = crate::Reg<u32, _PACKET0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACKET0;
#[doc = "`read()` method returns [packet0::R](packet0::R) reader structure"]
impl crate::Readable for PACKET0 {}
#[doc = "DCP work packet 0 status register"]
pub mod packet0;
#[doc = "DCP work packet 1 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [packet1](packet1) module"]
pub type PACKET1 = crate::Reg<u32, _PACKET1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACKET1;
#[doc = "`read()` method returns [packet1::R](packet1::R) reader structure"]
impl crate::Readable for PACKET1 {}
#[doc = "DCP work packet 1 status register"]
pub mod packet1;
#[doc = "DCP work packet 2 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [packet2](packet2) module"]
pub type PACKET2 = crate::Reg<u32, _PACKET2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACKET2;
#[doc = "`read()` method returns [packet2::R](packet2::R) reader structure"]
impl crate::Readable for PACKET2 {}
#[doc = "DCP work packet 2 status register"]
pub mod packet2;
#[doc = "DCP work packet 3 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [packet3](packet3) module"]
pub type PACKET3 = crate::Reg<u32, _PACKET3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACKET3;
#[doc = "`read()` method returns [packet3::R](packet3::R) reader structure"]
impl crate::Readable for PACKET3 {}
#[doc = "DCP work packet 3 status register"]
pub mod packet3;
#[doc = "DCP work packet 4 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [packet4](packet4) module"]
pub type PACKET4 = crate::Reg<u32, _PACKET4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACKET4;
#[doc = "`read()` method returns [packet4::R](packet4::R) reader structure"]
impl crate::Readable for PACKET4 {}
#[doc = "DCP work packet 4 status register"]
pub mod packet4;
#[doc = "DCP work packet 5 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [packet5](packet5) module"]
pub type PACKET5 = crate::Reg<u32, _PACKET5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACKET5;
#[doc = "`read()` method returns [packet5::R](packet5::R) reader structure"]
impl crate::Readable for PACKET5 {}
#[doc = "DCP work packet 5 status register"]
pub mod packet5;
#[doc = "DCP work packet 6 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [packet6](packet6) module"]
pub type PACKET6 = crate::Reg<u32, _PACKET6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACKET6;
#[doc = "`read()` method returns [packet6::R](packet6::R) reader structure"]
impl crate::Readable for PACKET6 {}
#[doc = "DCP work packet 6 status register"]
pub mod packet6;
#[doc = "DCP channel 0 command pointer address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0cmdptr](ch0cmdptr) module"]
pub type CH0CMDPTR = crate::Reg<u32, _CH0CMDPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0CMDPTR;
#[doc = "`read()` method returns [ch0cmdptr::R](ch0cmdptr::R) reader structure"]
impl crate::Readable for CH0CMDPTR {}
#[doc = "`write(|w| ..)` method takes [ch0cmdptr::W](ch0cmdptr::W) writer structure"]
impl crate::Writable for CH0CMDPTR {}
#[doc = "DCP channel 0 command pointer address register"]
pub mod ch0cmdptr;
#[doc = "DCP channel 0 semaphore register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0sema](ch0sema) module"]
pub type CH0SEMA = crate::Reg<u32, _CH0SEMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0SEMA;
#[doc = "`read()` method returns [ch0sema::R](ch0sema::R) reader structure"]
impl crate::Readable for CH0SEMA {}
#[doc = "`write(|w| ..)` method takes [ch0sema::W](ch0sema::W) writer structure"]
impl crate::Writable for CH0SEMA {}
#[doc = "DCP channel 0 semaphore register"]
pub mod ch0sema;
#[doc = "DCP channel 0 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0stat](ch0stat) module"]
pub type CH0STAT = crate::Reg<u32, _CH0STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0STAT;
#[doc = "`read()` method returns [ch0stat::R](ch0stat::R) reader structure"]
impl crate::Readable for CH0STAT {}
#[doc = "`write(|w| ..)` method takes [ch0stat::W](ch0stat::W) writer structure"]
impl crate::Writable for CH0STAT {}
#[doc = "DCP channel 0 status register"]
pub mod ch0stat;
#[doc = "DCP channel 0 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0stat_set](ch0stat_set) module"]
pub type CH0STAT_SET = crate::Reg<u32, _CH0STAT_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0STAT_SET;
#[doc = "`read()` method returns [ch0stat_set::R](ch0stat_set::R) reader structure"]
impl crate::Readable for CH0STAT_SET {}
#[doc = "`write(|w| ..)` method takes [ch0stat_set::W](ch0stat_set::W) writer structure"]
impl crate::Writable for CH0STAT_SET {}
#[doc = "DCP channel 0 status register"]
pub mod ch0stat_set;
#[doc = "DCP channel 0 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0stat_clr](ch0stat_clr) module"]
pub type CH0STAT_CLR = crate::Reg<u32, _CH0STAT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0STAT_CLR;
#[doc = "`read()` method returns [ch0stat_clr::R](ch0stat_clr::R) reader structure"]
impl crate::Readable for CH0STAT_CLR {}
#[doc = "`write(|w| ..)` method takes [ch0stat_clr::W](ch0stat_clr::W) writer structure"]
impl crate::Writable for CH0STAT_CLR {}
#[doc = "DCP channel 0 status register"]
pub mod ch0stat_clr;
#[doc = "DCP channel 0 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0stat_tog](ch0stat_tog) module"]
pub type CH0STAT_TOG = crate::Reg<u32, _CH0STAT_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0STAT_TOG;
#[doc = "`read()` method returns [ch0stat_tog::R](ch0stat_tog::R) reader structure"]
impl crate::Readable for CH0STAT_TOG {}
#[doc = "`write(|w| ..)` method takes [ch0stat_tog::W](ch0stat_tog::W) writer structure"]
impl crate::Writable for CH0STAT_TOG {}
#[doc = "DCP channel 0 status register"]
pub mod ch0stat_tog;
#[doc = "DCP channel 0 options register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0opts](ch0opts) module"]
pub type CH0OPTS = crate::Reg<u32, _CH0OPTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0OPTS;
#[doc = "`read()` method returns [ch0opts::R](ch0opts::R) reader structure"]
impl crate::Readable for CH0OPTS {}
#[doc = "`write(|w| ..)` method takes [ch0opts::W](ch0opts::W) writer structure"]
impl crate::Writable for CH0OPTS {}
#[doc = "DCP channel 0 options register"]
pub mod ch0opts;
#[doc = "DCP channel 0 options register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0opts_set](ch0opts_set) module"]
pub type CH0OPTS_SET = crate::Reg<u32, _CH0OPTS_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0OPTS_SET;
#[doc = "`read()` method returns [ch0opts_set::R](ch0opts_set::R) reader structure"]
impl crate::Readable for CH0OPTS_SET {}
#[doc = "`write(|w| ..)` method takes [ch0opts_set::W](ch0opts_set::W) writer structure"]
impl crate::Writable for CH0OPTS_SET {}
#[doc = "DCP channel 0 options register"]
pub mod ch0opts_set;
#[doc = "DCP channel 0 options register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0opts_clr](ch0opts_clr) module"]
pub type CH0OPTS_CLR = crate::Reg<u32, _CH0OPTS_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0OPTS_CLR;
#[doc = "`read()` method returns [ch0opts_clr::R](ch0opts_clr::R) reader structure"]
impl crate::Readable for CH0OPTS_CLR {}
#[doc = "`write(|w| ..)` method takes [ch0opts_clr::W](ch0opts_clr::W) writer structure"]
impl crate::Writable for CH0OPTS_CLR {}
#[doc = "DCP channel 0 options register"]
pub mod ch0opts_clr;
#[doc = "DCP channel 0 options register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0opts_tog](ch0opts_tog) module"]
pub type CH0OPTS_TOG = crate::Reg<u32, _CH0OPTS_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0OPTS_TOG;
#[doc = "`read()` method returns [ch0opts_tog::R](ch0opts_tog::R) reader structure"]
impl crate::Readable for CH0OPTS_TOG {}
#[doc = "`write(|w| ..)` method takes [ch0opts_tog::W](ch0opts_tog::W) writer structure"]
impl crate::Writable for CH0OPTS_TOG {}
#[doc = "DCP channel 0 options register"]
pub mod ch0opts_tog;
#[doc = "DCP channel 1 command pointer address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1cmdptr](ch1cmdptr) module"]
pub type CH1CMDPTR = crate::Reg<u32, _CH1CMDPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1CMDPTR;
#[doc = "`read()` method returns [ch1cmdptr::R](ch1cmdptr::R) reader structure"]
impl crate::Readable for CH1CMDPTR {}
#[doc = "`write(|w| ..)` method takes [ch1cmdptr::W](ch1cmdptr::W) writer structure"]
impl crate::Writable for CH1CMDPTR {}
#[doc = "DCP channel 1 command pointer address register"]
pub mod ch1cmdptr;
#[doc = "DCP channel 1 semaphore register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1sema](ch1sema) module"]
pub type CH1SEMA = crate::Reg<u32, _CH1SEMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1SEMA;
#[doc = "`read()` method returns [ch1sema::R](ch1sema::R) reader structure"]
impl crate::Readable for CH1SEMA {}
#[doc = "`write(|w| ..)` method takes [ch1sema::W](ch1sema::W) writer structure"]
impl crate::Writable for CH1SEMA {}
#[doc = "DCP channel 1 semaphore register"]
pub mod ch1sema;
#[doc = "DCP channel 1 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1stat](ch1stat) module"]
pub type CH1STAT = crate::Reg<u32, _CH1STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1STAT;
#[doc = "`read()` method returns [ch1stat::R](ch1stat::R) reader structure"]
impl crate::Readable for CH1STAT {}
#[doc = "`write(|w| ..)` method takes [ch1stat::W](ch1stat::W) writer structure"]
impl crate::Writable for CH1STAT {}
#[doc = "DCP channel 1 status register"]
pub mod ch1stat;
#[doc = "DCP channel 1 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1stat_set](ch1stat_set) module"]
pub type CH1STAT_SET = crate::Reg<u32, _CH1STAT_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1STAT_SET;
#[doc = "`read()` method returns [ch1stat_set::R](ch1stat_set::R) reader structure"]
impl crate::Readable for CH1STAT_SET {}
#[doc = "`write(|w| ..)` method takes [ch1stat_set::W](ch1stat_set::W) writer structure"]
impl crate::Writable for CH1STAT_SET {}
#[doc = "DCP channel 1 status register"]
pub mod ch1stat_set;
#[doc = "DCP channel 1 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1stat_clr](ch1stat_clr) module"]
pub type CH1STAT_CLR = crate::Reg<u32, _CH1STAT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1STAT_CLR;
#[doc = "`read()` method returns [ch1stat_clr::R](ch1stat_clr::R) reader structure"]
impl crate::Readable for CH1STAT_CLR {}
#[doc = "`write(|w| ..)` method takes [ch1stat_clr::W](ch1stat_clr::W) writer structure"]
impl crate::Writable for CH1STAT_CLR {}
#[doc = "DCP channel 1 status register"]
pub mod ch1stat_clr;
#[doc = "DCP channel 1 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1stat_tog](ch1stat_tog) module"]
pub type CH1STAT_TOG = crate::Reg<u32, _CH1STAT_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1STAT_TOG;
#[doc = "`read()` method returns [ch1stat_tog::R](ch1stat_tog::R) reader structure"]
impl crate::Readable for CH1STAT_TOG {}
#[doc = "`write(|w| ..)` method takes [ch1stat_tog::W](ch1stat_tog::W) writer structure"]
impl crate::Writable for CH1STAT_TOG {}
#[doc = "DCP channel 1 status register"]
pub mod ch1stat_tog;
#[doc = "DCP channel 1 options register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1opts](ch1opts) module"]
pub type CH1OPTS = crate::Reg<u32, _CH1OPTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1OPTS;
#[doc = "`read()` method returns [ch1opts::R](ch1opts::R) reader structure"]
impl crate::Readable for CH1OPTS {}
#[doc = "`write(|w| ..)` method takes [ch1opts::W](ch1opts::W) writer structure"]
impl crate::Writable for CH1OPTS {}
#[doc = "DCP channel 1 options register"]
pub mod ch1opts;
#[doc = "DCP channel 1 options register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1opts_set](ch1opts_set) module"]
pub type CH1OPTS_SET = crate::Reg<u32, _CH1OPTS_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1OPTS_SET;
#[doc = "`read()` method returns [ch1opts_set::R](ch1opts_set::R) reader structure"]
impl crate::Readable for CH1OPTS_SET {}
#[doc = "`write(|w| ..)` method takes [ch1opts_set::W](ch1opts_set::W) writer structure"]
impl crate::Writable for CH1OPTS_SET {}
#[doc = "DCP channel 1 options register"]
pub mod ch1opts_set;
#[doc = "DCP channel 1 options register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1opts_clr](ch1opts_clr) module"]
pub type CH1OPTS_CLR = crate::Reg<u32, _CH1OPTS_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1OPTS_CLR;
#[doc = "`read()` method returns [ch1opts_clr::R](ch1opts_clr::R) reader structure"]
impl crate::Readable for CH1OPTS_CLR {}
#[doc = "`write(|w| ..)` method takes [ch1opts_clr::W](ch1opts_clr::W) writer structure"]
impl crate::Writable for CH1OPTS_CLR {}
#[doc = "DCP channel 1 options register"]
pub mod ch1opts_clr;
#[doc = "DCP channel 1 options register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1opts_tog](ch1opts_tog) module"]
pub type CH1OPTS_TOG = crate::Reg<u32, _CH1OPTS_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1OPTS_TOG;
#[doc = "`read()` method returns [ch1opts_tog::R](ch1opts_tog::R) reader structure"]
impl crate::Readable for CH1OPTS_TOG {}
#[doc = "`write(|w| ..)` method takes [ch1opts_tog::W](ch1opts_tog::W) writer structure"]
impl crate::Writable for CH1OPTS_TOG {}
#[doc = "DCP channel 1 options register"]
pub mod ch1opts_tog;
#[doc = "DCP channel 2 command pointer address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2cmdptr](ch2cmdptr) module"]
pub type CH2CMDPTR = crate::Reg<u32, _CH2CMDPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2CMDPTR;
#[doc = "`read()` method returns [ch2cmdptr::R](ch2cmdptr::R) reader structure"]
impl crate::Readable for CH2CMDPTR {}
#[doc = "`write(|w| ..)` method takes [ch2cmdptr::W](ch2cmdptr::W) writer structure"]
impl crate::Writable for CH2CMDPTR {}
#[doc = "DCP channel 2 command pointer address register"]
pub mod ch2cmdptr;
#[doc = "DCP channel 2 semaphore register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2sema](ch2sema) module"]
pub type CH2SEMA = crate::Reg<u32, _CH2SEMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2SEMA;
#[doc = "`read()` method returns [ch2sema::R](ch2sema::R) reader structure"]
impl crate::Readable for CH2SEMA {}
#[doc = "`write(|w| ..)` method takes [ch2sema::W](ch2sema::W) writer structure"]
impl crate::Writable for CH2SEMA {}
#[doc = "DCP channel 2 semaphore register"]
pub mod ch2sema;
#[doc = "DCP channel 2 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2stat](ch2stat) module"]
pub type CH2STAT = crate::Reg<u32, _CH2STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2STAT;
#[doc = "`read()` method returns [ch2stat::R](ch2stat::R) reader structure"]
impl crate::Readable for CH2STAT {}
#[doc = "`write(|w| ..)` method takes [ch2stat::W](ch2stat::W) writer structure"]
impl crate::Writable for CH2STAT {}
#[doc = "DCP channel 2 status register"]
pub mod ch2stat;
#[doc = "DCP channel 2 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2stat_set](ch2stat_set) module"]
pub type CH2STAT_SET = crate::Reg<u32, _CH2STAT_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2STAT_SET;
#[doc = "`read()` method returns [ch2stat_set::R](ch2stat_set::R) reader structure"]
impl crate::Readable for CH2STAT_SET {}
#[doc = "`write(|w| ..)` method takes [ch2stat_set::W](ch2stat_set::W) writer structure"]
impl crate::Writable for CH2STAT_SET {}
#[doc = "DCP channel 2 status register"]
pub mod ch2stat_set;
#[doc = "DCP channel 2 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2stat_clr](ch2stat_clr) module"]
pub type CH2STAT_CLR = crate::Reg<u32, _CH2STAT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2STAT_CLR;
#[doc = "`read()` method returns [ch2stat_clr::R](ch2stat_clr::R) reader structure"]
impl crate::Readable for CH2STAT_CLR {}
#[doc = "`write(|w| ..)` method takes [ch2stat_clr::W](ch2stat_clr::W) writer structure"]
impl crate::Writable for CH2STAT_CLR {}
#[doc = "DCP channel 2 status register"]
pub mod ch2stat_clr;
#[doc = "DCP channel 2 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2stat_tog](ch2stat_tog) module"]
pub type CH2STAT_TOG = crate::Reg<u32, _CH2STAT_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2STAT_TOG;
#[doc = "`read()` method returns [ch2stat_tog::R](ch2stat_tog::R) reader structure"]
impl crate::Readable for CH2STAT_TOG {}
#[doc = "`write(|w| ..)` method takes [ch2stat_tog::W](ch2stat_tog::W) writer structure"]
impl crate::Writable for CH2STAT_TOG {}
#[doc = "DCP channel 2 status register"]
pub mod ch2stat_tog;
#[doc = "DCP channel 2 options register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2opts](ch2opts) module"]
pub type CH2OPTS = crate::Reg<u32, _CH2OPTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2OPTS;
#[doc = "`read()` method returns [ch2opts::R](ch2opts::R) reader structure"]
impl crate::Readable for CH2OPTS {}
#[doc = "`write(|w| ..)` method takes [ch2opts::W](ch2opts::W) writer structure"]
impl crate::Writable for CH2OPTS {}
#[doc = "DCP channel 2 options register"]
pub mod ch2opts;
#[doc = "DCP channel 2 options register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2opts_set](ch2opts_set) module"]
pub type CH2OPTS_SET = crate::Reg<u32, _CH2OPTS_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2OPTS_SET;
#[doc = "`read()` method returns [ch2opts_set::R](ch2opts_set::R) reader structure"]
impl crate::Readable for CH2OPTS_SET {}
#[doc = "`write(|w| ..)` method takes [ch2opts_set::W](ch2opts_set::W) writer structure"]
impl crate::Writable for CH2OPTS_SET {}
#[doc = "DCP channel 2 options register"]
pub mod ch2opts_set;
#[doc = "DCP channel 2 options register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2opts_clr](ch2opts_clr) module"]
pub type CH2OPTS_CLR = crate::Reg<u32, _CH2OPTS_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2OPTS_CLR;
#[doc = "`read()` method returns [ch2opts_clr::R](ch2opts_clr::R) reader structure"]
impl crate::Readable for CH2OPTS_CLR {}
#[doc = "`write(|w| ..)` method takes [ch2opts_clr::W](ch2opts_clr::W) writer structure"]
impl crate::Writable for CH2OPTS_CLR {}
#[doc = "DCP channel 2 options register"]
pub mod ch2opts_clr;
#[doc = "DCP channel 2 options register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2opts_tog](ch2opts_tog) module"]
pub type CH2OPTS_TOG = crate::Reg<u32, _CH2OPTS_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2OPTS_TOG;
#[doc = "`read()` method returns [ch2opts_tog::R](ch2opts_tog::R) reader structure"]
impl crate::Readable for CH2OPTS_TOG {}
#[doc = "`write(|w| ..)` method takes [ch2opts_tog::W](ch2opts_tog::W) writer structure"]
impl crate::Writable for CH2OPTS_TOG {}
#[doc = "DCP channel 2 options register"]
pub mod ch2opts_tog;
#[doc = "DCP channel 3 command pointer address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3cmdptr](ch3cmdptr) module"]
pub type CH3CMDPTR = crate::Reg<u32, _CH3CMDPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3CMDPTR;
#[doc = "`read()` method returns [ch3cmdptr::R](ch3cmdptr::R) reader structure"]
impl crate::Readable for CH3CMDPTR {}
#[doc = "`write(|w| ..)` method takes [ch3cmdptr::W](ch3cmdptr::W) writer structure"]
impl crate::Writable for CH3CMDPTR {}
#[doc = "DCP channel 3 command pointer address register"]
pub mod ch3cmdptr;
#[doc = "DCP channel 3 semaphore register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3sema](ch3sema) module"]
pub type CH3SEMA = crate::Reg<u32, _CH3SEMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3SEMA;
#[doc = "`read()` method returns [ch3sema::R](ch3sema::R) reader structure"]
impl crate::Readable for CH3SEMA {}
#[doc = "`write(|w| ..)` method takes [ch3sema::W](ch3sema::W) writer structure"]
impl crate::Writable for CH3SEMA {}
#[doc = "DCP channel 3 semaphore register"]
pub mod ch3sema;
#[doc = "DCP channel 3 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3stat](ch3stat) module"]
pub type CH3STAT = crate::Reg<u32, _CH3STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3STAT;
#[doc = "`read()` method returns [ch3stat::R](ch3stat::R) reader structure"]
impl crate::Readable for CH3STAT {}
#[doc = "`write(|w| ..)` method takes [ch3stat::W](ch3stat::W) writer structure"]
impl crate::Writable for CH3STAT {}
#[doc = "DCP channel 3 status register"]
pub mod ch3stat;
#[doc = "DCP channel 3 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3stat_set](ch3stat_set) module"]
pub type CH3STAT_SET = crate::Reg<u32, _CH3STAT_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3STAT_SET;
#[doc = "`read()` method returns [ch3stat_set::R](ch3stat_set::R) reader structure"]
impl crate::Readable for CH3STAT_SET {}
#[doc = "`write(|w| ..)` method takes [ch3stat_set::W](ch3stat_set::W) writer structure"]
impl crate::Writable for CH3STAT_SET {}
#[doc = "DCP channel 3 status register"]
pub mod ch3stat_set;
#[doc = "DCP channel 3 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3stat_clr](ch3stat_clr) module"]
pub type CH3STAT_CLR = crate::Reg<u32, _CH3STAT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3STAT_CLR;
#[doc = "`read()` method returns [ch3stat_clr::R](ch3stat_clr::R) reader structure"]
impl crate::Readable for CH3STAT_CLR {}
#[doc = "`write(|w| ..)` method takes [ch3stat_clr::W](ch3stat_clr::W) writer structure"]
impl crate::Writable for CH3STAT_CLR {}
#[doc = "DCP channel 3 status register"]
pub mod ch3stat_clr;
#[doc = "DCP channel 3 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3stat_tog](ch3stat_tog) module"]
pub type CH3STAT_TOG = crate::Reg<u32, _CH3STAT_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3STAT_TOG;
#[doc = "`read()` method returns [ch3stat_tog::R](ch3stat_tog::R) reader structure"]
impl crate::Readable for CH3STAT_TOG {}
#[doc = "`write(|w| ..)` method takes [ch3stat_tog::W](ch3stat_tog::W) writer structure"]
impl crate::Writable for CH3STAT_TOG {}
#[doc = "DCP channel 3 status register"]
pub mod ch3stat_tog;
#[doc = "DCP channel 3 options register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3opts](ch3opts) module"]
pub type CH3OPTS = crate::Reg<u32, _CH3OPTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3OPTS;
#[doc = "`read()` method returns [ch3opts::R](ch3opts::R) reader structure"]
impl crate::Readable for CH3OPTS {}
#[doc = "`write(|w| ..)` method takes [ch3opts::W](ch3opts::W) writer structure"]
impl crate::Writable for CH3OPTS {}
#[doc = "DCP channel 3 options register"]
pub mod ch3opts;
#[doc = "DCP channel 3 options register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3opts_set](ch3opts_set) module"]
pub type CH3OPTS_SET = crate::Reg<u32, _CH3OPTS_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3OPTS_SET;
#[doc = "`read()` method returns [ch3opts_set::R](ch3opts_set::R) reader structure"]
impl crate::Readable for CH3OPTS_SET {}
#[doc = "`write(|w| ..)` method takes [ch3opts_set::W](ch3opts_set::W) writer structure"]
impl crate::Writable for CH3OPTS_SET {}
#[doc = "DCP channel 3 options register"]
pub mod ch3opts_set;
#[doc = "DCP channel 3 options register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3opts_clr](ch3opts_clr) module"]
pub type CH3OPTS_CLR = crate::Reg<u32, _CH3OPTS_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3OPTS_CLR;
#[doc = "`read()` method returns [ch3opts_clr::R](ch3opts_clr::R) reader structure"]
impl crate::Readable for CH3OPTS_CLR {}
#[doc = "`write(|w| ..)` method takes [ch3opts_clr::W](ch3opts_clr::W) writer structure"]
impl crate::Writable for CH3OPTS_CLR {}
#[doc = "DCP channel 3 options register"]
pub mod ch3opts_clr;
#[doc = "DCP channel 3 options register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3opts_tog](ch3opts_tog) module"]
pub type CH3OPTS_TOG = crate::Reg<u32, _CH3OPTS_TOG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3OPTS_TOG;
#[doc = "`read()` method returns [ch3opts_tog::R](ch3opts_tog::R) reader structure"]
impl crate::Readable for CH3OPTS_TOG {}
#[doc = "`write(|w| ..)` method takes [ch3opts_tog::W](ch3opts_tog::W) writer structure"]
impl crate::Writable for CH3OPTS_TOG {}
#[doc = "DCP channel 3 options register"]
pub mod ch3opts_tog;
#[doc = "DCP debug select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgselect](dbgselect) module"]
pub type DBGSELECT = crate::Reg<u32, _DBGSELECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGSELECT;
#[doc = "`read()` method returns [dbgselect::R](dbgselect::R) reader structure"]
impl crate::Readable for DBGSELECT {}
#[doc = "`write(|w| ..)` method takes [dbgselect::W](dbgselect::W) writer structure"]
impl crate::Writable for DBGSELECT {}
#[doc = "DCP debug select register"]
pub mod dbgselect;
#[doc = "DCP debug data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgdata](dbgdata) module"]
pub type DBGDATA = crate::Reg<u32, _DBGDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGDATA;
#[doc = "`read()` method returns [dbgdata::R](dbgdata::R) reader structure"]
impl crate::Readable for DBGDATA {}
#[doc = "DCP debug data register"]
pub mod dbgdata;
#[doc = "DCP page table register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pagetable](pagetable) module"]
pub type PAGETABLE = crate::Reg<u32, _PAGETABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAGETABLE;
#[doc = "`read()` method returns [pagetable::R](pagetable::R) reader structure"]
impl crate::Readable for PAGETABLE {}
#[doc = "`write(|w| ..)` method takes [pagetable::W](pagetable::W) writer structure"]
impl crate::Writable for PAGETABLE {}
#[doc = "DCP page table register"]
pub mod pagetable;
#[doc = "DCP version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version](version) module"]
pub type VERSION = crate::Reg<u32, _VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERSION;
#[doc = "`read()` method returns [version::R](version::R) reader structure"]
impl crate::Readable for VERSION {}
#[doc = "DCP version register"]
pub mod version;
