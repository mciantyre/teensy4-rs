#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer Channel Compare Register 1"]
    pub comp10: COMP1,
    #[doc = "0x02 - Timer Channel Compare Register 2"]
    pub comp20: COMP2,
    #[doc = "0x04 - Timer Channel Capture Register"]
    pub capt0: CAPT,
    #[doc = "0x06 - Timer Channel Load Register"]
    pub load0: LOAD,
    #[doc = "0x08 - Timer Channel Hold Register"]
    pub hold0: HOLD,
    #[doc = "0x0a - Timer Channel Counter Register"]
    pub cntr0: CNTR,
    #[doc = "0x0c - Timer Channel Control Register"]
    pub ctrl0: CTRL,
    #[doc = "0x0e - Timer Channel Status and Control Register"]
    pub sctrl0: SCTRL,
    #[doc = "0x10 - Timer Channel Comparator Load Register 1"]
    pub cmpld10: CMPLD1,
    #[doc = "0x12 - Timer Channel Comparator Load Register 2"]
    pub cmpld20: CMPLD2,
    #[doc = "0x14 - Timer Channel Comparator Status and Control Register"]
    pub csctrl0: CSCTRL,
    #[doc = "0x16 - Timer Channel Input Filter Register"]
    pub filt0: FILT,
    #[doc = "0x18 - Timer Channel DMA Enable Register"]
    pub dma0: DMA,
    _reserved13: [u8; 4usize],
    #[doc = "0x1e - Timer Channel Enable Register"]
    pub enbl: ENBL,
    #[doc = "0x20 - Timer Channel Compare Register 1"]
    pub comp11: COMP1,
    #[doc = "0x22 - Timer Channel Compare Register 2"]
    pub comp21: COMP2,
    #[doc = "0x24 - Timer Channel Capture Register"]
    pub capt1: CAPT,
    #[doc = "0x26 - Timer Channel Load Register"]
    pub load1: LOAD,
    #[doc = "0x28 - Timer Channel Hold Register"]
    pub hold1: HOLD,
    #[doc = "0x2a - Timer Channel Counter Register"]
    pub cntr1: CNTR,
    #[doc = "0x2c - Timer Channel Control Register"]
    pub ctrl1: CTRL,
    #[doc = "0x2e - Timer Channel Status and Control Register"]
    pub sctrl1: SCTRL,
    #[doc = "0x30 - Timer Channel Comparator Load Register 1"]
    pub cmpld11: CMPLD1,
    #[doc = "0x32 - Timer Channel Comparator Load Register 2"]
    pub cmpld21: CMPLD2,
    #[doc = "0x34 - Timer Channel Comparator Status and Control Register"]
    pub csctrl1: CSCTRL,
    #[doc = "0x36 - Timer Channel Input Filter Register"]
    pub filt1: FILT,
    #[doc = "0x38 - Timer Channel DMA Enable Register"]
    pub dma1: DMA,
    _reserved27: [u8; 6usize],
    #[doc = "0x40 - Timer Channel Compare Register 1"]
    pub comp12: COMP1,
    #[doc = "0x42 - Timer Channel Compare Register 2"]
    pub comp22: COMP2,
    #[doc = "0x44 - Timer Channel Capture Register"]
    pub capt2: CAPT,
    #[doc = "0x46 - Timer Channel Load Register"]
    pub load2: LOAD,
    #[doc = "0x48 - Timer Channel Hold Register"]
    pub hold2: HOLD,
    #[doc = "0x4a - Timer Channel Counter Register"]
    pub cntr2: CNTR,
    #[doc = "0x4c - Timer Channel Control Register"]
    pub ctrl2: CTRL,
    #[doc = "0x4e - Timer Channel Status and Control Register"]
    pub sctrl2: SCTRL,
    #[doc = "0x50 - Timer Channel Comparator Load Register 1"]
    pub cmpld12: CMPLD1,
    #[doc = "0x52 - Timer Channel Comparator Load Register 2"]
    pub cmpld22: CMPLD2,
    #[doc = "0x54 - Timer Channel Comparator Status and Control Register"]
    pub csctrl2: CSCTRL,
    #[doc = "0x56 - Timer Channel Input Filter Register"]
    pub filt2: FILT,
    #[doc = "0x58 - Timer Channel DMA Enable Register"]
    pub dma2: DMA,
    _reserved40: [u8; 6usize],
    #[doc = "0x60 - Timer Channel Compare Register 1"]
    pub comp13: COMP1,
    #[doc = "0x62 - Timer Channel Compare Register 2"]
    pub comp23: COMP2,
    #[doc = "0x64 - Timer Channel Capture Register"]
    pub capt3: CAPT,
    #[doc = "0x66 - Timer Channel Load Register"]
    pub load3: LOAD,
    #[doc = "0x68 - Timer Channel Hold Register"]
    pub hold3: HOLD,
    #[doc = "0x6a - Timer Channel Counter Register"]
    pub cntr3: CNTR,
    #[doc = "0x6c - Timer Channel Control Register"]
    pub ctrl3: CTRL,
    #[doc = "0x6e - Timer Channel Status and Control Register"]
    pub sctrl3: SCTRL,
    #[doc = "0x70 - Timer Channel Comparator Load Register 1"]
    pub cmpld13: CMPLD1,
    #[doc = "0x72 - Timer Channel Comparator Load Register 2"]
    pub cmpld23: CMPLD2,
    #[doc = "0x74 - Timer Channel Comparator Status and Control Register"]
    pub csctrl3: CSCTRL,
    #[doc = "0x76 - Timer Channel Input Filter Register"]
    pub filt3: FILT,
    #[doc = "0x78 - Timer Channel DMA Enable Register"]
    pub dma3: DMA,
}
#[doc = "Timer Channel Compare Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp1](comp1) module"]
pub type COMP1 = crate::Reg<u16, _COMP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP1;
#[doc = "`read()` method returns [comp1::R](comp1::R) reader structure"]
impl crate::Readable for COMP1 {}
#[doc = "`write(|w| ..)` method takes [comp1::W](comp1::W) writer structure"]
impl crate::Writable for COMP1 {}
#[doc = "Timer Channel Compare Register 1"]
pub mod comp1;
#[doc = "Timer Channel Compare Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp2](comp2) module"]
pub type COMP2 = crate::Reg<u16, _COMP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP2;
#[doc = "`read()` method returns [comp2::R](comp2::R) reader structure"]
impl crate::Readable for COMP2 {}
#[doc = "`write(|w| ..)` method takes [comp2::W](comp2::W) writer structure"]
impl crate::Writable for COMP2 {}
#[doc = "Timer Channel Compare Register 2"]
pub mod comp2;
#[doc = "Timer Channel Capture Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capt](capt) module"]
pub type CAPT = crate::Reg<u16, _CAPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPT;
#[doc = "`read()` method returns [capt::R](capt::R) reader structure"]
impl crate::Readable for CAPT {}
#[doc = "`write(|w| ..)` method takes [capt::W](capt::W) writer structure"]
impl crate::Writable for CAPT {}
#[doc = "Timer Channel Capture Register"]
pub mod capt;
#[doc = "Timer Channel Load Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [load](load) module"]
pub type LOAD = crate::Reg<u16, _LOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOAD;
#[doc = "`read()` method returns [load::R](load::R) reader structure"]
impl crate::Readable for LOAD {}
#[doc = "`write(|w| ..)` method takes [load::W](load::W) writer structure"]
impl crate::Writable for LOAD {}
#[doc = "Timer Channel Load Register"]
pub mod load;
#[doc = "Timer Channel Hold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hold](hold) module"]
pub type HOLD = crate::Reg<u16, _HOLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOLD;
#[doc = "`read()` method returns [hold::R](hold::R) reader structure"]
impl crate::Readable for HOLD {}
#[doc = "`write(|w| ..)` method takes [hold::W](hold::W) writer structure"]
impl crate::Writable for HOLD {}
#[doc = "Timer Channel Hold Register"]
pub mod hold;
#[doc = "Timer Channel Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntr](cntr) module"]
pub type CNTR = crate::Reg<u16, _CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTR;
#[doc = "`read()` method returns [cntr::R](cntr::R) reader structure"]
impl crate::Readable for CNTR {}
#[doc = "`write(|w| ..)` method takes [cntr::W](cntr::W) writer structure"]
impl crate::Writable for CNTR {}
#[doc = "Timer Channel Counter Register"]
pub mod cntr;
#[doc = "Timer Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u16, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Timer Channel Control Register"]
pub mod ctrl;
#[doc = "Timer Channel Status and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctrl](sctrl) module"]
pub type SCTRL = crate::Reg<u16, _SCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTRL;
#[doc = "`read()` method returns [sctrl::R](sctrl::R) reader structure"]
impl crate::Readable for SCTRL {}
#[doc = "`write(|w| ..)` method takes [sctrl::W](sctrl::W) writer structure"]
impl crate::Writable for SCTRL {}
#[doc = "Timer Channel Status and Control Register"]
pub mod sctrl;
#[doc = "Timer Channel Comparator Load Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpld1](cmpld1) module"]
pub type CMPLD1 = crate::Reg<u16, _CMPLD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPLD1;
#[doc = "`read()` method returns [cmpld1::R](cmpld1::R) reader structure"]
impl crate::Readable for CMPLD1 {}
#[doc = "`write(|w| ..)` method takes [cmpld1::W](cmpld1::W) writer structure"]
impl crate::Writable for CMPLD1 {}
#[doc = "Timer Channel Comparator Load Register 1"]
pub mod cmpld1;
#[doc = "Timer Channel Comparator Load Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpld2](cmpld2) module"]
pub type CMPLD2 = crate::Reg<u16, _CMPLD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPLD2;
#[doc = "`read()` method returns [cmpld2::R](cmpld2::R) reader structure"]
impl crate::Readable for CMPLD2 {}
#[doc = "`write(|w| ..)` method takes [cmpld2::W](cmpld2::W) writer structure"]
impl crate::Writable for CMPLD2 {}
#[doc = "Timer Channel Comparator Load Register 2"]
pub mod cmpld2;
#[doc = "Timer Channel Comparator Status and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctrl](csctrl) module"]
pub type CSCTRL = crate::Reg<u16, _CSCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCTRL;
#[doc = "`read()` method returns [csctrl::R](csctrl::R) reader structure"]
impl crate::Readable for CSCTRL {}
#[doc = "`write(|w| ..)` method takes [csctrl::W](csctrl::W) writer structure"]
impl crate::Writable for CSCTRL {}
#[doc = "Timer Channel Comparator Status and Control Register"]
pub mod csctrl;
#[doc = "Timer Channel Input Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filt](filt) module"]
pub type FILT = crate::Reg<u16, _FILT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FILT;
#[doc = "`read()` method returns [filt::R](filt::R) reader structure"]
impl crate::Readable for FILT {}
#[doc = "`write(|w| ..)` method takes [filt::W](filt::W) writer structure"]
impl crate::Writable for FILT {}
#[doc = "Timer Channel Input Filter Register"]
pub mod filt;
#[doc = "Timer Channel DMA Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma](dma) module"]
pub type DMA = crate::Reg<u16, _DMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA;
#[doc = "`read()` method returns [dma::R](dma::R) reader structure"]
impl crate::Readable for DMA {}
#[doc = "`write(|w| ..)` method takes [dma::W](dma::W) writer structure"]
impl crate::Writable for DMA {}
#[doc = "Timer Channel DMA Enable Register"]
pub mod dma;
#[doc = "Timer Channel Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enbl](enbl) module"]
pub type ENBL = crate::Reg<u16, _ENBL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENBL;
#[doc = "`read()` method returns [enbl::R](enbl::R) reader structure"]
impl crate::Readable for ENBL {}
#[doc = "`write(|w| ..)` method takes [enbl::W](enbl::W) writer structure"]
impl crate::Writable for ENBL {}
#[doc = "Timer Channel Enable Register"]
pub mod enbl;
