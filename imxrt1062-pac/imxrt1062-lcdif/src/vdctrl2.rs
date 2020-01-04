#[doc = "Reader of register VDCTRL2"]
pub type R = crate::R<u32, super::VDCTRL2>;
#[doc = "Writer for register VDCTRL2"]
pub type W = crate::W<u32, super::VDCTRL2>;
#[doc = "Register VDCTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::VDCTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HSYNC_PERIOD`"]
pub type HSYNC_PERIOD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HSYNC_PERIOD`"]
pub struct HSYNC_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> HSYNC_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
#[doc = "Reader of field `HSYNC_PULSE_WIDTH`"]
pub type HSYNC_PULSE_WIDTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HSYNC_PULSE_WIDTH`"]
pub struct HSYNC_PULSE_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> HSYNC_PULSE_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 18)) | (((value as u32) & 0x3fff) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Total number of DISPLAY CLOCK (pix_clk) cycles between two positive or two negative edges of the HSYNC signal"]
    #[inline(always)]
    pub fn hsync_period(&self) -> HSYNC_PERIOD_R {
        HSYNC_PERIOD_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:31 - Number of DISPLAY CLOCK (pix_clk) cycles for which HSYNC signal is active."]
    #[inline(always)]
    pub fn hsync_pulse_width(&self) -> HSYNC_PULSE_WIDTH_R {
        HSYNC_PULSE_WIDTH_R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:17 - Total number of DISPLAY CLOCK (pix_clk) cycles between two positive or two negative edges of the HSYNC signal"]
    #[inline(always)]
    pub fn hsync_period(&mut self) -> HSYNC_PERIOD_W {
        HSYNC_PERIOD_W { w: self }
    }
    #[doc = "Bits 18:31 - Number of DISPLAY CLOCK (pix_clk) cycles for which HSYNC signal is active."]
    #[inline(always)]
    pub fn hsync_pulse_width(&mut self) -> HSYNC_PULSE_WIDTH_W {
        HSYNC_PULSE_WIDTH_W { w: self }
    }
}
