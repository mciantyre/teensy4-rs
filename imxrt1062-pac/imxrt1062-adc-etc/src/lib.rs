#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC_ETC Global Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - ETC DONE0 and DONE1 IRQ State Register"]
    pub done0_1_irq: DONE0_1_IRQ,
    #[doc = "0x08 - ETC DONE_2 and DONE_ERR IRQ State Register"]
    pub done2_err_irq: DONE2_ERR_IRQ,
    #[doc = "0x0c - ETC DMA control Register"]
    pub dma_ctrl: DMA_CTRL,
    #[doc = "0x10 - ETC_TRIG0 Control Register"]
    pub trig0_ctrl: TRIG0_CTRL,
    #[doc = "0x14 - ETC_TRIG0 Counter Register"]
    pub trig0_counter: TRIG0_COUNTER,
    #[doc = "0x18 - ETC_TRIG Chain 0/1 Register"]
    pub trig0_chain_1_0: TRIG0_CHAIN_1_0,
    #[doc = "0x1c - ETC_TRIG Chain 2/3 Register"]
    pub trig0_chain_3_2: TRIG0_CHAIN_3_2,
    #[doc = "0x20 - ETC_TRIG Chain 4/5 Register"]
    pub trig0_chain_5_4: TRIG0_CHAIN_5_4,
    #[doc = "0x24 - ETC_TRIG Chain 6/7 Register"]
    pub trig0_chain_7_6: TRIG0_CHAIN_7_6,
    #[doc = "0x28 - ETC_TRIG Result Data 1/0 Register"]
    pub trig0_result_1_0: TRIG0_RESULT_1_0,
    #[doc = "0x2c - ETC_TRIG Result Data 3/2 Register"]
    pub trig0_result_3_2: TRIG0_RESULT_3_2,
    #[doc = "0x30 - ETC_TRIG Result Data 5/4 Register"]
    pub trig0_result_5_4: TRIG0_RESULT_5_4,
    #[doc = "0x34 - ETC_TRIG Result Data 7/6 Register"]
    pub trig0_result_7_6: TRIG0_RESULT_7_6,
    #[doc = "0x38 - ETC_TRIG1 Control Register"]
    pub trig1_ctrl: TRIG1_CTRL,
    #[doc = "0x3c - ETC_TRIG1 Counter Register"]
    pub trig1_counter: TRIG1_COUNTER,
    #[doc = "0x40 - ETC_TRIG Chain 0/1 Register"]
    pub trig1_chain_1_0: TRIG1_CHAIN_1_0,
    #[doc = "0x44 - ETC_TRIG Chain 2/3 Register"]
    pub trig1_chain_3_2: TRIG1_CHAIN_3_2,
    #[doc = "0x48 - ETC_TRIG Chain 4/5 Register"]
    pub trig1_chain_5_4: TRIG1_CHAIN_5_4,
    #[doc = "0x4c - ETC_TRIG Chain 6/7 Register"]
    pub trig1_chain_7_6: TRIG1_CHAIN_7_6,
    #[doc = "0x50 - ETC_TRIG Result Data 1/0 Register"]
    pub trig1_result_1_0: TRIG1_RESULT_1_0,
    #[doc = "0x54 - ETC_TRIG Result Data 3/2 Register"]
    pub trig1_result_3_2: TRIG1_RESULT_3_2,
    #[doc = "0x58 - ETC_TRIG Result Data 5/4 Register"]
    pub trig1_result_5_4: TRIG1_RESULT_5_4,
    #[doc = "0x5c - ETC_TRIG Result Data 7/6 Register"]
    pub trig1_result_7_6: TRIG1_RESULT_7_6,
    #[doc = "0x60 - ETC_TRIG2 Control Register"]
    pub trig2_ctrl: TRIG2_CTRL,
    #[doc = "0x64 - ETC_TRIG2 Counter Register"]
    pub trig2_counter: TRIG2_COUNTER,
    #[doc = "0x68 - ETC_TRIG Chain 0/1 Register"]
    pub trig2_chain_1_0: TRIG2_CHAIN_1_0,
    #[doc = "0x6c - ETC_TRIG Chain 2/3 Register"]
    pub trig2_chain_3_2: TRIG2_CHAIN_3_2,
    #[doc = "0x70 - ETC_TRIG Chain 4/5 Register"]
    pub trig2_chain_5_4: TRIG2_CHAIN_5_4,
    #[doc = "0x74 - ETC_TRIG Chain 6/7 Register"]
    pub trig2_chain_7_6: TRIG2_CHAIN_7_6,
    #[doc = "0x78 - ETC_TRIG Result Data 1/0 Register"]
    pub trig2_result_1_0: TRIG2_RESULT_1_0,
    #[doc = "0x7c - ETC_TRIG Result Data 3/2 Register"]
    pub trig2_result_3_2: TRIG2_RESULT_3_2,
    #[doc = "0x80 - ETC_TRIG Result Data 5/4 Register"]
    pub trig2_result_5_4: TRIG2_RESULT_5_4,
    #[doc = "0x84 - ETC_TRIG Result Data 7/6 Register"]
    pub trig2_result_7_6: TRIG2_RESULT_7_6,
    #[doc = "0x88 - ETC_TRIG3 Control Register"]
    pub trig3_ctrl: TRIG3_CTRL,
    #[doc = "0x8c - ETC_TRIG3 Counter Register"]
    pub trig3_counter: TRIG3_COUNTER,
    #[doc = "0x90 - ETC_TRIG Chain 0/1 Register"]
    pub trig3_chain_1_0: TRIG3_CHAIN_1_0,
    #[doc = "0x94 - ETC_TRIG Chain 2/3 Register"]
    pub trig3_chain_3_2: TRIG3_CHAIN_3_2,
    #[doc = "0x98 - ETC_TRIG Chain 4/5 Register"]
    pub trig3_chain_5_4: TRIG3_CHAIN_5_4,
    #[doc = "0x9c - ETC_TRIG Chain 6/7 Register"]
    pub trig3_chain_7_6: TRIG3_CHAIN_7_6,
    #[doc = "0xa0 - ETC_TRIG Result Data 1/0 Register"]
    pub trig3_result_1_0: TRIG3_RESULT_1_0,
    #[doc = "0xa4 - ETC_TRIG Result Data 3/2 Register"]
    pub trig3_result_3_2: TRIG3_RESULT_3_2,
    #[doc = "0xa8 - ETC_TRIG Result Data 5/4 Register"]
    pub trig3_result_5_4: TRIG3_RESULT_5_4,
    #[doc = "0xac - ETC_TRIG Result Data 7/6 Register"]
    pub trig3_result_7_6: TRIG3_RESULT_7_6,
    #[doc = "0xb0 - ETC_TRIG4 Control Register"]
    pub trig4_ctrl: TRIG4_CTRL,
    #[doc = "0xb4 - ETC_TRIG4 Counter Register"]
    pub trig4_counter: TRIG4_COUNTER,
    #[doc = "0xb8 - ETC_TRIG Chain 0/1 Register"]
    pub trig4_chain_1_0: TRIG4_CHAIN_1_0,
    #[doc = "0xbc - ETC_TRIG Chain 2/3 Register"]
    pub trig4_chain_3_2: TRIG4_CHAIN_3_2,
    #[doc = "0xc0 - ETC_TRIG Chain 4/5 Register"]
    pub trig4_chain_5_4: TRIG4_CHAIN_5_4,
    #[doc = "0xc4 - ETC_TRIG Chain 6/7 Register"]
    pub trig4_chain_7_6: TRIG4_CHAIN_7_6,
    #[doc = "0xc8 - ETC_TRIG Result Data 1/0 Register"]
    pub trig4_result_1_0: TRIG4_RESULT_1_0,
    #[doc = "0xcc - ETC_TRIG Result Data 3/2 Register"]
    pub trig4_result_3_2: TRIG4_RESULT_3_2,
    #[doc = "0xd0 - ETC_TRIG Result Data 5/4 Register"]
    pub trig4_result_5_4: TRIG4_RESULT_5_4,
    #[doc = "0xd4 - ETC_TRIG Result Data 7/6 Register"]
    pub trig4_result_7_6: TRIG4_RESULT_7_6,
    #[doc = "0xd8 - ETC_TRIG5 Control Register"]
    pub trig5_ctrl: TRIG5_CTRL,
    #[doc = "0xdc - ETC_TRIG5 Counter Register"]
    pub trig5_counter: TRIG5_COUNTER,
    #[doc = "0xe0 - ETC_TRIG Chain 0/1 Register"]
    pub trig5_chain_1_0: TRIG5_CHAIN_1_0,
    #[doc = "0xe4 - ETC_TRIG Chain 2/3 Register"]
    pub trig5_chain_3_2: TRIG5_CHAIN_3_2,
    #[doc = "0xe8 - ETC_TRIG Chain 4/5 Register"]
    pub trig5_chain_5_4: TRIG5_CHAIN_5_4,
    #[doc = "0xec - ETC_TRIG Chain 6/7 Register"]
    pub trig5_chain_7_6: TRIG5_CHAIN_7_6,
    #[doc = "0xf0 - ETC_TRIG Result Data 1/0 Register"]
    pub trig5_result_1_0: TRIG5_RESULT_1_0,
    #[doc = "0xf4 - ETC_TRIG Result Data 3/2 Register"]
    pub trig5_result_3_2: TRIG5_RESULT_3_2,
    #[doc = "0xf8 - ETC_TRIG Result Data 5/4 Register"]
    pub trig5_result_5_4: TRIG5_RESULT_5_4,
    #[doc = "0xfc - ETC_TRIG Result Data 7/6 Register"]
    pub trig5_result_7_6: TRIG5_RESULT_7_6,
    #[doc = "0x100 - ETC_TRIG6 Control Register"]
    pub trig6_ctrl: TRIG6_CTRL,
    #[doc = "0x104 - ETC_TRIG6 Counter Register"]
    pub trig6_counter: TRIG6_COUNTER,
    #[doc = "0x108 - ETC_TRIG Chain 0/1 Register"]
    pub trig6_chain_1_0: TRIG6_CHAIN_1_0,
    #[doc = "0x10c - ETC_TRIG Chain 2/3 Register"]
    pub trig6_chain_3_2: TRIG6_CHAIN_3_2,
    #[doc = "0x110 - ETC_TRIG Chain 4/5 Register"]
    pub trig6_chain_5_4: TRIG6_CHAIN_5_4,
    #[doc = "0x114 - ETC_TRIG Chain 6/7 Register"]
    pub trig6_chain_7_6: TRIG6_CHAIN_7_6,
    #[doc = "0x118 - ETC_TRIG Result Data 1/0 Register"]
    pub trig6_result_1_0: TRIG6_RESULT_1_0,
    #[doc = "0x11c - ETC_TRIG Result Data 3/2 Register"]
    pub trig6_result_3_2: TRIG6_RESULT_3_2,
    #[doc = "0x120 - ETC_TRIG Result Data 5/4 Register"]
    pub trig6_result_5_4: TRIG6_RESULT_5_4,
    #[doc = "0x124 - ETC_TRIG Result Data 7/6 Register"]
    pub trig6_result_7_6: TRIG6_RESULT_7_6,
    #[doc = "0x128 - ETC_TRIG7 Control Register"]
    pub trig7_ctrl: TRIG7_CTRL,
    #[doc = "0x12c - ETC_TRIG7 Counter Register"]
    pub trig7_counter: TRIG7_COUNTER,
    #[doc = "0x130 - ETC_TRIG Chain 0/1 Register"]
    pub trig7_chain_1_0: TRIG7_CHAIN_1_0,
    #[doc = "0x134 - ETC_TRIG Chain 2/3 Register"]
    pub trig7_chain_3_2: TRIG7_CHAIN_3_2,
    #[doc = "0x138 - ETC_TRIG Chain 4/5 Register"]
    pub trig7_chain_5_4: TRIG7_CHAIN_5_4,
    #[doc = "0x13c - ETC_TRIG Chain 6/7 Register"]
    pub trig7_chain_7_6: TRIG7_CHAIN_7_6,
    #[doc = "0x140 - ETC_TRIG Result Data 1/0 Register"]
    pub trig7_result_1_0: TRIG7_RESULT_1_0,
    #[doc = "0x144 - ETC_TRIG Result Data 3/2 Register"]
    pub trig7_result_3_2: TRIG7_RESULT_3_2,
    #[doc = "0x148 - ETC_TRIG Result Data 5/4 Register"]
    pub trig7_result_5_4: TRIG7_RESULT_5_4,
    #[doc = "0x14c - ETC_TRIG Result Data 7/6 Register"]
    pub trig7_result_7_6: TRIG7_RESULT_7_6,
}
#[doc = "ADC_ETC Global Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "ADC_ETC Global Control Register"]
pub mod ctrl;
#[doc = "ETC DONE0 and DONE1 IRQ State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [done0_1_irq](done0_1_irq) module"]
pub type DONE0_1_IRQ = crate::Reg<u32, _DONE0_1_IRQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DONE0_1_IRQ;
#[doc = "`read()` method returns [done0_1_irq::R](done0_1_irq::R) reader structure"]
impl crate::Readable for DONE0_1_IRQ {}
#[doc = "`write(|w| ..)` method takes [done0_1_irq::W](done0_1_irq::W) writer structure"]
impl crate::Writable for DONE0_1_IRQ {}
#[doc = "ETC DONE0 and DONE1 IRQ State Register"]
pub mod done0_1_irq;
#[doc = "ETC DONE_2 and DONE_ERR IRQ State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [done2_err_irq](done2_err_irq) module"]
pub type DONE2_ERR_IRQ = crate::Reg<u32, _DONE2_ERR_IRQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DONE2_ERR_IRQ;
#[doc = "`read()` method returns [done2_err_irq::R](done2_err_irq::R) reader structure"]
impl crate::Readable for DONE2_ERR_IRQ {}
#[doc = "`write(|w| ..)` method takes [done2_err_irq::W](done2_err_irq::W) writer structure"]
impl crate::Writable for DONE2_ERR_IRQ {}
#[doc = "ETC DONE_2 and DONE_ERR IRQ State Register"]
pub mod done2_err_irq;
#[doc = "ETC DMA control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_ctrl](dma_ctrl) module"]
pub type DMA_CTRL = crate::Reg<u32, _DMA_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CTRL;
#[doc = "`read()` method returns [dma_ctrl::R](dma_ctrl::R) reader structure"]
impl crate::Readable for DMA_CTRL {}
#[doc = "`write(|w| ..)` method takes [dma_ctrl::W](dma_ctrl::W) writer structure"]
impl crate::Writable for DMA_CTRL {}
#[doc = "ETC DMA control Register"]
pub mod dma_ctrl;
#[doc = "ETC_TRIG0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig0_ctrl](trig0_ctrl) module"]
pub type TRIG0_CTRL = crate::Reg<u32, _TRIG0_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG0_CTRL;
#[doc = "`read()` method returns [trig0_ctrl::R](trig0_ctrl::R) reader structure"]
impl crate::Readable for TRIG0_CTRL {}
#[doc = "`write(|w| ..)` method takes [trig0_ctrl::W](trig0_ctrl::W) writer structure"]
impl crate::Writable for TRIG0_CTRL {}
#[doc = "ETC_TRIG0 Control Register"]
pub mod trig0_ctrl;
#[doc = "ETC_TRIG0 Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig0_counter](trig0_counter) module"]
pub type TRIG0_COUNTER = crate::Reg<u32, _TRIG0_COUNTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG0_COUNTER;
#[doc = "`read()` method returns [trig0_counter::R](trig0_counter::R) reader structure"]
impl crate::Readable for TRIG0_COUNTER {}
#[doc = "`write(|w| ..)` method takes [trig0_counter::W](trig0_counter::W) writer structure"]
impl crate::Writable for TRIG0_COUNTER {}
#[doc = "ETC_TRIG0 Counter Register"]
pub mod trig0_counter;
#[doc = "ETC_TRIG Chain 0/1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig0_chain_1_0](trig0_chain_1_0) module"]
pub type TRIG0_CHAIN_1_0 = crate::Reg<u32, _TRIG0_CHAIN_1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG0_CHAIN_1_0;
#[doc = "`read()` method returns [trig0_chain_1_0::R](trig0_chain_1_0::R) reader structure"]
impl crate::Readable for TRIG0_CHAIN_1_0 {}
#[doc = "`write(|w| ..)` method takes [trig0_chain_1_0::W](trig0_chain_1_0::W) writer structure"]
impl crate::Writable for TRIG0_CHAIN_1_0 {}
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod trig0_chain_1_0;
#[doc = "ETC_TRIG Chain 2/3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig0_chain_3_2](trig0_chain_3_2) module"]
pub type TRIG0_CHAIN_3_2 = crate::Reg<u32, _TRIG0_CHAIN_3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG0_CHAIN_3_2;
#[doc = "`read()` method returns [trig0_chain_3_2::R](trig0_chain_3_2::R) reader structure"]
impl crate::Readable for TRIG0_CHAIN_3_2 {}
#[doc = "`write(|w| ..)` method takes [trig0_chain_3_2::W](trig0_chain_3_2::W) writer structure"]
impl crate::Writable for TRIG0_CHAIN_3_2 {}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod trig0_chain_3_2;
#[doc = "ETC_TRIG Chain 4/5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig0_chain_5_4](trig0_chain_5_4) module"]
pub type TRIG0_CHAIN_5_4 = crate::Reg<u32, _TRIG0_CHAIN_5_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG0_CHAIN_5_4;
#[doc = "`read()` method returns [trig0_chain_5_4::R](trig0_chain_5_4::R) reader structure"]
impl crate::Readable for TRIG0_CHAIN_5_4 {}
#[doc = "`write(|w| ..)` method takes [trig0_chain_5_4::W](trig0_chain_5_4::W) writer structure"]
impl crate::Writable for TRIG0_CHAIN_5_4 {}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod trig0_chain_5_4;
#[doc = "ETC_TRIG Chain 6/7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig0_chain_7_6](trig0_chain_7_6) module"]
pub type TRIG0_CHAIN_7_6 = crate::Reg<u32, _TRIG0_CHAIN_7_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG0_CHAIN_7_6;
#[doc = "`read()` method returns [trig0_chain_7_6::R](trig0_chain_7_6::R) reader structure"]
impl crate::Readable for TRIG0_CHAIN_7_6 {}
#[doc = "`write(|w| ..)` method takes [trig0_chain_7_6::W](trig0_chain_7_6::W) writer structure"]
impl crate::Writable for TRIG0_CHAIN_7_6 {}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod trig0_chain_7_6;
#[doc = "ETC_TRIG Result Data 1/0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig0_result_1_0](trig0_result_1_0) module"]
pub type TRIG0_RESULT_1_0 = crate::Reg<u32, _TRIG0_RESULT_1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG0_RESULT_1_0;
#[doc = "`read()` method returns [trig0_result_1_0::R](trig0_result_1_0::R) reader structure"]
impl crate::Readable for TRIG0_RESULT_1_0 {}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod trig0_result_1_0;
#[doc = "ETC_TRIG Result Data 3/2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig0_result_3_2](trig0_result_3_2) module"]
pub type TRIG0_RESULT_3_2 = crate::Reg<u32, _TRIG0_RESULT_3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG0_RESULT_3_2;
#[doc = "`read()` method returns [trig0_result_3_2::R](trig0_result_3_2::R) reader structure"]
impl crate::Readable for TRIG0_RESULT_3_2 {}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod trig0_result_3_2;
#[doc = "ETC_TRIG Result Data 5/4 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig0_result_5_4](trig0_result_5_4) module"]
pub type TRIG0_RESULT_5_4 = crate::Reg<u32, _TRIG0_RESULT_5_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG0_RESULT_5_4;
#[doc = "`read()` method returns [trig0_result_5_4::R](trig0_result_5_4::R) reader structure"]
impl crate::Readable for TRIG0_RESULT_5_4 {}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod trig0_result_5_4;
#[doc = "ETC_TRIG Result Data 7/6 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig0_result_7_6](trig0_result_7_6) module"]
pub type TRIG0_RESULT_7_6 = crate::Reg<u32, _TRIG0_RESULT_7_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG0_RESULT_7_6;
#[doc = "`read()` method returns [trig0_result_7_6::R](trig0_result_7_6::R) reader structure"]
impl crate::Readable for TRIG0_RESULT_7_6 {}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod trig0_result_7_6;
#[doc = "ETC_TRIG1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig1_ctrl](trig1_ctrl) module"]
pub type TRIG1_CTRL = crate::Reg<u32, _TRIG1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG1_CTRL;
#[doc = "`read()` method returns [trig1_ctrl::R](trig1_ctrl::R) reader structure"]
impl crate::Readable for TRIG1_CTRL {}
#[doc = "`write(|w| ..)` method takes [trig1_ctrl::W](trig1_ctrl::W) writer structure"]
impl crate::Writable for TRIG1_CTRL {}
#[doc = "ETC_TRIG1 Control Register"]
pub mod trig1_ctrl;
#[doc = "ETC_TRIG1 Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig1_counter](trig1_counter) module"]
pub type TRIG1_COUNTER = crate::Reg<u32, _TRIG1_COUNTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG1_COUNTER;
#[doc = "`read()` method returns [trig1_counter::R](trig1_counter::R) reader structure"]
impl crate::Readable for TRIG1_COUNTER {}
#[doc = "`write(|w| ..)` method takes [trig1_counter::W](trig1_counter::W) writer structure"]
impl crate::Writable for TRIG1_COUNTER {}
#[doc = "ETC_TRIG1 Counter Register"]
pub mod trig1_counter;
#[doc = "ETC_TRIG Chain 0/1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig1_chain_1_0](trig1_chain_1_0) module"]
pub type TRIG1_CHAIN_1_0 = crate::Reg<u32, _TRIG1_CHAIN_1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG1_CHAIN_1_0;
#[doc = "`read()` method returns [trig1_chain_1_0::R](trig1_chain_1_0::R) reader structure"]
impl crate::Readable for TRIG1_CHAIN_1_0 {}
#[doc = "`write(|w| ..)` method takes [trig1_chain_1_0::W](trig1_chain_1_0::W) writer structure"]
impl crate::Writable for TRIG1_CHAIN_1_0 {}
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod trig1_chain_1_0;
#[doc = "ETC_TRIG Chain 2/3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig1_chain_3_2](trig1_chain_3_2) module"]
pub type TRIG1_CHAIN_3_2 = crate::Reg<u32, _TRIG1_CHAIN_3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG1_CHAIN_3_2;
#[doc = "`read()` method returns [trig1_chain_3_2::R](trig1_chain_3_2::R) reader structure"]
impl crate::Readable for TRIG1_CHAIN_3_2 {}
#[doc = "`write(|w| ..)` method takes [trig1_chain_3_2::W](trig1_chain_3_2::W) writer structure"]
impl crate::Writable for TRIG1_CHAIN_3_2 {}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod trig1_chain_3_2;
#[doc = "ETC_TRIG Chain 4/5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig1_chain_5_4](trig1_chain_5_4) module"]
pub type TRIG1_CHAIN_5_4 = crate::Reg<u32, _TRIG1_CHAIN_5_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG1_CHAIN_5_4;
#[doc = "`read()` method returns [trig1_chain_5_4::R](trig1_chain_5_4::R) reader structure"]
impl crate::Readable for TRIG1_CHAIN_5_4 {}
#[doc = "`write(|w| ..)` method takes [trig1_chain_5_4::W](trig1_chain_5_4::W) writer structure"]
impl crate::Writable for TRIG1_CHAIN_5_4 {}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod trig1_chain_5_4;
#[doc = "ETC_TRIG Chain 6/7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig1_chain_7_6](trig1_chain_7_6) module"]
pub type TRIG1_CHAIN_7_6 = crate::Reg<u32, _TRIG1_CHAIN_7_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG1_CHAIN_7_6;
#[doc = "`read()` method returns [trig1_chain_7_6::R](trig1_chain_7_6::R) reader structure"]
impl crate::Readable for TRIG1_CHAIN_7_6 {}
#[doc = "`write(|w| ..)` method takes [trig1_chain_7_6::W](trig1_chain_7_6::W) writer structure"]
impl crate::Writable for TRIG1_CHAIN_7_6 {}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod trig1_chain_7_6;
#[doc = "ETC_TRIG Result Data 1/0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig1_result_1_0](trig1_result_1_0) module"]
pub type TRIG1_RESULT_1_0 = crate::Reg<u32, _TRIG1_RESULT_1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG1_RESULT_1_0;
#[doc = "`read()` method returns [trig1_result_1_0::R](trig1_result_1_0::R) reader structure"]
impl crate::Readable for TRIG1_RESULT_1_0 {}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod trig1_result_1_0;
#[doc = "ETC_TRIG Result Data 3/2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig1_result_3_2](trig1_result_3_2) module"]
pub type TRIG1_RESULT_3_2 = crate::Reg<u32, _TRIG1_RESULT_3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG1_RESULT_3_2;
#[doc = "`read()` method returns [trig1_result_3_2::R](trig1_result_3_2::R) reader structure"]
impl crate::Readable for TRIG1_RESULT_3_2 {}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod trig1_result_3_2;
#[doc = "ETC_TRIG Result Data 5/4 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig1_result_5_4](trig1_result_5_4) module"]
pub type TRIG1_RESULT_5_4 = crate::Reg<u32, _TRIG1_RESULT_5_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG1_RESULT_5_4;
#[doc = "`read()` method returns [trig1_result_5_4::R](trig1_result_5_4::R) reader structure"]
impl crate::Readable for TRIG1_RESULT_5_4 {}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod trig1_result_5_4;
#[doc = "ETC_TRIG Result Data 7/6 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig1_result_7_6](trig1_result_7_6) module"]
pub type TRIG1_RESULT_7_6 = crate::Reg<u32, _TRIG1_RESULT_7_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG1_RESULT_7_6;
#[doc = "`read()` method returns [trig1_result_7_6::R](trig1_result_7_6::R) reader structure"]
impl crate::Readable for TRIG1_RESULT_7_6 {}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod trig1_result_7_6;
#[doc = "ETC_TRIG2 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig2_ctrl](trig2_ctrl) module"]
pub type TRIG2_CTRL = crate::Reg<u32, _TRIG2_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG2_CTRL;
#[doc = "`read()` method returns [trig2_ctrl::R](trig2_ctrl::R) reader structure"]
impl crate::Readable for TRIG2_CTRL {}
#[doc = "`write(|w| ..)` method takes [trig2_ctrl::W](trig2_ctrl::W) writer structure"]
impl crate::Writable for TRIG2_CTRL {}
#[doc = "ETC_TRIG2 Control Register"]
pub mod trig2_ctrl;
#[doc = "ETC_TRIG2 Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig2_counter](trig2_counter) module"]
pub type TRIG2_COUNTER = crate::Reg<u32, _TRIG2_COUNTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG2_COUNTER;
#[doc = "`read()` method returns [trig2_counter::R](trig2_counter::R) reader structure"]
impl crate::Readable for TRIG2_COUNTER {}
#[doc = "`write(|w| ..)` method takes [trig2_counter::W](trig2_counter::W) writer structure"]
impl crate::Writable for TRIG2_COUNTER {}
#[doc = "ETC_TRIG2 Counter Register"]
pub mod trig2_counter;
#[doc = "ETC_TRIG Chain 0/1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig2_chain_1_0](trig2_chain_1_0) module"]
pub type TRIG2_CHAIN_1_0 = crate::Reg<u32, _TRIG2_CHAIN_1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG2_CHAIN_1_0;
#[doc = "`read()` method returns [trig2_chain_1_0::R](trig2_chain_1_0::R) reader structure"]
impl crate::Readable for TRIG2_CHAIN_1_0 {}
#[doc = "`write(|w| ..)` method takes [trig2_chain_1_0::W](trig2_chain_1_0::W) writer structure"]
impl crate::Writable for TRIG2_CHAIN_1_0 {}
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod trig2_chain_1_0;
#[doc = "ETC_TRIG Chain 2/3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig2_chain_3_2](trig2_chain_3_2) module"]
pub type TRIG2_CHAIN_3_2 = crate::Reg<u32, _TRIG2_CHAIN_3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG2_CHAIN_3_2;
#[doc = "`read()` method returns [trig2_chain_3_2::R](trig2_chain_3_2::R) reader structure"]
impl crate::Readable for TRIG2_CHAIN_3_2 {}
#[doc = "`write(|w| ..)` method takes [trig2_chain_3_2::W](trig2_chain_3_2::W) writer structure"]
impl crate::Writable for TRIG2_CHAIN_3_2 {}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod trig2_chain_3_2;
#[doc = "ETC_TRIG Chain 4/5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig2_chain_5_4](trig2_chain_5_4) module"]
pub type TRIG2_CHAIN_5_4 = crate::Reg<u32, _TRIG2_CHAIN_5_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG2_CHAIN_5_4;
#[doc = "`read()` method returns [trig2_chain_5_4::R](trig2_chain_5_4::R) reader structure"]
impl crate::Readable for TRIG2_CHAIN_5_4 {}
#[doc = "`write(|w| ..)` method takes [trig2_chain_5_4::W](trig2_chain_5_4::W) writer structure"]
impl crate::Writable for TRIG2_CHAIN_5_4 {}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod trig2_chain_5_4;
#[doc = "ETC_TRIG Chain 6/7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig2_chain_7_6](trig2_chain_7_6) module"]
pub type TRIG2_CHAIN_7_6 = crate::Reg<u32, _TRIG2_CHAIN_7_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG2_CHAIN_7_6;
#[doc = "`read()` method returns [trig2_chain_7_6::R](trig2_chain_7_6::R) reader structure"]
impl crate::Readable for TRIG2_CHAIN_7_6 {}
#[doc = "`write(|w| ..)` method takes [trig2_chain_7_6::W](trig2_chain_7_6::W) writer structure"]
impl crate::Writable for TRIG2_CHAIN_7_6 {}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod trig2_chain_7_6;
#[doc = "ETC_TRIG Result Data 1/0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig2_result_1_0](trig2_result_1_0) module"]
pub type TRIG2_RESULT_1_0 = crate::Reg<u32, _TRIG2_RESULT_1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG2_RESULT_1_0;
#[doc = "`read()` method returns [trig2_result_1_0::R](trig2_result_1_0::R) reader structure"]
impl crate::Readable for TRIG2_RESULT_1_0 {}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod trig2_result_1_0;
#[doc = "ETC_TRIG Result Data 3/2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig2_result_3_2](trig2_result_3_2) module"]
pub type TRIG2_RESULT_3_2 = crate::Reg<u32, _TRIG2_RESULT_3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG2_RESULT_3_2;
#[doc = "`read()` method returns [trig2_result_3_2::R](trig2_result_3_2::R) reader structure"]
impl crate::Readable for TRIG2_RESULT_3_2 {}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod trig2_result_3_2;
#[doc = "ETC_TRIG Result Data 5/4 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig2_result_5_4](trig2_result_5_4) module"]
pub type TRIG2_RESULT_5_4 = crate::Reg<u32, _TRIG2_RESULT_5_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG2_RESULT_5_4;
#[doc = "`read()` method returns [trig2_result_5_4::R](trig2_result_5_4::R) reader structure"]
impl crate::Readable for TRIG2_RESULT_5_4 {}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod trig2_result_5_4;
#[doc = "ETC_TRIG Result Data 7/6 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig2_result_7_6](trig2_result_7_6) module"]
pub type TRIG2_RESULT_7_6 = crate::Reg<u32, _TRIG2_RESULT_7_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG2_RESULT_7_6;
#[doc = "`read()` method returns [trig2_result_7_6::R](trig2_result_7_6::R) reader structure"]
impl crate::Readable for TRIG2_RESULT_7_6 {}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod trig2_result_7_6;
#[doc = "ETC_TRIG3 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig3_ctrl](trig3_ctrl) module"]
pub type TRIG3_CTRL = crate::Reg<u32, _TRIG3_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG3_CTRL;
#[doc = "`read()` method returns [trig3_ctrl::R](trig3_ctrl::R) reader structure"]
impl crate::Readable for TRIG3_CTRL {}
#[doc = "`write(|w| ..)` method takes [trig3_ctrl::W](trig3_ctrl::W) writer structure"]
impl crate::Writable for TRIG3_CTRL {}
#[doc = "ETC_TRIG3 Control Register"]
pub mod trig3_ctrl;
#[doc = "ETC_TRIG3 Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig3_counter](trig3_counter) module"]
pub type TRIG3_COUNTER = crate::Reg<u32, _TRIG3_COUNTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG3_COUNTER;
#[doc = "`read()` method returns [trig3_counter::R](trig3_counter::R) reader structure"]
impl crate::Readable for TRIG3_COUNTER {}
#[doc = "`write(|w| ..)` method takes [trig3_counter::W](trig3_counter::W) writer structure"]
impl crate::Writable for TRIG3_COUNTER {}
#[doc = "ETC_TRIG3 Counter Register"]
pub mod trig3_counter;
#[doc = "ETC_TRIG Chain 0/1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig3_chain_1_0](trig3_chain_1_0) module"]
pub type TRIG3_CHAIN_1_0 = crate::Reg<u32, _TRIG3_CHAIN_1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG3_CHAIN_1_0;
#[doc = "`read()` method returns [trig3_chain_1_0::R](trig3_chain_1_0::R) reader structure"]
impl crate::Readable for TRIG3_CHAIN_1_0 {}
#[doc = "`write(|w| ..)` method takes [trig3_chain_1_0::W](trig3_chain_1_0::W) writer structure"]
impl crate::Writable for TRIG3_CHAIN_1_0 {}
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod trig3_chain_1_0;
#[doc = "ETC_TRIG Chain 2/3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig3_chain_3_2](trig3_chain_3_2) module"]
pub type TRIG3_CHAIN_3_2 = crate::Reg<u32, _TRIG3_CHAIN_3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG3_CHAIN_3_2;
#[doc = "`read()` method returns [trig3_chain_3_2::R](trig3_chain_3_2::R) reader structure"]
impl crate::Readable for TRIG3_CHAIN_3_2 {}
#[doc = "`write(|w| ..)` method takes [trig3_chain_3_2::W](trig3_chain_3_2::W) writer structure"]
impl crate::Writable for TRIG3_CHAIN_3_2 {}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod trig3_chain_3_2;
#[doc = "ETC_TRIG Chain 4/5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig3_chain_5_4](trig3_chain_5_4) module"]
pub type TRIG3_CHAIN_5_4 = crate::Reg<u32, _TRIG3_CHAIN_5_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG3_CHAIN_5_4;
#[doc = "`read()` method returns [trig3_chain_5_4::R](trig3_chain_5_4::R) reader structure"]
impl crate::Readable for TRIG3_CHAIN_5_4 {}
#[doc = "`write(|w| ..)` method takes [trig3_chain_5_4::W](trig3_chain_5_4::W) writer structure"]
impl crate::Writable for TRIG3_CHAIN_5_4 {}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod trig3_chain_5_4;
#[doc = "ETC_TRIG Chain 6/7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig3_chain_7_6](trig3_chain_7_6) module"]
pub type TRIG3_CHAIN_7_6 = crate::Reg<u32, _TRIG3_CHAIN_7_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG3_CHAIN_7_6;
#[doc = "`read()` method returns [trig3_chain_7_6::R](trig3_chain_7_6::R) reader structure"]
impl crate::Readable for TRIG3_CHAIN_7_6 {}
#[doc = "`write(|w| ..)` method takes [trig3_chain_7_6::W](trig3_chain_7_6::W) writer structure"]
impl crate::Writable for TRIG3_CHAIN_7_6 {}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod trig3_chain_7_6;
#[doc = "ETC_TRIG Result Data 1/0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig3_result_1_0](trig3_result_1_0) module"]
pub type TRIG3_RESULT_1_0 = crate::Reg<u32, _TRIG3_RESULT_1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG3_RESULT_1_0;
#[doc = "`read()` method returns [trig3_result_1_0::R](trig3_result_1_0::R) reader structure"]
impl crate::Readable for TRIG3_RESULT_1_0 {}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod trig3_result_1_0;
#[doc = "ETC_TRIG Result Data 3/2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig3_result_3_2](trig3_result_3_2) module"]
pub type TRIG3_RESULT_3_2 = crate::Reg<u32, _TRIG3_RESULT_3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG3_RESULT_3_2;
#[doc = "`read()` method returns [trig3_result_3_2::R](trig3_result_3_2::R) reader structure"]
impl crate::Readable for TRIG3_RESULT_3_2 {}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod trig3_result_3_2;
#[doc = "ETC_TRIG Result Data 5/4 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig3_result_5_4](trig3_result_5_4) module"]
pub type TRIG3_RESULT_5_4 = crate::Reg<u32, _TRIG3_RESULT_5_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG3_RESULT_5_4;
#[doc = "`read()` method returns [trig3_result_5_4::R](trig3_result_5_4::R) reader structure"]
impl crate::Readable for TRIG3_RESULT_5_4 {}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod trig3_result_5_4;
#[doc = "ETC_TRIG Result Data 7/6 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig3_result_7_6](trig3_result_7_6) module"]
pub type TRIG3_RESULT_7_6 = crate::Reg<u32, _TRIG3_RESULT_7_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG3_RESULT_7_6;
#[doc = "`read()` method returns [trig3_result_7_6::R](trig3_result_7_6::R) reader structure"]
impl crate::Readable for TRIG3_RESULT_7_6 {}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod trig3_result_7_6;
#[doc = "ETC_TRIG4 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig4_ctrl](trig4_ctrl) module"]
pub type TRIG4_CTRL = crate::Reg<u32, _TRIG4_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG4_CTRL;
#[doc = "`read()` method returns [trig4_ctrl::R](trig4_ctrl::R) reader structure"]
impl crate::Readable for TRIG4_CTRL {}
#[doc = "`write(|w| ..)` method takes [trig4_ctrl::W](trig4_ctrl::W) writer structure"]
impl crate::Writable for TRIG4_CTRL {}
#[doc = "ETC_TRIG4 Control Register"]
pub mod trig4_ctrl;
#[doc = "ETC_TRIG4 Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig4_counter](trig4_counter) module"]
pub type TRIG4_COUNTER = crate::Reg<u32, _TRIG4_COUNTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG4_COUNTER;
#[doc = "`read()` method returns [trig4_counter::R](trig4_counter::R) reader structure"]
impl crate::Readable for TRIG4_COUNTER {}
#[doc = "`write(|w| ..)` method takes [trig4_counter::W](trig4_counter::W) writer structure"]
impl crate::Writable for TRIG4_COUNTER {}
#[doc = "ETC_TRIG4 Counter Register"]
pub mod trig4_counter;
#[doc = "ETC_TRIG Chain 0/1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig4_chain_1_0](trig4_chain_1_0) module"]
pub type TRIG4_CHAIN_1_0 = crate::Reg<u32, _TRIG4_CHAIN_1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG4_CHAIN_1_0;
#[doc = "`read()` method returns [trig4_chain_1_0::R](trig4_chain_1_0::R) reader structure"]
impl crate::Readable for TRIG4_CHAIN_1_0 {}
#[doc = "`write(|w| ..)` method takes [trig4_chain_1_0::W](trig4_chain_1_0::W) writer structure"]
impl crate::Writable for TRIG4_CHAIN_1_0 {}
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod trig4_chain_1_0;
#[doc = "ETC_TRIG Chain 2/3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig4_chain_3_2](trig4_chain_3_2) module"]
pub type TRIG4_CHAIN_3_2 = crate::Reg<u32, _TRIG4_CHAIN_3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG4_CHAIN_3_2;
#[doc = "`read()` method returns [trig4_chain_3_2::R](trig4_chain_3_2::R) reader structure"]
impl crate::Readable for TRIG4_CHAIN_3_2 {}
#[doc = "`write(|w| ..)` method takes [trig4_chain_3_2::W](trig4_chain_3_2::W) writer structure"]
impl crate::Writable for TRIG4_CHAIN_3_2 {}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod trig4_chain_3_2;
#[doc = "ETC_TRIG Chain 4/5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig4_chain_5_4](trig4_chain_5_4) module"]
pub type TRIG4_CHAIN_5_4 = crate::Reg<u32, _TRIG4_CHAIN_5_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG4_CHAIN_5_4;
#[doc = "`read()` method returns [trig4_chain_5_4::R](trig4_chain_5_4::R) reader structure"]
impl crate::Readable for TRIG4_CHAIN_5_4 {}
#[doc = "`write(|w| ..)` method takes [trig4_chain_5_4::W](trig4_chain_5_4::W) writer structure"]
impl crate::Writable for TRIG4_CHAIN_5_4 {}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod trig4_chain_5_4;
#[doc = "ETC_TRIG Chain 6/7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig4_chain_7_6](trig4_chain_7_6) module"]
pub type TRIG4_CHAIN_7_6 = crate::Reg<u32, _TRIG4_CHAIN_7_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG4_CHAIN_7_6;
#[doc = "`read()` method returns [trig4_chain_7_6::R](trig4_chain_7_6::R) reader structure"]
impl crate::Readable for TRIG4_CHAIN_7_6 {}
#[doc = "`write(|w| ..)` method takes [trig4_chain_7_6::W](trig4_chain_7_6::W) writer structure"]
impl crate::Writable for TRIG4_CHAIN_7_6 {}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod trig4_chain_7_6;
#[doc = "ETC_TRIG Result Data 1/0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig4_result_1_0](trig4_result_1_0) module"]
pub type TRIG4_RESULT_1_0 = crate::Reg<u32, _TRIG4_RESULT_1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG4_RESULT_1_0;
#[doc = "`read()` method returns [trig4_result_1_0::R](trig4_result_1_0::R) reader structure"]
impl crate::Readable for TRIG4_RESULT_1_0 {}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod trig4_result_1_0;
#[doc = "ETC_TRIG Result Data 3/2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig4_result_3_2](trig4_result_3_2) module"]
pub type TRIG4_RESULT_3_2 = crate::Reg<u32, _TRIG4_RESULT_3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG4_RESULT_3_2;
#[doc = "`read()` method returns [trig4_result_3_2::R](trig4_result_3_2::R) reader structure"]
impl crate::Readable for TRIG4_RESULT_3_2 {}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod trig4_result_3_2;
#[doc = "ETC_TRIG Result Data 5/4 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig4_result_5_4](trig4_result_5_4) module"]
pub type TRIG4_RESULT_5_4 = crate::Reg<u32, _TRIG4_RESULT_5_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG4_RESULT_5_4;
#[doc = "`read()` method returns [trig4_result_5_4::R](trig4_result_5_4::R) reader structure"]
impl crate::Readable for TRIG4_RESULT_5_4 {}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod trig4_result_5_4;
#[doc = "ETC_TRIG Result Data 7/6 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig4_result_7_6](trig4_result_7_6) module"]
pub type TRIG4_RESULT_7_6 = crate::Reg<u32, _TRIG4_RESULT_7_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG4_RESULT_7_6;
#[doc = "`read()` method returns [trig4_result_7_6::R](trig4_result_7_6::R) reader structure"]
impl crate::Readable for TRIG4_RESULT_7_6 {}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod trig4_result_7_6;
#[doc = "ETC_TRIG5 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig5_ctrl](trig5_ctrl) module"]
pub type TRIG5_CTRL = crate::Reg<u32, _TRIG5_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG5_CTRL;
#[doc = "`read()` method returns [trig5_ctrl::R](trig5_ctrl::R) reader structure"]
impl crate::Readable for TRIG5_CTRL {}
#[doc = "`write(|w| ..)` method takes [trig5_ctrl::W](trig5_ctrl::W) writer structure"]
impl crate::Writable for TRIG5_CTRL {}
#[doc = "ETC_TRIG5 Control Register"]
pub mod trig5_ctrl;
#[doc = "ETC_TRIG5 Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig5_counter](trig5_counter) module"]
pub type TRIG5_COUNTER = crate::Reg<u32, _TRIG5_COUNTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG5_COUNTER;
#[doc = "`read()` method returns [trig5_counter::R](trig5_counter::R) reader structure"]
impl crate::Readable for TRIG5_COUNTER {}
#[doc = "`write(|w| ..)` method takes [trig5_counter::W](trig5_counter::W) writer structure"]
impl crate::Writable for TRIG5_COUNTER {}
#[doc = "ETC_TRIG5 Counter Register"]
pub mod trig5_counter;
#[doc = "ETC_TRIG Chain 0/1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig5_chain_1_0](trig5_chain_1_0) module"]
pub type TRIG5_CHAIN_1_0 = crate::Reg<u32, _TRIG5_CHAIN_1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG5_CHAIN_1_0;
#[doc = "`read()` method returns [trig5_chain_1_0::R](trig5_chain_1_0::R) reader structure"]
impl crate::Readable for TRIG5_CHAIN_1_0 {}
#[doc = "`write(|w| ..)` method takes [trig5_chain_1_0::W](trig5_chain_1_0::W) writer structure"]
impl crate::Writable for TRIG5_CHAIN_1_0 {}
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod trig5_chain_1_0;
#[doc = "ETC_TRIG Chain 2/3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig5_chain_3_2](trig5_chain_3_2) module"]
pub type TRIG5_CHAIN_3_2 = crate::Reg<u32, _TRIG5_CHAIN_3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG5_CHAIN_3_2;
#[doc = "`read()` method returns [trig5_chain_3_2::R](trig5_chain_3_2::R) reader structure"]
impl crate::Readable for TRIG5_CHAIN_3_2 {}
#[doc = "`write(|w| ..)` method takes [trig5_chain_3_2::W](trig5_chain_3_2::W) writer structure"]
impl crate::Writable for TRIG5_CHAIN_3_2 {}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod trig5_chain_3_2;
#[doc = "ETC_TRIG Chain 4/5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig5_chain_5_4](trig5_chain_5_4) module"]
pub type TRIG5_CHAIN_5_4 = crate::Reg<u32, _TRIG5_CHAIN_5_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG5_CHAIN_5_4;
#[doc = "`read()` method returns [trig5_chain_5_4::R](trig5_chain_5_4::R) reader structure"]
impl crate::Readable for TRIG5_CHAIN_5_4 {}
#[doc = "`write(|w| ..)` method takes [trig5_chain_5_4::W](trig5_chain_5_4::W) writer structure"]
impl crate::Writable for TRIG5_CHAIN_5_4 {}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod trig5_chain_5_4;
#[doc = "ETC_TRIG Chain 6/7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig5_chain_7_6](trig5_chain_7_6) module"]
pub type TRIG5_CHAIN_7_6 = crate::Reg<u32, _TRIG5_CHAIN_7_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG5_CHAIN_7_6;
#[doc = "`read()` method returns [trig5_chain_7_6::R](trig5_chain_7_6::R) reader structure"]
impl crate::Readable for TRIG5_CHAIN_7_6 {}
#[doc = "`write(|w| ..)` method takes [trig5_chain_7_6::W](trig5_chain_7_6::W) writer structure"]
impl crate::Writable for TRIG5_CHAIN_7_6 {}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod trig5_chain_7_6;
#[doc = "ETC_TRIG Result Data 1/0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig5_result_1_0](trig5_result_1_0) module"]
pub type TRIG5_RESULT_1_0 = crate::Reg<u32, _TRIG5_RESULT_1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG5_RESULT_1_0;
#[doc = "`read()` method returns [trig5_result_1_0::R](trig5_result_1_0::R) reader structure"]
impl crate::Readable for TRIG5_RESULT_1_0 {}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod trig5_result_1_0;
#[doc = "ETC_TRIG Result Data 3/2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig5_result_3_2](trig5_result_3_2) module"]
pub type TRIG5_RESULT_3_2 = crate::Reg<u32, _TRIG5_RESULT_3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG5_RESULT_3_2;
#[doc = "`read()` method returns [trig5_result_3_2::R](trig5_result_3_2::R) reader structure"]
impl crate::Readable for TRIG5_RESULT_3_2 {}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod trig5_result_3_2;
#[doc = "ETC_TRIG Result Data 5/4 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig5_result_5_4](trig5_result_5_4) module"]
pub type TRIG5_RESULT_5_4 = crate::Reg<u32, _TRIG5_RESULT_5_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG5_RESULT_5_4;
#[doc = "`read()` method returns [trig5_result_5_4::R](trig5_result_5_4::R) reader structure"]
impl crate::Readable for TRIG5_RESULT_5_4 {}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod trig5_result_5_4;
#[doc = "ETC_TRIG Result Data 7/6 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig5_result_7_6](trig5_result_7_6) module"]
pub type TRIG5_RESULT_7_6 = crate::Reg<u32, _TRIG5_RESULT_7_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG5_RESULT_7_6;
#[doc = "`read()` method returns [trig5_result_7_6::R](trig5_result_7_6::R) reader structure"]
impl crate::Readable for TRIG5_RESULT_7_6 {}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod trig5_result_7_6;
#[doc = "ETC_TRIG6 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig6_ctrl](trig6_ctrl) module"]
pub type TRIG6_CTRL = crate::Reg<u32, _TRIG6_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG6_CTRL;
#[doc = "`read()` method returns [trig6_ctrl::R](trig6_ctrl::R) reader structure"]
impl crate::Readable for TRIG6_CTRL {}
#[doc = "`write(|w| ..)` method takes [trig6_ctrl::W](trig6_ctrl::W) writer structure"]
impl crate::Writable for TRIG6_CTRL {}
#[doc = "ETC_TRIG6 Control Register"]
pub mod trig6_ctrl;
#[doc = "ETC_TRIG6 Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig6_counter](trig6_counter) module"]
pub type TRIG6_COUNTER = crate::Reg<u32, _TRIG6_COUNTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG6_COUNTER;
#[doc = "`read()` method returns [trig6_counter::R](trig6_counter::R) reader structure"]
impl crate::Readable for TRIG6_COUNTER {}
#[doc = "`write(|w| ..)` method takes [trig6_counter::W](trig6_counter::W) writer structure"]
impl crate::Writable for TRIG6_COUNTER {}
#[doc = "ETC_TRIG6 Counter Register"]
pub mod trig6_counter;
#[doc = "ETC_TRIG Chain 0/1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig6_chain_1_0](trig6_chain_1_0) module"]
pub type TRIG6_CHAIN_1_0 = crate::Reg<u32, _TRIG6_CHAIN_1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG6_CHAIN_1_0;
#[doc = "`read()` method returns [trig6_chain_1_0::R](trig6_chain_1_0::R) reader structure"]
impl crate::Readable for TRIG6_CHAIN_1_0 {}
#[doc = "`write(|w| ..)` method takes [trig6_chain_1_0::W](trig6_chain_1_0::W) writer structure"]
impl crate::Writable for TRIG6_CHAIN_1_0 {}
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod trig6_chain_1_0;
#[doc = "ETC_TRIG Chain 2/3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig6_chain_3_2](trig6_chain_3_2) module"]
pub type TRIG6_CHAIN_3_2 = crate::Reg<u32, _TRIG6_CHAIN_3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG6_CHAIN_3_2;
#[doc = "`read()` method returns [trig6_chain_3_2::R](trig6_chain_3_2::R) reader structure"]
impl crate::Readable for TRIG6_CHAIN_3_2 {}
#[doc = "`write(|w| ..)` method takes [trig6_chain_3_2::W](trig6_chain_3_2::W) writer structure"]
impl crate::Writable for TRIG6_CHAIN_3_2 {}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod trig6_chain_3_2;
#[doc = "ETC_TRIG Chain 4/5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig6_chain_5_4](trig6_chain_5_4) module"]
pub type TRIG6_CHAIN_5_4 = crate::Reg<u32, _TRIG6_CHAIN_5_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG6_CHAIN_5_4;
#[doc = "`read()` method returns [trig6_chain_5_4::R](trig6_chain_5_4::R) reader structure"]
impl crate::Readable for TRIG6_CHAIN_5_4 {}
#[doc = "`write(|w| ..)` method takes [trig6_chain_5_4::W](trig6_chain_5_4::W) writer structure"]
impl crate::Writable for TRIG6_CHAIN_5_4 {}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod trig6_chain_5_4;
#[doc = "ETC_TRIG Chain 6/7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig6_chain_7_6](trig6_chain_7_6) module"]
pub type TRIG6_CHAIN_7_6 = crate::Reg<u32, _TRIG6_CHAIN_7_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG6_CHAIN_7_6;
#[doc = "`read()` method returns [trig6_chain_7_6::R](trig6_chain_7_6::R) reader structure"]
impl crate::Readable for TRIG6_CHAIN_7_6 {}
#[doc = "`write(|w| ..)` method takes [trig6_chain_7_6::W](trig6_chain_7_6::W) writer structure"]
impl crate::Writable for TRIG6_CHAIN_7_6 {}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod trig6_chain_7_6;
#[doc = "ETC_TRIG Result Data 1/0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig6_result_1_0](trig6_result_1_0) module"]
pub type TRIG6_RESULT_1_0 = crate::Reg<u32, _TRIG6_RESULT_1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG6_RESULT_1_0;
#[doc = "`read()` method returns [trig6_result_1_0::R](trig6_result_1_0::R) reader structure"]
impl crate::Readable for TRIG6_RESULT_1_0 {}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod trig6_result_1_0;
#[doc = "ETC_TRIG Result Data 3/2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig6_result_3_2](trig6_result_3_2) module"]
pub type TRIG6_RESULT_3_2 = crate::Reg<u32, _TRIG6_RESULT_3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG6_RESULT_3_2;
#[doc = "`read()` method returns [trig6_result_3_2::R](trig6_result_3_2::R) reader structure"]
impl crate::Readable for TRIG6_RESULT_3_2 {}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod trig6_result_3_2;
#[doc = "ETC_TRIG Result Data 5/4 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig6_result_5_4](trig6_result_5_4) module"]
pub type TRIG6_RESULT_5_4 = crate::Reg<u32, _TRIG6_RESULT_5_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG6_RESULT_5_4;
#[doc = "`read()` method returns [trig6_result_5_4::R](trig6_result_5_4::R) reader structure"]
impl crate::Readable for TRIG6_RESULT_5_4 {}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod trig6_result_5_4;
#[doc = "ETC_TRIG Result Data 7/6 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig6_result_7_6](trig6_result_7_6) module"]
pub type TRIG6_RESULT_7_6 = crate::Reg<u32, _TRIG6_RESULT_7_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG6_RESULT_7_6;
#[doc = "`read()` method returns [trig6_result_7_6::R](trig6_result_7_6::R) reader structure"]
impl crate::Readable for TRIG6_RESULT_7_6 {}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod trig6_result_7_6;
#[doc = "ETC_TRIG7 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig7_ctrl](trig7_ctrl) module"]
pub type TRIG7_CTRL = crate::Reg<u32, _TRIG7_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG7_CTRL;
#[doc = "`read()` method returns [trig7_ctrl::R](trig7_ctrl::R) reader structure"]
impl crate::Readable for TRIG7_CTRL {}
#[doc = "`write(|w| ..)` method takes [trig7_ctrl::W](trig7_ctrl::W) writer structure"]
impl crate::Writable for TRIG7_CTRL {}
#[doc = "ETC_TRIG7 Control Register"]
pub mod trig7_ctrl;
#[doc = "ETC_TRIG7 Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig7_counter](trig7_counter) module"]
pub type TRIG7_COUNTER = crate::Reg<u32, _TRIG7_COUNTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG7_COUNTER;
#[doc = "`read()` method returns [trig7_counter::R](trig7_counter::R) reader structure"]
impl crate::Readable for TRIG7_COUNTER {}
#[doc = "`write(|w| ..)` method takes [trig7_counter::W](trig7_counter::W) writer structure"]
impl crate::Writable for TRIG7_COUNTER {}
#[doc = "ETC_TRIG7 Counter Register"]
pub mod trig7_counter;
#[doc = "ETC_TRIG Chain 0/1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig7_chain_1_0](trig7_chain_1_0) module"]
pub type TRIG7_CHAIN_1_0 = crate::Reg<u32, _TRIG7_CHAIN_1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG7_CHAIN_1_0;
#[doc = "`read()` method returns [trig7_chain_1_0::R](trig7_chain_1_0::R) reader structure"]
impl crate::Readable for TRIG7_CHAIN_1_0 {}
#[doc = "`write(|w| ..)` method takes [trig7_chain_1_0::W](trig7_chain_1_0::W) writer structure"]
impl crate::Writable for TRIG7_CHAIN_1_0 {}
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod trig7_chain_1_0;
#[doc = "ETC_TRIG Chain 2/3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig7_chain_3_2](trig7_chain_3_2) module"]
pub type TRIG7_CHAIN_3_2 = crate::Reg<u32, _TRIG7_CHAIN_3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG7_CHAIN_3_2;
#[doc = "`read()` method returns [trig7_chain_3_2::R](trig7_chain_3_2::R) reader structure"]
impl crate::Readable for TRIG7_CHAIN_3_2 {}
#[doc = "`write(|w| ..)` method takes [trig7_chain_3_2::W](trig7_chain_3_2::W) writer structure"]
impl crate::Writable for TRIG7_CHAIN_3_2 {}
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod trig7_chain_3_2;
#[doc = "ETC_TRIG Chain 4/5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig7_chain_5_4](trig7_chain_5_4) module"]
pub type TRIG7_CHAIN_5_4 = crate::Reg<u32, _TRIG7_CHAIN_5_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG7_CHAIN_5_4;
#[doc = "`read()` method returns [trig7_chain_5_4::R](trig7_chain_5_4::R) reader structure"]
impl crate::Readable for TRIG7_CHAIN_5_4 {}
#[doc = "`write(|w| ..)` method takes [trig7_chain_5_4::W](trig7_chain_5_4::W) writer structure"]
impl crate::Writable for TRIG7_CHAIN_5_4 {}
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod trig7_chain_5_4;
#[doc = "ETC_TRIG Chain 6/7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig7_chain_7_6](trig7_chain_7_6) module"]
pub type TRIG7_CHAIN_7_6 = crate::Reg<u32, _TRIG7_CHAIN_7_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG7_CHAIN_7_6;
#[doc = "`read()` method returns [trig7_chain_7_6::R](trig7_chain_7_6::R) reader structure"]
impl crate::Readable for TRIG7_CHAIN_7_6 {}
#[doc = "`write(|w| ..)` method takes [trig7_chain_7_6::W](trig7_chain_7_6::W) writer structure"]
impl crate::Writable for TRIG7_CHAIN_7_6 {}
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod trig7_chain_7_6;
#[doc = "ETC_TRIG Result Data 1/0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig7_result_1_0](trig7_result_1_0) module"]
pub type TRIG7_RESULT_1_0 = crate::Reg<u32, _TRIG7_RESULT_1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG7_RESULT_1_0;
#[doc = "`read()` method returns [trig7_result_1_0::R](trig7_result_1_0::R) reader structure"]
impl crate::Readable for TRIG7_RESULT_1_0 {}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod trig7_result_1_0;
#[doc = "ETC_TRIG Result Data 3/2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig7_result_3_2](trig7_result_3_2) module"]
pub type TRIG7_RESULT_3_2 = crate::Reg<u32, _TRIG7_RESULT_3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG7_RESULT_3_2;
#[doc = "`read()` method returns [trig7_result_3_2::R](trig7_result_3_2::R) reader structure"]
impl crate::Readable for TRIG7_RESULT_3_2 {}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod trig7_result_3_2;
#[doc = "ETC_TRIG Result Data 5/4 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig7_result_5_4](trig7_result_5_4) module"]
pub type TRIG7_RESULT_5_4 = crate::Reg<u32, _TRIG7_RESULT_5_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG7_RESULT_5_4;
#[doc = "`read()` method returns [trig7_result_5_4::R](trig7_result_5_4::R) reader structure"]
impl crate::Readable for TRIG7_RESULT_5_4 {}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod trig7_result_5_4;
#[doc = "ETC_TRIG Result Data 7/6 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig7_result_7_6](trig7_result_7_6) module"]
pub type TRIG7_RESULT_7_6 = crate::Reg<u32, _TRIG7_RESULT_7_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIG7_RESULT_7_6;
#[doc = "`read()` method returns [trig7_result_7_6::R](trig7_result_7_6::R) reader structure"]
impl crate::Readable for TRIG7_RESULT_7_6 {}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod trig7_result_7_6;
