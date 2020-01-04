#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 544usize],
    #[doc = "0x220 - PGC Mega Control Register"]
    pub mega_ctrl: MEGA_CTRL,
    #[doc = "0x224 - PGC Mega Power Up Sequence Control Register"]
    pub mega_pupscr: MEGA_PUPSCR,
    #[doc = "0x228 - PGC Mega Pull Down Sequence Control Register"]
    pub mega_pdnscr: MEGA_PDNSCR,
    #[doc = "0x22c - PGC Mega Power Gating Controller Status Register"]
    pub mega_sr: MEGA_SR,
    _reserved4: [u8; 112usize],
    #[doc = "0x2a0 - PGC CPU Control Register"]
    pub cpu_ctrl: CPU_CTRL,
    #[doc = "0x2a4 - PGC CPU Power Up Sequence Control Register"]
    pub cpu_pupscr: CPU_PUPSCR,
    #[doc = "0x2a8 - PGC CPU Pull Down Sequence Control Register"]
    pub cpu_pdnscr: CPU_PDNSCR,
    #[doc = "0x2ac - PGC CPU Power Gating Controller Status Register"]
    pub cpu_sr: CPU_SR,
}
#[doc = "PGC Mega Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mega_ctrl](mega_ctrl) module"]
pub type MEGA_CTRL = crate::Reg<u32, _MEGA_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEGA_CTRL;
#[doc = "`read()` method returns [mega_ctrl::R](mega_ctrl::R) reader structure"]
impl crate::Readable for MEGA_CTRL {}
#[doc = "`write(|w| ..)` method takes [mega_ctrl::W](mega_ctrl::W) writer structure"]
impl crate::Writable for MEGA_CTRL {}
#[doc = "PGC Mega Control Register"]
pub mod mega_ctrl;
#[doc = "PGC Mega Power Up Sequence Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mega_pupscr](mega_pupscr) module"]
pub type MEGA_PUPSCR = crate::Reg<u32, _MEGA_PUPSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEGA_PUPSCR;
#[doc = "`read()` method returns [mega_pupscr::R](mega_pupscr::R) reader structure"]
impl crate::Readable for MEGA_PUPSCR {}
#[doc = "`write(|w| ..)` method takes [mega_pupscr::W](mega_pupscr::W) writer structure"]
impl crate::Writable for MEGA_PUPSCR {}
#[doc = "PGC Mega Power Up Sequence Control Register"]
pub mod mega_pupscr;
#[doc = "PGC Mega Pull Down Sequence Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mega_pdnscr](mega_pdnscr) module"]
pub type MEGA_PDNSCR = crate::Reg<u32, _MEGA_PDNSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEGA_PDNSCR;
#[doc = "`read()` method returns [mega_pdnscr::R](mega_pdnscr::R) reader structure"]
impl crate::Readable for MEGA_PDNSCR {}
#[doc = "`write(|w| ..)` method takes [mega_pdnscr::W](mega_pdnscr::W) writer structure"]
impl crate::Writable for MEGA_PDNSCR {}
#[doc = "PGC Mega Pull Down Sequence Control Register"]
pub mod mega_pdnscr;
#[doc = "PGC Mega Power Gating Controller Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mega_sr](mega_sr) module"]
pub type MEGA_SR = crate::Reg<u32, _MEGA_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEGA_SR;
#[doc = "`read()` method returns [mega_sr::R](mega_sr::R) reader structure"]
impl crate::Readable for MEGA_SR {}
#[doc = "`write(|w| ..)` method takes [mega_sr::W](mega_sr::W) writer structure"]
impl crate::Writable for MEGA_SR {}
#[doc = "PGC Mega Power Gating Controller Status Register"]
pub mod mega_sr;
#[doc = "PGC CPU Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_ctrl](cpu_ctrl) module"]
pub type CPU_CTRL = crate::Reg<u32, _CPU_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPU_CTRL;
#[doc = "`read()` method returns [cpu_ctrl::R](cpu_ctrl::R) reader structure"]
impl crate::Readable for CPU_CTRL {}
#[doc = "`write(|w| ..)` method takes [cpu_ctrl::W](cpu_ctrl::W) writer structure"]
impl crate::Writable for CPU_CTRL {}
#[doc = "PGC CPU Control Register"]
pub mod cpu_ctrl;
#[doc = "PGC CPU Power Up Sequence Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_pupscr](cpu_pupscr) module"]
pub type CPU_PUPSCR = crate::Reg<u32, _CPU_PUPSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPU_PUPSCR;
#[doc = "`read()` method returns [cpu_pupscr::R](cpu_pupscr::R) reader structure"]
impl crate::Readable for CPU_PUPSCR {}
#[doc = "`write(|w| ..)` method takes [cpu_pupscr::W](cpu_pupscr::W) writer structure"]
impl crate::Writable for CPU_PUPSCR {}
#[doc = "PGC CPU Power Up Sequence Control Register"]
pub mod cpu_pupscr;
#[doc = "PGC CPU Pull Down Sequence Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_pdnscr](cpu_pdnscr) module"]
pub type CPU_PDNSCR = crate::Reg<u32, _CPU_PDNSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPU_PDNSCR;
#[doc = "`read()` method returns [cpu_pdnscr::R](cpu_pdnscr::R) reader structure"]
impl crate::Readable for CPU_PDNSCR {}
#[doc = "`write(|w| ..)` method takes [cpu_pdnscr::W](cpu_pdnscr::W) writer structure"]
impl crate::Writable for CPU_PDNSCR {}
#[doc = "PGC CPU Pull Down Sequence Control Register"]
pub mod cpu_pdnscr;
#[doc = "PGC CPU Power Gating Controller Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_sr](cpu_sr) module"]
pub type CPU_SR = crate::Reg<u32, _CPU_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPU_SR;
#[doc = "`read()` method returns [cpu_sr::R](cpu_sr::R) reader structure"]
impl crate::Readable for CPU_SR {}
#[doc = "`write(|w| ..)` method takes [cpu_sr::W](cpu_sr::W) writer structure"]
impl crate::Writable for CPU_SR {}
#[doc = "PGC CPU Power Gating Controller Status Register"]
pub mod cpu_sr;
