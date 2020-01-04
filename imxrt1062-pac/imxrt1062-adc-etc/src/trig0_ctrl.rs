#[doc = "Reader of register TRIG0_CTRL"]
pub type R = crate::R<u32, super::TRIG0_CTRL>;
#[doc = "Writer for register TRIG0_CTRL"]
pub type W = crate::W<u32, super::TRIG0_CTRL>;
#[doc = "Register TRIG0_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::TRIG0_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SW_TRIG`"]
pub type SW_TRIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_TRIG`"]
pub struct SW_TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_TRIG_W<'a> {
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
#[doc = "Reader of field `TRIG_MODE`"]
pub type TRIG_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG_MODE`"]
pub struct TRIG_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TRIG_CHAIN`"]
pub type TRIG_CHAIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIG_CHAIN`"]
pub struct TRIG_CHAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG_CHAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `TRIG_PRIORITY`"]
pub type TRIG_PRIORITY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIG_PRIORITY`"]
pub struct TRIG_PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG_PRIORITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `SYNC_MODE`"]
pub type SYNC_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNC_MODE`"]
pub struct SYNC_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_MODE_W<'a> {
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
    #[doc = "Bit 0 - Software write 1 as the TRIGGER. This register is self-clearing."]
    #[inline(always)]
    pub fn sw_trig(&self) -> SW_TRIG_R {
        SW_TRIG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - TRIG mode register. 1'b0: hardware trigger. 1'b1: software trigger."]
    #[inline(always)]
    pub fn trig_mode(&self) -> TRIG_MODE_R {
        TRIG_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - TRIG chain length to the ADC. 0: Trig length is 1; ... 7: Trig length is 8;"]
    #[inline(always)]
    pub fn trig_chain(&self) -> TRIG_CHAIN_R {
        TRIG_CHAIN_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - External trigger priority, 7 is highest, 0 is lowest ."]
    #[inline(always)]
    pub fn trig_priority(&self) -> TRIG_PRIORITY_R {
        TRIG_PRIORITY_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 16 - TRIG mode control . 1'b0: Disable sync mode; 1'b1: Enable sync mode"]
    #[inline(always)]
    pub fn sync_mode(&self) -> SYNC_MODE_R {
        SYNC_MODE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software write 1 as the TRIGGER. This register is self-clearing."]
    #[inline(always)]
    pub fn sw_trig(&mut self) -> SW_TRIG_W {
        SW_TRIG_W { w: self }
    }
    #[doc = "Bit 4 - TRIG mode register. 1'b0: hardware trigger. 1'b1: software trigger."]
    #[inline(always)]
    pub fn trig_mode(&mut self) -> TRIG_MODE_W {
        TRIG_MODE_W { w: self }
    }
    #[doc = "Bits 8:10 - TRIG chain length to the ADC. 0: Trig length is 1; ... 7: Trig length is 8;"]
    #[inline(always)]
    pub fn trig_chain(&mut self) -> TRIG_CHAIN_W {
        TRIG_CHAIN_W { w: self }
    }
    #[doc = "Bits 12:14 - External trigger priority, 7 is highest, 0 is lowest ."]
    #[inline(always)]
    pub fn trig_priority(&mut self) -> TRIG_PRIORITY_W {
        TRIG_PRIORITY_W { w: self }
    }
    #[doc = "Bit 16 - TRIG mode control . 1'b0: Disable sync mode; 1'b1: Enable sync mode"]
    #[inline(always)]
    pub fn sync_mode(&mut self) -> SYNC_MODE_W {
        SYNC_MODE_W { w: self }
    }
}
