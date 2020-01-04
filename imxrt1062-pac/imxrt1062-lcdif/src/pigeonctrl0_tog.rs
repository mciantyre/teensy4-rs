#[doc = "Reader of register PIGEONCTRL0_TOG"]
pub type R = crate::R<u32, super::PIGEONCTRL0_TOG>;
#[doc = "Writer for register PIGEONCTRL0_TOG"]
pub type W = crate::W<u32, super::PIGEONCTRL0_TOG>;
#[doc = "Register PIGEONCTRL0_TOG `reset()`'s with value 0"]
impl crate::ResetValue for super::PIGEONCTRL0_TOG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FD_PERIOD`"]
pub type FD_PERIOD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FD_PERIOD`"]
pub struct FD_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> FD_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `LD_PERIOD`"]
pub type LD_PERIOD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LD_PERIOD`"]
pub struct LD_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> LD_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Period of line counter during FD phase"]
    #[inline(always)]
    pub fn fd_period(&self) -> FD_PERIOD_R {
        FD_PERIOD_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Period of pclk counter during LD phase"]
    #[inline(always)]
    pub fn ld_period(&self) -> LD_PERIOD_R {
        LD_PERIOD_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Period of line counter during FD phase"]
    #[inline(always)]
    pub fn fd_period(&mut self) -> FD_PERIOD_W {
        FD_PERIOD_W { w: self }
    }
    #[doc = "Bits 16:27 - Period of pclk counter during LD phase"]
    #[inline(always)]
    pub fn ld_period(&mut self) -> LD_PERIOD_W {
        LD_PERIOD_W { w: self }
    }
}
