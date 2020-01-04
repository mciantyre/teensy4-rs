#[doc = "Reader of register HPRTCLR"]
pub type R = crate::R<u32, super::HPRTCLR>;
#[doc = "Writer for register HPRTCLR"]
pub type W = crate::W<u32, super::HPRTCLR>;
#[doc = "Register HPRTCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::HPRTCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC`"]
pub type RTC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RTC`"]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - HP Real Time Counter least-significant 32 bits"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - HP Real Time Counter least-significant 32 bits"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
}
