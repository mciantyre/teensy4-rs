#[doc = "Reader of register MCTL"]
pub type R = crate::R<u32, super::MCTL>;
#[doc = "Writer for register MCTL"]
pub type W = crate::W<u32, super::MCTL>;
#[doc = "Register MCTL `reset()`'s with value 0x0001_2001"]
impl crate::ResetValue for super::MCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_2001
    }
}
#[doc = "Sample Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAMP_MODE_A {
    #[doc = "0: use Von Neumann data into both Entropy shifter and Statistical Checker"]
    SAMP_MODE_0 = 0,
    #[doc = "1: use raw data into both Entropy shifter and Statistical Checker"]
    SAMP_MODE_1 = 1,
    #[doc = "2: use Von Neumann data into Entropy shifter. Use raw data into Statistical Checker"]
    SAMP_MODE_2 = 2,
    #[doc = "3: undefined/reserved."]
    SAMP_MODE_3 = 3,
}
impl From<SAMP_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: SAMP_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SAMP_MODE`"]
pub type SAMP_MODE_R = crate::R<u8, SAMP_MODE_A>;
impl SAMP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMP_MODE_A {
        match self.bits {
            0 => SAMP_MODE_A::SAMP_MODE_0,
            1 => SAMP_MODE_A::SAMP_MODE_1,
            2 => SAMP_MODE_A::SAMP_MODE_2,
            3 => SAMP_MODE_A::SAMP_MODE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SAMP_MODE_0`"]
    #[inline(always)]
    pub fn is_samp_mode_0(&self) -> bool {
        *self == SAMP_MODE_A::SAMP_MODE_0
    }
    #[doc = "Checks if the value of the field is `SAMP_MODE_1`"]
    #[inline(always)]
    pub fn is_samp_mode_1(&self) -> bool {
        *self == SAMP_MODE_A::SAMP_MODE_1
    }
    #[doc = "Checks if the value of the field is `SAMP_MODE_2`"]
    #[inline(always)]
    pub fn is_samp_mode_2(&self) -> bool {
        *self == SAMP_MODE_A::SAMP_MODE_2
    }
    #[doc = "Checks if the value of the field is `SAMP_MODE_3`"]
    #[inline(always)]
    pub fn is_samp_mode_3(&self) -> bool {
        *self == SAMP_MODE_A::SAMP_MODE_3
    }
}
#[doc = "Write proxy for field `SAMP_MODE`"]
pub struct SAMP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMP_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "use Von Neumann data into both Entropy shifter and Statistical Checker"]
    #[inline(always)]
    pub fn samp_mode_0(self) -> &'a mut W {
        self.variant(SAMP_MODE_A::SAMP_MODE_0)
    }
    #[doc = "use raw data into both Entropy shifter and Statistical Checker"]
    #[inline(always)]
    pub fn samp_mode_1(self) -> &'a mut W {
        self.variant(SAMP_MODE_A::SAMP_MODE_1)
    }
    #[doc = "use Von Neumann data into Entropy shifter. Use raw data into Statistical Checker"]
    #[inline(always)]
    pub fn samp_mode_2(self) -> &'a mut W {
        self.variant(SAMP_MODE_A::SAMP_MODE_2)
    }
    #[doc = "undefined/reserved."]
    #[inline(always)]
    pub fn samp_mode_3(self) -> &'a mut W {
        self.variant(SAMP_MODE_A::SAMP_MODE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Oscillator Divide\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSC_DIV_A {
    #[doc = "0: use ring oscillator with no divide"]
    OSC_DIV_0 = 0,
    #[doc = "1: use ring oscillator divided-by-2"]
    OSC_DIV_1 = 1,
    #[doc = "2: use ring oscillator divided-by-4"]
    OSC_DIV_2 = 2,
    #[doc = "3: use ring oscillator divided-by-8"]
    OSC_DIV_3 = 3,
}
impl From<OSC_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: OSC_DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OSC_DIV`"]
pub type OSC_DIV_R = crate::R<u8, OSC_DIV_A>;
impl OSC_DIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSC_DIV_A {
        match self.bits {
            0 => OSC_DIV_A::OSC_DIV_0,
            1 => OSC_DIV_A::OSC_DIV_1,
            2 => OSC_DIV_A::OSC_DIV_2,
            3 => OSC_DIV_A::OSC_DIV_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OSC_DIV_0`"]
    #[inline(always)]
    pub fn is_osc_div_0(&self) -> bool {
        *self == OSC_DIV_A::OSC_DIV_0
    }
    #[doc = "Checks if the value of the field is `OSC_DIV_1`"]
    #[inline(always)]
    pub fn is_osc_div_1(&self) -> bool {
        *self == OSC_DIV_A::OSC_DIV_1
    }
    #[doc = "Checks if the value of the field is `OSC_DIV_2`"]
    #[inline(always)]
    pub fn is_osc_div_2(&self) -> bool {
        *self == OSC_DIV_A::OSC_DIV_2
    }
    #[doc = "Checks if the value of the field is `OSC_DIV_3`"]
    #[inline(always)]
    pub fn is_osc_div_3(&self) -> bool {
        *self == OSC_DIV_A::OSC_DIV_3
    }
}
#[doc = "Write proxy for field `OSC_DIV`"]
pub struct OSC_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSC_DIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "use ring oscillator with no divide"]
    #[inline(always)]
    pub fn osc_div_0(self) -> &'a mut W {
        self.variant(OSC_DIV_A::OSC_DIV_0)
    }
    #[doc = "use ring oscillator divided-by-2"]
    #[inline(always)]
    pub fn osc_div_1(self) -> &'a mut W {
        self.variant(OSC_DIV_A::OSC_DIV_1)
    }
    #[doc = "use ring oscillator divided-by-4"]
    #[inline(always)]
    pub fn osc_div_2(self) -> &'a mut W {
        self.variant(OSC_DIV_A::OSC_DIV_2)
    }
    #[doc = "use ring oscillator divided-by-8"]
    #[inline(always)]
    pub fn osc_div_3(self) -> &'a mut W {
        self.variant(OSC_DIV_A::OSC_DIV_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `UNUSED4`"]
pub type UNUSED4_R = crate::R<bool, bool>;
#[doc = "Reader of field `UNUSED5`"]
pub type UNUSED5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST_DEF`"]
pub struct RST_DEF_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_DEF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `FOR_SCLK`"]
pub type FOR_SCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FOR_SCLK`"]
pub struct FOR_SCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> FOR_SCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `FCT_FAIL`"]
pub type FCT_FAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `FCT_VAL`"]
pub type FCT_VAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENT_VAL`"]
pub type ENT_VAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TST_OUT`"]
pub type TST_OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERR`"]
pub type ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERR`"]
pub struct ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TSTOP_OK`"]
pub type TSTOP_OK_R = crate::R<bool, bool>;
#[doc = "Reader of field `LRUN_CONT`"]
pub type LRUN_CONT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LRUN_CONT`"]
pub struct LRUN_CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> LRUN_CONT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `PRGM`"]
pub type PRGM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRGM`"]
pub struct PRGM_W<'a> {
    w: &'a mut W,
}
impl<'a> PRGM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Sample Mode"]
    #[inline(always)]
    pub fn samp_mode(&self) -> SAMP_MODE_R {
        SAMP_MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Oscillator Divide"]
    #[inline(always)]
    pub fn osc_div(&self) -> OSC_DIV_R {
        OSC_DIV_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - This bit is unused. Always reads zero."]
    #[inline(always)]
    pub fn unused4(&self) -> UNUSED4_R {
        UNUSED4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit is unused. Always reads zero."]
    #[inline(always)]
    pub fn unused5(&self) -> UNUSED5_R {
        UNUSED5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Force System Clock"]
    #[inline(always)]
    pub fn for_sclk(&self) -> FOR_SCLK_R {
        FOR_SCLK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Read only: Frequency Count Fail"]
    #[inline(always)]
    pub fn fct_fail(&self) -> FCT_FAIL_R {
        FCT_FAIL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Read only: Frequency Count Valid. Indicates that a valid frequency count may be read from FRQCNT."]
    #[inline(always)]
    pub fn fct_val(&self) -> FCT_VAL_R {
        FCT_VAL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Read only: Entropy Valid"]
    #[inline(always)]
    pub fn ent_val(&self) -> ENT_VAL_R {
        ENT_VAL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Read only: Test point inside ring oscillator."]
    #[inline(always)]
    pub fn tst_out(&self) -> TST_OUT_R {
        TST_OUT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Read: Error status"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TRNG_OK_TO_STOP"]
    #[inline(always)]
    pub fn tstop_ok(&self) -> TSTOP_OK_R {
        TSTOP_OK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Long run count continues between entropy generations"]
    #[inline(always)]
    pub fn lrun_cont(&self) -> LRUN_CONT_R {
        LRUN_CONT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Programming Mode Select"]
    #[inline(always)]
    pub fn prgm(&self) -> PRGM_R {
        PRGM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sample Mode"]
    #[inline(always)]
    pub fn samp_mode(&mut self) -> SAMP_MODE_W {
        SAMP_MODE_W { w: self }
    }
    #[doc = "Bits 2:3 - Oscillator Divide"]
    #[inline(always)]
    pub fn osc_div(&mut self) -> OSC_DIV_W {
        OSC_DIV_W { w: self }
    }
    #[doc = "Bit 6 - Reset Defaults"]
    #[inline(always)]
    pub fn rst_def(&mut self) -> RST_DEF_W {
        RST_DEF_W { w: self }
    }
    #[doc = "Bit 7 - Force System Clock"]
    #[inline(always)]
    pub fn for_sclk(&mut self) -> FOR_SCLK_W {
        FOR_SCLK_W { w: self }
    }
    #[doc = "Bit 12 - Read: Error status"]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W {
        ERR_W { w: self }
    }
    #[doc = "Bit 14 - Long run count continues between entropy generations"]
    #[inline(always)]
    pub fn lrun_cont(&mut self) -> LRUN_CONT_W {
        LRUN_CONT_W { w: self }
    }
    #[doc = "Bit 16 - Programming Mode Select"]
    #[inline(always)]
    pub fn prgm(&mut self) -> PRGM_W {
        PRGM_W { w: self }
    }
}
