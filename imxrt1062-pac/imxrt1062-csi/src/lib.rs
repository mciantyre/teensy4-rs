#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CSI Control Register 1"]
    pub csicr1: CSICR1,
    #[doc = "0x04 - CSI Control Register 2"]
    pub csicr2: CSICR2,
    #[doc = "0x08 - CSI Control Register 3"]
    pub csicr3: CSICR3,
    #[doc = "0x0c - CSI Statistic FIFO Register"]
    pub csistatfifo: CSISTATFIFO,
    #[doc = "0x10 - CSI RX FIFO Register"]
    pub csirfifo: CSIRFIFO,
    #[doc = "0x14 - CSI RX Count Register"]
    pub csirxcnt: CSIRXCNT,
    #[doc = "0x18 - CSI Status Register"]
    pub csisr: CSISR,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - CSI DMA Start Address Register - for STATFIFO"]
    pub csidmasa_statfifo: CSIDMASA_STATFIFO,
    #[doc = "0x24 - CSI DMA Transfer Size Register - for STATFIFO"]
    pub csidmats_statfifo: CSIDMATS_STATFIFO,
    #[doc = "0x28 - CSI DMA Start Address Register - for Frame Buffer1"]
    pub csidmasa_fb1: CSIDMASA_FB1,
    #[doc = "0x2c - CSI DMA Transfer Size Register - for Frame Buffer2"]
    pub csidmasa_fb2: CSIDMASA_FB2,
    #[doc = "0x30 - CSI Frame Buffer Parameter Register"]
    pub csifbuf_para: CSIFBUF_PARA,
    #[doc = "0x34 - CSI Image Parameter Register"]
    pub csiimag_para: CSIIMAG_PARA,
    _reserved13: [u8; 16usize],
    #[doc = "0x48 - CSI Control Register 18"]
    pub csicr18: CSICR18,
    #[doc = "0x4c - CSI Control Register 19"]
    pub csicr19: CSICR19,
}
#[doc = "CSI Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csicr1](csicr1) module"]
pub type CSICR1 = crate::Reg<u32, _CSICR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSICR1;
#[doc = "`read()` method returns [csicr1::R](csicr1::R) reader structure"]
impl crate::Readable for CSICR1 {}
#[doc = "`write(|w| ..)` method takes [csicr1::W](csicr1::W) writer structure"]
impl crate::Writable for CSICR1 {}
#[doc = "CSI Control Register 1"]
pub mod csicr1;
#[doc = "CSI Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csicr2](csicr2) module"]
pub type CSICR2 = crate::Reg<u32, _CSICR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSICR2;
#[doc = "`read()` method returns [csicr2::R](csicr2::R) reader structure"]
impl crate::Readable for CSICR2 {}
#[doc = "`write(|w| ..)` method takes [csicr2::W](csicr2::W) writer structure"]
impl crate::Writable for CSICR2 {}
#[doc = "CSI Control Register 2"]
pub mod csicr2;
#[doc = "CSI Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csicr3](csicr3) module"]
pub type CSICR3 = crate::Reg<u32, _CSICR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSICR3;
#[doc = "`read()` method returns [csicr3::R](csicr3::R) reader structure"]
impl crate::Readable for CSICR3 {}
#[doc = "`write(|w| ..)` method takes [csicr3::W](csicr3::W) writer structure"]
impl crate::Writable for CSICR3 {}
#[doc = "CSI Control Register 3"]
pub mod csicr3;
#[doc = "CSI Statistic FIFO Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csistatfifo](csistatfifo) module"]
pub type CSISTATFIFO = crate::Reg<u32, _CSISTATFIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSISTATFIFO;
#[doc = "`read()` method returns [csistatfifo::R](csistatfifo::R) reader structure"]
impl crate::Readable for CSISTATFIFO {}
#[doc = "CSI Statistic FIFO Register"]
pub mod csistatfifo;
#[doc = "CSI RX FIFO Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csirfifo](csirfifo) module"]
pub type CSIRFIFO = crate::Reg<u32, _CSIRFIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIRFIFO;
#[doc = "`read()` method returns [csirfifo::R](csirfifo::R) reader structure"]
impl crate::Readable for CSIRFIFO {}
#[doc = "CSI RX FIFO Register"]
pub mod csirfifo;
#[doc = "CSI RX Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csirxcnt](csirxcnt) module"]
pub type CSIRXCNT = crate::Reg<u32, _CSIRXCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIRXCNT;
#[doc = "`read()` method returns [csirxcnt::R](csirxcnt::R) reader structure"]
impl crate::Readable for CSIRXCNT {}
#[doc = "`write(|w| ..)` method takes [csirxcnt::W](csirxcnt::W) writer structure"]
impl crate::Writable for CSIRXCNT {}
#[doc = "CSI RX Count Register"]
pub mod csirxcnt;
#[doc = "CSI Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csisr](csisr) module"]
pub type CSISR = crate::Reg<u32, _CSISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSISR;
#[doc = "`read()` method returns [csisr::R](csisr::R) reader structure"]
impl crate::Readable for CSISR {}
#[doc = "`write(|w| ..)` method takes [csisr::W](csisr::W) writer structure"]
impl crate::Writable for CSISR {}
#[doc = "CSI Status Register"]
pub mod csisr;
#[doc = "CSI DMA Start Address Register - for STATFIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csidmasa_statfifo](csidmasa_statfifo) module"]
pub type CSIDMASA_STATFIFO = crate::Reg<u32, _CSIDMASA_STATFIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIDMASA_STATFIFO;
#[doc = "`read()` method returns [csidmasa_statfifo::R](csidmasa_statfifo::R) reader structure"]
impl crate::Readable for CSIDMASA_STATFIFO {}
#[doc = "`write(|w| ..)` method takes [csidmasa_statfifo::W](csidmasa_statfifo::W) writer structure"]
impl crate::Writable for CSIDMASA_STATFIFO {}
#[doc = "CSI DMA Start Address Register - for STATFIFO"]
pub mod csidmasa_statfifo;
#[doc = "CSI DMA Transfer Size Register - for STATFIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csidmats_statfifo](csidmats_statfifo) module"]
pub type CSIDMATS_STATFIFO = crate::Reg<u32, _CSIDMATS_STATFIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIDMATS_STATFIFO;
#[doc = "`read()` method returns [csidmats_statfifo::R](csidmats_statfifo::R) reader structure"]
impl crate::Readable for CSIDMATS_STATFIFO {}
#[doc = "`write(|w| ..)` method takes [csidmats_statfifo::W](csidmats_statfifo::W) writer structure"]
impl crate::Writable for CSIDMATS_STATFIFO {}
#[doc = "CSI DMA Transfer Size Register - for STATFIFO"]
pub mod csidmats_statfifo;
#[doc = "CSI DMA Start Address Register - for Frame Buffer1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csidmasa_fb1](csidmasa_fb1) module"]
pub type CSIDMASA_FB1 = crate::Reg<u32, _CSIDMASA_FB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIDMASA_FB1;
#[doc = "`read()` method returns [csidmasa_fb1::R](csidmasa_fb1::R) reader structure"]
impl crate::Readable for CSIDMASA_FB1 {}
#[doc = "`write(|w| ..)` method takes [csidmasa_fb1::W](csidmasa_fb1::W) writer structure"]
impl crate::Writable for CSIDMASA_FB1 {}
#[doc = "CSI DMA Start Address Register - for Frame Buffer1"]
pub mod csidmasa_fb1;
#[doc = "CSI DMA Transfer Size Register - for Frame Buffer2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csidmasa_fb2](csidmasa_fb2) module"]
pub type CSIDMASA_FB2 = crate::Reg<u32, _CSIDMASA_FB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIDMASA_FB2;
#[doc = "`read()` method returns [csidmasa_fb2::R](csidmasa_fb2::R) reader structure"]
impl crate::Readable for CSIDMASA_FB2 {}
#[doc = "`write(|w| ..)` method takes [csidmasa_fb2::W](csidmasa_fb2::W) writer structure"]
impl crate::Writable for CSIDMASA_FB2 {}
#[doc = "CSI DMA Transfer Size Register - for Frame Buffer2"]
pub mod csidmasa_fb2;
#[doc = "CSI Frame Buffer Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csifbuf_para](csifbuf_para) module"]
pub type CSIFBUF_PARA = crate::Reg<u32, _CSIFBUF_PARA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIFBUF_PARA;
#[doc = "`read()` method returns [csifbuf_para::R](csifbuf_para::R) reader structure"]
impl crate::Readable for CSIFBUF_PARA {}
#[doc = "`write(|w| ..)` method takes [csifbuf_para::W](csifbuf_para::W) writer structure"]
impl crate::Writable for CSIFBUF_PARA {}
#[doc = "CSI Frame Buffer Parameter Register"]
pub mod csifbuf_para;
#[doc = "CSI Image Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csiimag_para](csiimag_para) module"]
pub type CSIIMAG_PARA = crate::Reg<u32, _CSIIMAG_PARA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIIMAG_PARA;
#[doc = "`read()` method returns [csiimag_para::R](csiimag_para::R) reader structure"]
impl crate::Readable for CSIIMAG_PARA {}
#[doc = "`write(|w| ..)` method takes [csiimag_para::W](csiimag_para::W) writer structure"]
impl crate::Writable for CSIIMAG_PARA {}
#[doc = "CSI Image Parameter Register"]
pub mod csiimag_para;
#[doc = "CSI Control Register 18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csicr18](csicr18) module"]
pub type CSICR18 = crate::Reg<u32, _CSICR18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSICR18;
#[doc = "`read()` method returns [csicr18::R](csicr18::R) reader structure"]
impl crate::Readable for CSICR18 {}
#[doc = "`write(|w| ..)` method takes [csicr18::W](csicr18::W) writer structure"]
impl crate::Writable for CSICR18 {}
#[doc = "CSI Control Register 18"]
pub mod csicr18;
#[doc = "CSI Control Register 19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csicr19](csicr19) module"]
pub type CSICR19 = crate::Reg<u32, _CSICR19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSICR19;
#[doc = "`read()` method returns [csicr19::R](csicr19::R) reader structure"]
impl crate::Readable for CSICR19 {}
#[doc = "`write(|w| ..)` method takes [csicr19::W](csicr19::W) writer structure"]
impl crate::Writable for CSICR19 {}
#[doc = "CSI Control Register 19"]
pub mod csicr19;
