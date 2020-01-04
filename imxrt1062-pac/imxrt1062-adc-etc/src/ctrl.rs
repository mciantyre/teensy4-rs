#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0xc000_0000"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc000_0000
    }
}
#[doc = "Reader of field `TRIG_ENABLE`"]
pub type TRIG_ENABLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIG_ENABLE`"]
pub struct TRIG_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `EXT0_TRIG_ENABLE`"]
pub type EXT0_TRIG_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXT0_TRIG_ENABLE`"]
pub struct EXT0_TRIG_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT0_TRIG_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `EXT0_TRIG_PRIORITY`"]
pub type EXT0_TRIG_PRIORITY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXT0_TRIG_PRIORITY`"]
pub struct EXT0_TRIG_PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT0_TRIG_PRIORITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Reader of field `EXT1_TRIG_ENABLE`"]
pub type EXT1_TRIG_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXT1_TRIG_ENABLE`"]
pub struct EXT1_TRIG_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT1_TRIG_ENABLE_W<'a> {
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
#[doc = "Reader of field `EXT1_TRIG_PRIORITY`"]
pub type EXT1_TRIG_PRIORITY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXT1_TRIG_PRIORITY`"]
pub struct EXT1_TRIG_PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT1_TRIG_PRIORITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Reader of field `PRE_DIVIDER`"]
pub type PRE_DIVIDER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRE_DIVIDER`"]
pub struct PRE_DIVIDER_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_DIVIDER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DMA_MODE_SEL`"]
pub type DMA_MODE_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_MODE_SEL`"]
pub struct DMA_MODE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_MODE_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `TSC_BYPASS`"]
pub type TSC_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSC_BYPASS`"]
pub struct TSC_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> TSC_BYPASS_W<'a> {
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
#[doc = "Reader of field `SOFTRST`"]
pub type SOFTRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTRST`"]
pub struct SOFTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - TRIG enable register"]
    #[inline(always)]
    pub fn trig_enable(&self) -> TRIG_ENABLE_R {
        TRIG_ENABLE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - TSC0 TRIG enable register. 1'b1: enable external TSC0 trigger. 1'b0: disable external TSC0 trigger."]
    #[inline(always)]
    pub fn ext0_trig_enable(&self) -> EXT0_TRIG_ENABLE_R {
        EXT0_TRIG_ENABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:11 - External TSC0 trigger priority, 7 is Highest, 0 is lowest ."]
    #[inline(always)]
    pub fn ext0_trig_priority(&self) -> EXT0_TRIG_PRIORITY_R {
        EXT0_TRIG_PRIORITY_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bit 12 - TSC1 TRIG enable register. 1'b1: enable external TSC1 trigger. 1'b0: disable external TSC1 trigger."]
    #[inline(always)]
    pub fn ext1_trig_enable(&self) -> EXT1_TRIG_ENABLE_R {
        EXT1_TRIG_ENABLE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - External TSC1 trigger priority, 7 is Highest, 0 is lowest ."]
    #[inline(always)]
    pub fn ext1_trig_priority(&self) -> EXT1_TRIG_PRIORITY_R {
        EXT1_TRIG_PRIORITY_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 16:23 - Pre-divider for trig delay and interval ."]
    #[inline(always)]
    pub fn pre_divider(&self) -> PRE_DIVIDER_R {
        PRE_DIVIDER_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 29 - 1'b0: Trig DMA_REQ with latched signal, REQ will be cleared when ACK and source request cleared"]
    #[inline(always)]
    pub fn dma_mode_sel(&self) -> DMA_MODE_SEL_R {
        DMA_MODE_SEL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 1'b1: TSC is bypassed to ADC2. 1'b0: TSC not bypassed. To use ADC2, this bit should be cleared."]
    #[inline(always)]
    pub fn tsc_bypass(&self) -> TSC_BYPASS_R {
        TSC_BYPASS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Software reset, high active. When write 1 ,all logical will be reset."]
    #[inline(always)]
    pub fn softrst(&self) -> SOFTRST_R {
        SOFTRST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - TRIG enable register"]
    #[inline(always)]
    pub fn trig_enable(&mut self) -> TRIG_ENABLE_W {
        TRIG_ENABLE_W { w: self }
    }
    #[doc = "Bit 8 - TSC0 TRIG enable register. 1'b1: enable external TSC0 trigger. 1'b0: disable external TSC0 trigger."]
    #[inline(always)]
    pub fn ext0_trig_enable(&mut self) -> EXT0_TRIG_ENABLE_W {
        EXT0_TRIG_ENABLE_W { w: self }
    }
    #[doc = "Bits 9:11 - External TSC0 trigger priority, 7 is Highest, 0 is lowest ."]
    #[inline(always)]
    pub fn ext0_trig_priority(&mut self) -> EXT0_TRIG_PRIORITY_W {
        EXT0_TRIG_PRIORITY_W { w: self }
    }
    #[doc = "Bit 12 - TSC1 TRIG enable register. 1'b1: enable external TSC1 trigger. 1'b0: disable external TSC1 trigger."]
    #[inline(always)]
    pub fn ext1_trig_enable(&mut self) -> EXT1_TRIG_ENABLE_W {
        EXT1_TRIG_ENABLE_W { w: self }
    }
    #[doc = "Bits 13:15 - External TSC1 trigger priority, 7 is Highest, 0 is lowest ."]
    #[inline(always)]
    pub fn ext1_trig_priority(&mut self) -> EXT1_TRIG_PRIORITY_W {
        EXT1_TRIG_PRIORITY_W { w: self }
    }
    #[doc = "Bits 16:23 - Pre-divider for trig delay and interval ."]
    #[inline(always)]
    pub fn pre_divider(&mut self) -> PRE_DIVIDER_W {
        PRE_DIVIDER_W { w: self }
    }
    #[doc = "Bit 29 - 1'b0: Trig DMA_REQ with latched signal, REQ will be cleared when ACK and source request cleared"]
    #[inline(always)]
    pub fn dma_mode_sel(&mut self) -> DMA_MODE_SEL_W {
        DMA_MODE_SEL_W { w: self }
    }
    #[doc = "Bit 30 - 1'b1: TSC is bypassed to ADC2. 1'b0: TSC not bypassed. To use ADC2, this bit should be cleared."]
    #[inline(always)]
    pub fn tsc_bypass(&mut self) -> TSC_BYPASS_W {
        TSC_BYPASS_W { w: self }
    }
    #[doc = "Bit 31 - Software reset, high active. When write 1 ,all logical will be reset."]
    #[inline(always)]
    pub fn softrst(&mut self) -> SOFTRST_W {
        SOFTRST_W { w: self }
    }
}
