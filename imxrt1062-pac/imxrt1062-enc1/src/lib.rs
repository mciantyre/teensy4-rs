#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x02 - Input Filter Register"]
    pub filt: FILT,
    #[doc = "0x04 - Watchdog Timeout Register"]
    pub wtr: WTR,
    #[doc = "0x06 - Position Difference Counter Register"]
    pub posd: POSD,
    #[doc = "0x08 - Position Difference Hold Register"]
    pub posdh: POSDH,
    #[doc = "0x0a - Revolution Counter Register"]
    pub rev: REV,
    #[doc = "0x0c - Revolution Hold Register"]
    pub revh: REVH,
    #[doc = "0x0e - Upper Position Counter Register"]
    pub upos: UPOS,
    #[doc = "0x10 - Lower Position Counter Register"]
    pub lpos: LPOS,
    #[doc = "0x12 - Upper Position Hold Register"]
    pub uposh: UPOSH,
    #[doc = "0x14 - Lower Position Hold Register"]
    pub lposh: LPOSH,
    #[doc = "0x16 - Upper Initialization Register"]
    pub uinit: UINIT,
    #[doc = "0x18 - Lower Initialization Register"]
    pub linit: LINIT,
    #[doc = "0x1a - Input Monitor Register"]
    pub imr: IMR,
    #[doc = "0x1c - Test Register"]
    pub tst: TST,
    #[doc = "0x1e - Control 2 Register"]
    pub ctrl2: CTRL2,
    #[doc = "0x20 - Upper Modulus Register"]
    pub umod: UMOD,
    #[doc = "0x22 - Lower Modulus Register"]
    pub lmod: LMOD,
    #[doc = "0x24 - Upper Position Compare Register"]
    pub ucomp: UCOMP,
    #[doc = "0x26 - Lower Position Compare Register"]
    pub lcomp: LCOMP,
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u16, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Input Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filt](filt) module"]
pub type FILT = crate::Reg<u16, _FILT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FILT;
#[doc = "`read()` method returns [filt::R](filt::R) reader structure"]
impl crate::Readable for FILT {}
#[doc = "`write(|w| ..)` method takes [filt::W](filt::W) writer structure"]
impl crate::Writable for FILT {}
#[doc = "Input Filter Register"]
pub mod filt;
#[doc = "Watchdog Timeout Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtr](wtr) module"]
pub type WTR = crate::Reg<u16, _WTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WTR;
#[doc = "`read()` method returns [wtr::R](wtr::R) reader structure"]
impl crate::Readable for WTR {}
#[doc = "`write(|w| ..)` method takes [wtr::W](wtr::W) writer structure"]
impl crate::Writable for WTR {}
#[doc = "Watchdog Timeout Register"]
pub mod wtr;
#[doc = "Position Difference Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [posd](posd) module"]
pub type POSD = crate::Reg<u16, _POSD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POSD;
#[doc = "`read()` method returns [posd::R](posd::R) reader structure"]
impl crate::Readable for POSD {}
#[doc = "`write(|w| ..)` method takes [posd::W](posd::W) writer structure"]
impl crate::Writable for POSD {}
#[doc = "Position Difference Counter Register"]
pub mod posd;
#[doc = "Position Difference Hold Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [posdh](posdh) module"]
pub type POSDH = crate::Reg<u16, _POSDH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POSDH;
#[doc = "`read()` method returns [posdh::R](posdh::R) reader structure"]
impl crate::Readable for POSDH {}
#[doc = "Position Difference Hold Register"]
pub mod posdh;
#[doc = "Revolution Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rev](rev) module"]
pub type REV = crate::Reg<u16, _REV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REV;
#[doc = "`read()` method returns [rev::R](rev::R) reader structure"]
impl crate::Readable for REV {}
#[doc = "`write(|w| ..)` method takes [rev::W](rev::W) writer structure"]
impl crate::Writable for REV {}
#[doc = "Revolution Counter Register"]
pub mod rev;
#[doc = "Revolution Hold Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [revh](revh) module"]
pub type REVH = crate::Reg<u16, _REVH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REVH;
#[doc = "`read()` method returns [revh::R](revh::R) reader structure"]
impl crate::Readable for REVH {}
#[doc = "Revolution Hold Register"]
pub mod revh;
#[doc = "Upper Position Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upos](upos) module"]
pub type UPOS = crate::Reg<u16, _UPOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPOS;
#[doc = "`read()` method returns [upos::R](upos::R) reader structure"]
impl crate::Readable for UPOS {}
#[doc = "`write(|w| ..)` method takes [upos::W](upos::W) writer structure"]
impl crate::Writable for UPOS {}
#[doc = "Upper Position Counter Register"]
pub mod upos;
#[doc = "Lower Position Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpos](lpos) module"]
pub type LPOS = crate::Reg<u16, _LPOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPOS;
#[doc = "`read()` method returns [lpos::R](lpos::R) reader structure"]
impl crate::Readable for LPOS {}
#[doc = "`write(|w| ..)` method takes [lpos::W](lpos::W) writer structure"]
impl crate::Writable for LPOS {}
#[doc = "Lower Position Counter Register"]
pub mod lpos;
#[doc = "Upper Position Hold Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uposh](uposh) module"]
pub type UPOSH = crate::Reg<u16, _UPOSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPOSH;
#[doc = "`read()` method returns [uposh::R](uposh::R) reader structure"]
impl crate::Readable for UPOSH {}
#[doc = "Upper Position Hold Register"]
pub mod uposh;
#[doc = "Lower Position Hold Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lposh](lposh) module"]
pub type LPOSH = crate::Reg<u16, _LPOSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPOSH;
#[doc = "`read()` method returns [lposh::R](lposh::R) reader structure"]
impl crate::Readable for LPOSH {}
#[doc = "Lower Position Hold Register"]
pub mod lposh;
#[doc = "Upper Initialization Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uinit](uinit) module"]
pub type UINIT = crate::Reg<u16, _UINIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UINIT;
#[doc = "`read()` method returns [uinit::R](uinit::R) reader structure"]
impl crate::Readable for UINIT {}
#[doc = "`write(|w| ..)` method takes [uinit::W](uinit::W) writer structure"]
impl crate::Writable for UINIT {}
#[doc = "Upper Initialization Register"]
pub mod uinit;
#[doc = "Lower Initialization Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [linit](linit) module"]
pub type LINIT = crate::Reg<u16, _LINIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LINIT;
#[doc = "`read()` method returns [linit::R](linit::R) reader structure"]
impl crate::Readable for LINIT {}
#[doc = "`write(|w| ..)` method takes [linit::W](linit::W) writer structure"]
impl crate::Writable for LINIT {}
#[doc = "Lower Initialization Register"]
pub mod linit;
#[doc = "Input Monitor Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](imr) module"]
pub type IMR = crate::Reg<u16, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "Input Monitor Register"]
pub mod imr;
#[doc = "Test Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tst](tst) module"]
pub type TST = crate::Reg<u16, _TST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TST;
#[doc = "`read()` method returns [tst::R](tst::R) reader structure"]
impl crate::Readable for TST {}
#[doc = "`write(|w| ..)` method takes [tst::W](tst::W) writer structure"]
impl crate::Writable for TST {}
#[doc = "Test Register"]
pub mod tst;
#[doc = "Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](ctrl2) module"]
pub type CTRL2 = crate::Reg<u16, _CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL2;
#[doc = "`read()` method returns [ctrl2::R](ctrl2::R) reader structure"]
impl crate::Readable for CTRL2 {}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](ctrl2::W) writer structure"]
impl crate::Writable for CTRL2 {}
#[doc = "Control 2 Register"]
pub mod ctrl2;
#[doc = "Upper Modulus Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [umod](umod) module"]
pub type UMOD = crate::Reg<u16, _UMOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UMOD;
#[doc = "`read()` method returns [umod::R](umod::R) reader structure"]
impl crate::Readable for UMOD {}
#[doc = "`write(|w| ..)` method takes [umod::W](umod::W) writer structure"]
impl crate::Writable for UMOD {}
#[doc = "Upper Modulus Register"]
pub mod umod;
#[doc = "Lower Modulus Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmod](lmod) module"]
pub type LMOD = crate::Reg<u16, _LMOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LMOD;
#[doc = "`read()` method returns [lmod::R](lmod::R) reader structure"]
impl crate::Readable for LMOD {}
#[doc = "`write(|w| ..)` method takes [lmod::W](lmod::W) writer structure"]
impl crate::Writable for LMOD {}
#[doc = "Lower Modulus Register"]
pub mod lmod;
#[doc = "Upper Position Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucomp](ucomp) module"]
pub type UCOMP = crate::Reg<u16, _UCOMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCOMP;
#[doc = "`read()` method returns [ucomp::R](ucomp::R) reader structure"]
impl crate::Readable for UCOMP {}
#[doc = "`write(|w| ..)` method takes [ucomp::W](ucomp::W) writer structure"]
impl crate::Writable for UCOMP {}
#[doc = "Upper Position Compare Register"]
pub mod ucomp;
#[doc = "Lower Position Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcomp](lcomp) module"]
pub type LCOMP = crate::Reg<u16, _LCOMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCOMP;
#[doc = "`read()` method returns [lcomp::R](lcomp::R) reader structure"]
impl crate::Readable for LCOMP {}
#[doc = "`write(|w| ..)` method takes [lcomp::W](lcomp::W) writer structure"]
impl crate::Writable for LCOMP {}
#[doc = "Lower Position Compare Register"]
pub mod lcomp;
