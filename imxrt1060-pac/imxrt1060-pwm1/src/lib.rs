#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

include!("../../generic.rs");

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Counter Register"]
    pub sm0cnt: SM0CNT,
    #[doc = "0x02 - Initial Count Register"]
    pub sm0init: SM0INIT,
    #[doc = "0x04 - Control 2 Register"]
    pub sm0ctrl2: SM0CTRL2,
    #[doc = "0x06 - Control Register"]
    pub sm0ctrl: SM0CTRL,
    _reserved4: [u8; 2usize],
    #[doc = "0x0a - Value Register 0"]
    pub sm0val0: SM0VAL0,
    #[doc = "0x0c - Fractional Value Register 1"]
    pub sm0fracval1: SM0FRACVAL1,
    #[doc = "0x0e - Value Register 1"]
    pub sm0val1: SM0VAL1,
    #[doc = "0x10 - Fractional Value Register 2"]
    pub sm0fracval2: SM0FRACVAL2,
    #[doc = "0x12 - Value Register 2"]
    pub sm0val2: SM0VAL2,
    #[doc = "0x14 - Fractional Value Register 3"]
    pub sm0fracval3: SM0FRACVAL3,
    #[doc = "0x16 - Value Register 3"]
    pub sm0val3: SM0VAL3,
    #[doc = "0x18 - Fractional Value Register 4"]
    pub sm0fracval4: SM0FRACVAL4,
    #[doc = "0x1a - Value Register 4"]
    pub sm0val4: SM0VAL4,
    #[doc = "0x1c - Fractional Value Register 5"]
    pub sm0fracval5: SM0FRACVAL5,
    #[doc = "0x1e - Value Register 5"]
    pub sm0val5: SM0VAL5,
    #[doc = "0x20 - Fractional Control Register"]
    pub sm0frctrl: SM0FRCTRL,
    #[doc = "0x22 - Output Control Register"]
    pub sm0octrl: SM0OCTRL,
    #[doc = "0x24 - Status Register"]
    pub sm0sts: SM0STS,
    #[doc = "0x26 - Interrupt Enable Register"]
    pub sm0inten: SM0INTEN,
    #[doc = "0x28 - DMA Enable Register"]
    pub sm0dmaen: SM0DMAEN,
    #[doc = "0x2a - Output Trigger Control Register"]
    pub sm0tctrl: SM0TCTRL,
    #[doc = "0x2c - Fault Disable Mapping Register 0"]
    pub sm0dismap0: SM0DISMAP0,
    #[doc = "0x2e - Fault Disable Mapping Register 1"]
    pub sm0dismap1: SM0DISMAP1,
    #[doc = "0x30 - Deadtime Count Register 0"]
    pub sm0dtcnt0: SM0DTCNT0,
    #[doc = "0x32 - Deadtime Count Register 1"]
    pub sm0dtcnt1: SM0DTCNT1,
    #[doc = "0x34 - Capture Control A Register"]
    pub sm0captctrla: SM0CAPTCTRLA,
    #[doc = "0x36 - Capture Compare A Register"]
    pub sm0captcompa: SM0CAPTCOMPA,
    #[doc = "0x38 - Capture Control B Register"]
    pub sm0captctrlb: SM0CAPTCTRLB,
    #[doc = "0x3a - Capture Compare B Register"]
    pub sm0captcompb: SM0CAPTCOMPB,
    #[doc = "0x3c - Capture Control X Register"]
    pub sm0captctrlx: SM0CAPTCTRLX,
    #[doc = "0x3e - Capture Compare X Register"]
    pub sm0captcompx: SM0CAPTCOMPX,
    #[doc = "0x40 - Capture Value 0 Register"]
    pub sm0cval0: SM0CVAL0,
    #[doc = "0x42 - Capture Value 0 Cycle Register"]
    pub sm0cval0cyc: SM0CVAL0CYC,
    #[doc = "0x44 - Capture Value 1 Register"]
    pub sm0cval1: SM0CVAL1,
    #[doc = "0x46 - Capture Value 1 Cycle Register"]
    pub sm0cval1cyc: SM0CVAL1CYC,
    #[doc = "0x48 - Capture Value 2 Register"]
    pub sm0cval2: SM0CVAL2,
    #[doc = "0x4a - Capture Value 2 Cycle Register"]
    pub sm0cval2cyc: SM0CVAL2CYC,
    #[doc = "0x4c - Capture Value 3 Register"]
    pub sm0cval3: SM0CVAL3,
    #[doc = "0x4e - Capture Value 3 Cycle Register"]
    pub sm0cval3cyc: SM0CVAL3CYC,
    #[doc = "0x50 - Capture Value 4 Register"]
    pub sm0cval4: SM0CVAL4,
    #[doc = "0x52 - Capture Value 4 Cycle Register"]
    pub sm0cval4cyc: SM0CVAL4CYC,
    #[doc = "0x54 - Capture Value 5 Register"]
    pub sm0cval5: SM0CVAL5,
    #[doc = "0x56 - Capture Value 5 Cycle Register"]
    pub sm0cval5cyc: SM0CVAL5CYC,
    _reserved43: [u8; 8usize],
    #[doc = "0x60 - Counter Register"]
    pub sm1cnt: SM1CNT,
    #[doc = "0x62 - Initial Count Register"]
    pub sm1init: SM1INIT,
    #[doc = "0x64 - Control 2 Register"]
    pub sm1ctrl2: SM1CTRL2,
    #[doc = "0x66 - Control Register"]
    pub sm1ctrl: SM1CTRL,
    _reserved47: [u8; 2usize],
    #[doc = "0x6a - Value Register 0"]
    pub sm1val0: SM1VAL0,
    #[doc = "0x6c - Fractional Value Register 1"]
    pub sm1fracval1: SM1FRACVAL1,
    #[doc = "0x6e - Value Register 1"]
    pub sm1val1: SM1VAL1,
    #[doc = "0x70 - Fractional Value Register 2"]
    pub sm1fracval2: SM1FRACVAL2,
    #[doc = "0x72 - Value Register 2"]
    pub sm1val2: SM1VAL2,
    #[doc = "0x74 - Fractional Value Register 3"]
    pub sm1fracval3: SM1FRACVAL3,
    #[doc = "0x76 - Value Register 3"]
    pub sm1val3: SM1VAL3,
    #[doc = "0x78 - Fractional Value Register 4"]
    pub sm1fracval4: SM1FRACVAL4,
    #[doc = "0x7a - Value Register 4"]
    pub sm1val4: SM1VAL4,
    #[doc = "0x7c - Fractional Value Register 5"]
    pub sm1fracval5: SM1FRACVAL5,
    #[doc = "0x7e - Value Register 5"]
    pub sm1val5: SM1VAL5,
    #[doc = "0x80 - Fractional Control Register"]
    pub sm1frctrl: SM1FRCTRL,
    #[doc = "0x82 - Output Control Register"]
    pub sm1octrl: SM1OCTRL,
    #[doc = "0x84 - Status Register"]
    pub sm1sts: SM1STS,
    #[doc = "0x86 - Interrupt Enable Register"]
    pub sm1inten: SM1INTEN,
    #[doc = "0x88 - DMA Enable Register"]
    pub sm1dmaen: SM1DMAEN,
    #[doc = "0x8a - Output Trigger Control Register"]
    pub sm1tctrl: SM1TCTRL,
    #[doc = "0x8c - Fault Disable Mapping Register 0"]
    pub sm1dismap0: SM1DISMAP0,
    #[doc = "0x8e - Fault Disable Mapping Register 1"]
    pub sm1dismap1: SM1DISMAP1,
    #[doc = "0x90 - Deadtime Count Register 0"]
    pub sm1dtcnt0: SM1DTCNT0,
    #[doc = "0x92 - Deadtime Count Register 1"]
    pub sm1dtcnt1: SM1DTCNT1,
    #[doc = "0x94 - Capture Control A Register"]
    pub sm1captctrla: SM1CAPTCTRLA,
    #[doc = "0x96 - Capture Compare A Register"]
    pub sm1captcompa: SM1CAPTCOMPA,
    #[doc = "0x98 - Capture Control B Register"]
    pub sm1captctrlb: SM1CAPTCTRLB,
    #[doc = "0x9a - Capture Compare B Register"]
    pub sm1captcompb: SM1CAPTCOMPB,
    #[doc = "0x9c - Capture Control X Register"]
    pub sm1captctrlx: SM1CAPTCTRLX,
    #[doc = "0x9e - Capture Compare X Register"]
    pub sm1captcompx: SM1CAPTCOMPX,
    #[doc = "0xa0 - Capture Value 0 Register"]
    pub sm1cval0: SM1CVAL0,
    #[doc = "0xa2 - Capture Value 0 Cycle Register"]
    pub sm1cval0cyc: SM1CVAL0CYC,
    #[doc = "0xa4 - Capture Value 1 Register"]
    pub sm1cval1: SM1CVAL1,
    #[doc = "0xa6 - Capture Value 1 Cycle Register"]
    pub sm1cval1cyc: SM1CVAL1CYC,
    #[doc = "0xa8 - Capture Value 2 Register"]
    pub sm1cval2: SM1CVAL2,
    #[doc = "0xaa - Capture Value 2 Cycle Register"]
    pub sm1cval2cyc: SM1CVAL2CYC,
    #[doc = "0xac - Capture Value 3 Register"]
    pub sm1cval3: SM1CVAL3,
    #[doc = "0xae - Capture Value 3 Cycle Register"]
    pub sm1cval3cyc: SM1CVAL3CYC,
    #[doc = "0xb0 - Capture Value 4 Register"]
    pub sm1cval4: SM1CVAL4,
    #[doc = "0xb2 - Capture Value 4 Cycle Register"]
    pub sm1cval4cyc: SM1CVAL4CYC,
    #[doc = "0xb4 - Capture Value 5 Register"]
    pub sm1cval5: SM1CVAL5,
    #[doc = "0xb6 - Capture Value 5 Cycle Register"]
    pub sm1cval5cyc: SM1CVAL5CYC,
    _reserved86: [u8; 8usize],
    #[doc = "0xc0 - Counter Register"]
    pub sm2cnt: SM2CNT,
    #[doc = "0xc2 - Initial Count Register"]
    pub sm2init: SM2INIT,
    #[doc = "0xc4 - Control 2 Register"]
    pub sm2ctrl2: SM2CTRL2,
    #[doc = "0xc6 - Control Register"]
    pub sm2ctrl: SM2CTRL,
    _reserved90: [u8; 2usize],
    #[doc = "0xca - Value Register 0"]
    pub sm2val0: SM2VAL0,
    #[doc = "0xcc - Fractional Value Register 1"]
    pub sm2fracval1: SM2FRACVAL1,
    #[doc = "0xce - Value Register 1"]
    pub sm2val1: SM2VAL1,
    #[doc = "0xd0 - Fractional Value Register 2"]
    pub sm2fracval2: SM2FRACVAL2,
    #[doc = "0xd2 - Value Register 2"]
    pub sm2val2: SM2VAL2,
    #[doc = "0xd4 - Fractional Value Register 3"]
    pub sm2fracval3: SM2FRACVAL3,
    #[doc = "0xd6 - Value Register 3"]
    pub sm2val3: SM2VAL3,
    #[doc = "0xd8 - Fractional Value Register 4"]
    pub sm2fracval4: SM2FRACVAL4,
    #[doc = "0xda - Value Register 4"]
    pub sm2val4: SM2VAL4,
    #[doc = "0xdc - Fractional Value Register 5"]
    pub sm2fracval5: SM2FRACVAL5,
    #[doc = "0xde - Value Register 5"]
    pub sm2val5: SM2VAL5,
    #[doc = "0xe0 - Fractional Control Register"]
    pub sm2frctrl: SM2FRCTRL,
    #[doc = "0xe2 - Output Control Register"]
    pub sm2octrl: SM2OCTRL,
    #[doc = "0xe4 - Status Register"]
    pub sm2sts: SM2STS,
    #[doc = "0xe6 - Interrupt Enable Register"]
    pub sm2inten: SM2INTEN,
    #[doc = "0xe8 - DMA Enable Register"]
    pub sm2dmaen: SM2DMAEN,
    #[doc = "0xea - Output Trigger Control Register"]
    pub sm2tctrl: SM2TCTRL,
    #[doc = "0xec - Fault Disable Mapping Register 0"]
    pub sm2dismap0: SM2DISMAP0,
    #[doc = "0xee - Fault Disable Mapping Register 1"]
    pub sm2dismap1: SM2DISMAP1,
    #[doc = "0xf0 - Deadtime Count Register 0"]
    pub sm2dtcnt0: SM2DTCNT0,
    #[doc = "0xf2 - Deadtime Count Register 1"]
    pub sm2dtcnt1: SM2DTCNT1,
    #[doc = "0xf4 - Capture Control A Register"]
    pub sm2captctrla: SM2CAPTCTRLA,
    #[doc = "0xf6 - Capture Compare A Register"]
    pub sm2captcompa: SM2CAPTCOMPA,
    #[doc = "0xf8 - Capture Control B Register"]
    pub sm2captctrlb: SM2CAPTCTRLB,
    #[doc = "0xfa - Capture Compare B Register"]
    pub sm2captcompb: SM2CAPTCOMPB,
    #[doc = "0xfc - Capture Control X Register"]
    pub sm2captctrlx: SM2CAPTCTRLX,
    #[doc = "0xfe - Capture Compare X Register"]
    pub sm2captcompx: SM2CAPTCOMPX,
    #[doc = "0x100 - Capture Value 0 Register"]
    pub sm2cval0: SM2CVAL0,
    #[doc = "0x102 - Capture Value 0 Cycle Register"]
    pub sm2cval0cyc: SM2CVAL0CYC,
    #[doc = "0x104 - Capture Value 1 Register"]
    pub sm2cval1: SM2CVAL1,
    #[doc = "0x106 - Capture Value 1 Cycle Register"]
    pub sm2cval1cyc: SM2CVAL1CYC,
    #[doc = "0x108 - Capture Value 2 Register"]
    pub sm2cval2: SM2CVAL2,
    #[doc = "0x10a - Capture Value 2 Cycle Register"]
    pub sm2cval2cyc: SM2CVAL2CYC,
    #[doc = "0x10c - Capture Value 3 Register"]
    pub sm2cval3: SM2CVAL3,
    #[doc = "0x10e - Capture Value 3 Cycle Register"]
    pub sm2cval3cyc: SM2CVAL3CYC,
    #[doc = "0x110 - Capture Value 4 Register"]
    pub sm2cval4: SM2CVAL4,
    #[doc = "0x112 - Capture Value 4 Cycle Register"]
    pub sm2cval4cyc: SM2CVAL4CYC,
    #[doc = "0x114 - Capture Value 5 Register"]
    pub sm2cval5: SM2CVAL5,
    #[doc = "0x116 - Capture Value 5 Cycle Register"]
    pub sm2cval5cyc: SM2CVAL5CYC,
    _reserved129: [u8; 8usize],
    #[doc = "0x120 - Counter Register"]
    pub sm3cnt: SM3CNT,
    #[doc = "0x122 - Initial Count Register"]
    pub sm3init: SM3INIT,
    #[doc = "0x124 - Control 2 Register"]
    pub sm3ctrl2: SM3CTRL2,
    #[doc = "0x126 - Control Register"]
    pub sm3ctrl: SM3CTRL,
    _reserved133: [u8; 2usize],
    #[doc = "0x12a - Value Register 0"]
    pub sm3val0: SM3VAL0,
    #[doc = "0x12c - Fractional Value Register 1"]
    pub sm3fracval1: SM3FRACVAL1,
    #[doc = "0x12e - Value Register 1"]
    pub sm3val1: SM3VAL1,
    #[doc = "0x130 - Fractional Value Register 2"]
    pub sm3fracval2: SM3FRACVAL2,
    #[doc = "0x132 - Value Register 2"]
    pub sm3val2: SM3VAL2,
    #[doc = "0x134 - Fractional Value Register 3"]
    pub sm3fracval3: SM3FRACVAL3,
    #[doc = "0x136 - Value Register 3"]
    pub sm3val3: SM3VAL3,
    #[doc = "0x138 - Fractional Value Register 4"]
    pub sm3fracval4: SM3FRACVAL4,
    #[doc = "0x13a - Value Register 4"]
    pub sm3val4: SM3VAL4,
    #[doc = "0x13c - Fractional Value Register 5"]
    pub sm3fracval5: SM3FRACVAL5,
    #[doc = "0x13e - Value Register 5"]
    pub sm3val5: SM3VAL5,
    #[doc = "0x140 - Fractional Control Register"]
    pub sm3frctrl: SM3FRCTRL,
    #[doc = "0x142 - Output Control Register"]
    pub sm3octrl: SM3OCTRL,
    #[doc = "0x144 - Status Register"]
    pub sm3sts: SM3STS,
    #[doc = "0x146 - Interrupt Enable Register"]
    pub sm3inten: SM3INTEN,
    #[doc = "0x148 - DMA Enable Register"]
    pub sm3dmaen: SM3DMAEN,
    #[doc = "0x14a - Output Trigger Control Register"]
    pub sm3tctrl: SM3TCTRL,
    #[doc = "0x14c - Fault Disable Mapping Register 0"]
    pub sm3dismap0: SM3DISMAP0,
    #[doc = "0x14e - Fault Disable Mapping Register 1"]
    pub sm3dismap1: SM3DISMAP1,
    #[doc = "0x150 - Deadtime Count Register 0"]
    pub sm3dtcnt0: SM3DTCNT0,
    #[doc = "0x152 - Deadtime Count Register 1"]
    pub sm3dtcnt1: SM3DTCNT1,
    #[doc = "0x154 - Capture Control A Register"]
    pub sm3captctrla: SM3CAPTCTRLA,
    #[doc = "0x156 - Capture Compare A Register"]
    pub sm3captcompa: SM3CAPTCOMPA,
    #[doc = "0x158 - Capture Control B Register"]
    pub sm3captctrlb: SM3CAPTCTRLB,
    #[doc = "0x15a - Capture Compare B Register"]
    pub sm3captcompb: SM3CAPTCOMPB,
    #[doc = "0x15c - Capture Control X Register"]
    pub sm3captctrlx: SM3CAPTCTRLX,
    #[doc = "0x15e - Capture Compare X Register"]
    pub sm3captcompx: SM3CAPTCOMPX,
    #[doc = "0x160 - Capture Value 0 Register"]
    pub sm3cval0: SM3CVAL0,
    #[doc = "0x162 - Capture Value 0 Cycle Register"]
    pub sm3cval0cyc: SM3CVAL0CYC,
    #[doc = "0x164 - Capture Value 1 Register"]
    pub sm3cval1: SM3CVAL1,
    #[doc = "0x166 - Capture Value 1 Cycle Register"]
    pub sm3cval1cyc: SM3CVAL1CYC,
    #[doc = "0x168 - Capture Value 2 Register"]
    pub sm3cval2: SM3CVAL2,
    #[doc = "0x16a - Capture Value 2 Cycle Register"]
    pub sm3cval2cyc: SM3CVAL2CYC,
    #[doc = "0x16c - Capture Value 3 Register"]
    pub sm3cval3: SM3CVAL3,
    #[doc = "0x16e - Capture Value 3 Cycle Register"]
    pub sm3cval3cyc: SM3CVAL3CYC,
    #[doc = "0x170 - Capture Value 4 Register"]
    pub sm3cval4: SM3CVAL4,
    #[doc = "0x172 - Capture Value 4 Cycle Register"]
    pub sm3cval4cyc: SM3CVAL4CYC,
    #[doc = "0x174 - Capture Value 5 Register"]
    pub sm3cval5: SM3CVAL5,
    #[doc = "0x176 - Capture Value 5 Cycle Register"]
    pub sm3cval5cyc: SM3CVAL5CYC,
    _reserved172: [u8; 8usize],
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
#[doc = "Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0cnt](sm0cnt) module"]
pub type SM0CNT = crate::Reg<u16, _SM0CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0CNT;
#[doc = "`read()` method returns [sm0cnt::R](sm0cnt::R) reader structure"]
impl crate::Readable for SM0CNT {}
#[doc = "Counter Register"]
pub mod sm0cnt;
#[doc = "Initial Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0init](sm0init) module"]
pub type SM0INIT = crate::Reg<u16, _SM0INIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0INIT;
#[doc = "`read()` method returns [sm0init::R](sm0init::R) reader structure"]
impl crate::Readable for SM0INIT {}
#[doc = "`write(|w| ..)` method takes [sm0init::W](sm0init::W) writer structure"]
impl crate::Writable for SM0INIT {}
#[doc = "Initial Count Register"]
pub mod sm0init;
#[doc = "Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0ctrl2](sm0ctrl2) module"]
pub type SM0CTRL2 = crate::Reg<u16, _SM0CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0CTRL2;
#[doc = "`read()` method returns [sm0ctrl2::R](sm0ctrl2::R) reader structure"]
impl crate::Readable for SM0CTRL2 {}
#[doc = "`write(|w| ..)` method takes [sm0ctrl2::W](sm0ctrl2::W) writer structure"]
impl crate::Writable for SM0CTRL2 {}
#[doc = "Control 2 Register"]
pub mod sm0ctrl2;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0ctrl](sm0ctrl) module"]
pub type SM0CTRL = crate::Reg<u16, _SM0CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0CTRL;
#[doc = "`read()` method returns [sm0ctrl::R](sm0ctrl::R) reader structure"]
impl crate::Readable for SM0CTRL {}
#[doc = "`write(|w| ..)` method takes [sm0ctrl::W](sm0ctrl::W) writer structure"]
impl crate::Writable for SM0CTRL {}
#[doc = "Control Register"]
pub mod sm0ctrl;
#[doc = "Value Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0val0](sm0val0) module"]
pub type SM0VAL0 = crate::Reg<u16, _SM0VAL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0VAL0;
#[doc = "`read()` method returns [sm0val0::R](sm0val0::R) reader structure"]
impl crate::Readable for SM0VAL0 {}
#[doc = "`write(|w| ..)` method takes [sm0val0::W](sm0val0::W) writer structure"]
impl crate::Writable for SM0VAL0 {}
#[doc = "Value Register 0"]
pub mod sm0val0;
#[doc = "Fractional Value Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0fracval1](sm0fracval1) module"]
pub type SM0FRACVAL1 = crate::Reg<u16, _SM0FRACVAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0FRACVAL1;
#[doc = "`read()` method returns [sm0fracval1::R](sm0fracval1::R) reader structure"]
impl crate::Readable for SM0FRACVAL1 {}
#[doc = "`write(|w| ..)` method takes [sm0fracval1::W](sm0fracval1::W) writer structure"]
impl crate::Writable for SM0FRACVAL1 {}
#[doc = "Fractional Value Register 1"]
pub mod sm0fracval1;
#[doc = "Value Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0val1](sm0val1) module"]
pub type SM0VAL1 = crate::Reg<u16, _SM0VAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0VAL1;
#[doc = "`read()` method returns [sm0val1::R](sm0val1::R) reader structure"]
impl crate::Readable for SM0VAL1 {}
#[doc = "`write(|w| ..)` method takes [sm0val1::W](sm0val1::W) writer structure"]
impl crate::Writable for SM0VAL1 {}
#[doc = "Value Register 1"]
pub mod sm0val1;
#[doc = "Fractional Value Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0fracval2](sm0fracval2) module"]
pub type SM0FRACVAL2 = crate::Reg<u16, _SM0FRACVAL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0FRACVAL2;
#[doc = "`read()` method returns [sm0fracval2::R](sm0fracval2::R) reader structure"]
impl crate::Readable for SM0FRACVAL2 {}
#[doc = "`write(|w| ..)` method takes [sm0fracval2::W](sm0fracval2::W) writer structure"]
impl crate::Writable for SM0FRACVAL2 {}
#[doc = "Fractional Value Register 2"]
pub mod sm0fracval2;
#[doc = "Value Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0val2](sm0val2) module"]
pub type SM0VAL2 = crate::Reg<u16, _SM0VAL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0VAL2;
#[doc = "`read()` method returns [sm0val2::R](sm0val2::R) reader structure"]
impl crate::Readable for SM0VAL2 {}
#[doc = "`write(|w| ..)` method takes [sm0val2::W](sm0val2::W) writer structure"]
impl crate::Writable for SM0VAL2 {}
#[doc = "Value Register 2"]
pub mod sm0val2;
#[doc = "Fractional Value Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0fracval3](sm0fracval3) module"]
pub type SM0FRACVAL3 = crate::Reg<u16, _SM0FRACVAL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0FRACVAL3;
#[doc = "`read()` method returns [sm0fracval3::R](sm0fracval3::R) reader structure"]
impl crate::Readable for SM0FRACVAL3 {}
#[doc = "`write(|w| ..)` method takes [sm0fracval3::W](sm0fracval3::W) writer structure"]
impl crate::Writable for SM0FRACVAL3 {}
#[doc = "Fractional Value Register 3"]
pub mod sm0fracval3;
#[doc = "Value Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0val3](sm0val3) module"]
pub type SM0VAL3 = crate::Reg<u16, _SM0VAL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0VAL3;
#[doc = "`read()` method returns [sm0val3::R](sm0val3::R) reader structure"]
impl crate::Readable for SM0VAL3 {}
#[doc = "`write(|w| ..)` method takes [sm0val3::W](sm0val3::W) writer structure"]
impl crate::Writable for SM0VAL3 {}
#[doc = "Value Register 3"]
pub mod sm0val3;
#[doc = "Fractional Value Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0fracval4](sm0fracval4) module"]
pub type SM0FRACVAL4 = crate::Reg<u16, _SM0FRACVAL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0FRACVAL4;
#[doc = "`read()` method returns [sm0fracval4::R](sm0fracval4::R) reader structure"]
impl crate::Readable for SM0FRACVAL4 {}
#[doc = "`write(|w| ..)` method takes [sm0fracval4::W](sm0fracval4::W) writer structure"]
impl crate::Writable for SM0FRACVAL4 {}
#[doc = "Fractional Value Register 4"]
pub mod sm0fracval4;
#[doc = "Value Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0val4](sm0val4) module"]
pub type SM0VAL4 = crate::Reg<u16, _SM0VAL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0VAL4;
#[doc = "`read()` method returns [sm0val4::R](sm0val4::R) reader structure"]
impl crate::Readable for SM0VAL4 {}
#[doc = "`write(|w| ..)` method takes [sm0val4::W](sm0val4::W) writer structure"]
impl crate::Writable for SM0VAL4 {}
#[doc = "Value Register 4"]
pub mod sm0val4;
#[doc = "Fractional Value Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0fracval5](sm0fracval5) module"]
pub type SM0FRACVAL5 = crate::Reg<u16, _SM0FRACVAL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0FRACVAL5;
#[doc = "`read()` method returns [sm0fracval5::R](sm0fracval5::R) reader structure"]
impl crate::Readable for SM0FRACVAL5 {}
#[doc = "`write(|w| ..)` method takes [sm0fracval5::W](sm0fracval5::W) writer structure"]
impl crate::Writable for SM0FRACVAL5 {}
#[doc = "Fractional Value Register 5"]
pub mod sm0fracval5;
#[doc = "Value Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0val5](sm0val5) module"]
pub type SM0VAL5 = crate::Reg<u16, _SM0VAL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0VAL5;
#[doc = "`read()` method returns [sm0val5::R](sm0val5::R) reader structure"]
impl crate::Readable for SM0VAL5 {}
#[doc = "`write(|w| ..)` method takes [sm0val5::W](sm0val5::W) writer structure"]
impl crate::Writable for SM0VAL5 {}
#[doc = "Value Register 5"]
pub mod sm0val5;
#[doc = "Fractional Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0frctrl](sm0frctrl) module"]
pub type SM0FRCTRL = crate::Reg<u16, _SM0FRCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0FRCTRL;
#[doc = "`read()` method returns [sm0frctrl::R](sm0frctrl::R) reader structure"]
impl crate::Readable for SM0FRCTRL {}
#[doc = "`write(|w| ..)` method takes [sm0frctrl::W](sm0frctrl::W) writer structure"]
impl crate::Writable for SM0FRCTRL {}
#[doc = "Fractional Control Register"]
pub mod sm0frctrl;
#[doc = "Output Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0octrl](sm0octrl) module"]
pub type SM0OCTRL = crate::Reg<u16, _SM0OCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0OCTRL;
#[doc = "`read()` method returns [sm0octrl::R](sm0octrl::R) reader structure"]
impl crate::Readable for SM0OCTRL {}
#[doc = "`write(|w| ..)` method takes [sm0octrl::W](sm0octrl::W) writer structure"]
impl crate::Writable for SM0OCTRL {}
#[doc = "Output Control Register"]
pub mod sm0octrl;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0sts](sm0sts) module"]
pub type SM0STS = crate::Reg<u16, _SM0STS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0STS;
#[doc = "`read()` method returns [sm0sts::R](sm0sts::R) reader structure"]
impl crate::Readable for SM0STS {}
#[doc = "`write(|w| ..)` method takes [sm0sts::W](sm0sts::W) writer structure"]
impl crate::Writable for SM0STS {}
#[doc = "Status Register"]
pub mod sm0sts;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0inten](sm0inten) module"]
pub type SM0INTEN = crate::Reg<u16, _SM0INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0INTEN;
#[doc = "`read()` method returns [sm0inten::R](sm0inten::R) reader structure"]
impl crate::Readable for SM0INTEN {}
#[doc = "`write(|w| ..)` method takes [sm0inten::W](sm0inten::W) writer structure"]
impl crate::Writable for SM0INTEN {}
#[doc = "Interrupt Enable Register"]
pub mod sm0inten;
#[doc = "DMA Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0dmaen](sm0dmaen) module"]
pub type SM0DMAEN = crate::Reg<u16, _SM0DMAEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0DMAEN;
#[doc = "`read()` method returns [sm0dmaen::R](sm0dmaen::R) reader structure"]
impl crate::Readable for SM0DMAEN {}
#[doc = "`write(|w| ..)` method takes [sm0dmaen::W](sm0dmaen::W) writer structure"]
impl crate::Writable for SM0DMAEN {}
#[doc = "DMA Enable Register"]
pub mod sm0dmaen;
#[doc = "Output Trigger Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0tctrl](sm0tctrl) module"]
pub type SM0TCTRL = crate::Reg<u16, _SM0TCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0TCTRL;
#[doc = "`read()` method returns [sm0tctrl::R](sm0tctrl::R) reader structure"]
impl crate::Readable for SM0TCTRL {}
#[doc = "`write(|w| ..)` method takes [sm0tctrl::W](sm0tctrl::W) writer structure"]
impl crate::Writable for SM0TCTRL {}
#[doc = "Output Trigger Control Register"]
pub mod sm0tctrl;
#[doc = "Fault Disable Mapping Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0dismap0](sm0dismap0) module"]
pub type SM0DISMAP0 = crate::Reg<u16, _SM0DISMAP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0DISMAP0;
#[doc = "`read()` method returns [sm0dismap0::R](sm0dismap0::R) reader structure"]
impl crate::Readable for SM0DISMAP0 {}
#[doc = "`write(|w| ..)` method takes [sm0dismap0::W](sm0dismap0::W) writer structure"]
impl crate::Writable for SM0DISMAP0 {}
#[doc = "Fault Disable Mapping Register 0"]
pub mod sm0dismap0;
#[doc = "Fault Disable Mapping Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0dismap1](sm0dismap1) module"]
pub type SM0DISMAP1 = crate::Reg<u16, _SM0DISMAP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0DISMAP1;
#[doc = "`read()` method returns [sm0dismap1::R](sm0dismap1::R) reader structure"]
impl crate::Readable for SM0DISMAP1 {}
#[doc = "`write(|w| ..)` method takes [sm0dismap1::W](sm0dismap1::W) writer structure"]
impl crate::Writable for SM0DISMAP1 {}
#[doc = "Fault Disable Mapping Register 1"]
pub mod sm0dismap1;
#[doc = "Deadtime Count Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0dtcnt0](sm0dtcnt0) module"]
pub type SM0DTCNT0 = crate::Reg<u16, _SM0DTCNT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0DTCNT0;
#[doc = "`read()` method returns [sm0dtcnt0::R](sm0dtcnt0::R) reader structure"]
impl crate::Readable for SM0DTCNT0 {}
#[doc = "`write(|w| ..)` method takes [sm0dtcnt0::W](sm0dtcnt0::W) writer structure"]
impl crate::Writable for SM0DTCNT0 {}
#[doc = "Deadtime Count Register 0"]
pub mod sm0dtcnt0;
#[doc = "Deadtime Count Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0dtcnt1](sm0dtcnt1) module"]
pub type SM0DTCNT1 = crate::Reg<u16, _SM0DTCNT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0DTCNT1;
#[doc = "`read()` method returns [sm0dtcnt1::R](sm0dtcnt1::R) reader structure"]
impl crate::Readable for SM0DTCNT1 {}
#[doc = "`write(|w| ..)` method takes [sm0dtcnt1::W](sm0dtcnt1::W) writer structure"]
impl crate::Writable for SM0DTCNT1 {}
#[doc = "Deadtime Count Register 1"]
pub mod sm0dtcnt1;
#[doc = "Capture Control A Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0captctrla](sm0captctrla) module"]
pub type SM0CAPTCTRLA = crate::Reg<u16, _SM0CAPTCTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0CAPTCTRLA;
#[doc = "`read()` method returns [sm0captctrla::R](sm0captctrla::R) reader structure"]
impl crate::Readable for SM0CAPTCTRLA {}
#[doc = "`write(|w| ..)` method takes [sm0captctrla::W](sm0captctrla::W) writer structure"]
impl crate::Writable for SM0CAPTCTRLA {}
#[doc = "Capture Control A Register"]
pub mod sm0captctrla;
#[doc = "Capture Compare A Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0captcompa](sm0captcompa) module"]
pub type SM0CAPTCOMPA = crate::Reg<u16, _SM0CAPTCOMPA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0CAPTCOMPA;
#[doc = "`read()` method returns [sm0captcompa::R](sm0captcompa::R) reader structure"]
impl crate::Readable for SM0CAPTCOMPA {}
#[doc = "`write(|w| ..)` method takes [sm0captcompa::W](sm0captcompa::W) writer structure"]
impl crate::Writable for SM0CAPTCOMPA {}
#[doc = "Capture Compare A Register"]
pub mod sm0captcompa;
#[doc = "Capture Control B Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0captctrlb](sm0captctrlb) module"]
pub type SM0CAPTCTRLB = crate::Reg<u16, _SM0CAPTCTRLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0CAPTCTRLB;
#[doc = "`read()` method returns [sm0captctrlb::R](sm0captctrlb::R) reader structure"]
impl crate::Readable for SM0CAPTCTRLB {}
#[doc = "`write(|w| ..)` method takes [sm0captctrlb::W](sm0captctrlb::W) writer structure"]
impl crate::Writable for SM0CAPTCTRLB {}
#[doc = "Capture Control B Register"]
pub mod sm0captctrlb;
#[doc = "Capture Compare B Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0captcompb](sm0captcompb) module"]
pub type SM0CAPTCOMPB = crate::Reg<u16, _SM0CAPTCOMPB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0CAPTCOMPB;
#[doc = "`read()` method returns [sm0captcompb::R](sm0captcompb::R) reader structure"]
impl crate::Readable for SM0CAPTCOMPB {}
#[doc = "`write(|w| ..)` method takes [sm0captcompb::W](sm0captcompb::W) writer structure"]
impl crate::Writable for SM0CAPTCOMPB {}
#[doc = "Capture Compare B Register"]
pub mod sm0captcompb;
#[doc = "Capture Control X Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0captctrlx](sm0captctrlx) module"]
pub type SM0CAPTCTRLX = crate::Reg<u16, _SM0CAPTCTRLX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0CAPTCTRLX;
#[doc = "`read()` method returns [sm0captctrlx::R](sm0captctrlx::R) reader structure"]
impl crate::Readable for SM0CAPTCTRLX {}
#[doc = "`write(|w| ..)` method takes [sm0captctrlx::W](sm0captctrlx::W) writer structure"]
impl crate::Writable for SM0CAPTCTRLX {}
#[doc = "Capture Control X Register"]
pub mod sm0captctrlx;
#[doc = "Capture Compare X Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0captcompx](sm0captcompx) module"]
pub type SM0CAPTCOMPX = crate::Reg<u16, _SM0CAPTCOMPX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0CAPTCOMPX;
#[doc = "`read()` method returns [sm0captcompx::R](sm0captcompx::R) reader structure"]
impl crate::Readable for SM0CAPTCOMPX {}
#[doc = "`write(|w| ..)` method takes [sm0captcompx::W](sm0captcompx::W) writer structure"]
impl crate::Writable for SM0CAPTCOMPX {}
#[doc = "Capture Compare X Register"]
pub mod sm0captcompx;
#[doc = "Capture Value 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0cval0](sm0cval0) module"]
pub type SM0CVAL0 = crate::Reg<u16, _SM0CVAL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0CVAL0;
#[doc = "`read()` method returns [sm0cval0::R](sm0cval0::R) reader structure"]
impl crate::Readable for SM0CVAL0 {}
#[doc = "Capture Value 0 Register"]
pub mod sm0cval0;
#[doc = "Capture Value 0 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0cval0cyc](sm0cval0cyc) module"]
pub type SM0CVAL0CYC = crate::Reg<u16, _SM0CVAL0CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0CVAL0CYC;
#[doc = "`read()` method returns [sm0cval0cyc::R](sm0cval0cyc::R) reader structure"]
impl crate::Readable for SM0CVAL0CYC {}
#[doc = "Capture Value 0 Cycle Register"]
pub mod sm0cval0cyc;
#[doc = "Capture Value 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0cval1](sm0cval1) module"]
pub type SM0CVAL1 = crate::Reg<u16, _SM0CVAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0CVAL1;
#[doc = "`read()` method returns [sm0cval1::R](sm0cval1::R) reader structure"]
impl crate::Readable for SM0CVAL1 {}
#[doc = "Capture Value 1 Register"]
pub mod sm0cval1;
#[doc = "Capture Value 1 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0cval1cyc](sm0cval1cyc) module"]
pub type SM0CVAL1CYC = crate::Reg<u16, _SM0CVAL1CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0CVAL1CYC;
#[doc = "`read()` method returns [sm0cval1cyc::R](sm0cval1cyc::R) reader structure"]
impl crate::Readable for SM0CVAL1CYC {}
#[doc = "Capture Value 1 Cycle Register"]
pub mod sm0cval1cyc;
#[doc = "Capture Value 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0cval2](sm0cval2) module"]
pub type SM0CVAL2 = crate::Reg<u16, _SM0CVAL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0CVAL2;
#[doc = "`read()` method returns [sm0cval2::R](sm0cval2::R) reader structure"]
impl crate::Readable for SM0CVAL2 {}
#[doc = "Capture Value 2 Register"]
pub mod sm0cval2;
#[doc = "Capture Value 2 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0cval2cyc](sm0cval2cyc) module"]
pub type SM0CVAL2CYC = crate::Reg<u16, _SM0CVAL2CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0CVAL2CYC;
#[doc = "`read()` method returns [sm0cval2cyc::R](sm0cval2cyc::R) reader structure"]
impl crate::Readable for SM0CVAL2CYC {}
#[doc = "Capture Value 2 Cycle Register"]
pub mod sm0cval2cyc;
#[doc = "Capture Value 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0cval3](sm0cval3) module"]
pub type SM0CVAL3 = crate::Reg<u16, _SM0CVAL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0CVAL3;
#[doc = "`read()` method returns [sm0cval3::R](sm0cval3::R) reader structure"]
impl crate::Readable for SM0CVAL3 {}
#[doc = "Capture Value 3 Register"]
pub mod sm0cval3;
#[doc = "Capture Value 3 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0cval3cyc](sm0cval3cyc) module"]
pub type SM0CVAL3CYC = crate::Reg<u16, _SM0CVAL3CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0CVAL3CYC;
#[doc = "`read()` method returns [sm0cval3cyc::R](sm0cval3cyc::R) reader structure"]
impl crate::Readable for SM0CVAL3CYC {}
#[doc = "Capture Value 3 Cycle Register"]
pub mod sm0cval3cyc;
#[doc = "Capture Value 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0cval4](sm0cval4) module"]
pub type SM0CVAL4 = crate::Reg<u16, _SM0CVAL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0CVAL4;
#[doc = "`read()` method returns [sm0cval4::R](sm0cval4::R) reader structure"]
impl crate::Readable for SM0CVAL4 {}
#[doc = "Capture Value 4 Register"]
pub mod sm0cval4;
#[doc = "Capture Value 4 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0cval4cyc](sm0cval4cyc) module"]
pub type SM0CVAL4CYC = crate::Reg<u16, _SM0CVAL4CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0CVAL4CYC;
#[doc = "`read()` method returns [sm0cval4cyc::R](sm0cval4cyc::R) reader structure"]
impl crate::Readable for SM0CVAL4CYC {}
#[doc = "Capture Value 4 Cycle Register"]
pub mod sm0cval4cyc;
#[doc = "Capture Value 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0cval5](sm0cval5) module"]
pub type SM0CVAL5 = crate::Reg<u16, _SM0CVAL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0CVAL5;
#[doc = "`read()` method returns [sm0cval5::R](sm0cval5::R) reader structure"]
impl crate::Readable for SM0CVAL5 {}
#[doc = "Capture Value 5 Register"]
pub mod sm0cval5;
#[doc = "Capture Value 5 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm0cval5cyc](sm0cval5cyc) module"]
pub type SM0CVAL5CYC = crate::Reg<u16, _SM0CVAL5CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM0CVAL5CYC;
#[doc = "`read()` method returns [sm0cval5cyc::R](sm0cval5cyc::R) reader structure"]
impl crate::Readable for SM0CVAL5CYC {}
#[doc = "Capture Value 5 Cycle Register"]
pub mod sm0cval5cyc;
#[doc = "Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1cnt](sm1cnt) module"]
pub type SM1CNT = crate::Reg<u16, _SM1CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1CNT;
#[doc = "`read()` method returns [sm1cnt::R](sm1cnt::R) reader structure"]
impl crate::Readable for SM1CNT {}
#[doc = "Counter Register"]
pub mod sm1cnt;
#[doc = "Initial Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1init](sm1init) module"]
pub type SM1INIT = crate::Reg<u16, _SM1INIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1INIT;
#[doc = "`read()` method returns [sm1init::R](sm1init::R) reader structure"]
impl crate::Readable for SM1INIT {}
#[doc = "`write(|w| ..)` method takes [sm1init::W](sm1init::W) writer structure"]
impl crate::Writable for SM1INIT {}
#[doc = "Initial Count Register"]
pub mod sm1init;
#[doc = "Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1ctrl2](sm1ctrl2) module"]
pub type SM1CTRL2 = crate::Reg<u16, _SM1CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1CTRL2;
#[doc = "`read()` method returns [sm1ctrl2::R](sm1ctrl2::R) reader structure"]
impl crate::Readable for SM1CTRL2 {}
#[doc = "`write(|w| ..)` method takes [sm1ctrl2::W](sm1ctrl2::W) writer structure"]
impl crate::Writable for SM1CTRL2 {}
#[doc = "Control 2 Register"]
pub mod sm1ctrl2;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1ctrl](sm1ctrl) module"]
pub type SM1CTRL = crate::Reg<u16, _SM1CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1CTRL;
#[doc = "`read()` method returns [sm1ctrl::R](sm1ctrl::R) reader structure"]
impl crate::Readable for SM1CTRL {}
#[doc = "`write(|w| ..)` method takes [sm1ctrl::W](sm1ctrl::W) writer structure"]
impl crate::Writable for SM1CTRL {}
#[doc = "Control Register"]
pub mod sm1ctrl;
#[doc = "Value Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1val0](sm1val0) module"]
pub type SM1VAL0 = crate::Reg<u16, _SM1VAL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1VAL0;
#[doc = "`read()` method returns [sm1val0::R](sm1val0::R) reader structure"]
impl crate::Readable for SM1VAL0 {}
#[doc = "`write(|w| ..)` method takes [sm1val0::W](sm1val0::W) writer structure"]
impl crate::Writable for SM1VAL0 {}
#[doc = "Value Register 0"]
pub mod sm1val0;
#[doc = "Fractional Value Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1fracval1](sm1fracval1) module"]
pub type SM1FRACVAL1 = crate::Reg<u16, _SM1FRACVAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1FRACVAL1;
#[doc = "`read()` method returns [sm1fracval1::R](sm1fracval1::R) reader structure"]
impl crate::Readable for SM1FRACVAL1 {}
#[doc = "`write(|w| ..)` method takes [sm1fracval1::W](sm1fracval1::W) writer structure"]
impl crate::Writable for SM1FRACVAL1 {}
#[doc = "Fractional Value Register 1"]
pub mod sm1fracval1;
#[doc = "Value Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1val1](sm1val1) module"]
pub type SM1VAL1 = crate::Reg<u16, _SM1VAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1VAL1;
#[doc = "`read()` method returns [sm1val1::R](sm1val1::R) reader structure"]
impl crate::Readable for SM1VAL1 {}
#[doc = "`write(|w| ..)` method takes [sm1val1::W](sm1val1::W) writer structure"]
impl crate::Writable for SM1VAL1 {}
#[doc = "Value Register 1"]
pub mod sm1val1;
#[doc = "Fractional Value Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1fracval2](sm1fracval2) module"]
pub type SM1FRACVAL2 = crate::Reg<u16, _SM1FRACVAL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1FRACVAL2;
#[doc = "`read()` method returns [sm1fracval2::R](sm1fracval2::R) reader structure"]
impl crate::Readable for SM1FRACVAL2 {}
#[doc = "`write(|w| ..)` method takes [sm1fracval2::W](sm1fracval2::W) writer structure"]
impl crate::Writable for SM1FRACVAL2 {}
#[doc = "Fractional Value Register 2"]
pub mod sm1fracval2;
#[doc = "Value Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1val2](sm1val2) module"]
pub type SM1VAL2 = crate::Reg<u16, _SM1VAL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1VAL2;
#[doc = "`read()` method returns [sm1val2::R](sm1val2::R) reader structure"]
impl crate::Readable for SM1VAL2 {}
#[doc = "`write(|w| ..)` method takes [sm1val2::W](sm1val2::W) writer structure"]
impl crate::Writable for SM1VAL2 {}
#[doc = "Value Register 2"]
pub mod sm1val2;
#[doc = "Fractional Value Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1fracval3](sm1fracval3) module"]
pub type SM1FRACVAL3 = crate::Reg<u16, _SM1FRACVAL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1FRACVAL3;
#[doc = "`read()` method returns [sm1fracval3::R](sm1fracval3::R) reader structure"]
impl crate::Readable for SM1FRACVAL3 {}
#[doc = "`write(|w| ..)` method takes [sm1fracval3::W](sm1fracval3::W) writer structure"]
impl crate::Writable for SM1FRACVAL3 {}
#[doc = "Fractional Value Register 3"]
pub mod sm1fracval3;
#[doc = "Value Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1val3](sm1val3) module"]
pub type SM1VAL3 = crate::Reg<u16, _SM1VAL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1VAL3;
#[doc = "`read()` method returns [sm1val3::R](sm1val3::R) reader structure"]
impl crate::Readable for SM1VAL3 {}
#[doc = "`write(|w| ..)` method takes [sm1val3::W](sm1val3::W) writer structure"]
impl crate::Writable for SM1VAL3 {}
#[doc = "Value Register 3"]
pub mod sm1val3;
#[doc = "Fractional Value Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1fracval4](sm1fracval4) module"]
pub type SM1FRACVAL4 = crate::Reg<u16, _SM1FRACVAL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1FRACVAL4;
#[doc = "`read()` method returns [sm1fracval4::R](sm1fracval4::R) reader structure"]
impl crate::Readable for SM1FRACVAL4 {}
#[doc = "`write(|w| ..)` method takes [sm1fracval4::W](sm1fracval4::W) writer structure"]
impl crate::Writable for SM1FRACVAL4 {}
#[doc = "Fractional Value Register 4"]
pub mod sm1fracval4;
#[doc = "Value Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1val4](sm1val4) module"]
pub type SM1VAL4 = crate::Reg<u16, _SM1VAL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1VAL4;
#[doc = "`read()` method returns [sm1val4::R](sm1val4::R) reader structure"]
impl crate::Readable for SM1VAL4 {}
#[doc = "`write(|w| ..)` method takes [sm1val4::W](sm1val4::W) writer structure"]
impl crate::Writable for SM1VAL4 {}
#[doc = "Value Register 4"]
pub mod sm1val4;
#[doc = "Fractional Value Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1fracval5](sm1fracval5) module"]
pub type SM1FRACVAL5 = crate::Reg<u16, _SM1FRACVAL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1FRACVAL5;
#[doc = "`read()` method returns [sm1fracval5::R](sm1fracval5::R) reader structure"]
impl crate::Readable for SM1FRACVAL5 {}
#[doc = "`write(|w| ..)` method takes [sm1fracval5::W](sm1fracval5::W) writer structure"]
impl crate::Writable for SM1FRACVAL5 {}
#[doc = "Fractional Value Register 5"]
pub mod sm1fracval5;
#[doc = "Value Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1val5](sm1val5) module"]
pub type SM1VAL5 = crate::Reg<u16, _SM1VAL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1VAL5;
#[doc = "`read()` method returns [sm1val5::R](sm1val5::R) reader structure"]
impl crate::Readable for SM1VAL5 {}
#[doc = "`write(|w| ..)` method takes [sm1val5::W](sm1val5::W) writer structure"]
impl crate::Writable for SM1VAL5 {}
#[doc = "Value Register 5"]
pub mod sm1val5;
#[doc = "Fractional Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1frctrl](sm1frctrl) module"]
pub type SM1FRCTRL = crate::Reg<u16, _SM1FRCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1FRCTRL;
#[doc = "`read()` method returns [sm1frctrl::R](sm1frctrl::R) reader structure"]
impl crate::Readable for SM1FRCTRL {}
#[doc = "`write(|w| ..)` method takes [sm1frctrl::W](sm1frctrl::W) writer structure"]
impl crate::Writable for SM1FRCTRL {}
#[doc = "Fractional Control Register"]
pub mod sm1frctrl;
#[doc = "Output Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1octrl](sm1octrl) module"]
pub type SM1OCTRL = crate::Reg<u16, _SM1OCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1OCTRL;
#[doc = "`read()` method returns [sm1octrl::R](sm1octrl::R) reader structure"]
impl crate::Readable for SM1OCTRL {}
#[doc = "`write(|w| ..)` method takes [sm1octrl::W](sm1octrl::W) writer structure"]
impl crate::Writable for SM1OCTRL {}
#[doc = "Output Control Register"]
pub mod sm1octrl;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1sts](sm1sts) module"]
pub type SM1STS = crate::Reg<u16, _SM1STS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1STS;
#[doc = "`read()` method returns [sm1sts::R](sm1sts::R) reader structure"]
impl crate::Readable for SM1STS {}
#[doc = "`write(|w| ..)` method takes [sm1sts::W](sm1sts::W) writer structure"]
impl crate::Writable for SM1STS {}
#[doc = "Status Register"]
pub mod sm1sts;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1inten](sm1inten) module"]
pub type SM1INTEN = crate::Reg<u16, _SM1INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1INTEN;
#[doc = "`read()` method returns [sm1inten::R](sm1inten::R) reader structure"]
impl crate::Readable for SM1INTEN {}
#[doc = "`write(|w| ..)` method takes [sm1inten::W](sm1inten::W) writer structure"]
impl crate::Writable for SM1INTEN {}
#[doc = "Interrupt Enable Register"]
pub mod sm1inten;
#[doc = "DMA Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1dmaen](sm1dmaen) module"]
pub type SM1DMAEN = crate::Reg<u16, _SM1DMAEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1DMAEN;
#[doc = "`read()` method returns [sm1dmaen::R](sm1dmaen::R) reader structure"]
impl crate::Readable for SM1DMAEN {}
#[doc = "`write(|w| ..)` method takes [sm1dmaen::W](sm1dmaen::W) writer structure"]
impl crate::Writable for SM1DMAEN {}
#[doc = "DMA Enable Register"]
pub mod sm1dmaen;
#[doc = "Output Trigger Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1tctrl](sm1tctrl) module"]
pub type SM1TCTRL = crate::Reg<u16, _SM1TCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1TCTRL;
#[doc = "`read()` method returns [sm1tctrl::R](sm1tctrl::R) reader structure"]
impl crate::Readable for SM1TCTRL {}
#[doc = "`write(|w| ..)` method takes [sm1tctrl::W](sm1tctrl::W) writer structure"]
impl crate::Writable for SM1TCTRL {}
#[doc = "Output Trigger Control Register"]
pub mod sm1tctrl;
#[doc = "Fault Disable Mapping Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1dismap0](sm1dismap0) module"]
pub type SM1DISMAP0 = crate::Reg<u16, _SM1DISMAP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1DISMAP0;
#[doc = "`read()` method returns [sm1dismap0::R](sm1dismap0::R) reader structure"]
impl crate::Readable for SM1DISMAP0 {}
#[doc = "`write(|w| ..)` method takes [sm1dismap0::W](sm1dismap0::W) writer structure"]
impl crate::Writable for SM1DISMAP0 {}
#[doc = "Fault Disable Mapping Register 0"]
pub mod sm1dismap0;
#[doc = "Fault Disable Mapping Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1dismap1](sm1dismap1) module"]
pub type SM1DISMAP1 = crate::Reg<u16, _SM1DISMAP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1DISMAP1;
#[doc = "`read()` method returns [sm1dismap1::R](sm1dismap1::R) reader structure"]
impl crate::Readable for SM1DISMAP1 {}
#[doc = "`write(|w| ..)` method takes [sm1dismap1::W](sm1dismap1::W) writer structure"]
impl crate::Writable for SM1DISMAP1 {}
#[doc = "Fault Disable Mapping Register 1"]
pub mod sm1dismap1;
#[doc = "Deadtime Count Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1dtcnt0](sm1dtcnt0) module"]
pub type SM1DTCNT0 = crate::Reg<u16, _SM1DTCNT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1DTCNT0;
#[doc = "`read()` method returns [sm1dtcnt0::R](sm1dtcnt0::R) reader structure"]
impl crate::Readable for SM1DTCNT0 {}
#[doc = "`write(|w| ..)` method takes [sm1dtcnt0::W](sm1dtcnt0::W) writer structure"]
impl crate::Writable for SM1DTCNT0 {}
#[doc = "Deadtime Count Register 0"]
pub mod sm1dtcnt0;
#[doc = "Deadtime Count Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1dtcnt1](sm1dtcnt1) module"]
pub type SM1DTCNT1 = crate::Reg<u16, _SM1DTCNT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1DTCNT1;
#[doc = "`read()` method returns [sm1dtcnt1::R](sm1dtcnt1::R) reader structure"]
impl crate::Readable for SM1DTCNT1 {}
#[doc = "`write(|w| ..)` method takes [sm1dtcnt1::W](sm1dtcnt1::W) writer structure"]
impl crate::Writable for SM1DTCNT1 {}
#[doc = "Deadtime Count Register 1"]
pub mod sm1dtcnt1;
#[doc = "Capture Control A Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1captctrla](sm1captctrla) module"]
pub type SM1CAPTCTRLA = crate::Reg<u16, _SM1CAPTCTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1CAPTCTRLA;
#[doc = "`read()` method returns [sm1captctrla::R](sm1captctrla::R) reader structure"]
impl crate::Readable for SM1CAPTCTRLA {}
#[doc = "`write(|w| ..)` method takes [sm1captctrla::W](sm1captctrla::W) writer structure"]
impl crate::Writable for SM1CAPTCTRLA {}
#[doc = "Capture Control A Register"]
pub mod sm1captctrla;
#[doc = "Capture Compare A Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1captcompa](sm1captcompa) module"]
pub type SM1CAPTCOMPA = crate::Reg<u16, _SM1CAPTCOMPA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1CAPTCOMPA;
#[doc = "`read()` method returns [sm1captcompa::R](sm1captcompa::R) reader structure"]
impl crate::Readable for SM1CAPTCOMPA {}
#[doc = "`write(|w| ..)` method takes [sm1captcompa::W](sm1captcompa::W) writer structure"]
impl crate::Writable for SM1CAPTCOMPA {}
#[doc = "Capture Compare A Register"]
pub mod sm1captcompa;
#[doc = "Capture Control B Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1captctrlb](sm1captctrlb) module"]
pub type SM1CAPTCTRLB = crate::Reg<u16, _SM1CAPTCTRLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1CAPTCTRLB;
#[doc = "`read()` method returns [sm1captctrlb::R](sm1captctrlb::R) reader structure"]
impl crate::Readable for SM1CAPTCTRLB {}
#[doc = "`write(|w| ..)` method takes [sm1captctrlb::W](sm1captctrlb::W) writer structure"]
impl crate::Writable for SM1CAPTCTRLB {}
#[doc = "Capture Control B Register"]
pub mod sm1captctrlb;
#[doc = "Capture Compare B Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1captcompb](sm1captcompb) module"]
pub type SM1CAPTCOMPB = crate::Reg<u16, _SM1CAPTCOMPB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1CAPTCOMPB;
#[doc = "`read()` method returns [sm1captcompb::R](sm1captcompb::R) reader structure"]
impl crate::Readable for SM1CAPTCOMPB {}
#[doc = "`write(|w| ..)` method takes [sm1captcompb::W](sm1captcompb::W) writer structure"]
impl crate::Writable for SM1CAPTCOMPB {}
#[doc = "Capture Compare B Register"]
pub mod sm1captcompb;
#[doc = "Capture Control X Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1captctrlx](sm1captctrlx) module"]
pub type SM1CAPTCTRLX = crate::Reg<u16, _SM1CAPTCTRLX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1CAPTCTRLX;
#[doc = "`read()` method returns [sm1captctrlx::R](sm1captctrlx::R) reader structure"]
impl crate::Readable for SM1CAPTCTRLX {}
#[doc = "`write(|w| ..)` method takes [sm1captctrlx::W](sm1captctrlx::W) writer structure"]
impl crate::Writable for SM1CAPTCTRLX {}
#[doc = "Capture Control X Register"]
pub mod sm1captctrlx;
#[doc = "Capture Compare X Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1captcompx](sm1captcompx) module"]
pub type SM1CAPTCOMPX = crate::Reg<u16, _SM1CAPTCOMPX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1CAPTCOMPX;
#[doc = "`read()` method returns [sm1captcompx::R](sm1captcompx::R) reader structure"]
impl crate::Readable for SM1CAPTCOMPX {}
#[doc = "`write(|w| ..)` method takes [sm1captcompx::W](sm1captcompx::W) writer structure"]
impl crate::Writable for SM1CAPTCOMPX {}
#[doc = "Capture Compare X Register"]
pub mod sm1captcompx;
#[doc = "Capture Value 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1cval0](sm1cval0) module"]
pub type SM1CVAL0 = crate::Reg<u16, _SM1CVAL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1CVAL0;
#[doc = "`read()` method returns [sm1cval0::R](sm1cval0::R) reader structure"]
impl crate::Readable for SM1CVAL0 {}
#[doc = "Capture Value 0 Register"]
pub mod sm1cval0;
#[doc = "Capture Value 0 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1cval0cyc](sm1cval0cyc) module"]
pub type SM1CVAL0CYC = crate::Reg<u16, _SM1CVAL0CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1CVAL0CYC;
#[doc = "`read()` method returns [sm1cval0cyc::R](sm1cval0cyc::R) reader structure"]
impl crate::Readable for SM1CVAL0CYC {}
#[doc = "Capture Value 0 Cycle Register"]
pub mod sm1cval0cyc;
#[doc = "Capture Value 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1cval1](sm1cval1) module"]
pub type SM1CVAL1 = crate::Reg<u16, _SM1CVAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1CVAL1;
#[doc = "`read()` method returns [sm1cval1::R](sm1cval1::R) reader structure"]
impl crate::Readable for SM1CVAL1 {}
#[doc = "Capture Value 1 Register"]
pub mod sm1cval1;
#[doc = "Capture Value 1 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1cval1cyc](sm1cval1cyc) module"]
pub type SM1CVAL1CYC = crate::Reg<u16, _SM1CVAL1CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1CVAL1CYC;
#[doc = "`read()` method returns [sm1cval1cyc::R](sm1cval1cyc::R) reader structure"]
impl crate::Readable for SM1CVAL1CYC {}
#[doc = "Capture Value 1 Cycle Register"]
pub mod sm1cval1cyc;
#[doc = "Capture Value 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1cval2](sm1cval2) module"]
pub type SM1CVAL2 = crate::Reg<u16, _SM1CVAL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1CVAL2;
#[doc = "`read()` method returns [sm1cval2::R](sm1cval2::R) reader structure"]
impl crate::Readable for SM1CVAL2 {}
#[doc = "Capture Value 2 Register"]
pub mod sm1cval2;
#[doc = "Capture Value 2 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1cval2cyc](sm1cval2cyc) module"]
pub type SM1CVAL2CYC = crate::Reg<u16, _SM1CVAL2CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1CVAL2CYC;
#[doc = "`read()` method returns [sm1cval2cyc::R](sm1cval2cyc::R) reader structure"]
impl crate::Readable for SM1CVAL2CYC {}
#[doc = "Capture Value 2 Cycle Register"]
pub mod sm1cval2cyc;
#[doc = "Capture Value 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1cval3](sm1cval3) module"]
pub type SM1CVAL3 = crate::Reg<u16, _SM1CVAL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1CVAL3;
#[doc = "`read()` method returns [sm1cval3::R](sm1cval3::R) reader structure"]
impl crate::Readable for SM1CVAL3 {}
#[doc = "Capture Value 3 Register"]
pub mod sm1cval3;
#[doc = "Capture Value 3 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1cval3cyc](sm1cval3cyc) module"]
pub type SM1CVAL3CYC = crate::Reg<u16, _SM1CVAL3CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1CVAL3CYC;
#[doc = "`read()` method returns [sm1cval3cyc::R](sm1cval3cyc::R) reader structure"]
impl crate::Readable for SM1CVAL3CYC {}
#[doc = "Capture Value 3 Cycle Register"]
pub mod sm1cval3cyc;
#[doc = "Capture Value 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1cval4](sm1cval4) module"]
pub type SM1CVAL4 = crate::Reg<u16, _SM1CVAL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1CVAL4;
#[doc = "`read()` method returns [sm1cval4::R](sm1cval4::R) reader structure"]
impl crate::Readable for SM1CVAL4 {}
#[doc = "Capture Value 4 Register"]
pub mod sm1cval4;
#[doc = "Capture Value 4 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1cval4cyc](sm1cval4cyc) module"]
pub type SM1CVAL4CYC = crate::Reg<u16, _SM1CVAL4CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1CVAL4CYC;
#[doc = "`read()` method returns [sm1cval4cyc::R](sm1cval4cyc::R) reader structure"]
impl crate::Readable for SM1CVAL4CYC {}
#[doc = "Capture Value 4 Cycle Register"]
pub mod sm1cval4cyc;
#[doc = "Capture Value 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1cval5](sm1cval5) module"]
pub type SM1CVAL5 = crate::Reg<u16, _SM1CVAL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1CVAL5;
#[doc = "`read()` method returns [sm1cval5::R](sm1cval5::R) reader structure"]
impl crate::Readable for SM1CVAL5 {}
#[doc = "Capture Value 5 Register"]
pub mod sm1cval5;
#[doc = "Capture Value 5 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm1cval5cyc](sm1cval5cyc) module"]
pub type SM1CVAL5CYC = crate::Reg<u16, _SM1CVAL5CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM1CVAL5CYC;
#[doc = "`read()` method returns [sm1cval5cyc::R](sm1cval5cyc::R) reader structure"]
impl crate::Readable for SM1CVAL5CYC {}
#[doc = "Capture Value 5 Cycle Register"]
pub mod sm1cval5cyc;
#[doc = "Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2cnt](sm2cnt) module"]
pub type SM2CNT = crate::Reg<u16, _SM2CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2CNT;
#[doc = "`read()` method returns [sm2cnt::R](sm2cnt::R) reader structure"]
impl crate::Readable for SM2CNT {}
#[doc = "Counter Register"]
pub mod sm2cnt;
#[doc = "Initial Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2init](sm2init) module"]
pub type SM2INIT = crate::Reg<u16, _SM2INIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2INIT;
#[doc = "`read()` method returns [sm2init::R](sm2init::R) reader structure"]
impl crate::Readable for SM2INIT {}
#[doc = "`write(|w| ..)` method takes [sm2init::W](sm2init::W) writer structure"]
impl crate::Writable for SM2INIT {}
#[doc = "Initial Count Register"]
pub mod sm2init;
#[doc = "Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2ctrl2](sm2ctrl2) module"]
pub type SM2CTRL2 = crate::Reg<u16, _SM2CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2CTRL2;
#[doc = "`read()` method returns [sm2ctrl2::R](sm2ctrl2::R) reader structure"]
impl crate::Readable for SM2CTRL2 {}
#[doc = "`write(|w| ..)` method takes [sm2ctrl2::W](sm2ctrl2::W) writer structure"]
impl crate::Writable for SM2CTRL2 {}
#[doc = "Control 2 Register"]
pub mod sm2ctrl2;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2ctrl](sm2ctrl) module"]
pub type SM2CTRL = crate::Reg<u16, _SM2CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2CTRL;
#[doc = "`read()` method returns [sm2ctrl::R](sm2ctrl::R) reader structure"]
impl crate::Readable for SM2CTRL {}
#[doc = "`write(|w| ..)` method takes [sm2ctrl::W](sm2ctrl::W) writer structure"]
impl crate::Writable for SM2CTRL {}
#[doc = "Control Register"]
pub mod sm2ctrl;
#[doc = "Value Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2val0](sm2val0) module"]
pub type SM2VAL0 = crate::Reg<u16, _SM2VAL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2VAL0;
#[doc = "`read()` method returns [sm2val0::R](sm2val0::R) reader structure"]
impl crate::Readable for SM2VAL0 {}
#[doc = "`write(|w| ..)` method takes [sm2val0::W](sm2val0::W) writer structure"]
impl crate::Writable for SM2VAL0 {}
#[doc = "Value Register 0"]
pub mod sm2val0;
#[doc = "Fractional Value Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2fracval1](sm2fracval1) module"]
pub type SM2FRACVAL1 = crate::Reg<u16, _SM2FRACVAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2FRACVAL1;
#[doc = "`read()` method returns [sm2fracval1::R](sm2fracval1::R) reader structure"]
impl crate::Readable for SM2FRACVAL1 {}
#[doc = "`write(|w| ..)` method takes [sm2fracval1::W](sm2fracval1::W) writer structure"]
impl crate::Writable for SM2FRACVAL1 {}
#[doc = "Fractional Value Register 1"]
pub mod sm2fracval1;
#[doc = "Value Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2val1](sm2val1) module"]
pub type SM2VAL1 = crate::Reg<u16, _SM2VAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2VAL1;
#[doc = "`read()` method returns [sm2val1::R](sm2val1::R) reader structure"]
impl crate::Readable for SM2VAL1 {}
#[doc = "`write(|w| ..)` method takes [sm2val1::W](sm2val1::W) writer structure"]
impl crate::Writable for SM2VAL1 {}
#[doc = "Value Register 1"]
pub mod sm2val1;
#[doc = "Fractional Value Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2fracval2](sm2fracval2) module"]
pub type SM2FRACVAL2 = crate::Reg<u16, _SM2FRACVAL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2FRACVAL2;
#[doc = "`read()` method returns [sm2fracval2::R](sm2fracval2::R) reader structure"]
impl crate::Readable for SM2FRACVAL2 {}
#[doc = "`write(|w| ..)` method takes [sm2fracval2::W](sm2fracval2::W) writer structure"]
impl crate::Writable for SM2FRACVAL2 {}
#[doc = "Fractional Value Register 2"]
pub mod sm2fracval2;
#[doc = "Value Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2val2](sm2val2) module"]
pub type SM2VAL2 = crate::Reg<u16, _SM2VAL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2VAL2;
#[doc = "`read()` method returns [sm2val2::R](sm2val2::R) reader structure"]
impl crate::Readable for SM2VAL2 {}
#[doc = "`write(|w| ..)` method takes [sm2val2::W](sm2val2::W) writer structure"]
impl crate::Writable for SM2VAL2 {}
#[doc = "Value Register 2"]
pub mod sm2val2;
#[doc = "Fractional Value Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2fracval3](sm2fracval3) module"]
pub type SM2FRACVAL3 = crate::Reg<u16, _SM2FRACVAL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2FRACVAL3;
#[doc = "`read()` method returns [sm2fracval3::R](sm2fracval3::R) reader structure"]
impl crate::Readable for SM2FRACVAL3 {}
#[doc = "`write(|w| ..)` method takes [sm2fracval3::W](sm2fracval3::W) writer structure"]
impl crate::Writable for SM2FRACVAL3 {}
#[doc = "Fractional Value Register 3"]
pub mod sm2fracval3;
#[doc = "Value Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2val3](sm2val3) module"]
pub type SM2VAL3 = crate::Reg<u16, _SM2VAL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2VAL3;
#[doc = "`read()` method returns [sm2val3::R](sm2val3::R) reader structure"]
impl crate::Readable for SM2VAL3 {}
#[doc = "`write(|w| ..)` method takes [sm2val3::W](sm2val3::W) writer structure"]
impl crate::Writable for SM2VAL3 {}
#[doc = "Value Register 3"]
pub mod sm2val3;
#[doc = "Fractional Value Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2fracval4](sm2fracval4) module"]
pub type SM2FRACVAL4 = crate::Reg<u16, _SM2FRACVAL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2FRACVAL4;
#[doc = "`read()` method returns [sm2fracval4::R](sm2fracval4::R) reader structure"]
impl crate::Readable for SM2FRACVAL4 {}
#[doc = "`write(|w| ..)` method takes [sm2fracval4::W](sm2fracval4::W) writer structure"]
impl crate::Writable for SM2FRACVAL4 {}
#[doc = "Fractional Value Register 4"]
pub mod sm2fracval4;
#[doc = "Value Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2val4](sm2val4) module"]
pub type SM2VAL4 = crate::Reg<u16, _SM2VAL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2VAL4;
#[doc = "`read()` method returns [sm2val4::R](sm2val4::R) reader structure"]
impl crate::Readable for SM2VAL4 {}
#[doc = "`write(|w| ..)` method takes [sm2val4::W](sm2val4::W) writer structure"]
impl crate::Writable for SM2VAL4 {}
#[doc = "Value Register 4"]
pub mod sm2val4;
#[doc = "Fractional Value Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2fracval5](sm2fracval5) module"]
pub type SM2FRACVAL5 = crate::Reg<u16, _SM2FRACVAL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2FRACVAL5;
#[doc = "`read()` method returns [sm2fracval5::R](sm2fracval5::R) reader structure"]
impl crate::Readable for SM2FRACVAL5 {}
#[doc = "`write(|w| ..)` method takes [sm2fracval5::W](sm2fracval5::W) writer structure"]
impl crate::Writable for SM2FRACVAL5 {}
#[doc = "Fractional Value Register 5"]
pub mod sm2fracval5;
#[doc = "Value Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2val5](sm2val5) module"]
pub type SM2VAL5 = crate::Reg<u16, _SM2VAL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2VAL5;
#[doc = "`read()` method returns [sm2val5::R](sm2val5::R) reader structure"]
impl crate::Readable for SM2VAL5 {}
#[doc = "`write(|w| ..)` method takes [sm2val5::W](sm2val5::W) writer structure"]
impl crate::Writable for SM2VAL5 {}
#[doc = "Value Register 5"]
pub mod sm2val5;
#[doc = "Fractional Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2frctrl](sm2frctrl) module"]
pub type SM2FRCTRL = crate::Reg<u16, _SM2FRCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2FRCTRL;
#[doc = "`read()` method returns [sm2frctrl::R](sm2frctrl::R) reader structure"]
impl crate::Readable for SM2FRCTRL {}
#[doc = "`write(|w| ..)` method takes [sm2frctrl::W](sm2frctrl::W) writer structure"]
impl crate::Writable for SM2FRCTRL {}
#[doc = "Fractional Control Register"]
pub mod sm2frctrl;
#[doc = "Output Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2octrl](sm2octrl) module"]
pub type SM2OCTRL = crate::Reg<u16, _SM2OCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2OCTRL;
#[doc = "`read()` method returns [sm2octrl::R](sm2octrl::R) reader structure"]
impl crate::Readable for SM2OCTRL {}
#[doc = "`write(|w| ..)` method takes [sm2octrl::W](sm2octrl::W) writer structure"]
impl crate::Writable for SM2OCTRL {}
#[doc = "Output Control Register"]
pub mod sm2octrl;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2sts](sm2sts) module"]
pub type SM2STS = crate::Reg<u16, _SM2STS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2STS;
#[doc = "`read()` method returns [sm2sts::R](sm2sts::R) reader structure"]
impl crate::Readable for SM2STS {}
#[doc = "`write(|w| ..)` method takes [sm2sts::W](sm2sts::W) writer structure"]
impl crate::Writable for SM2STS {}
#[doc = "Status Register"]
pub mod sm2sts;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2inten](sm2inten) module"]
pub type SM2INTEN = crate::Reg<u16, _SM2INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2INTEN;
#[doc = "`read()` method returns [sm2inten::R](sm2inten::R) reader structure"]
impl crate::Readable for SM2INTEN {}
#[doc = "`write(|w| ..)` method takes [sm2inten::W](sm2inten::W) writer structure"]
impl crate::Writable for SM2INTEN {}
#[doc = "Interrupt Enable Register"]
pub mod sm2inten;
#[doc = "DMA Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2dmaen](sm2dmaen) module"]
pub type SM2DMAEN = crate::Reg<u16, _SM2DMAEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2DMAEN;
#[doc = "`read()` method returns [sm2dmaen::R](sm2dmaen::R) reader structure"]
impl crate::Readable for SM2DMAEN {}
#[doc = "`write(|w| ..)` method takes [sm2dmaen::W](sm2dmaen::W) writer structure"]
impl crate::Writable for SM2DMAEN {}
#[doc = "DMA Enable Register"]
pub mod sm2dmaen;
#[doc = "Output Trigger Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2tctrl](sm2tctrl) module"]
pub type SM2TCTRL = crate::Reg<u16, _SM2TCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2TCTRL;
#[doc = "`read()` method returns [sm2tctrl::R](sm2tctrl::R) reader structure"]
impl crate::Readable for SM2TCTRL {}
#[doc = "`write(|w| ..)` method takes [sm2tctrl::W](sm2tctrl::W) writer structure"]
impl crate::Writable for SM2TCTRL {}
#[doc = "Output Trigger Control Register"]
pub mod sm2tctrl;
#[doc = "Fault Disable Mapping Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2dismap0](sm2dismap0) module"]
pub type SM2DISMAP0 = crate::Reg<u16, _SM2DISMAP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2DISMAP0;
#[doc = "`read()` method returns [sm2dismap0::R](sm2dismap0::R) reader structure"]
impl crate::Readable for SM2DISMAP0 {}
#[doc = "`write(|w| ..)` method takes [sm2dismap0::W](sm2dismap0::W) writer structure"]
impl crate::Writable for SM2DISMAP0 {}
#[doc = "Fault Disable Mapping Register 0"]
pub mod sm2dismap0;
#[doc = "Fault Disable Mapping Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2dismap1](sm2dismap1) module"]
pub type SM2DISMAP1 = crate::Reg<u16, _SM2DISMAP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2DISMAP1;
#[doc = "`read()` method returns [sm2dismap1::R](sm2dismap1::R) reader structure"]
impl crate::Readable for SM2DISMAP1 {}
#[doc = "`write(|w| ..)` method takes [sm2dismap1::W](sm2dismap1::W) writer structure"]
impl crate::Writable for SM2DISMAP1 {}
#[doc = "Fault Disable Mapping Register 1"]
pub mod sm2dismap1;
#[doc = "Deadtime Count Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2dtcnt0](sm2dtcnt0) module"]
pub type SM2DTCNT0 = crate::Reg<u16, _SM2DTCNT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2DTCNT0;
#[doc = "`read()` method returns [sm2dtcnt0::R](sm2dtcnt0::R) reader structure"]
impl crate::Readable for SM2DTCNT0 {}
#[doc = "`write(|w| ..)` method takes [sm2dtcnt0::W](sm2dtcnt0::W) writer structure"]
impl crate::Writable for SM2DTCNT0 {}
#[doc = "Deadtime Count Register 0"]
pub mod sm2dtcnt0;
#[doc = "Deadtime Count Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2dtcnt1](sm2dtcnt1) module"]
pub type SM2DTCNT1 = crate::Reg<u16, _SM2DTCNT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2DTCNT1;
#[doc = "`read()` method returns [sm2dtcnt1::R](sm2dtcnt1::R) reader structure"]
impl crate::Readable for SM2DTCNT1 {}
#[doc = "`write(|w| ..)` method takes [sm2dtcnt1::W](sm2dtcnt1::W) writer structure"]
impl crate::Writable for SM2DTCNT1 {}
#[doc = "Deadtime Count Register 1"]
pub mod sm2dtcnt1;
#[doc = "Capture Control A Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2captctrla](sm2captctrla) module"]
pub type SM2CAPTCTRLA = crate::Reg<u16, _SM2CAPTCTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2CAPTCTRLA;
#[doc = "`read()` method returns [sm2captctrla::R](sm2captctrla::R) reader structure"]
impl crate::Readable for SM2CAPTCTRLA {}
#[doc = "`write(|w| ..)` method takes [sm2captctrla::W](sm2captctrla::W) writer structure"]
impl crate::Writable for SM2CAPTCTRLA {}
#[doc = "Capture Control A Register"]
pub mod sm2captctrla;
#[doc = "Capture Compare A Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2captcompa](sm2captcompa) module"]
pub type SM2CAPTCOMPA = crate::Reg<u16, _SM2CAPTCOMPA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2CAPTCOMPA;
#[doc = "`read()` method returns [sm2captcompa::R](sm2captcompa::R) reader structure"]
impl crate::Readable for SM2CAPTCOMPA {}
#[doc = "`write(|w| ..)` method takes [sm2captcompa::W](sm2captcompa::W) writer structure"]
impl crate::Writable for SM2CAPTCOMPA {}
#[doc = "Capture Compare A Register"]
pub mod sm2captcompa;
#[doc = "Capture Control B Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2captctrlb](sm2captctrlb) module"]
pub type SM2CAPTCTRLB = crate::Reg<u16, _SM2CAPTCTRLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2CAPTCTRLB;
#[doc = "`read()` method returns [sm2captctrlb::R](sm2captctrlb::R) reader structure"]
impl crate::Readable for SM2CAPTCTRLB {}
#[doc = "`write(|w| ..)` method takes [sm2captctrlb::W](sm2captctrlb::W) writer structure"]
impl crate::Writable for SM2CAPTCTRLB {}
#[doc = "Capture Control B Register"]
pub mod sm2captctrlb;
#[doc = "Capture Compare B Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2captcompb](sm2captcompb) module"]
pub type SM2CAPTCOMPB = crate::Reg<u16, _SM2CAPTCOMPB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2CAPTCOMPB;
#[doc = "`read()` method returns [sm2captcompb::R](sm2captcompb::R) reader structure"]
impl crate::Readable for SM2CAPTCOMPB {}
#[doc = "`write(|w| ..)` method takes [sm2captcompb::W](sm2captcompb::W) writer structure"]
impl crate::Writable for SM2CAPTCOMPB {}
#[doc = "Capture Compare B Register"]
pub mod sm2captcompb;
#[doc = "Capture Control X Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2captctrlx](sm2captctrlx) module"]
pub type SM2CAPTCTRLX = crate::Reg<u16, _SM2CAPTCTRLX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2CAPTCTRLX;
#[doc = "`read()` method returns [sm2captctrlx::R](sm2captctrlx::R) reader structure"]
impl crate::Readable for SM2CAPTCTRLX {}
#[doc = "`write(|w| ..)` method takes [sm2captctrlx::W](sm2captctrlx::W) writer structure"]
impl crate::Writable for SM2CAPTCTRLX {}
#[doc = "Capture Control X Register"]
pub mod sm2captctrlx;
#[doc = "Capture Compare X Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2captcompx](sm2captcompx) module"]
pub type SM2CAPTCOMPX = crate::Reg<u16, _SM2CAPTCOMPX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2CAPTCOMPX;
#[doc = "`read()` method returns [sm2captcompx::R](sm2captcompx::R) reader structure"]
impl crate::Readable for SM2CAPTCOMPX {}
#[doc = "`write(|w| ..)` method takes [sm2captcompx::W](sm2captcompx::W) writer structure"]
impl crate::Writable for SM2CAPTCOMPX {}
#[doc = "Capture Compare X Register"]
pub mod sm2captcompx;
#[doc = "Capture Value 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2cval0](sm2cval0) module"]
pub type SM2CVAL0 = crate::Reg<u16, _SM2CVAL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2CVAL0;
#[doc = "`read()` method returns [sm2cval0::R](sm2cval0::R) reader structure"]
impl crate::Readable for SM2CVAL0 {}
#[doc = "Capture Value 0 Register"]
pub mod sm2cval0;
#[doc = "Capture Value 0 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2cval0cyc](sm2cval0cyc) module"]
pub type SM2CVAL0CYC = crate::Reg<u16, _SM2CVAL0CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2CVAL0CYC;
#[doc = "`read()` method returns [sm2cval0cyc::R](sm2cval0cyc::R) reader structure"]
impl crate::Readable for SM2CVAL0CYC {}
#[doc = "Capture Value 0 Cycle Register"]
pub mod sm2cval0cyc;
#[doc = "Capture Value 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2cval1](sm2cval1) module"]
pub type SM2CVAL1 = crate::Reg<u16, _SM2CVAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2CVAL1;
#[doc = "`read()` method returns [sm2cval1::R](sm2cval1::R) reader structure"]
impl crate::Readable for SM2CVAL1 {}
#[doc = "Capture Value 1 Register"]
pub mod sm2cval1;
#[doc = "Capture Value 1 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2cval1cyc](sm2cval1cyc) module"]
pub type SM2CVAL1CYC = crate::Reg<u16, _SM2CVAL1CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2CVAL1CYC;
#[doc = "`read()` method returns [sm2cval1cyc::R](sm2cval1cyc::R) reader structure"]
impl crate::Readable for SM2CVAL1CYC {}
#[doc = "Capture Value 1 Cycle Register"]
pub mod sm2cval1cyc;
#[doc = "Capture Value 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2cval2](sm2cval2) module"]
pub type SM2CVAL2 = crate::Reg<u16, _SM2CVAL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2CVAL2;
#[doc = "`read()` method returns [sm2cval2::R](sm2cval2::R) reader structure"]
impl crate::Readable for SM2CVAL2 {}
#[doc = "Capture Value 2 Register"]
pub mod sm2cval2;
#[doc = "Capture Value 2 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2cval2cyc](sm2cval2cyc) module"]
pub type SM2CVAL2CYC = crate::Reg<u16, _SM2CVAL2CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2CVAL2CYC;
#[doc = "`read()` method returns [sm2cval2cyc::R](sm2cval2cyc::R) reader structure"]
impl crate::Readable for SM2CVAL2CYC {}
#[doc = "Capture Value 2 Cycle Register"]
pub mod sm2cval2cyc;
#[doc = "Capture Value 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2cval3](sm2cval3) module"]
pub type SM2CVAL3 = crate::Reg<u16, _SM2CVAL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2CVAL3;
#[doc = "`read()` method returns [sm2cval3::R](sm2cval3::R) reader structure"]
impl crate::Readable for SM2CVAL3 {}
#[doc = "Capture Value 3 Register"]
pub mod sm2cval3;
#[doc = "Capture Value 3 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2cval3cyc](sm2cval3cyc) module"]
pub type SM2CVAL3CYC = crate::Reg<u16, _SM2CVAL3CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2CVAL3CYC;
#[doc = "`read()` method returns [sm2cval3cyc::R](sm2cval3cyc::R) reader structure"]
impl crate::Readable for SM2CVAL3CYC {}
#[doc = "Capture Value 3 Cycle Register"]
pub mod sm2cval3cyc;
#[doc = "Capture Value 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2cval4](sm2cval4) module"]
pub type SM2CVAL4 = crate::Reg<u16, _SM2CVAL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2CVAL4;
#[doc = "`read()` method returns [sm2cval4::R](sm2cval4::R) reader structure"]
impl crate::Readable for SM2CVAL4 {}
#[doc = "Capture Value 4 Register"]
pub mod sm2cval4;
#[doc = "Capture Value 4 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2cval4cyc](sm2cval4cyc) module"]
pub type SM2CVAL4CYC = crate::Reg<u16, _SM2CVAL4CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2CVAL4CYC;
#[doc = "`read()` method returns [sm2cval4cyc::R](sm2cval4cyc::R) reader structure"]
impl crate::Readable for SM2CVAL4CYC {}
#[doc = "Capture Value 4 Cycle Register"]
pub mod sm2cval4cyc;
#[doc = "Capture Value 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2cval5](sm2cval5) module"]
pub type SM2CVAL5 = crate::Reg<u16, _SM2CVAL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2CVAL5;
#[doc = "`read()` method returns [sm2cval5::R](sm2cval5::R) reader structure"]
impl crate::Readable for SM2CVAL5 {}
#[doc = "Capture Value 5 Register"]
pub mod sm2cval5;
#[doc = "Capture Value 5 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm2cval5cyc](sm2cval5cyc) module"]
pub type SM2CVAL5CYC = crate::Reg<u16, _SM2CVAL5CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM2CVAL5CYC;
#[doc = "`read()` method returns [sm2cval5cyc::R](sm2cval5cyc::R) reader structure"]
impl crate::Readable for SM2CVAL5CYC {}
#[doc = "Capture Value 5 Cycle Register"]
pub mod sm2cval5cyc;
#[doc = "Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3cnt](sm3cnt) module"]
pub type SM3CNT = crate::Reg<u16, _SM3CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3CNT;
#[doc = "`read()` method returns [sm3cnt::R](sm3cnt::R) reader structure"]
impl crate::Readable for SM3CNT {}
#[doc = "Counter Register"]
pub mod sm3cnt;
#[doc = "Initial Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3init](sm3init) module"]
pub type SM3INIT = crate::Reg<u16, _SM3INIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3INIT;
#[doc = "`read()` method returns [sm3init::R](sm3init::R) reader structure"]
impl crate::Readable for SM3INIT {}
#[doc = "`write(|w| ..)` method takes [sm3init::W](sm3init::W) writer structure"]
impl crate::Writable for SM3INIT {}
#[doc = "Initial Count Register"]
pub mod sm3init;
#[doc = "Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3ctrl2](sm3ctrl2) module"]
pub type SM3CTRL2 = crate::Reg<u16, _SM3CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3CTRL2;
#[doc = "`read()` method returns [sm3ctrl2::R](sm3ctrl2::R) reader structure"]
impl crate::Readable for SM3CTRL2 {}
#[doc = "`write(|w| ..)` method takes [sm3ctrl2::W](sm3ctrl2::W) writer structure"]
impl crate::Writable for SM3CTRL2 {}
#[doc = "Control 2 Register"]
pub mod sm3ctrl2;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3ctrl](sm3ctrl) module"]
pub type SM3CTRL = crate::Reg<u16, _SM3CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3CTRL;
#[doc = "`read()` method returns [sm3ctrl::R](sm3ctrl::R) reader structure"]
impl crate::Readable for SM3CTRL {}
#[doc = "`write(|w| ..)` method takes [sm3ctrl::W](sm3ctrl::W) writer structure"]
impl crate::Writable for SM3CTRL {}
#[doc = "Control Register"]
pub mod sm3ctrl;
#[doc = "Value Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3val0](sm3val0) module"]
pub type SM3VAL0 = crate::Reg<u16, _SM3VAL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3VAL0;
#[doc = "`read()` method returns [sm3val0::R](sm3val0::R) reader structure"]
impl crate::Readable for SM3VAL0 {}
#[doc = "`write(|w| ..)` method takes [sm3val0::W](sm3val0::W) writer structure"]
impl crate::Writable for SM3VAL0 {}
#[doc = "Value Register 0"]
pub mod sm3val0;
#[doc = "Fractional Value Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3fracval1](sm3fracval1) module"]
pub type SM3FRACVAL1 = crate::Reg<u16, _SM3FRACVAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3FRACVAL1;
#[doc = "`read()` method returns [sm3fracval1::R](sm3fracval1::R) reader structure"]
impl crate::Readable for SM3FRACVAL1 {}
#[doc = "`write(|w| ..)` method takes [sm3fracval1::W](sm3fracval1::W) writer structure"]
impl crate::Writable for SM3FRACVAL1 {}
#[doc = "Fractional Value Register 1"]
pub mod sm3fracval1;
#[doc = "Value Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3val1](sm3val1) module"]
pub type SM3VAL1 = crate::Reg<u16, _SM3VAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3VAL1;
#[doc = "`read()` method returns [sm3val1::R](sm3val1::R) reader structure"]
impl crate::Readable for SM3VAL1 {}
#[doc = "`write(|w| ..)` method takes [sm3val1::W](sm3val1::W) writer structure"]
impl crate::Writable for SM3VAL1 {}
#[doc = "Value Register 1"]
pub mod sm3val1;
#[doc = "Fractional Value Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3fracval2](sm3fracval2) module"]
pub type SM3FRACVAL2 = crate::Reg<u16, _SM3FRACVAL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3FRACVAL2;
#[doc = "`read()` method returns [sm3fracval2::R](sm3fracval2::R) reader structure"]
impl crate::Readable for SM3FRACVAL2 {}
#[doc = "`write(|w| ..)` method takes [sm3fracval2::W](sm3fracval2::W) writer structure"]
impl crate::Writable for SM3FRACVAL2 {}
#[doc = "Fractional Value Register 2"]
pub mod sm3fracval2;
#[doc = "Value Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3val2](sm3val2) module"]
pub type SM3VAL2 = crate::Reg<u16, _SM3VAL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3VAL2;
#[doc = "`read()` method returns [sm3val2::R](sm3val2::R) reader structure"]
impl crate::Readable for SM3VAL2 {}
#[doc = "`write(|w| ..)` method takes [sm3val2::W](sm3val2::W) writer structure"]
impl crate::Writable for SM3VAL2 {}
#[doc = "Value Register 2"]
pub mod sm3val2;
#[doc = "Fractional Value Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3fracval3](sm3fracval3) module"]
pub type SM3FRACVAL3 = crate::Reg<u16, _SM3FRACVAL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3FRACVAL3;
#[doc = "`read()` method returns [sm3fracval3::R](sm3fracval3::R) reader structure"]
impl crate::Readable for SM3FRACVAL3 {}
#[doc = "`write(|w| ..)` method takes [sm3fracval3::W](sm3fracval3::W) writer structure"]
impl crate::Writable for SM3FRACVAL3 {}
#[doc = "Fractional Value Register 3"]
pub mod sm3fracval3;
#[doc = "Value Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3val3](sm3val3) module"]
pub type SM3VAL3 = crate::Reg<u16, _SM3VAL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3VAL3;
#[doc = "`read()` method returns [sm3val3::R](sm3val3::R) reader structure"]
impl crate::Readable for SM3VAL3 {}
#[doc = "`write(|w| ..)` method takes [sm3val3::W](sm3val3::W) writer structure"]
impl crate::Writable for SM3VAL3 {}
#[doc = "Value Register 3"]
pub mod sm3val3;
#[doc = "Fractional Value Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3fracval4](sm3fracval4) module"]
pub type SM3FRACVAL4 = crate::Reg<u16, _SM3FRACVAL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3FRACVAL4;
#[doc = "`read()` method returns [sm3fracval4::R](sm3fracval4::R) reader structure"]
impl crate::Readable for SM3FRACVAL4 {}
#[doc = "`write(|w| ..)` method takes [sm3fracval4::W](sm3fracval4::W) writer structure"]
impl crate::Writable for SM3FRACVAL4 {}
#[doc = "Fractional Value Register 4"]
pub mod sm3fracval4;
#[doc = "Value Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3val4](sm3val4) module"]
pub type SM3VAL4 = crate::Reg<u16, _SM3VAL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3VAL4;
#[doc = "`read()` method returns [sm3val4::R](sm3val4::R) reader structure"]
impl crate::Readable for SM3VAL4 {}
#[doc = "`write(|w| ..)` method takes [sm3val4::W](sm3val4::W) writer structure"]
impl crate::Writable for SM3VAL4 {}
#[doc = "Value Register 4"]
pub mod sm3val4;
#[doc = "Fractional Value Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3fracval5](sm3fracval5) module"]
pub type SM3FRACVAL5 = crate::Reg<u16, _SM3FRACVAL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3FRACVAL5;
#[doc = "`read()` method returns [sm3fracval5::R](sm3fracval5::R) reader structure"]
impl crate::Readable for SM3FRACVAL5 {}
#[doc = "`write(|w| ..)` method takes [sm3fracval5::W](sm3fracval5::W) writer structure"]
impl crate::Writable for SM3FRACVAL5 {}
#[doc = "Fractional Value Register 5"]
pub mod sm3fracval5;
#[doc = "Value Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3val5](sm3val5) module"]
pub type SM3VAL5 = crate::Reg<u16, _SM3VAL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3VAL5;
#[doc = "`read()` method returns [sm3val5::R](sm3val5::R) reader structure"]
impl crate::Readable for SM3VAL5 {}
#[doc = "`write(|w| ..)` method takes [sm3val5::W](sm3val5::W) writer structure"]
impl crate::Writable for SM3VAL5 {}
#[doc = "Value Register 5"]
pub mod sm3val5;
#[doc = "Fractional Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3frctrl](sm3frctrl) module"]
pub type SM3FRCTRL = crate::Reg<u16, _SM3FRCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3FRCTRL;
#[doc = "`read()` method returns [sm3frctrl::R](sm3frctrl::R) reader structure"]
impl crate::Readable for SM3FRCTRL {}
#[doc = "`write(|w| ..)` method takes [sm3frctrl::W](sm3frctrl::W) writer structure"]
impl crate::Writable for SM3FRCTRL {}
#[doc = "Fractional Control Register"]
pub mod sm3frctrl;
#[doc = "Output Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3octrl](sm3octrl) module"]
pub type SM3OCTRL = crate::Reg<u16, _SM3OCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3OCTRL;
#[doc = "`read()` method returns [sm3octrl::R](sm3octrl::R) reader structure"]
impl crate::Readable for SM3OCTRL {}
#[doc = "`write(|w| ..)` method takes [sm3octrl::W](sm3octrl::W) writer structure"]
impl crate::Writable for SM3OCTRL {}
#[doc = "Output Control Register"]
pub mod sm3octrl;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3sts](sm3sts) module"]
pub type SM3STS = crate::Reg<u16, _SM3STS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3STS;
#[doc = "`read()` method returns [sm3sts::R](sm3sts::R) reader structure"]
impl crate::Readable for SM3STS {}
#[doc = "`write(|w| ..)` method takes [sm3sts::W](sm3sts::W) writer structure"]
impl crate::Writable for SM3STS {}
#[doc = "Status Register"]
pub mod sm3sts;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3inten](sm3inten) module"]
pub type SM3INTEN = crate::Reg<u16, _SM3INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3INTEN;
#[doc = "`read()` method returns [sm3inten::R](sm3inten::R) reader structure"]
impl crate::Readable for SM3INTEN {}
#[doc = "`write(|w| ..)` method takes [sm3inten::W](sm3inten::W) writer structure"]
impl crate::Writable for SM3INTEN {}
#[doc = "Interrupt Enable Register"]
pub mod sm3inten;
#[doc = "DMA Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3dmaen](sm3dmaen) module"]
pub type SM3DMAEN = crate::Reg<u16, _SM3DMAEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3DMAEN;
#[doc = "`read()` method returns [sm3dmaen::R](sm3dmaen::R) reader structure"]
impl crate::Readable for SM3DMAEN {}
#[doc = "`write(|w| ..)` method takes [sm3dmaen::W](sm3dmaen::W) writer structure"]
impl crate::Writable for SM3DMAEN {}
#[doc = "DMA Enable Register"]
pub mod sm3dmaen;
#[doc = "Output Trigger Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3tctrl](sm3tctrl) module"]
pub type SM3TCTRL = crate::Reg<u16, _SM3TCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3TCTRL;
#[doc = "`read()` method returns [sm3tctrl::R](sm3tctrl::R) reader structure"]
impl crate::Readable for SM3TCTRL {}
#[doc = "`write(|w| ..)` method takes [sm3tctrl::W](sm3tctrl::W) writer structure"]
impl crate::Writable for SM3TCTRL {}
#[doc = "Output Trigger Control Register"]
pub mod sm3tctrl;
#[doc = "Fault Disable Mapping Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3dismap0](sm3dismap0) module"]
pub type SM3DISMAP0 = crate::Reg<u16, _SM3DISMAP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3DISMAP0;
#[doc = "`read()` method returns [sm3dismap0::R](sm3dismap0::R) reader structure"]
impl crate::Readable for SM3DISMAP0 {}
#[doc = "`write(|w| ..)` method takes [sm3dismap0::W](sm3dismap0::W) writer structure"]
impl crate::Writable for SM3DISMAP0 {}
#[doc = "Fault Disable Mapping Register 0"]
pub mod sm3dismap0;
#[doc = "Fault Disable Mapping Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3dismap1](sm3dismap1) module"]
pub type SM3DISMAP1 = crate::Reg<u16, _SM3DISMAP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3DISMAP1;
#[doc = "`read()` method returns [sm3dismap1::R](sm3dismap1::R) reader structure"]
impl crate::Readable for SM3DISMAP1 {}
#[doc = "`write(|w| ..)` method takes [sm3dismap1::W](sm3dismap1::W) writer structure"]
impl crate::Writable for SM3DISMAP1 {}
#[doc = "Fault Disable Mapping Register 1"]
pub mod sm3dismap1;
#[doc = "Deadtime Count Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3dtcnt0](sm3dtcnt0) module"]
pub type SM3DTCNT0 = crate::Reg<u16, _SM3DTCNT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3DTCNT0;
#[doc = "`read()` method returns [sm3dtcnt0::R](sm3dtcnt0::R) reader structure"]
impl crate::Readable for SM3DTCNT0 {}
#[doc = "`write(|w| ..)` method takes [sm3dtcnt0::W](sm3dtcnt0::W) writer structure"]
impl crate::Writable for SM3DTCNT0 {}
#[doc = "Deadtime Count Register 0"]
pub mod sm3dtcnt0;
#[doc = "Deadtime Count Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3dtcnt1](sm3dtcnt1) module"]
pub type SM3DTCNT1 = crate::Reg<u16, _SM3DTCNT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3DTCNT1;
#[doc = "`read()` method returns [sm3dtcnt1::R](sm3dtcnt1::R) reader structure"]
impl crate::Readable for SM3DTCNT1 {}
#[doc = "`write(|w| ..)` method takes [sm3dtcnt1::W](sm3dtcnt1::W) writer structure"]
impl crate::Writable for SM3DTCNT1 {}
#[doc = "Deadtime Count Register 1"]
pub mod sm3dtcnt1;
#[doc = "Capture Control A Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3captctrla](sm3captctrla) module"]
pub type SM3CAPTCTRLA = crate::Reg<u16, _SM3CAPTCTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3CAPTCTRLA;
#[doc = "`read()` method returns [sm3captctrla::R](sm3captctrla::R) reader structure"]
impl crate::Readable for SM3CAPTCTRLA {}
#[doc = "`write(|w| ..)` method takes [sm3captctrla::W](sm3captctrla::W) writer structure"]
impl crate::Writable for SM3CAPTCTRLA {}
#[doc = "Capture Control A Register"]
pub mod sm3captctrla;
#[doc = "Capture Compare A Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3captcompa](sm3captcompa) module"]
pub type SM3CAPTCOMPA = crate::Reg<u16, _SM3CAPTCOMPA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3CAPTCOMPA;
#[doc = "`read()` method returns [sm3captcompa::R](sm3captcompa::R) reader structure"]
impl crate::Readable for SM3CAPTCOMPA {}
#[doc = "`write(|w| ..)` method takes [sm3captcompa::W](sm3captcompa::W) writer structure"]
impl crate::Writable for SM3CAPTCOMPA {}
#[doc = "Capture Compare A Register"]
pub mod sm3captcompa;
#[doc = "Capture Control B Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3captctrlb](sm3captctrlb) module"]
pub type SM3CAPTCTRLB = crate::Reg<u16, _SM3CAPTCTRLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3CAPTCTRLB;
#[doc = "`read()` method returns [sm3captctrlb::R](sm3captctrlb::R) reader structure"]
impl crate::Readable for SM3CAPTCTRLB {}
#[doc = "`write(|w| ..)` method takes [sm3captctrlb::W](sm3captctrlb::W) writer structure"]
impl crate::Writable for SM3CAPTCTRLB {}
#[doc = "Capture Control B Register"]
pub mod sm3captctrlb;
#[doc = "Capture Compare B Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3captcompb](sm3captcompb) module"]
pub type SM3CAPTCOMPB = crate::Reg<u16, _SM3CAPTCOMPB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3CAPTCOMPB;
#[doc = "`read()` method returns [sm3captcompb::R](sm3captcompb::R) reader structure"]
impl crate::Readable for SM3CAPTCOMPB {}
#[doc = "`write(|w| ..)` method takes [sm3captcompb::W](sm3captcompb::W) writer structure"]
impl crate::Writable for SM3CAPTCOMPB {}
#[doc = "Capture Compare B Register"]
pub mod sm3captcompb;
#[doc = "Capture Control X Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3captctrlx](sm3captctrlx) module"]
pub type SM3CAPTCTRLX = crate::Reg<u16, _SM3CAPTCTRLX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3CAPTCTRLX;
#[doc = "`read()` method returns [sm3captctrlx::R](sm3captctrlx::R) reader structure"]
impl crate::Readable for SM3CAPTCTRLX {}
#[doc = "`write(|w| ..)` method takes [sm3captctrlx::W](sm3captctrlx::W) writer structure"]
impl crate::Writable for SM3CAPTCTRLX {}
#[doc = "Capture Control X Register"]
pub mod sm3captctrlx;
#[doc = "Capture Compare X Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3captcompx](sm3captcompx) module"]
pub type SM3CAPTCOMPX = crate::Reg<u16, _SM3CAPTCOMPX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3CAPTCOMPX;
#[doc = "`read()` method returns [sm3captcompx::R](sm3captcompx::R) reader structure"]
impl crate::Readable for SM3CAPTCOMPX {}
#[doc = "`write(|w| ..)` method takes [sm3captcompx::W](sm3captcompx::W) writer structure"]
impl crate::Writable for SM3CAPTCOMPX {}
#[doc = "Capture Compare X Register"]
pub mod sm3captcompx;
#[doc = "Capture Value 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3cval0](sm3cval0) module"]
pub type SM3CVAL0 = crate::Reg<u16, _SM3CVAL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3CVAL0;
#[doc = "`read()` method returns [sm3cval0::R](sm3cval0::R) reader structure"]
impl crate::Readable for SM3CVAL0 {}
#[doc = "Capture Value 0 Register"]
pub mod sm3cval0;
#[doc = "Capture Value 0 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3cval0cyc](sm3cval0cyc) module"]
pub type SM3CVAL0CYC = crate::Reg<u16, _SM3CVAL0CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3CVAL0CYC;
#[doc = "`read()` method returns [sm3cval0cyc::R](sm3cval0cyc::R) reader structure"]
impl crate::Readable for SM3CVAL0CYC {}
#[doc = "Capture Value 0 Cycle Register"]
pub mod sm3cval0cyc;
#[doc = "Capture Value 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3cval1](sm3cval1) module"]
pub type SM3CVAL1 = crate::Reg<u16, _SM3CVAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3CVAL1;
#[doc = "`read()` method returns [sm3cval1::R](sm3cval1::R) reader structure"]
impl crate::Readable for SM3CVAL1 {}
#[doc = "Capture Value 1 Register"]
pub mod sm3cval1;
#[doc = "Capture Value 1 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3cval1cyc](sm3cval1cyc) module"]
pub type SM3CVAL1CYC = crate::Reg<u16, _SM3CVAL1CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3CVAL1CYC;
#[doc = "`read()` method returns [sm3cval1cyc::R](sm3cval1cyc::R) reader structure"]
impl crate::Readable for SM3CVAL1CYC {}
#[doc = "Capture Value 1 Cycle Register"]
pub mod sm3cval1cyc;
#[doc = "Capture Value 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3cval2](sm3cval2) module"]
pub type SM3CVAL2 = crate::Reg<u16, _SM3CVAL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3CVAL2;
#[doc = "`read()` method returns [sm3cval2::R](sm3cval2::R) reader structure"]
impl crate::Readable for SM3CVAL2 {}
#[doc = "Capture Value 2 Register"]
pub mod sm3cval2;
#[doc = "Capture Value 2 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3cval2cyc](sm3cval2cyc) module"]
pub type SM3CVAL2CYC = crate::Reg<u16, _SM3CVAL2CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3CVAL2CYC;
#[doc = "`read()` method returns [sm3cval2cyc::R](sm3cval2cyc::R) reader structure"]
impl crate::Readable for SM3CVAL2CYC {}
#[doc = "Capture Value 2 Cycle Register"]
pub mod sm3cval2cyc;
#[doc = "Capture Value 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3cval3](sm3cval3) module"]
pub type SM3CVAL3 = crate::Reg<u16, _SM3CVAL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3CVAL3;
#[doc = "`read()` method returns [sm3cval3::R](sm3cval3::R) reader structure"]
impl crate::Readable for SM3CVAL3 {}
#[doc = "Capture Value 3 Register"]
pub mod sm3cval3;
#[doc = "Capture Value 3 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3cval3cyc](sm3cval3cyc) module"]
pub type SM3CVAL3CYC = crate::Reg<u16, _SM3CVAL3CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3CVAL3CYC;
#[doc = "`read()` method returns [sm3cval3cyc::R](sm3cval3cyc::R) reader structure"]
impl crate::Readable for SM3CVAL3CYC {}
#[doc = "Capture Value 3 Cycle Register"]
pub mod sm3cval3cyc;
#[doc = "Capture Value 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3cval4](sm3cval4) module"]
pub type SM3CVAL4 = crate::Reg<u16, _SM3CVAL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3CVAL4;
#[doc = "`read()` method returns [sm3cval4::R](sm3cval4::R) reader structure"]
impl crate::Readable for SM3CVAL4 {}
#[doc = "Capture Value 4 Register"]
pub mod sm3cval4;
#[doc = "Capture Value 4 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3cval4cyc](sm3cval4cyc) module"]
pub type SM3CVAL4CYC = crate::Reg<u16, _SM3CVAL4CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3CVAL4CYC;
#[doc = "`read()` method returns [sm3cval4cyc::R](sm3cval4cyc::R) reader structure"]
impl crate::Readable for SM3CVAL4CYC {}
#[doc = "Capture Value 4 Cycle Register"]
pub mod sm3cval4cyc;
#[doc = "Capture Value 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3cval5](sm3cval5) module"]
pub type SM3CVAL5 = crate::Reg<u16, _SM3CVAL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3CVAL5;
#[doc = "`read()` method returns [sm3cval5::R](sm3cval5::R) reader structure"]
impl crate::Readable for SM3CVAL5 {}
#[doc = "Capture Value 5 Register"]
pub mod sm3cval5;
#[doc = "Capture Value 5 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sm3cval5cyc](sm3cval5cyc) module"]
pub type SM3CVAL5CYC = crate::Reg<u16, _SM3CVAL5CYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SM3CVAL5CYC;
#[doc = "`read()` method returns [sm3cval5cyc::R](sm3cval5cyc::R) reader structure"]
impl crate::Readable for SM3CVAL5CYC {}
#[doc = "Capture Value 5 Cycle Register"]
pub mod sm3cval5cyc;
#[doc = "Output Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [outen](outen) module"]
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
#[doc = "Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mask](mask) module"]
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
#[doc = "Software Controlled Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [swcout](swcout) module"]
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
#[doc = "PWM Source Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dtsrcsel](dtsrcsel) module"]
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
#[doc = "Master Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mctrl](mctrl) module"]
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
#[doc = "Master Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mctrl2](mctrl2) module"]
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
#[doc = "Fault Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fctrl0](fctrl0) module"]
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
#[doc = "Fault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fsts0](fsts0) module"]
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
#[doc = "Fault Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ffilt0](ffilt0) module"]
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
#[doc = "Fault Test Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ftst0](ftst0) module"]
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
#[doc = "Fault Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fctrl20](fctrl20) module"]
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
