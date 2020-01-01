#[doc = "Reader of register HPRTCMR"]
pub type R = crate::R<u32, super::HPRTCMR>;
#[doc = "Writer for register HPRTCMR"]
pub type W = crate::W<u32, super::HPRTCMR>;
#[doc = "Register HPRTCMR `reset()`'s with value 0"]
impl crate::ResetValue for super::HPRTCMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC`"]
pub type RTC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RTC`"]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - HP Real Time Counter The most-significant 15 bits of the RTC"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - HP Real Time Counter The most-significant 15 bits of the RTC"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
}
