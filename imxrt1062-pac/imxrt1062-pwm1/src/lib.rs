#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Submodule"]
    pub sm0: SM,
    _reserved1: [u8; 8usize],
    #[doc = "0x60 - PWM Submodule"]
    pub sm1: SM,
    _reserved2: [u8; 8usize],
    #[doc = "0xc0 - PWM Submodule"]
    pub sm2: SM,
    _reserved3: [u8; 8usize],
    #[doc = "0x120 - PWM Submodule"]
    pub sm3: SM,
    _reserved4: [u8; 8usize],
    #[doc = "0x180 - Output Enable Register"]
    pub outen: OUTEN,
    #[doc = "0x182 - Mask Register"]
    pub mask: MASK,
    #[doc = "0x184 - Software Controlled Output Register"]
    pub swcout: SWCOUT,
    #[doc = "0x186 - PWM Source Select Register"]
    pub dtsrcsel: DTSRCSEL,
    #[doc = "0x188 - Master Control Register"]
    pub mctrl: MCTRL,
    #[doc = "0x18a - Master Control 2 Register"]
    pub mctrl2: MCTRL2,
    #[doc = "0x18c - Fault Control Register"]
    pub fctrl0: FCTRL0,
    #[doc = "0x18e - Fault Status Register"]
    pub fsts0: FSTS0,
    #[doc = "0x190 - Fault Filter Register"]
    pub ffilt0: FFILT0,
    #[doc = "0x192 - Fault Test Register"]
    pub ftst0: FTST0,
    #[doc = "0x194 - Fault Control 2 Register"]
    pub fctrl20: FCTRL20,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct SM {
    #[doc = "0x00 - Counter Register"]
    pub smcnt: self::sm::SMCNT,
    #[doc = "0x02 - Initial Count Register"]
    pub sminit: self::sm::SMINIT,
    #[doc = "0x04 - Control 2 Register"]
    pub smctrl2: self::sm::SMCTRL2,
    #[doc = "0x06 - Control Register"]
    pub smctrl: self::sm::SMCTRL,
    _reserved4: [u8; 2usize],
    #[doc = "0x0a - Value Register 0"]
    pub smval0: self::sm::SMVAL0,
    #[doc = "0x0c - Fractional Value Register 1"]
    pub smfracval1: self::sm::SMFRACVAL1,
    #[doc = "0x0e - Value Register 1"]
    pub smval1: self::sm::SMVAL1,
    #[doc = "0x10 - Fractional Value Register 2"]
    pub smfracval2: self::sm::SMFRACVAL2,
    #[doc = "0x12 - Value Register 2"]
    pub smval2: self::sm::SMVAL2,
    #[doc = "0x14 - Fractional Value Register 3"]
    pub smfracval3: self::sm::SMFRACVAL3,
    #[doc = "0x16 - Value Register 3"]
    pub smval3: self::sm::SMVAL3,
    #[doc = "0x18 - Fractional Value Register 4"]
    pub smfracval4: self::sm::SMFRACVAL4,
    #[doc = "0x1a - Value Register 4"]
    pub smval4: self::sm::SMVAL4,
    #[doc = "0x1c - Fractional Value Register 5"]
    pub smfracval5: self::sm::SMFRACVAL5,
    #[doc = "0x1e - Value Register 5"]
    pub smval5: self::sm::SMVAL5,
    #[doc = "0x20 - Fractional Control Register"]
    pub smfrctrl: self::sm::SMFRCTRL,
    #[doc = "0x22 - Output Control Register"]
    pub smoctrl: self::sm::SMOCTRL,
    #[doc = "0x24 - Status Register"]
    pub smsts: self::sm::SMSTS,
    #[doc = "0x26 - Interrupt Enable Register"]
    pub sminten: self::sm::SMINTEN,
    #[doc = "0x28 - DMA Enable Register"]
    pub smdmaen: self::sm::SMDMAEN,
    #[doc = "0x2a - Output Trigger Control Register"]
    pub smtctrl: self::sm::SMTCTRL,
    #[doc = "0x2c - Fault Disable Mapping Register 0"]
    pub smdismap0: self::sm::SMDISMAP0,
    #[doc = "0x2e - Fault Disable Mapping Register 1"]
    pub smdismap1: self::sm::SMDISMAP1,
    #[doc = "0x30 - Deadtime Count Register 0"]
    pub smdtcnt0: self::sm::SMDTCNT0,
    #[doc = "0x32 - Deadtime Count Register 1"]
    pub smdtcnt1: self::sm::SMDTCNT1,
    #[doc = "0x34 - Capture Control A Register"]
    pub smcaptctrla: self::sm::SMCAPTCTRLA,
    #[doc = "0x36 - Capture Compare A Register"]
    pub smcaptcompa: self::sm::SMCAPTCOMPA,
    #[doc = "0x38 - Capture Control B Register"]
    pub smcaptctrlb: self::sm::SMCAPTCTRLB,
    #[doc = "0x3a - Capture Compare B Register"]
    pub smcaptcompb: self::sm::SMCAPTCOMPB,
    #[doc = "0x3c - Capture Control X Register"]
    pub smcaptctrlx: self::sm::SMCAPTCTRLX,
    #[doc = "0x3e - Capture Compare X Register"]
    pub smcaptcompx: self::sm::SMCAPTCOMPX,
    #[doc = "0x40 - Capture Value 0 Register"]
    pub smcval0: self::sm::SMCVAL0,
    #[doc = "0x42 - Capture Value 0 Cycle Register"]
    pub smcval0cyc: self::sm::SMCVAL0CYC,
    #[doc = "0x44 - Capture Value 1 Register"]
    pub smcval1: self::sm::SMCVAL1,
    #[doc = "0x46 - Capture Value 1 Cycle Register"]
    pub smcval1cyc: self::sm::SMCVAL1CYC,
    #[doc = "0x48 - Capture Value 2 Register"]
    pub smcval2: self::sm::SMCVAL2,
    #[doc = "0x4a - Capture Value 2 Cycle Register"]
    pub smcval2cyc: self::sm::SMCVAL2CYC,
    #[doc = "0x4c - Capture Value 3 Register"]
    pub smcval3: self::sm::SMCVAL3,
    #[doc = "0x4e - Capture Value 3 Cycle Register"]
    pub smcval3cyc: self::sm::SMCVAL3CYC,
    #[doc = "0x50 - Capture Value 4 Register"]
    pub smcval4: self::sm::SMCVAL4,
    #[doc = "0x52 - Capture Value 4 Cycle Register"]
    pub smcval4cyc: self::sm::SMCVAL4CYC,
    #[doc = "0x54 - Capture Value 5 Register"]
    pub smcval5: self::sm::SMCVAL5,
    #[doc = "0x56 - Capture Value 5 Cycle Register"]
    pub smcval5cyc: self::sm::SMCVAL5CYC,
}
#[doc = r"Register block"]
#[doc = "PWM Submodule"]
pub mod sm;
#[doc = "Output Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outen](outen) module"]
pub type OUTEN = crate::Reg<u16, _OUTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTEN;
#[doc = "`read()` method returns [outen::R](outen::R) reader structure"]
impl crate::Readable for OUTEN {}
#[doc = "`write(|w| ..)` method takes [outen::W](outen::W) writer structure"]
impl crate::Writable for OUTEN {}
#[doc = "Output Enable Register"]
pub mod outen;
#[doc = "Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask](mask) module"]
pub type MASK = crate::Reg<u16, _MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK;
#[doc = "`read()` method returns [mask::R](mask::R) reader structure"]
impl crate::Readable for MASK {}
#[doc = "`write(|w| ..)` method takes [mask::W](mask::W) writer structure"]
impl crate::Writable for MASK {}
#[doc = "Mask Register"]
pub mod mask;
#[doc = "Software Controlled Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swcout](swcout) module"]
pub type SWCOUT = crate::Reg<u16, _SWCOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWCOUT;
#[doc = "`read()` method returns [swcout::R](swcout::R) reader structure"]
impl crate::Readable for SWCOUT {}
#[doc = "`write(|w| ..)` method takes [swcout::W](swcout::W) writer structure"]
impl crate::Writable for SWCOUT {}
#[doc = "Software Controlled Output Register"]
pub mod swcout;
#[doc = "PWM Source Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtsrcsel](dtsrcsel) module"]
pub type DTSRCSEL = crate::Reg<u16, _DTSRCSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTSRCSEL;
#[doc = "`read()` method returns [dtsrcsel::R](dtsrcsel::R) reader structure"]
impl crate::Readable for DTSRCSEL {}
#[doc = "`write(|w| ..)` method takes [dtsrcsel::W](dtsrcsel::W) writer structure"]
impl crate::Writable for DTSRCSEL {}
#[doc = "PWM Source Select Register"]
pub mod dtsrcsel;
#[doc = "Master Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctrl](mctrl) module"]
pub type MCTRL = crate::Reg<u16, _MCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTRL;
#[doc = "`read()` method returns [mctrl::R](mctrl::R) reader structure"]
impl crate::Readable for MCTRL {}
#[doc = "`write(|w| ..)` method takes [mctrl::W](mctrl::W) writer structure"]
impl crate::Writable for MCTRL {}
#[doc = "Master Control Register"]
pub mod mctrl;
#[doc = "Master Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctrl2](mctrl2) module"]
pub type MCTRL2 = crate::Reg<u16, _MCTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTRL2;
#[doc = "`read()` method returns [mctrl2::R](mctrl2::R) reader structure"]
impl crate::Readable for MCTRL2 {}
#[doc = "`write(|w| ..)` method takes [mctrl2::W](mctrl2::W) writer structure"]
impl crate::Writable for MCTRL2 {}
#[doc = "Master Control 2 Register"]
pub mod mctrl2;
#[doc = "Fault Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fctrl0](fctrl0) module"]
pub type FCTRL0 = crate::Reg<u16, _FCTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCTRL0;
#[doc = "`read()` method returns [fctrl0::R](fctrl0::R) reader structure"]
impl crate::Readable for FCTRL0 {}
#[doc = "`write(|w| ..)` method takes [fctrl0::W](fctrl0::W) writer structure"]
impl crate::Writable for FCTRL0 {}
#[doc = "Fault Control Register"]
pub mod fctrl0;
#[doc = "Fault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsts0](fsts0) module"]
pub type FSTS0 = crate::Reg<u16, _FSTS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSTS0;
#[doc = "`read()` method returns [fsts0::R](fsts0::R) reader structure"]
impl crate::Readable for FSTS0 {}
#[doc = "`write(|w| ..)` method takes [fsts0::W](fsts0::W) writer structure"]
impl crate::Writable for FSTS0 {}
#[doc = "Fault Status Register"]
pub mod fsts0;
#[doc = "Fault Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffilt0](ffilt0) module"]
pub type FFILT0 = crate::Reg<u16, _FFILT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FFILT0;
#[doc = "`read()` method returns [ffilt0::R](ffilt0::R) reader structure"]
impl crate::Readable for FFILT0 {}
#[doc = "`write(|w| ..)` method takes [ffilt0::W](ffilt0::W) writer structure"]
impl crate::Writable for FFILT0 {}
#[doc = "Fault Filter Register"]
pub mod ffilt0;
#[doc = "Fault Test Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftst0](ftst0) module"]
pub type FTST0 = crate::Reg<u16, _FTST0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FTST0;
#[doc = "`read()` method returns [ftst0::R](ftst0::R) reader structure"]
impl crate::Readable for FTST0 {}
#[doc = "`write(|w| ..)` method takes [ftst0::W](ftst0::W) writer structure"]
impl crate::Writable for FTST0 {}
#[doc = "Fault Test Register"]
pub mod ftst0;
#[doc = "Fault Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fctrl20](fctrl20) module"]
pub type FCTRL20 = crate::Reg<u16, _FCTRL20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCTRL20;
#[doc = "`read()` method returns [fctrl20::R](fctrl20::R) reader structure"]
impl crate::Readable for FCTRL20 {}
#[doc = "`write(|w| ..)` method takes [fctrl20::W](fctrl20::W) writer structure"]
impl crate::Writable for FCTRL20 {}
#[doc = "Fault Control 2 Register"]
pub mod fctrl20;
