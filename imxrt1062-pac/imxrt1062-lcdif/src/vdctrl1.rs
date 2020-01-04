#[doc = "Reader of register VDCTRL1"]
pub type R = crate::R<u32, super::VDCTRL1>;
#[doc = "Writer for register VDCTRL1"]
pub type W = crate::W<u32, super::VDCTRL1>;
#[doc = "Register VDCTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::VDCTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VSYNC_PERIOD`"]
pub type VSYNC_PERIOD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `VSYNC_PERIOD`"]
pub struct VSYNC_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> VSYNC_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Total number of units between two positive or two negative edges of the VSYNC signal"]
    #[inline(always)]
    pub fn vsync_period(&self) -> VSYNC_PERIOD_R {
        VSYNC_PERIOD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Total number of units between two positive or two negative edges of the VSYNC signal"]
    #[inline(always)]
    pub fn vsync_period(&mut self) -> VSYNC_PERIOD_W {
        VSYNC_PERIOD_W { w: self }
    }
}
