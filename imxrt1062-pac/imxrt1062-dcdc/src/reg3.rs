#[doc = "Reader of register REG3"]
pub type R = crate::R<u32, super::REG3>;
#[doc = "Writer for register REG3"]
pub type W = crate::W<u32, super::REG3>;
#[doc = "Register REG3 `reset()`'s with value 0x010e"]
impl crate::ResetValue for super::REG3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x010e
    }
}
#[doc = "Reader of field `TRG`"]
pub type TRG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRG`"]
pub struct TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `TARGET_LP`"]
pub type TARGET_LP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TARGET_LP`"]
pub struct TARGET_LP_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGET_LP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `MINPWR_DC_HALFCLK`"]
pub type MINPWR_DC_HALFCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MINPWR_DC_HALFCLK`"]
pub struct MINPWR_DC_HALFCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> MINPWR_DC_HALFCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `MISC_DELAY_TIMING`"]
pub type MISC_DELAY_TIMING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MISC_DELAY_TIMING`"]
pub struct MISC_DELAY_TIMING_W<'a> {
    w: &'a mut W,
}
impl<'a> MISC_DELAY_TIMING_W<'a> {
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
#[doc = "Reader of field `MISC_DISABLEFET_LOGIC`"]
pub type MISC_DISABLEFET_LOGIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MISC_DISABLEFET_LOGIC`"]
pub struct MISC_DISABLEFET_LOGIC_W<'a> {
    w: &'a mut W,
}
impl<'a> MISC_DISABLEFET_LOGIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `DISABLE_STEP`"]
pub type DISABLE_STEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLE_STEP`"]
pub struct DISABLE_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_STEP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Target value of VDD_SOC, 25 mV each step 0x0: 0.8V 0xE: 1.15V 0x1F:1.575V"]
    #[inline(always)]
    pub fn trg(&self) -> TRG_R {
        TRG_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - Target value of standby (low power) mode 0x0: 0"]
    #[inline(always)]
    pub fn target_lp(&self) -> TARGET_LP_R {
        TARGET_LP_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 24 - Set DCDC clock to half freqeuncy for continuous mode"]
    #[inline(always)]
    pub fn minpwr_dc_halfclk(&self) -> MINPWR_DC_HALFCLK_R {
        MINPWR_DC_HALFCLK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Ajust delay to reduce ground noise"]
    #[inline(always)]
    pub fn misc_delay_timing(&self) -> MISC_DELAY_TIMING_R {
        MISC_DELAY_TIMING_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn misc_disablefet_logic(&self) -> MISC_DISABLEFET_LOGIC_R {
        MISC_DISABLEFET_LOGIC_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Disable stepping for the output VDD_SOC of DCDC"]
    #[inline(always)]
    pub fn disable_step(&self) -> DISABLE_STEP_R {
        DISABLE_STEP_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Target value of VDD_SOC, 25 mV each step 0x0: 0.8V 0xE: 1.15V 0x1F:1.575V"]
    #[inline(always)]
    pub fn trg(&mut self) -> TRG_W {
        TRG_W { w: self }
    }
    #[doc = "Bits 8:10 - Target value of standby (low power) mode 0x0: 0"]
    #[inline(always)]
    pub fn target_lp(&mut self) -> TARGET_LP_W {
        TARGET_LP_W { w: self }
    }
    #[doc = "Bit 24 - Set DCDC clock to half freqeuncy for continuous mode"]
    #[inline(always)]
    pub fn minpwr_dc_halfclk(&mut self) -> MINPWR_DC_HALFCLK_W {
        MINPWR_DC_HALFCLK_W { w: self }
    }
    #[doc = "Bit 27 - Ajust delay to reduce ground noise"]
    #[inline(always)]
    pub fn misc_delay_timing(&mut self) -> MISC_DELAY_TIMING_W {
        MISC_DELAY_TIMING_W { w: self }
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn misc_disablefet_logic(&mut self) -> MISC_DISABLEFET_LOGIC_W {
        MISC_DISABLEFET_LOGIC_W { w: self }
    }
    #[doc = "Bit 30 - Disable stepping for the output VDD_SOC of DCDC"]
    #[inline(always)]
    pub fn disable_step(&mut self) -> DISABLE_STEP_W {
        DISABLE_STEP_W { w: self }
    }
}
