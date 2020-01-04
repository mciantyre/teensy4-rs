#[doc = "Reader of register CFGR1"]
pub type R = crate::R<u32, super::CFGR1>;
#[doc = "Writer for register CFGR1"]
pub type W = crate::W<u32, super::CFGR1>;
#[doc = "Register CFGR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Master Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASTER_A {
    #[doc = "0: Slave mode"]
    MASTER_0 = 0,
    #[doc = "1: Master mode"]
    MASTER_1 = 1,
}
impl From<MASTER_A> for bool {
    #[inline(always)]
    fn from(variant: MASTER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MASTER`"]
pub type MASTER_R = crate::R<bool, MASTER_A>;
impl MASTER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASTER_A {
        match self.bits {
            false => MASTER_A::MASTER_0,
            true => MASTER_A::MASTER_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASTER_0`"]
    #[inline(always)]
    pub fn is_master_0(&self) -> bool {
        *self == MASTER_A::MASTER_0
    }
    #[doc = "Checks if the value of the field is `MASTER_1`"]
    #[inline(always)]
    pub fn is_master_1(&self) -> bool {
        *self == MASTER_A::MASTER_1
    }
}
#[doc = "Write proxy for field `MASTER`"]
pub struct MASTER_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASTER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn master_0(self) -> &'a mut W {
        self.variant(MASTER_A::MASTER_0)
    }
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn master_1(self) -> &'a mut W {
        self.variant(MASTER_A::MASTER_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Sample Point\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLE_A {
    #[doc = "0: Input data is sampled on SCK edge"]
    SAMPLE_0 = 0,
    #[doc = "1: Input data is sampled on delayed SCK edge"]
    SAMPLE_1 = 1,
}
impl From<SAMPLE_A> for bool {
    #[inline(always)]
    fn from(variant: SAMPLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAMPLE`"]
pub type SAMPLE_R = crate::R<bool, SAMPLE_A>;
impl SAMPLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMPLE_A {
        match self.bits {
            false => SAMPLE_A::SAMPLE_0,
            true => SAMPLE_A::SAMPLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAMPLE_0`"]
    #[inline(always)]
    pub fn is_sample_0(&self) -> bool {
        *self == SAMPLE_A::SAMPLE_0
    }
    #[doc = "Checks if the value of the field is `SAMPLE_1`"]
    #[inline(always)]
    pub fn is_sample_1(&self) -> bool {
        *self == SAMPLE_A::SAMPLE_1
    }
}
#[doc = "Write proxy for field `SAMPLE`"]
pub struct SAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input data is sampled on SCK edge"]
    #[inline(always)]
    pub fn sample_0(self) -> &'a mut W {
        self.variant(SAMPLE_A::SAMPLE_0)
    }
    #[doc = "Input data is sampled on delayed SCK edge"]
    #[inline(always)]
    pub fn sample_1(self) -> &'a mut W {
        self.variant(SAMPLE_A::SAMPLE_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Automatic PCS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOPCS_A {
    #[doc = "0: Automatic PCS generation is disabled"]
    AUTOPCS_0 = 0,
    #[doc = "1: Automatic PCS generation is enabled"]
    AUTOPCS_1 = 1,
}
impl From<AUTOPCS_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOPCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUTOPCS`"]
pub type AUTOPCS_R = crate::R<bool, AUTOPCS_A>;
impl AUTOPCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOPCS_A {
        match self.bits {
            false => AUTOPCS_A::AUTOPCS_0,
            true => AUTOPCS_A::AUTOPCS_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUTOPCS_0`"]
    #[inline(always)]
    pub fn is_autopcs_0(&self) -> bool {
        *self == AUTOPCS_A::AUTOPCS_0
    }
    #[doc = "Checks if the value of the field is `AUTOPCS_1`"]
    #[inline(always)]
    pub fn is_autopcs_1(&self) -> bool {
        *self == AUTOPCS_A::AUTOPCS_1
    }
}
#[doc = "Write proxy for field `AUTOPCS`"]
pub struct AUTOPCS_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOPCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTOPCS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Automatic PCS generation is disabled"]
    #[inline(always)]
    pub fn autopcs_0(self) -> &'a mut W {
        self.variant(AUTOPCS_A::AUTOPCS_0)
    }
    #[doc = "Automatic PCS generation is enabled"]
    #[inline(always)]
    pub fn autopcs_1(self) -> &'a mut W {
        self.variant(AUTOPCS_A::AUTOPCS_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "No Stall\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOSTALL_A {
    #[doc = "0: Transfers will stall when the transmit FIFO is empty or the receive FIFO is full"]
    NOSTALL_0 = 0,
    #[doc = "1: Transfers will not stall, allowing transmit FIFO underruns or receive FIFO overruns to occur"]
    NOSTALL_1 = 1,
}
impl From<NOSTALL_A> for bool {
    #[inline(always)]
    fn from(variant: NOSTALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NOSTALL`"]
pub type NOSTALL_R = crate::R<bool, NOSTALL_A>;
impl NOSTALL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOSTALL_A {
        match self.bits {
            false => NOSTALL_A::NOSTALL_0,
            true => NOSTALL_A::NOSTALL_1,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTALL_0`"]
    #[inline(always)]
    pub fn is_nostall_0(&self) -> bool {
        *self == NOSTALL_A::NOSTALL_0
    }
    #[doc = "Checks if the value of the field is `NOSTALL_1`"]
    #[inline(always)]
    pub fn is_nostall_1(&self) -> bool {
        *self == NOSTALL_A::NOSTALL_1
    }
}
#[doc = "Write proxy for field `NOSTALL`"]
pub struct NOSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> NOSTALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOSTALL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transfers will stall when the transmit FIFO is empty or the receive FIFO is full"]
    #[inline(always)]
    pub fn nostall_0(self) -> &'a mut W {
        self.variant(NOSTALL_A::NOSTALL_0)
    }
    #[doc = "Transfers will not stall, allowing transmit FIFO underruns or receive FIFO overruns to occur"]
    #[inline(always)]
    pub fn nostall_1(self) -> &'a mut W {
        self.variant(NOSTALL_A::NOSTALL_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Peripheral Chip Select Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCSPOL_A {
    #[doc = "0: The Peripheral Chip Select pin PCSx is active low"]
    PCSPOL_0 = 0,
    #[doc = "1: The Peripheral Chip Select pin PCSx is active high"]
    PCSPOL_1 = 1,
}
impl From<PCSPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: PCSPOL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCSPOL`"]
pub type PCSPOL_R = crate::R<u8, PCSPOL_A>;
impl PCSPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PCSPOL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PCSPOL_A::PCSPOL_0),
            1 => Val(PCSPOL_A::PCSPOL_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PCSPOL_0`"]
    #[inline(always)]
    pub fn is_pcspol_0(&self) -> bool {
        *self == PCSPOL_A::PCSPOL_0
    }
    #[doc = "Checks if the value of the field is `PCSPOL_1`"]
    #[inline(always)]
    pub fn is_pcspol_1(&self) -> bool {
        *self == PCSPOL_A::PCSPOL_1
    }
}
#[doc = "Write proxy for field `PCSPOL`"]
pub struct PCSPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCSPOL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The Peripheral Chip Select pin PCSx is active low"]
    #[inline(always)]
    pub fn pcspol_0(self) -> &'a mut W {
        self.variant(PCSPOL_A::PCSPOL_0)
    }
    #[doc = "The Peripheral Chip Select pin PCSx is active high"]
    #[inline(always)]
    pub fn pcspol_1(self) -> &'a mut W {
        self.variant(PCSPOL_A::PCSPOL_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Match Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MATCFG_A {
    #[doc = "0: Match is disabled"]
    MATCFG_0 = 0,
    #[doc = "2: 010b - Match is enabled, if 1st data word equals MATCH0 OR MATCH1, i.e., (1st data word = MATCH0 + MATCH1)"]
    MATCFG_2 = 2,
    #[doc = "3: 011b - Match is enabled, if any data word equals MATCH0 OR MATCH1, i.e., (any data word = MATCH0 + MATCH1)"]
    MATCFG_3 = 3,
    #[doc = "4: 100b - Match is enabled, if 1st data word equals MATCH0 AND 2nd data word equals MATCH1, i.e., \\[(1st data word = MATCH0) * (2nd data word = MATCH1)\\]"]
    MATCFG_4 = 4,
    #[doc = "5: 101b - Match is enabled, if any data word equals MATCH0 AND the next data word equals MATCH1, i.e., \\[(any data word = MATCH0) * (next data word = MATCH1)\\]"]
    MATCFG_5 = 5,
    #[doc = "6: 110b - Match is enabled, if (1st data word AND MATCH1) equals (MATCH0 AND MATCH1), i.e., \\[(1st data word * MATCH1) = (MATCH0 * MATCH1)\\]"]
    MATCFG_6 = 6,
    #[doc = "7: 111b - Match is enabled, if (any data word AND MATCH1) equals (MATCH0 AND MATCH1), i.e., \\[(any data word * MATCH1) = (MATCH0 * MATCH1)\\]"]
    MATCFG_7 = 7,
}
impl From<MATCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: MATCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MATCFG`"]
pub type MATCFG_R = crate::R<u8, MATCFG_A>;
impl MATCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MATCFG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MATCFG_A::MATCFG_0),
            2 => Val(MATCFG_A::MATCFG_2),
            3 => Val(MATCFG_A::MATCFG_3),
            4 => Val(MATCFG_A::MATCFG_4),
            5 => Val(MATCFG_A::MATCFG_5),
            6 => Val(MATCFG_A::MATCFG_6),
            7 => Val(MATCFG_A::MATCFG_7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MATCFG_0`"]
    #[inline(always)]
    pub fn is_matcfg_0(&self) -> bool {
        *self == MATCFG_A::MATCFG_0
    }
    #[doc = "Checks if the value of the field is `MATCFG_2`"]
    #[inline(always)]
    pub fn is_matcfg_2(&self) -> bool {
        *self == MATCFG_A::MATCFG_2
    }
    #[doc = "Checks if the value of the field is `MATCFG_3`"]
    #[inline(always)]
    pub fn is_matcfg_3(&self) -> bool {
        *self == MATCFG_A::MATCFG_3
    }
    #[doc = "Checks if the value of the field is `MATCFG_4`"]
    #[inline(always)]
    pub fn is_matcfg_4(&self) -> bool {
        *self == MATCFG_A::MATCFG_4
    }
    #[doc = "Checks if the value of the field is `MATCFG_5`"]
    #[inline(always)]
    pub fn is_matcfg_5(&self) -> bool {
        *self == MATCFG_A::MATCFG_5
    }
    #[doc = "Checks if the value of the field is `MATCFG_6`"]
    #[inline(always)]
    pub fn is_matcfg_6(&self) -> bool {
        *self == MATCFG_A::MATCFG_6
    }
    #[doc = "Checks if the value of the field is `MATCFG_7`"]
    #[inline(always)]
    pub fn is_matcfg_7(&self) -> bool {
        *self == MATCFG_A::MATCFG_7
    }
}
#[doc = "Write proxy for field `MATCFG`"]
pub struct MATCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MATCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Match is disabled"]
    #[inline(always)]
    pub fn matcfg_0(self) -> &'a mut W {
        self.variant(MATCFG_A::MATCFG_0)
    }
    #[doc = "010b - Match is enabled, if 1st data word equals MATCH0 OR MATCH1, i.e., (1st data word = MATCH0 + MATCH1)"]
    #[inline(always)]
    pub fn matcfg_2(self) -> &'a mut W {
        self.variant(MATCFG_A::MATCFG_2)
    }
    #[doc = "011b - Match is enabled, if any data word equals MATCH0 OR MATCH1, i.e., (any data word = MATCH0 + MATCH1)"]
    #[inline(always)]
    pub fn matcfg_3(self) -> &'a mut W {
        self.variant(MATCFG_A::MATCFG_3)
    }
    #[doc = "100b - Match is enabled, if 1st data word equals MATCH0 AND 2nd data word equals MATCH1, i.e., \\[(1st data word = MATCH0) * (2nd data word = MATCH1)\\]"]
    #[inline(always)]
    pub fn matcfg_4(self) -> &'a mut W {
        self.variant(MATCFG_A::MATCFG_4)
    }
    #[doc = "101b - Match is enabled, if any data word equals MATCH0 AND the next data word equals MATCH1, i.e., \\[(any data word = MATCH0) * (next data word = MATCH1)\\]"]
    #[inline(always)]
    pub fn matcfg_5(self) -> &'a mut W {
        self.variant(MATCFG_A::MATCFG_5)
    }
    #[doc = "110b - Match is enabled, if (1st data word AND MATCH1) equals (MATCH0 AND MATCH1), i.e., \\[(1st data word * MATCH1) = (MATCH0 * MATCH1)\\]"]
    #[inline(always)]
    pub fn matcfg_6(self) -> &'a mut W {
        self.variant(MATCFG_A::MATCFG_6)
    }
    #[doc = "111b - Match is enabled, if (any data word AND MATCH1) equals (MATCH0 AND MATCH1), i.e., \\[(any data word * MATCH1) = (MATCH0 * MATCH1)\\]"]
    #[inline(always)]
    pub fn matcfg_7(self) -> &'a mut W {
        self.variant(MATCFG_A::MATCFG_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Pin Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PINCFG_A {
    #[doc = "0: SIN is used for input data and SOUT is used for output data"]
    PINCFG_0 = 0,
    #[doc = "1: SIN is used for both input and output data"]
    PINCFG_1 = 1,
    #[doc = "2: SOUT is used for both input and output data"]
    PINCFG_2 = 2,
    #[doc = "3: SOUT is used for input data and SIN is used for output data"]
    PINCFG_3 = 3,
}
impl From<PINCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: PINCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PINCFG`"]
pub type PINCFG_R = crate::R<u8, PINCFG_A>;
impl PINCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINCFG_A {
        match self.bits {
            0 => PINCFG_A::PINCFG_0,
            1 => PINCFG_A::PINCFG_1,
            2 => PINCFG_A::PINCFG_2,
            3 => PINCFG_A::PINCFG_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PINCFG_0`"]
    #[inline(always)]
    pub fn is_pincfg_0(&self) -> bool {
        *self == PINCFG_A::PINCFG_0
    }
    #[doc = "Checks if the value of the field is `PINCFG_1`"]
    #[inline(always)]
    pub fn is_pincfg_1(&self) -> bool {
        *self == PINCFG_A::PINCFG_1
    }
    #[doc = "Checks if the value of the field is `PINCFG_2`"]
    #[inline(always)]
    pub fn is_pincfg_2(&self) -> bool {
        *self == PINCFG_A::PINCFG_2
    }
    #[doc = "Checks if the value of the field is `PINCFG_3`"]
    #[inline(always)]
    pub fn is_pincfg_3(&self) -> bool {
        *self == PINCFG_A::PINCFG_3
    }
}
#[doc = "Write proxy for field `PINCFG`"]
pub struct PINCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "SIN is used for input data and SOUT is used for output data"]
    #[inline(always)]
    pub fn pincfg_0(self) -> &'a mut W {
        self.variant(PINCFG_A::PINCFG_0)
    }
    #[doc = "SIN is used for both input and output data"]
    #[inline(always)]
    pub fn pincfg_1(self) -> &'a mut W {
        self.variant(PINCFG_A::PINCFG_1)
    }
    #[doc = "SOUT is used for both input and output data"]
    #[inline(always)]
    pub fn pincfg_2(self) -> &'a mut W {
        self.variant(PINCFG_A::PINCFG_2)
    }
    #[doc = "SOUT is used for input data and SIN is used for output data"]
    #[inline(always)]
    pub fn pincfg_3(self) -> &'a mut W {
        self.variant(PINCFG_A::PINCFG_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Output Config\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCFG_A {
    #[doc = "0: Output data retains last value when chip select is negated"]
    OUTCFG_0 = 0,
    #[doc = "1: Output data is tristated when chip select is negated"]
    OUTCFG_1 = 1,
}
impl From<OUTCFG_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTCFG`"]
pub type OUTCFG_R = crate::R<bool, OUTCFG_A>;
impl OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCFG_A {
        match self.bits {
            false => OUTCFG_A::OUTCFG_0,
            true => OUTCFG_A::OUTCFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `OUTCFG_0`"]
    #[inline(always)]
    pub fn is_outcfg_0(&self) -> bool {
        *self == OUTCFG_A::OUTCFG_0
    }
    #[doc = "Checks if the value of the field is `OUTCFG_1`"]
    #[inline(always)]
    pub fn is_outcfg_1(&self) -> bool {
        *self == OUTCFG_A::OUTCFG_1
    }
}
#[doc = "Write proxy for field `OUTCFG`"]
pub struct OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output data retains last value when chip select is negated"]
    #[inline(always)]
    pub fn outcfg_0(self) -> &'a mut W {
        self.variant(OUTCFG_A::OUTCFG_0)
    }
    #[doc = "Output data is tristated when chip select is negated"]
    #[inline(always)]
    pub fn outcfg_1(self) -> &'a mut W {
        self.variant(OUTCFG_A::OUTCFG_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Peripheral Chip Select Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCSCFG_A {
    #[doc = "0: PCS\\[3:2\\]
are enabled"]
    PCSCFG_0 = 0,
    #[doc = "1: PCS\\[3:2\\]
are disabled"]
    PCSCFG_1 = 1,
}
impl From<PCSCFG_A> for bool {
    #[inline(always)]
    fn from(variant: PCSCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PCSCFG`"]
pub type PCSCFG_R = crate::R<bool, PCSCFG_A>;
impl PCSCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCSCFG_A {
        match self.bits {
            false => PCSCFG_A::PCSCFG_0,
            true => PCSCFG_A::PCSCFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `PCSCFG_0`"]
    #[inline(always)]
    pub fn is_pcscfg_0(&self) -> bool {
        *self == PCSCFG_A::PCSCFG_0
    }
    #[doc = "Checks if the value of the field is `PCSCFG_1`"]
    #[inline(always)]
    pub fn is_pcscfg_1(&self) -> bool {
        *self == PCSCFG_A::PCSCFG_1
    }
}
#[doc = "Write proxy for field `PCSCFG`"]
pub struct PCSCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCSCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PCS\\[3:2\\]
are enabled"]
    #[inline(always)]
    pub fn pcscfg_0(self) -> &'a mut W {
        self.variant(PCSCFG_A::PCSCFG_0)
    }
    #[doc = "PCS\\[3:2\\]
are disabled"]
    #[inline(always)]
    pub fn pcscfg_1(self) -> &'a mut W {
        self.variant(PCSCFG_A::PCSCFG_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Master Mode"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sample Point"]
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Automatic PCS"]
    #[inline(always)]
    pub fn autopcs(&self) -> AUTOPCS_R {
        AUTOPCS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - No Stall"]
    #[inline(always)]
    pub fn nostall(&self) -> NOSTALL_R {
        NOSTALL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Peripheral Chip Select Polarity"]
    #[inline(always)]
    pub fn pcspol(&self) -> PCSPOL_R {
        PCSPOL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - Match Configuration"]
    #[inline(always)]
    pub fn matcfg(&self) -> MATCFG_R {
        MATCFG_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 24:25 - Pin Configuration"]
    #[inline(always)]
    pub fn pincfg(&self) -> PINCFG_R {
        PINCFG_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Output Config"]
    #[inline(always)]
    pub fn outcfg(&self) -> OUTCFG_R {
        OUTCFG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Peripheral Chip Select Configuration"]
    #[inline(always)]
    pub fn pcscfg(&self) -> PCSCFG_R {
        PCSCFG_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Mode"]
    #[inline(always)]
    pub fn master(&mut self) -> MASTER_W {
        MASTER_W { w: self }
    }
    #[doc = "Bit 1 - Sample Point"]
    #[inline(always)]
    pub fn sample(&mut self) -> SAMPLE_W {
        SAMPLE_W { w: self }
    }
    #[doc = "Bit 2 - Automatic PCS"]
    #[inline(always)]
    pub fn autopcs(&mut self) -> AUTOPCS_W {
        AUTOPCS_W { w: self }
    }
    #[doc = "Bit 3 - No Stall"]
    #[inline(always)]
    pub fn nostall(&mut self) -> NOSTALL_W {
        NOSTALL_W { w: self }
    }
    #[doc = "Bits 8:11 - Peripheral Chip Select Polarity"]
    #[inline(always)]
    pub fn pcspol(&mut self) -> PCSPOL_W {
        PCSPOL_W { w: self }
    }
    #[doc = "Bits 16:18 - Match Configuration"]
    #[inline(always)]
    pub fn matcfg(&mut self) -> MATCFG_W {
        MATCFG_W { w: self }
    }
    #[doc = "Bits 24:25 - Pin Configuration"]
    #[inline(always)]
    pub fn pincfg(&mut self) -> PINCFG_W {
        PINCFG_W { w: self }
    }
    #[doc = "Bit 26 - Output Config"]
    #[inline(always)]
    pub fn outcfg(&mut self) -> OUTCFG_W {
        OUTCFG_W { w: self }
    }
    #[doc = "Bit 27 - Peripheral Chip Select Configuration"]
    #[inline(always)]
    pub fn pcscfg(&mut self) -> PCSCFG_W {
        PCSCFG_W { w: self }
    }
}
