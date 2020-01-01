#[doc = "Reader of register TUNING_CTRL"]
pub type R = crate::R<u32, super::TUNING_CTRL>;
#[doc = "Writer for register TUNING_CTRL"]
pub type W = crate::W<u32, super::TUNING_CTRL>;
#[doc = "Register TUNING_CTRL `reset()`'s with value 0x0021_2800"]
impl crate::ResetValue for super::TUNING_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0021_2800
    }
}
#[doc = "Reader of field `TUNING_START_TAP`"]
pub type TUNING_START_TAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TUNING_START_TAP`"]
pub struct TUNING_START_TAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNING_START_TAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `TUNING_COUNTER`"]
pub type TUNING_COUNTER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TUNING_COUNTER`"]
pub struct TUNING_COUNTER_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNING_COUNTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `TUNING_STEP`"]
pub type TUNING_STEP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TUNING_STEP`"]
pub struct TUNING_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNING_STEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `TUNING_WINDOW`"]
pub type TUNING_WINDOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TUNING_WINDOW`"]
pub struct TUNING_WINDOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNING_WINDOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `STD_TUNING_EN`"]
pub type STD_TUNING_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STD_TUNING_EN`"]
pub struct STD_TUNING_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> STD_TUNING_EN_W<'a> {
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
impl R {
    #[doc = "Bits 0:7 - TUNING_START_TAP"]
    #[inline(always)]
    pub fn tuning_start_tap(&self) -> TUNING_START_TAP_R {
        TUNING_START_TAP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - TUNING_COUNTER"]
    #[inline(always)]
    pub fn tuning_counter(&self) -> TUNING_COUNTER_R {
        TUNING_COUNTER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - TUNING_STEP"]
    #[inline(always)]
    pub fn tuning_step(&self) -> TUNING_STEP_R {
        TUNING_STEP_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - TUNING_WINDOW"]
    #[inline(always)]
    pub fn tuning_window(&self) -> TUNING_WINDOW_R {
        TUNING_WINDOW_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 24 - STD_TUNING_EN"]
    #[inline(always)]
    pub fn std_tuning_en(&self) -> STD_TUNING_EN_R {
        STD_TUNING_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - TUNING_START_TAP"]
    #[inline(always)]
    pub fn tuning_start_tap(&mut self) -> TUNING_START_TAP_W {
        TUNING_START_TAP_W { w: self }
    }
    #[doc = "Bits 8:15 - TUNING_COUNTER"]
    #[inline(always)]
    pub fn tuning_counter(&mut self) -> TUNING_COUNTER_W {
        TUNING_COUNTER_W { w: self }
    }
    #[doc = "Bits 16:18 - TUNING_STEP"]
    #[inline(always)]
    pub fn tuning_step(&mut self) -> TUNING_STEP_W {
        TUNING_STEP_W { w: self }
    }
    #[doc = "Bits 20:22 - TUNING_WINDOW"]
    #[inline(always)]
    pub fn tuning_window(&mut self) -> TUNING_WINDOW_W {
        TUNING_WINDOW_W { w: self }
    }
    #[doc = "Bit 24 - STD_TUNING_EN"]
    #[inline(always)]
    pub fn std_tuning_en(&mut self) -> STD_TUNING_EN_W {
        STD_TUNING_EN_W { w: self }
    }
}
