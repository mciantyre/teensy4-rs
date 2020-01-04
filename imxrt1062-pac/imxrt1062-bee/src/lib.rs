#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - BEE Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - no description available"]
    pub addr_offset0: ADDR_OFFSET0,
    #[doc = "0x08 - no description available"]
    pub addr_offset1: ADDR_OFFSET1,
    #[doc = "0x0c - no description available"]
    pub aes_key0_w0: AES_KEY0_W0,
    #[doc = "0x10 - no description available"]
    pub aes_key0_w1: AES_KEY0_W1,
    #[doc = "0x14 - no description available"]
    pub aes_key0_w2: AES_KEY0_W2,
    #[doc = "0x18 - no description available"]
    pub aes_key0_w3: AES_KEY0_W3,
    #[doc = "0x1c - no description available"]
    pub status: STATUS,
    #[doc = "0x20 - no description available"]
    pub ctr_nonce0_w0: CTR_NONCE0_W0,
    #[doc = "0x24 - no description available"]
    pub ctr_nonce0_w1: CTR_NONCE0_W1,
    #[doc = "0x28 - no description available"]
    pub ctr_nonce0_w2: CTR_NONCE0_W2,
    #[doc = "0x2c - no description available"]
    pub ctr_nonce0_w3: CTR_NONCE0_W3,
    #[doc = "0x30 - no description available"]
    pub ctr_nonce1_w0: CTR_NONCE1_W0,
    #[doc = "0x34 - no description available"]
    pub ctr_nonce1_w1: CTR_NONCE1_W1,
    #[doc = "0x38 - no description available"]
    pub ctr_nonce1_w2: CTR_NONCE1_W2,
    #[doc = "0x3c - no description available"]
    pub ctr_nonce1_w3: CTR_NONCE1_W3,
    #[doc = "0x40 - no description available"]
    pub region1_top: REGION1_TOP,
    #[doc = "0x44 - no description available"]
    pub region1_bot: REGION1_BOT,
}
#[doc = "BEE Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "BEE Control Register"]
pub mod ctrl;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr_offset0](addr_offset0) module"]
pub type ADDR_OFFSET0 = crate::Reg<u32, _ADDR_OFFSET0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR_OFFSET0;
#[doc = "`read()` method returns [addr_offset0::R](addr_offset0::R) reader structure"]
impl crate::Readable for ADDR_OFFSET0 {}
#[doc = "`write(|w| ..)` method takes [addr_offset0::W](addr_offset0::W) writer structure"]
impl crate::Writable for ADDR_OFFSET0 {}
#[doc = "no description available"]
pub mod addr_offset0;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr_offset1](addr_offset1) module"]
pub type ADDR_OFFSET1 = crate::Reg<u32, _ADDR_OFFSET1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR_OFFSET1;
#[doc = "`read()` method returns [addr_offset1::R](addr_offset1::R) reader structure"]
impl crate::Readable for ADDR_OFFSET1 {}
#[doc = "`write(|w| ..)` method takes [addr_offset1::W](addr_offset1::W) writer structure"]
impl crate::Writable for ADDR_OFFSET1 {}
#[doc = "no description available"]
pub mod addr_offset1;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_key0_w0](aes_key0_w0) module"]
pub type AES_KEY0_W0 = crate::Reg<u32, _AES_KEY0_W0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_KEY0_W0;
#[doc = "`read()` method returns [aes_key0_w0::R](aes_key0_w0::R) reader structure"]
impl crate::Readable for AES_KEY0_W0 {}
#[doc = "`write(|w| ..)` method takes [aes_key0_w0::W](aes_key0_w0::W) writer structure"]
impl crate::Writable for AES_KEY0_W0 {}
#[doc = "no description available"]
pub mod aes_key0_w0;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_key0_w1](aes_key0_w1) module"]
pub type AES_KEY0_W1 = crate::Reg<u32, _AES_KEY0_W1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_KEY0_W1;
#[doc = "`read()` method returns [aes_key0_w1::R](aes_key0_w1::R) reader structure"]
impl crate::Readable for AES_KEY0_W1 {}
#[doc = "`write(|w| ..)` method takes [aes_key0_w1::W](aes_key0_w1::W) writer structure"]
impl crate::Writable for AES_KEY0_W1 {}
#[doc = "no description available"]
pub mod aes_key0_w1;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_key0_w2](aes_key0_w2) module"]
pub type AES_KEY0_W2 = crate::Reg<u32, _AES_KEY0_W2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_KEY0_W2;
#[doc = "`read()` method returns [aes_key0_w2::R](aes_key0_w2::R) reader structure"]
impl crate::Readable for AES_KEY0_W2 {}
#[doc = "`write(|w| ..)` method takes [aes_key0_w2::W](aes_key0_w2::W) writer structure"]
impl crate::Writable for AES_KEY0_W2 {}
#[doc = "no description available"]
pub mod aes_key0_w2;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_key0_w3](aes_key0_w3) module"]
pub type AES_KEY0_W3 = crate::Reg<u32, _AES_KEY0_W3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_KEY0_W3;
#[doc = "`read()` method returns [aes_key0_w3::R](aes_key0_w3::R) reader structure"]
impl crate::Readable for AES_KEY0_W3 {}
#[doc = "`write(|w| ..)` method takes [aes_key0_w3::W](aes_key0_w3::W) writer structure"]
impl crate::Writable for AES_KEY0_W3 {}
#[doc = "no description available"]
pub mod aes_key0_w3;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "no description available"]
pub mod status;
#[doc = "no description available\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr_nonce0_w0](ctr_nonce0_w0) module"]
pub type CTR_NONCE0_W0 = crate::Reg<u32, _CTR_NONCE0_W0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTR_NONCE0_W0;
#[doc = "`write(|w| ..)` method takes [ctr_nonce0_w0::W](ctr_nonce0_w0::W) writer structure"]
impl crate::Writable for CTR_NONCE0_W0 {}
#[doc = "no description available"]
pub mod ctr_nonce0_w0;
#[doc = "no description available\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr_nonce0_w1](ctr_nonce0_w1) module"]
pub type CTR_NONCE0_W1 = crate::Reg<u32, _CTR_NONCE0_W1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTR_NONCE0_W1;
#[doc = "`write(|w| ..)` method takes [ctr_nonce0_w1::W](ctr_nonce0_w1::W) writer structure"]
impl crate::Writable for CTR_NONCE0_W1 {}
#[doc = "no description available"]
pub mod ctr_nonce0_w1;
#[doc = "no description available\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr_nonce0_w2](ctr_nonce0_w2) module"]
pub type CTR_NONCE0_W2 = crate::Reg<u32, _CTR_NONCE0_W2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTR_NONCE0_W2;
#[doc = "`write(|w| ..)` method takes [ctr_nonce0_w2::W](ctr_nonce0_w2::W) writer structure"]
impl crate::Writable for CTR_NONCE0_W2 {}
#[doc = "no description available"]
pub mod ctr_nonce0_w2;
#[doc = "no description available\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr_nonce0_w3](ctr_nonce0_w3) module"]
pub type CTR_NONCE0_W3 = crate::Reg<u32, _CTR_NONCE0_W3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTR_NONCE0_W3;
#[doc = "`write(|w| ..)` method takes [ctr_nonce0_w3::W](ctr_nonce0_w3::W) writer structure"]
impl crate::Writable for CTR_NONCE0_W3 {}
#[doc = "no description available"]
pub mod ctr_nonce0_w3;
#[doc = "no description available\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr_nonce1_w0](ctr_nonce1_w0) module"]
pub type CTR_NONCE1_W0 = crate::Reg<u32, _CTR_NONCE1_W0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTR_NONCE1_W0;
#[doc = "`write(|w| ..)` method takes [ctr_nonce1_w0::W](ctr_nonce1_w0::W) writer structure"]
impl crate::Writable for CTR_NONCE1_W0 {}
#[doc = "no description available"]
pub mod ctr_nonce1_w0;
#[doc = "no description available\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr_nonce1_w1](ctr_nonce1_w1) module"]
pub type CTR_NONCE1_W1 = crate::Reg<u32, _CTR_NONCE1_W1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTR_NONCE1_W1;
#[doc = "`write(|w| ..)` method takes [ctr_nonce1_w1::W](ctr_nonce1_w1::W) writer structure"]
impl crate::Writable for CTR_NONCE1_W1 {}
#[doc = "no description available"]
pub mod ctr_nonce1_w1;
#[doc = "no description available\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr_nonce1_w2](ctr_nonce1_w2) module"]
pub type CTR_NONCE1_W2 = crate::Reg<u32, _CTR_NONCE1_W2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTR_NONCE1_W2;
#[doc = "`write(|w| ..)` method takes [ctr_nonce1_w2::W](ctr_nonce1_w2::W) writer structure"]
impl crate::Writable for CTR_NONCE1_W2 {}
#[doc = "no description available"]
pub mod ctr_nonce1_w2;
#[doc = "no description available\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr_nonce1_w3](ctr_nonce1_w3) module"]
pub type CTR_NONCE1_W3 = crate::Reg<u32, _CTR_NONCE1_W3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTR_NONCE1_W3;
#[doc = "`write(|w| ..)` method takes [ctr_nonce1_w3::W](ctr_nonce1_w3::W) writer structure"]
impl crate::Writable for CTR_NONCE1_W3 {}
#[doc = "no description available"]
pub mod ctr_nonce1_w3;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region1_top](region1_top) module"]
pub type REGION1_TOP = crate::Reg<u32, _REGION1_TOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION1_TOP;
#[doc = "`read()` method returns [region1_top::R](region1_top::R) reader structure"]
impl crate::Readable for REGION1_TOP {}
#[doc = "`write(|w| ..)` method takes [region1_top::W](region1_top::W) writer structure"]
impl crate::Writable for REGION1_TOP {}
#[doc = "no description available"]
pub mod region1_top;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region1_bot](region1_bot) module"]
pub type REGION1_BOT = crate::Reg<u32, _REGION1_BOT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION1_BOT;
#[doc = "`read()` method returns [region1_bot::R](region1_bot::R) reader structure"]
impl crate::Readable for REGION1_BOT {}
#[doc = "`write(|w| ..)` method takes [region1_bot::W](region1_bot::W) writer structure"]
impl crate::Writable for REGION1_BOT {}
#[doc = "no description available"]
pub mod region1_bot;
