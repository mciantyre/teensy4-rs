#[doc = "Reader of register PIGEONCTRL1"]
pub type R = crate::R<u32, super::PIGEONCTRL1>;
#[doc = "Writer for register PIGEONCTRL1"]
pub type W = crate::W<u32, super::PIGEONCTRL1>;
#[doc = "Register PIGEONCTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PIGEONCTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRAME_CNT_PERIOD`"]
pub type FRAME_CNT_PERIOD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FRAME_CNT_PERIOD`"]
pub struct FRAME_CNT_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_CNT_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `FRAME_CNT_CYCLES`"]
pub type FRAME_CNT_CYCLES_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FRAME_CNT_CYCLES`"]
pub struct FRAME_CNT_CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_CNT_CYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Period of frame counter"]
    #[inline(always)]
    pub fn frame_cnt_period(&self) -> FRAME_CNT_PERIOD_R {
        FRAME_CNT_PERIOD_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Max cycles of frame counter"]
    #[inline(always)]
    pub fn frame_cnt_cycles(&self) -> FRAME_CNT_CYCLES_R {
        FRAME_CNT_CYCLES_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Period of frame counter"]
    #[inline(always)]
    pub fn frame_cnt_period(&mut self) -> FRAME_CNT_PERIOD_W {
        FRAME_CNT_PERIOD_W { w: self }
    }
    #[doc = "Bits 16:27 - Max cycles of frame counter"]
    #[inline(always)]
    pub fn frame_cnt_cycles(&mut self) -> FRAME_CNT_CYCLES_W {
        FRAME_CNT_CYCLES_W { w: self }
    }
}
