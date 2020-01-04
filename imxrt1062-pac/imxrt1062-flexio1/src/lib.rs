#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub verid: VERID,
    #[doc = "0x04 - Parameter Register"]
    pub param: PARAM,
    #[doc = "0x08 - FlexIO Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x0c - Pin State Register"]
    pub pin: PIN,
    #[doc = "0x10 - Shifter Status Register"]
    pub shiftstat: SHIFTSTAT,
    #[doc = "0x14 - Shifter Error Register"]
    pub shifterr: SHIFTERR,
    #[doc = "0x18 - Timer Status Register"]
    pub timstat: TIMSTAT,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - Shifter Status Interrupt Enable"]
    pub shiftsien: SHIFTSIEN,
    #[doc = "0x24 - Shifter Error Interrupt Enable"]
    pub shifteien: SHIFTEIEN,
    #[doc = "0x28 - Timer Interrupt Enable Register"]
    pub timien: TIMIEN,
    _reserved10: [u8; 4usize],
    #[doc = "0x30 - Shifter Status DMA Enable"]
    pub shiftsden: SHIFTSDEN,
    _reserved11: [u8; 12usize],
    #[doc = "0x40 - Shifter State Register"]
    pub shiftstate: SHIFTSTATE,
    _reserved12: [u8; 60usize],
    #[doc = "0x80 - Shifter Control N Register"]
    pub shiftctl: [SHIFTCTL; 4],
    _reserved13: [u8; 112usize],
    #[doc = "0x100 - Shifter Configuration N Register"]
    pub shiftcfg: [SHIFTCFG; 4],
    _reserved14: [u8; 240usize],
    #[doc = "0x200 - Shifter Buffer N Register"]
    pub shiftbuf: [SHIFTBUF; 4],
    _reserved15: [u8; 112usize],
    #[doc = "0x280 - Shifter Buffer N Bit Swapped Register"]
    pub shiftbufbis: [SHIFTBUFBIS; 4],
    _reserved16: [u8; 112usize],
    #[doc = "0x300 - Shifter Buffer N Byte Swapped Register"]
    pub shiftbufbys: [SHIFTBUFBYS; 4],
    _reserved17: [u8; 112usize],
    #[doc = "0x380 - Shifter Buffer N Bit Byte Swapped Register"]
    pub shiftbufbbs: [SHIFTBUFBBS; 4],
    _reserved18: [u8; 112usize],
    #[doc = "0x400 - Timer Control N Register"]
    pub timctl: [TIMCTL; 4],
    _reserved19: [u8; 112usize],
    #[doc = "0x480 - Timer Configuration N Register"]
    pub timcfg: [TIMCFG; 4],
    _reserved20: [u8; 112usize],
    #[doc = "0x500 - Timer Compare N Register"]
    pub timcmp: [TIMCMP; 4],
    _reserved21: [u8; 368usize],
    #[doc = "0x680 - Shifter Buffer N Nibble Byte Swapped Register"]
    pub shiftbufnbs: [SHIFTBUFNBS; 4],
    _reserved22: [u8; 112usize],
    #[doc = "0x700 - Shifter Buffer N Half Word Swapped Register"]
    pub shiftbufhws: [SHIFTBUFHWS; 4],
    _reserved23: [u8; 112usize],
    #[doc = "0x780 - Shifter Buffer N Nibble Swapped Register"]
    pub shiftbufnis: [SHIFTBUFNIS; 4],
}
#[doc = "Version ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [verid](verid) module"]
pub type VERID = crate::Reg<u32, _VERID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERID;
#[doc = "`read()` method returns [verid::R](verid::R) reader structure"]
impl crate::Readable for VERID {}
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [param](param) module"]
pub type PARAM = crate::Reg<u32, _PARAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PARAM;
#[doc = "`read()` method returns [param::R](param::R) reader structure"]
impl crate::Readable for PARAM {}
#[doc = "Parameter Register"]
pub mod param;
#[doc = "FlexIO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "FlexIO Control Register"]
pub mod ctrl;
#[doc = "Pin State Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin](pin) module"]
pub type PIN = crate::Reg<u32, _PIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN;
#[doc = "`read()` method returns [pin::R](pin::R) reader structure"]
impl crate::Readable for PIN {}
#[doc = "Pin State Register"]
pub mod pin;
#[doc = "Shifter Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftstat](shiftstat) module"]
pub type SHIFTSTAT = crate::Reg<u32, _SHIFTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTSTAT;
#[doc = "`read()` method returns [shiftstat::R](shiftstat::R) reader structure"]
impl crate::Readable for SHIFTSTAT {}
#[doc = "`write(|w| ..)` method takes [shiftstat::W](shiftstat::W) writer structure"]
impl crate::Writable for SHIFTSTAT {}
#[doc = "Shifter Status Register"]
pub mod shiftstat;
#[doc = "Shifter Error Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shifterr](shifterr) module"]
pub type SHIFTERR = crate::Reg<u32, _SHIFTERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTERR;
#[doc = "`read()` method returns [shifterr::R](shifterr::R) reader structure"]
impl crate::Readable for SHIFTERR {}
#[doc = "`write(|w| ..)` method takes [shifterr::W](shifterr::W) writer structure"]
impl crate::Writable for SHIFTERR {}
#[doc = "Shifter Error Register"]
pub mod shifterr;
#[doc = "Timer Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timstat](timstat) module"]
pub type TIMSTAT = crate::Reg<u32, _TIMSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMSTAT;
#[doc = "`read()` method returns [timstat::R](timstat::R) reader structure"]
impl crate::Readable for TIMSTAT {}
#[doc = "`write(|w| ..)` method takes [timstat::W](timstat::W) writer structure"]
impl crate::Writable for TIMSTAT {}
#[doc = "Timer Status Register"]
pub mod timstat;
#[doc = "Shifter Status Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftsien](shiftsien) module"]
pub type SHIFTSIEN = crate::Reg<u32, _SHIFTSIEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTSIEN;
#[doc = "`read()` method returns [shiftsien::R](shiftsien::R) reader structure"]
impl crate::Readable for SHIFTSIEN {}
#[doc = "`write(|w| ..)` method takes [shiftsien::W](shiftsien::W) writer structure"]
impl crate::Writable for SHIFTSIEN {}
#[doc = "Shifter Status Interrupt Enable"]
pub mod shiftsien;
#[doc = "Shifter Error Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shifteien](shifteien) module"]
pub type SHIFTEIEN = crate::Reg<u32, _SHIFTEIEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTEIEN;
#[doc = "`read()` method returns [shifteien::R](shifteien::R) reader structure"]
impl crate::Readable for SHIFTEIEN {}
#[doc = "`write(|w| ..)` method takes [shifteien::W](shifteien::W) writer structure"]
impl crate::Writable for SHIFTEIEN {}
#[doc = "Shifter Error Interrupt Enable"]
pub mod shifteien;
#[doc = "Timer Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timien](timien) module"]
pub type TIMIEN = crate::Reg<u32, _TIMIEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMIEN;
#[doc = "`read()` method returns [timien::R](timien::R) reader structure"]
impl crate::Readable for TIMIEN {}
#[doc = "`write(|w| ..)` method takes [timien::W](timien::W) writer structure"]
impl crate::Writable for TIMIEN {}
#[doc = "Timer Interrupt Enable Register"]
pub mod timien;
#[doc = "Shifter Status DMA Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftsden](shiftsden) module"]
pub type SHIFTSDEN = crate::Reg<u32, _SHIFTSDEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTSDEN;
#[doc = "`read()` method returns [shiftsden::R](shiftsden::R) reader structure"]
impl crate::Readable for SHIFTSDEN {}
#[doc = "`write(|w| ..)` method takes [shiftsden::W](shiftsden::W) writer structure"]
impl crate::Writable for SHIFTSDEN {}
#[doc = "Shifter Status DMA Enable"]
pub mod shiftsden;
#[doc = "Shifter State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftstate](shiftstate) module"]
pub type SHIFTSTATE = crate::Reg<u32, _SHIFTSTATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTSTATE;
#[doc = "`read()` method returns [shiftstate::R](shiftstate::R) reader structure"]
impl crate::Readable for SHIFTSTATE {}
#[doc = "`write(|w| ..)` method takes [shiftstate::W](shiftstate::W) writer structure"]
impl crate::Writable for SHIFTSTATE {}
#[doc = "Shifter State Register"]
pub mod shiftstate;
#[doc = "Shifter Control N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftctl](shiftctl) module"]
pub type SHIFTCTL = crate::Reg<u32, _SHIFTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTCTL;
#[doc = "`read()` method returns [shiftctl::R](shiftctl::R) reader structure"]
impl crate::Readable for SHIFTCTL {}
#[doc = "`write(|w| ..)` method takes [shiftctl::W](shiftctl::W) writer structure"]
impl crate::Writable for SHIFTCTL {}
#[doc = "Shifter Control N Register"]
pub mod shiftctl;
#[doc = "Shifter Configuration N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftcfg](shiftcfg) module"]
pub type SHIFTCFG = crate::Reg<u32, _SHIFTCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTCFG;
#[doc = "`read()` method returns [shiftcfg::R](shiftcfg::R) reader structure"]
impl crate::Readable for SHIFTCFG {}
#[doc = "`write(|w| ..)` method takes [shiftcfg::W](shiftcfg::W) writer structure"]
impl crate::Writable for SHIFTCFG {}
#[doc = "Shifter Configuration N Register"]
pub mod shiftcfg;
#[doc = "Shifter Buffer N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbuf](shiftbuf) module"]
pub type SHIFTBUF = crate::Reg<u32, _SHIFTBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTBUF;
#[doc = "`read()` method returns [shiftbuf::R](shiftbuf::R) reader structure"]
impl crate::Readable for SHIFTBUF {}
#[doc = "`write(|w| ..)` method takes [shiftbuf::W](shiftbuf::W) writer structure"]
impl crate::Writable for SHIFTBUF {}
#[doc = "Shifter Buffer N Register"]
pub mod shiftbuf;
#[doc = "Shifter Buffer N Bit Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufbis](shiftbufbis) module"]
pub type SHIFTBUFBIS = crate::Reg<u32, _SHIFTBUFBIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTBUFBIS;
#[doc = "`read()` method returns [shiftbufbis::R](shiftbufbis::R) reader structure"]
impl crate::Readable for SHIFTBUFBIS {}
#[doc = "`write(|w| ..)` method takes [shiftbufbis::W](shiftbufbis::W) writer structure"]
impl crate::Writable for SHIFTBUFBIS {}
#[doc = "Shifter Buffer N Bit Swapped Register"]
pub mod shiftbufbis;
#[doc = "Shifter Buffer N Byte Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufbys](shiftbufbys) module"]
pub type SHIFTBUFBYS = crate::Reg<u32, _SHIFTBUFBYS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTBUFBYS;
#[doc = "`read()` method returns [shiftbufbys::R](shiftbufbys::R) reader structure"]
impl crate::Readable for SHIFTBUFBYS {}
#[doc = "`write(|w| ..)` method takes [shiftbufbys::W](shiftbufbys::W) writer structure"]
impl crate::Writable for SHIFTBUFBYS {}
#[doc = "Shifter Buffer N Byte Swapped Register"]
pub mod shiftbufbys;
#[doc = "Shifter Buffer N Bit Byte Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufbbs](shiftbufbbs) module"]
pub type SHIFTBUFBBS = crate::Reg<u32, _SHIFTBUFBBS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTBUFBBS;
#[doc = "`read()` method returns [shiftbufbbs::R](shiftbufbbs::R) reader structure"]
impl crate::Readable for SHIFTBUFBBS {}
#[doc = "`write(|w| ..)` method takes [shiftbufbbs::W](shiftbufbbs::W) writer structure"]
impl crate::Writable for SHIFTBUFBBS {}
#[doc = "Shifter Buffer N Bit Byte Swapped Register"]
pub mod shiftbufbbs;
#[doc = "Timer Control N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timctl](timctl) module"]
pub type TIMCTL = crate::Reg<u32, _TIMCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCTL;
#[doc = "`read()` method returns [timctl::R](timctl::R) reader structure"]
impl crate::Readable for TIMCTL {}
#[doc = "`write(|w| ..)` method takes [timctl::W](timctl::W) writer structure"]
impl crate::Writable for TIMCTL {}
#[doc = "Timer Control N Register"]
pub mod timctl;
#[doc = "Timer Configuration N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timcfg](timcfg) module"]
pub type TIMCFG = crate::Reg<u32, _TIMCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCFG;
#[doc = "`read()` method returns [timcfg::R](timcfg::R) reader structure"]
impl crate::Readable for TIMCFG {}
#[doc = "`write(|w| ..)` method takes [timcfg::W](timcfg::W) writer structure"]
impl crate::Writable for TIMCFG {}
#[doc = "Timer Configuration N Register"]
pub mod timcfg;
#[doc = "Timer Compare N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timcmp](timcmp) module"]
pub type TIMCMP = crate::Reg<u32, _TIMCMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCMP;
#[doc = "`read()` method returns [timcmp::R](timcmp::R) reader structure"]
impl crate::Readable for TIMCMP {}
#[doc = "`write(|w| ..)` method takes [timcmp::W](timcmp::W) writer structure"]
impl crate::Writable for TIMCMP {}
#[doc = "Timer Compare N Register"]
pub mod timcmp;
#[doc = "Shifter Buffer N Nibble Byte Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufnbs](shiftbufnbs) module"]
pub type SHIFTBUFNBS = crate::Reg<u32, _SHIFTBUFNBS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTBUFNBS;
#[doc = "`read()` method returns [shiftbufnbs::R](shiftbufnbs::R) reader structure"]
impl crate::Readable for SHIFTBUFNBS {}
#[doc = "`write(|w| ..)` method takes [shiftbufnbs::W](shiftbufnbs::W) writer structure"]
impl crate::Writable for SHIFTBUFNBS {}
#[doc = "Shifter Buffer N Nibble Byte Swapped Register"]
pub mod shiftbufnbs;
#[doc = "Shifter Buffer N Half Word Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufhws](shiftbufhws) module"]
pub type SHIFTBUFHWS = crate::Reg<u32, _SHIFTBUFHWS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTBUFHWS;
#[doc = "`read()` method returns [shiftbufhws::R](shiftbufhws::R) reader structure"]
impl crate::Readable for SHIFTBUFHWS {}
#[doc = "`write(|w| ..)` method takes [shiftbufhws::W](shiftbufhws::W) writer structure"]
impl crate::Writable for SHIFTBUFHWS {}
#[doc = "Shifter Buffer N Half Word Swapped Register"]
pub mod shiftbufhws;
#[doc = "Shifter Buffer N Nibble Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufnis](shiftbufnis) module"]
pub type SHIFTBUFNIS = crate::Reg<u32, _SHIFTBUFNIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTBUFNIS;
#[doc = "`read()` method returns [shiftbufnis::R](shiftbufnis::R) reader structure"]
impl crate::Readable for SHIFTBUFNIS {}
#[doc = "`write(|w| ..)` method takes [shiftbufnis::W](shiftbufnis::W) writer structure"]
impl crate::Writable for SHIFTBUFNIS {}
#[doc = "Shifter Buffer N Nibble Swapped Register"]
pub mod shiftbufnis;
