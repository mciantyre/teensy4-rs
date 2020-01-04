#[doc = "Reader of register USB1_MISC_CLR"]
pub type R = crate::R<u32, super::USB1_MISC_CLR>;
#[doc = "Writer for register USB1_MISC_CLR"]
pub type W = crate::W<u32, super::USB1_MISC_CLR>;
#[doc = "Register USB1_MISC_CLR `reset()`'s with value 0x02"]
impl crate::ResetValue for super::USB1_MISC_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `HS_USE_EXTERNAL_R`"]
pub type HS_USE_EXTERNAL_R_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HS_USE_EXTERNAL_R`"]
pub struct HS_USE_EXTERNAL_R_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_USE_EXTERNAL_R_W<'a> {
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
#[doc = "Reader of field `EN_DEGLITCH`"]
pub type EN_DEGLITCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_DEGLITCH`"]
pub struct EN_DEGLITCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_DEGLITCH_W<'a> {
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
#[doc = "Reader of field `EN_CLK_UTMI`"]
pub type EN_CLK_UTMI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_CLK_UTMI`"]
pub struct EN_CLK_UTMI_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_CLK_UTMI_W<'a> {
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
    #[doc = "Bit 0 - Use external resistor to generate the current bias for the high speed transmitter"]
    #[inline(always)]
    pub fn hs_use_external_r(&self) -> HS_USE_EXTERNAL_R_R {
        HS_USE_EXTERNAL_R_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable the deglitching circuit of the USB PLL output."]
    #[inline(always)]
    pub fn en_deglitch(&self) -> EN_DEGLITCH_R {
        EN_DEGLITCH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enables the clk to the UTMI block."]
    #[inline(always)]
    pub fn en_clk_utmi(&self) -> EN_CLK_UTMI_R {
        EN_CLK_UTMI_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Use external resistor to generate the current bias for the high speed transmitter"]
    #[inline(always)]
    pub fn hs_use_external_r(&mut self) -> HS_USE_EXTERNAL_R_W {
        HS_USE_EXTERNAL_R_W { w: self }
    }
    #[doc = "Bit 1 - Enable the deglitching circuit of the USB PLL output."]
    #[inline(always)]
    pub fn en_deglitch(&mut self) -> EN_DEGLITCH_W {
        EN_DEGLITCH_W { w: self }
    }
    #[doc = "Bit 30 - Enables the clk to the UTMI block."]
    #[inline(always)]
    pub fn en_clk_utmi(&mut self) -> EN_CLK_UTMI_W {
        EN_CLK_UTMI_W { w: self }
    }
}
