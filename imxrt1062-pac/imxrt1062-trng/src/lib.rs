#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Miscellaneous Control Register"]
    pub mctl: MCTL,
    #[doc = "0x04 - Statistical Check Miscellaneous Register"]
    pub scmisc: SCMISC,
    #[doc = "0x08 - Poker Range Register"]
    pub pkrrng: PKRRNG,
    _reserved_3_pkrsq: [u8; 4usize],
    #[doc = "0x10 - Seed Control Register"]
    pub sdctl: SDCTL,
    _reserved_5_sblim: [u8; 4usize],
    #[doc = "0x18 - Frequency Count Minimum Limit Register"]
    pub frqmin: FRQMIN,
    _reserved_7_frqcnt: [u8; 4usize],
    _reserved_8_scmc: [u8; 4usize],
    _reserved_9_scr: [u8; 4usize],
    _reserved_10_scr: [u8; 4usize],
    _reserved_11_scr: [u8; 4usize],
    _reserved_12_scr: [u8; 4usize],
    _reserved_13_scr: [u8; 4usize],
    _reserved_14_scr: [u8; 4usize],
    #[doc = "0x3c - Status Register"]
    pub status: STATUS,
    #[doc = "0x40 - Entropy Read Register"]
    pub ent: [ENT; 16],
    #[doc = "0x80 - Statistical Check Poker Count 1 and 0 Register"]
    pub pkrcnt10: PKRCNT10,
    #[doc = "0x84 - Statistical Check Poker Count 3 and 2 Register"]
    pub pkrcnt32: PKRCNT32,
    #[doc = "0x88 - Statistical Check Poker Count 5 and 4 Register"]
    pub pkrcnt54: PKRCNT54,
    #[doc = "0x8c - Statistical Check Poker Count 7 and 6 Register"]
    pub pkrcnt76: PKRCNT76,
    #[doc = "0x90 - Statistical Check Poker Count 9 and 8 Register"]
    pub pkrcnt98: PKRCNT98,
    #[doc = "0x94 - Statistical Check Poker Count B and A Register"]
    pub pkrcntba: PKRCNTBA,
    #[doc = "0x98 - Statistical Check Poker Count D and C Register"]
    pub pkrcntdc: PKRCNTDC,
    #[doc = "0x9c - Statistical Check Poker Count F and E Register"]
    pub pkrcntfe: PKRCNTFE,
    #[doc = "0xa0 - Security Configuration Register"]
    pub sec_cfg: SEC_CFG,
    #[doc = "0xa4 - Interrupt Control Register"]
    pub int_ctrl: INT_CTRL,
    #[doc = "0xa8 - Mask Register"]
    pub int_mask: INT_MASK,
    #[doc = "0xac - Interrupt Status Register"]
    pub int_status: INT_STATUS,
    _reserved29: [u8; 64usize],
    #[doc = "0xf0 - Version ID Register (MS)"]
    pub vid1: VID1,
    #[doc = "0xf4 - Version ID Register (LS)"]
    pub vid2: VID2,
}
impl RegisterBlock {
    #[doc = "0x0c - Poker Square Calculation Result Register"]
    #[inline(always)]
    pub fn pkrsq(&self) -> &PKRSQ {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const PKRSQ) }
    }
    #[doc = "0x0c - Poker Square Calculation Result Register"]
    #[inline(always)]
    pub fn pkrsq_mut(&self) -> &mut PKRSQ {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut PKRSQ) }
    }
    #[doc = "0x0c - Poker Maximum Limit Register"]
    #[inline(always)]
    pub fn pkrmax(&self) -> &PKRMAX {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const PKRMAX) }
    }
    #[doc = "0x0c - Poker Maximum Limit Register"]
    #[inline(always)]
    pub fn pkrmax_mut(&self) -> &mut PKRMAX {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut PKRMAX) }
    }
    #[doc = "0x14 - Total Samples Register"]
    #[inline(always)]
    pub fn totsam(&self) -> &TOTSAM {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const TOTSAM) }
    }
    #[doc = "0x14 - Total Samples Register"]
    #[inline(always)]
    pub fn totsam_mut(&self) -> &mut TOTSAM {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(20usize) as *mut TOTSAM) }
    }
    #[doc = "0x14 - Sparse Bit Limit Register"]
    #[inline(always)]
    pub fn sblim(&self) -> &SBLIM {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const SBLIM) }
    }
    #[doc = "0x14 - Sparse Bit Limit Register"]
    #[inline(always)]
    pub fn sblim_mut(&self) -> &mut SBLIM {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(20usize) as *mut SBLIM) }
    }
    #[doc = "0x1c - Frequency Count Maximum Limit Register"]
    #[inline(always)]
    pub fn frqmax(&self) -> &FRQMAX {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const FRQMAX) }
    }
    #[doc = "0x1c - Frequency Count Maximum Limit Register"]
    #[inline(always)]
    pub fn frqmax_mut(&self) -> &mut FRQMAX {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut FRQMAX) }
    }
    #[doc = "0x1c - Frequency Count Register"]
    #[inline(always)]
    pub fn frqcnt(&self) -> &FRQCNT {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const FRQCNT) }
    }
    #[doc = "0x1c - Frequency Count Register"]
    #[inline(always)]
    pub fn frqcnt_mut(&self) -> &mut FRQCNT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut FRQCNT) }
    }
    #[doc = "0x20 - Statistical Check Monobit Limit Register"]
    #[inline(always)]
    pub fn scml(&self) -> &SCML {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const SCML) }
    }
    #[doc = "0x20 - Statistical Check Monobit Limit Register"]
    #[inline(always)]
    pub fn scml_mut(&self) -> &mut SCML {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(32usize) as *mut SCML) }
    }
    #[doc = "0x20 - Statistical Check Monobit Count Register"]
    #[inline(always)]
    pub fn scmc(&self) -> &SCMC {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const SCMC) }
    }
    #[doc = "0x20 - Statistical Check Monobit Count Register"]
    #[inline(always)]
    pub fn scmc_mut(&self) -> &mut SCMC {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(32usize) as *mut SCMC) }
    }
    #[doc = "0x24 - Statistical Check Run Length 1 Limit Register"]
    #[inline(always)]
    pub fn scr1l(&self) -> &SCR1L {
        unsafe { &*(((self as *const Self) as *const u8).add(36usize) as *const SCR1L) }
    }
    #[doc = "0x24 - Statistical Check Run Length 1 Limit Register"]
    #[inline(always)]
    pub fn scr1l_mut(&self) -> &mut SCR1L {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(36usize) as *mut SCR1L) }
    }
    #[doc = "0x24 - Statistical Check Run Length 1 Count Register"]
    #[inline(always)]
    pub fn scr1c(&self) -> &SCR1C {
        unsafe { &*(((self as *const Self) as *const u8).add(36usize) as *const SCR1C) }
    }
    #[doc = "0x24 - Statistical Check Run Length 1 Count Register"]
    #[inline(always)]
    pub fn scr1c_mut(&self) -> &mut SCR1C {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(36usize) as *mut SCR1C) }
    }
    #[doc = "0x28 - Statistical Check Run Length 2 Limit Register"]
    #[inline(always)]
    pub fn scr2l(&self) -> &SCR2L {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const SCR2L) }
    }
    #[doc = "0x28 - Statistical Check Run Length 2 Limit Register"]
    #[inline(always)]
    pub fn scr2l_mut(&self) -> &mut SCR2L {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut SCR2L) }
    }
    #[doc = "0x28 - Statistical Check Run Length 2 Count Register"]
    #[inline(always)]
    pub fn scr2c(&self) -> &SCR2C {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const SCR2C) }
    }
    #[doc = "0x28 - Statistical Check Run Length 2 Count Register"]
    #[inline(always)]
    pub fn scr2c_mut(&self) -> &mut SCR2C {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut SCR2C) }
    }
    #[doc = "0x2c - Statistical Check Run Length 3 Limit Register"]
    #[inline(always)]
    pub fn scr3l(&self) -> &SCR3L {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const SCR3L) }
    }
    #[doc = "0x2c - Statistical Check Run Length 3 Limit Register"]
    #[inline(always)]
    pub fn scr3l_mut(&self) -> &mut SCR3L {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(44usize) as *mut SCR3L) }
    }
    #[doc = "0x2c - Statistical Check Run Length 3 Count Register"]
    #[inline(always)]
    pub fn scr3c(&self) -> &SCR3C {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const SCR3C) }
    }
    #[doc = "0x2c - Statistical Check Run Length 3 Count Register"]
    #[inline(always)]
    pub fn scr3c_mut(&self) -> &mut SCR3C {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(44usize) as *mut SCR3C) }
    }
    #[doc = "0x30 - Statistical Check Run Length 4 Limit Register"]
    #[inline(always)]
    pub fn scr4l(&self) -> &SCR4L {
        unsafe { &*(((self as *const Self) as *const u8).add(48usize) as *const SCR4L) }
    }
    #[doc = "0x30 - Statistical Check Run Length 4 Limit Register"]
    #[inline(always)]
    pub fn scr4l_mut(&self) -> &mut SCR4L {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(48usize) as *mut SCR4L) }
    }
    #[doc = "0x30 - Statistical Check Run Length 4 Count Register"]
    #[inline(always)]
    pub fn scr4c(&self) -> &SCR4C {
        unsafe { &*(((self as *const Self) as *const u8).add(48usize) as *const SCR4C) }
    }
    #[doc = "0x30 - Statistical Check Run Length 4 Count Register"]
    #[inline(always)]
    pub fn scr4c_mut(&self) -> &mut SCR4C {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(48usize) as *mut SCR4C) }
    }
    #[doc = "0x34 - Statistical Check Run Length 5 Limit Register"]
    #[inline(always)]
    pub fn scr5l(&self) -> &SCR5L {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const SCR5L) }
    }
    #[doc = "0x34 - Statistical Check Run Length 5 Limit Register"]
    #[inline(always)]
    pub fn scr5l_mut(&self) -> &mut SCR5L {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(52usize) as *mut SCR5L) }
    }
    #[doc = "0x34 - Statistical Check Run Length 5 Count Register"]
    #[inline(always)]
    pub fn scr5c(&self) -> &SCR5C {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const SCR5C) }
    }
    #[doc = "0x34 - Statistical Check Run Length 5 Count Register"]
    #[inline(always)]
    pub fn scr5c_mut(&self) -> &mut SCR5C {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(52usize) as *mut SCR5C) }
    }
    #[doc = "0x38 - Statistical Check Run Length 6+ Limit Register"]
    #[inline(always)]
    pub fn scr6pl(&self) -> &SCR6PL {
        unsafe { &*(((self as *const Self) as *const u8).add(56usize) as *const SCR6PL) }
    }
    #[doc = "0x38 - Statistical Check Run Length 6+ Limit Register"]
    #[inline(always)]
    pub fn scr6pl_mut(&self) -> &mut SCR6PL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(56usize) as *mut SCR6PL) }
    }
    #[doc = "0x38 - Statistical Check Run Length 6+ Count Register"]
    #[inline(always)]
    pub fn scr6pc(&self) -> &SCR6PC {
        unsafe { &*(((self as *const Self) as *const u8).add(56usize) as *const SCR6PC) }
    }
    #[doc = "0x38 - Statistical Check Run Length 6+ Count Register"]
    #[inline(always)]
    pub fn scr6pc_mut(&self) -> &mut SCR6PC {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(56usize) as *mut SCR6PC) }
    }
}
#[doc = "Miscellaneous Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctl](mctl) module"]
pub type MCTL = crate::Reg<u32, _MCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTL;
#[doc = "`read()` method returns [mctl::R](mctl::R) reader structure"]
impl crate::Readable for MCTL {}
#[doc = "`write(|w| ..)` method takes [mctl::W](mctl::W) writer structure"]
impl crate::Writable for MCTL {}
#[doc = "Miscellaneous Control Register"]
pub mod mctl;
#[doc = "Statistical Check Miscellaneous Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmisc](scmisc) module"]
pub type SCMISC = crate::Reg<u32, _SCMISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCMISC;
#[doc = "`read()` method returns [scmisc::R](scmisc::R) reader structure"]
impl crate::Readable for SCMISC {}
#[doc = "`write(|w| ..)` method takes [scmisc::W](scmisc::W) writer structure"]
impl crate::Writable for SCMISC {}
#[doc = "Statistical Check Miscellaneous Register"]
pub mod scmisc;
#[doc = "Poker Range Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkrrng](pkrrng) module"]
pub type PKRRNG = crate::Reg<u32, _PKRRNG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKRRNG;
#[doc = "`read()` method returns [pkrrng::R](pkrrng::R) reader structure"]
impl crate::Readable for PKRRNG {}
#[doc = "`write(|w| ..)` method takes [pkrrng::W](pkrrng::W) writer structure"]
impl crate::Writable for PKRRNG {}
#[doc = "Poker Range Register"]
pub mod pkrrng;
#[doc = "Poker Maximum Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkrmax](pkrmax) module"]
pub type PKRMAX = crate::Reg<u32, _PKRMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKRMAX;
#[doc = "`read()` method returns [pkrmax::R](pkrmax::R) reader structure"]
impl crate::Readable for PKRMAX {}
#[doc = "`write(|w| ..)` method takes [pkrmax::W](pkrmax::W) writer structure"]
impl crate::Writable for PKRMAX {}
#[doc = "Poker Maximum Limit Register"]
pub mod pkrmax;
#[doc = "Poker Square Calculation Result Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkrsq](pkrsq) module"]
pub type PKRSQ = crate::Reg<u32, _PKRSQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKRSQ;
#[doc = "`read()` method returns [pkrsq::R](pkrsq::R) reader structure"]
impl crate::Readable for PKRSQ {}
#[doc = "Poker Square Calculation Result Register"]
pub mod pkrsq;
#[doc = "Seed Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdctl](sdctl) module"]
pub type SDCTL = crate::Reg<u32, _SDCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDCTL;
#[doc = "`read()` method returns [sdctl::R](sdctl::R) reader structure"]
impl crate::Readable for SDCTL {}
#[doc = "`write(|w| ..)` method takes [sdctl::W](sdctl::W) writer structure"]
impl crate::Writable for SDCTL {}
#[doc = "Seed Control Register"]
pub mod sdctl;
#[doc = "Sparse Bit Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sblim](sblim) module"]
pub type SBLIM = crate::Reg<u32, _SBLIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBLIM;
#[doc = "`read()` method returns [sblim::R](sblim::R) reader structure"]
impl crate::Readable for SBLIM {}
#[doc = "`write(|w| ..)` method takes [sblim::W](sblim::W) writer structure"]
impl crate::Writable for SBLIM {}
#[doc = "Sparse Bit Limit Register"]
pub mod sblim;
#[doc = "Total Samples Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [totsam](totsam) module"]
pub type TOTSAM = crate::Reg<u32, _TOTSAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOTSAM;
#[doc = "`read()` method returns [totsam::R](totsam::R) reader structure"]
impl crate::Readable for TOTSAM {}
#[doc = "Total Samples Register"]
pub mod totsam;
#[doc = "Frequency Count Minimum Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frqmin](frqmin) module"]
pub type FRQMIN = crate::Reg<u32, _FRQMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRQMIN;
#[doc = "`read()` method returns [frqmin::R](frqmin::R) reader structure"]
impl crate::Readable for FRQMIN {}
#[doc = "`write(|w| ..)` method takes [frqmin::W](frqmin::W) writer structure"]
impl crate::Writable for FRQMIN {}
#[doc = "Frequency Count Minimum Limit Register"]
pub mod frqmin;
#[doc = "Frequency Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frqcnt](frqcnt) module"]
pub type FRQCNT = crate::Reg<u32, _FRQCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRQCNT;
#[doc = "`read()` method returns [frqcnt::R](frqcnt::R) reader structure"]
impl crate::Readable for FRQCNT {}
#[doc = "Frequency Count Register"]
pub mod frqcnt;
#[doc = "Frequency Count Maximum Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frqmax](frqmax) module"]
pub type FRQMAX = crate::Reg<u32, _FRQMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRQMAX;
#[doc = "`read()` method returns [frqmax::R](frqmax::R) reader structure"]
impl crate::Readable for FRQMAX {}
#[doc = "`write(|w| ..)` method takes [frqmax::W](frqmax::W) writer structure"]
impl crate::Writable for FRQMAX {}
#[doc = "Frequency Count Maximum Limit Register"]
pub mod frqmax;
#[doc = "Statistical Check Monobit Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmc](scmc) module"]
pub type SCMC = crate::Reg<u32, _SCMC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCMC;
#[doc = "`read()` method returns [scmc::R](scmc::R) reader structure"]
impl crate::Readable for SCMC {}
#[doc = "Statistical Check Monobit Count Register"]
pub mod scmc;
#[doc = "Statistical Check Monobit Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scml](scml) module"]
pub type SCML = crate::Reg<u32, _SCML>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCML;
#[doc = "`read()` method returns [scml::R](scml::R) reader structure"]
impl crate::Readable for SCML {}
#[doc = "`write(|w| ..)` method takes [scml::W](scml::W) writer structure"]
impl crate::Writable for SCML {}
#[doc = "Statistical Check Monobit Limit Register"]
pub mod scml;
#[doc = "Statistical Check Run Length 1 Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr1c](scr1c) module"]
pub type SCR1C = crate::Reg<u32, _SCR1C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR1C;
#[doc = "`read()` method returns [scr1c::R](scr1c::R) reader structure"]
impl crate::Readable for SCR1C {}
#[doc = "Statistical Check Run Length 1 Count Register"]
pub mod scr1c;
#[doc = "Statistical Check Run Length 1 Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr1l](scr1l) module"]
pub type SCR1L = crate::Reg<u32, _SCR1L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR1L;
#[doc = "`read()` method returns [scr1l::R](scr1l::R) reader structure"]
impl crate::Readable for SCR1L {}
#[doc = "`write(|w| ..)` method takes [scr1l::W](scr1l::W) writer structure"]
impl crate::Writable for SCR1L {}
#[doc = "Statistical Check Run Length 1 Limit Register"]
pub mod scr1l;
#[doc = "Statistical Check Run Length 2 Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr2c](scr2c) module"]
pub type SCR2C = crate::Reg<u32, _SCR2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR2C;
#[doc = "`read()` method returns [scr2c::R](scr2c::R) reader structure"]
impl crate::Readable for SCR2C {}
#[doc = "Statistical Check Run Length 2 Count Register"]
pub mod scr2c;
#[doc = "Statistical Check Run Length 2 Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr2l](scr2l) module"]
pub type SCR2L = crate::Reg<u32, _SCR2L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR2L;
#[doc = "`read()` method returns [scr2l::R](scr2l::R) reader structure"]
impl crate::Readable for SCR2L {}
#[doc = "`write(|w| ..)` method takes [scr2l::W](scr2l::W) writer structure"]
impl crate::Writable for SCR2L {}
#[doc = "Statistical Check Run Length 2 Limit Register"]
pub mod scr2l;
#[doc = "Statistical Check Run Length 3 Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr3c](scr3c) module"]
pub type SCR3C = crate::Reg<u32, _SCR3C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR3C;
#[doc = "`read()` method returns [scr3c::R](scr3c::R) reader structure"]
impl crate::Readable for SCR3C {}
#[doc = "Statistical Check Run Length 3 Count Register"]
pub mod scr3c;
#[doc = "Statistical Check Run Length 3 Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr3l](scr3l) module"]
pub type SCR3L = crate::Reg<u32, _SCR3L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR3L;
#[doc = "`read()` method returns [scr3l::R](scr3l::R) reader structure"]
impl crate::Readable for SCR3L {}
#[doc = "`write(|w| ..)` method takes [scr3l::W](scr3l::W) writer structure"]
impl crate::Writable for SCR3L {}
#[doc = "Statistical Check Run Length 3 Limit Register"]
pub mod scr3l;
#[doc = "Statistical Check Run Length 4 Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr4c](scr4c) module"]
pub type SCR4C = crate::Reg<u32, _SCR4C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR4C;
#[doc = "`read()` method returns [scr4c::R](scr4c::R) reader structure"]
impl crate::Readable for SCR4C {}
#[doc = "Statistical Check Run Length 4 Count Register"]
pub mod scr4c;
#[doc = "Statistical Check Run Length 4 Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr4l](scr4l) module"]
pub type SCR4L = crate::Reg<u32, _SCR4L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR4L;
#[doc = "`read()` method returns [scr4l::R](scr4l::R) reader structure"]
impl crate::Readable for SCR4L {}
#[doc = "`write(|w| ..)` method takes [scr4l::W](scr4l::W) writer structure"]
impl crate::Writable for SCR4L {}
#[doc = "Statistical Check Run Length 4 Limit Register"]
pub mod scr4l;
#[doc = "Statistical Check Run Length 5 Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr5c](scr5c) module"]
pub type SCR5C = crate::Reg<u32, _SCR5C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR5C;
#[doc = "`read()` method returns [scr5c::R](scr5c::R) reader structure"]
impl crate::Readable for SCR5C {}
#[doc = "Statistical Check Run Length 5 Count Register"]
pub mod scr5c;
#[doc = "Statistical Check Run Length 5 Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr5l](scr5l) module"]
pub type SCR5L = crate::Reg<u32, _SCR5L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR5L;
#[doc = "`read()` method returns [scr5l::R](scr5l::R) reader structure"]
impl crate::Readable for SCR5L {}
#[doc = "`write(|w| ..)` method takes [scr5l::W](scr5l::W) writer structure"]
impl crate::Writable for SCR5L {}
#[doc = "Statistical Check Run Length 5 Limit Register"]
pub mod scr5l;
#[doc = "Statistical Check Run Length 6+ Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr6pc](scr6pc) module"]
pub type SCR6PC = crate::Reg<u32, _SCR6PC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR6PC;
#[doc = "`read()` method returns [scr6pc::R](scr6pc::R) reader structure"]
impl crate::Readable for SCR6PC {}
#[doc = "Statistical Check Run Length 6+ Count Register"]
pub mod scr6pc;
#[doc = "Statistical Check Run Length 6+ Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr6pl](scr6pl) module"]
pub type SCR6PL = crate::Reg<u32, _SCR6PL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR6PL;
#[doc = "`read()` method returns [scr6pl::R](scr6pl::R) reader structure"]
impl crate::Readable for SCR6PL {}
#[doc = "`write(|w| ..)` method takes [scr6pl::W](scr6pl::W) writer structure"]
impl crate::Writable for SCR6PL {}
#[doc = "Statistical Check Run Length 6+ Limit Register"]
pub mod scr6pl;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status Register"]
pub mod status;
#[doc = "Entropy Read Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ent](ent) module"]
pub type ENT = crate::Reg<u32, _ENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENT;
#[doc = "`read()` method returns [ent::R](ent::R) reader structure"]
impl crate::Readable for ENT {}
#[doc = "Entropy Read Register"]
pub mod ent;
#[doc = "Statistical Check Poker Count 1 and 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkrcnt10](pkrcnt10) module"]
pub type PKRCNT10 = crate::Reg<u32, _PKRCNT10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKRCNT10;
#[doc = "`read()` method returns [pkrcnt10::R](pkrcnt10::R) reader structure"]
impl crate::Readable for PKRCNT10 {}
#[doc = "Statistical Check Poker Count 1 and 0 Register"]
pub mod pkrcnt10;
#[doc = "Statistical Check Poker Count 3 and 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkrcnt32](pkrcnt32) module"]
pub type PKRCNT32 = crate::Reg<u32, _PKRCNT32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKRCNT32;
#[doc = "`read()` method returns [pkrcnt32::R](pkrcnt32::R) reader structure"]
impl crate::Readable for PKRCNT32 {}
#[doc = "Statistical Check Poker Count 3 and 2 Register"]
pub mod pkrcnt32;
#[doc = "Statistical Check Poker Count 5 and 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkrcnt54](pkrcnt54) module"]
pub type PKRCNT54 = crate::Reg<u32, _PKRCNT54>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKRCNT54;
#[doc = "`read()` method returns [pkrcnt54::R](pkrcnt54::R) reader structure"]
impl crate::Readable for PKRCNT54 {}
#[doc = "Statistical Check Poker Count 5 and 4 Register"]
pub mod pkrcnt54;
#[doc = "Statistical Check Poker Count 7 and 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkrcnt76](pkrcnt76) module"]
pub type PKRCNT76 = crate::Reg<u32, _PKRCNT76>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKRCNT76;
#[doc = "`read()` method returns [pkrcnt76::R](pkrcnt76::R) reader structure"]
impl crate::Readable for PKRCNT76 {}
#[doc = "Statistical Check Poker Count 7 and 6 Register"]
pub mod pkrcnt76;
#[doc = "Statistical Check Poker Count 9 and 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkrcnt98](pkrcnt98) module"]
pub type PKRCNT98 = crate::Reg<u32, _PKRCNT98>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKRCNT98;
#[doc = "`read()` method returns [pkrcnt98::R](pkrcnt98::R) reader structure"]
impl crate::Readable for PKRCNT98 {}
#[doc = "Statistical Check Poker Count 9 and 8 Register"]
pub mod pkrcnt98;
#[doc = "Statistical Check Poker Count B and A Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkrcntba](pkrcntba) module"]
pub type PKRCNTBA = crate::Reg<u32, _PKRCNTBA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKRCNTBA;
#[doc = "`read()` method returns [pkrcntba::R](pkrcntba::R) reader structure"]
impl crate::Readable for PKRCNTBA {}
#[doc = "Statistical Check Poker Count B and A Register"]
pub mod pkrcntba;
#[doc = "Statistical Check Poker Count D and C Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkrcntdc](pkrcntdc) module"]
pub type PKRCNTDC = crate::Reg<u32, _PKRCNTDC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKRCNTDC;
#[doc = "`read()` method returns [pkrcntdc::R](pkrcntdc::R) reader structure"]
impl crate::Readable for PKRCNTDC {}
#[doc = "Statistical Check Poker Count D and C Register"]
pub mod pkrcntdc;
#[doc = "Statistical Check Poker Count F and E Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkrcntfe](pkrcntfe) module"]
pub type PKRCNTFE = crate::Reg<u32, _PKRCNTFE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PKRCNTFE;
#[doc = "`read()` method returns [pkrcntfe::R](pkrcntfe::R) reader structure"]
impl crate::Readable for PKRCNTFE {}
#[doc = "Statistical Check Poker Count F and E Register"]
pub mod pkrcntfe;
#[doc = "Security Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_cfg](sec_cfg) module"]
pub type SEC_CFG = crate::Reg<u32, _SEC_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC_CFG;
#[doc = "`read()` method returns [sec_cfg::R](sec_cfg::R) reader structure"]
impl crate::Readable for SEC_CFG {}
#[doc = "`write(|w| ..)` method takes [sec_cfg::W](sec_cfg::W) writer structure"]
impl crate::Writable for SEC_CFG {}
#[doc = "Security Configuration Register"]
pub mod sec_cfg;
#[doc = "Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ctrl](int_ctrl) module"]
pub type INT_CTRL = crate::Reg<u32, _INT_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_CTRL;
#[doc = "`read()` method returns [int_ctrl::R](int_ctrl::R) reader structure"]
impl crate::Readable for INT_CTRL {}
#[doc = "`write(|w| ..)` method takes [int_ctrl::W](int_ctrl::W) writer structure"]
impl crate::Writable for INT_CTRL {}
#[doc = "Interrupt Control Register"]
pub mod int_ctrl;
#[doc = "Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_mask](int_mask) module"]
pub type INT_MASK = crate::Reg<u32, _INT_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_MASK;
#[doc = "`read()` method returns [int_mask::R](int_mask::R) reader structure"]
impl crate::Readable for INT_MASK {}
#[doc = "`write(|w| ..)` method takes [int_mask::W](int_mask::W) writer structure"]
impl crate::Writable for INT_MASK {}
#[doc = "Mask Register"]
pub mod int_mask;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_status](int_status) module"]
pub type INT_STATUS = crate::Reg<u32, _INT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_STATUS;
#[doc = "`read()` method returns [int_status::R](int_status::R) reader structure"]
impl crate::Readable for INT_STATUS {}
#[doc = "Interrupt Status Register"]
pub mod int_status;
#[doc = "Version ID Register (MS)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vid1](vid1) module"]
pub type VID1 = crate::Reg<u32, _VID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VID1;
#[doc = "`read()` method returns [vid1::R](vid1::R) reader structure"]
impl crate::Readable for VID1 {}
#[doc = "Version ID Register (MS)"]
pub mod vid1;
#[doc = "Version ID Register (LS)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vid2](vid2) module"]
pub type VID2 = crate::Reg<u32, _VID2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VID2;
#[doc = "`read()` method returns [vid2::R](vid2::R) reader structure"]
impl crate::Readable for VID2 {}
#[doc = "Version ID Register (LS)"]
pub mod vid2;
